#![recursion_limit = "512"]

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde_json;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use syn;
use syn::{Ident, LitStr, TypePath};

use serlo_he_spec_meta::{Attribute, Multiplicity, Plugin, Specification};

mod serde;
mod util;

use crate::util::{syn_identifier_from_locator};

#[proc_macro]
pub fn plugin_spec(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = PathBuf::from(
        syn::parse::<LitStr>(input)
            .expect("could not parse input as string!")
            .value(),
    );
    let mut out = TokenStream::new();

    let spec = read_spec(&path);
    for plugin in &spec.plugins {
        out.extend(impl_plugin_struct(&plugin))
    }
    out.extend(impl_plugins_enum(&spec));
    out.extend(crate::serde::impl_serde(&spec.plugins));
    out.into()
}

fn impl_attribute(attribute: &Attribute) -> TokenStream {
    let ident = Ident::new(&attribute.identifier, Span::call_site());
    let content_type: TypePath = syn::parse_str(&attribute.content_type)
        .expect("could not parse attribute type:");

    let attr_type = match attribute.multiplicity {
        Multiplicity::Optional => quote! {
            Option<#content_type>
        },
        Multiplicity::Once => quote! {
            #content_type
        },
        Multiplicity::Arbitrary => quote! {
            Vec<#content_type>
        },
        Multiplicity::MinOnce => quote! {
            Vec<#content_type>
        },
    };
    quote! {
        pub #ident: #attr_type
    }
}

fn impl_plugins_enum(spec: &Specification) -> TokenStream {
    let identifier_vec = spec
        .plugins
        .iter()
        .map(|p| syn_identifier_from_locator(&p.identifier.name))
        .collect::<Vec<Ident>>();
    let identifiers = &identifier_vec;
    let identifiers2 = &identifier_vec;
    let descriptions = spec
        .plugins
        .iter()
        .map(|p| LitStr::new(&p.description, Span::call_site()));
    let serial_spec = LitStr::new(
        &serde_json::to_string(&spec).expect("could not serialize plugin spec!"),
        Span::call_site(),
    );
    quote! {
        /// Common behaviour for plugins.
        pub trait Plugin {
            /// Specification of this plugin.
            fn specification() -> serlo_he_spec_meta::Plugin;

            /// Unique instance id.
            fn uuid(&self) -> &Uuid;

            /// Plugin identifier as defined in specification.
            fn identifier() -> serlo_he_spec_meta::Identifier;
        }

        /// The specified plugins.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Plugins {
            #(
                #[doc = #descriptions]
                #identifiers(#identifiers2)
            ),*
        }

        /// increments the node part of a uuid by one.
        /// FIXME: overflows
        fn inc_uuid(id: &Uuid) -> Uuid {
            let fields = id.as_fields();
            let mut last = fields.3.clone();
            last[7] += 1;
            Uuid::from_fields(fields.0, fields.1, fields.2, &last)
                .unwrap_or_else(|_| unreachable!())
        }

        #[derive(Debug, Clone, PartialEq, Serialize)]
        /// Represents a editor plugin instance, only used during ser/de.
        pub struct HEPluginInstance<T> {
            id: Uuid,
            cells: [EditorCell<T>; 1],
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        /// Represents a editor cell, only used during ser/de.
        struct EditorCell<T> {
            id: Uuid,
            content: CellContent<T>,
            rows: Option<()>,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        /// Represents the editor cell, with plugin identifier and state.
        struct CellContent<T> {
            plugin: serlo_he_spec_meta::Identifier,
            state: T,
        }

        impl<T: Plugin + Default> Default for HEPluginInstance<T> {
            fn default() -> Self {
                Self {
                    id: T::default().uuid().clone(),
                    cells: [EditorCell::default()]
                }
            }
        }

        impl<T: Plugin + Default> Default for EditorCell<T> {
            fn default() -> Self {
                Self {
                    id: inc_uuid(T::default().uuid()),
                    content: CellContent::default(),
                    rows: None
                }
            }
        }

        impl<T: Plugin + Default> Default for CellContent<T> {
            fn default() -> Self {
                Self {
                    plugin: T::identifier(),
                    state: T::default()
                }
            }
        }

        impl Plugins {
            /// Uuid of the wrapped plugin.
            pub fn instance_uuid(&self) -> &Uuid {
                match self {
                    #(
                        Plugins::#identifiers(p) => &p.id
                    ),*
                }
            }

            /// Get the specification of a variant.
            pub fn specification(&self) -> serlo_he_spec_meta::Plugin {
                match self {
                    #(
                        Plugins::#identifiers(p) => #identifiers2::specification()
                    ),*
                }
            }

            /// Get the identifier of a variant.
            pub fn identifier(&self) -> serlo_he_spec_meta::Identifier {
                match self {
                    #(
                        Plugins::#identifiers(p) => #identifiers2::identifier()
                    ),*
                }
            }

            /// The complete specification object for all plugins.
            pub fn whole_specification() -> serlo_he_spec_meta::Specification {
                serde_json::from_str(#serial_spec).expect("invalid specification in code!")
            }
        }

        impl From<HEPluginInstance<Plugins>> for Plugins {
            fn from(mut instance: HEPluginInstance<Plugins>) -> Self {
                let mut state = match instance.cells {
                    [cell] => cell.content.state,
                    _ => unreachable!(),
                };
                match &mut state {
                    #(
                        Plugins::#identifiers(ref mut p) => p.id = instance.id
                    ),*
                };
                state
            }
        }

        impl From<Plugins> for HEPluginInstance<Plugins> {
            fn from(state: Plugins) -> Self {
                HEPluginInstance {
                    id: state.instance_uuid().clone(),
                    cells: [
                        EditorCell {
                            id: inc_uuid(state.instance_uuid()),
                            content: CellContent {
                                plugin: state.identifier(),
                                state: state,
                            },
                            rows: None
                        }
                    ]
                }
            }
        }
    }
}

fn impl_plugin_struct(plugin: &Plugin) -> TokenStream {
    let ident = syn_identifier_from_locator(&plugin.identifier.name);
    let description = LitStr::new(&plugin.description, Span::call_site());
    let documentation = LitStr::new(&plugin.documentation, Span::call_site());
    let serial_spec = LitStr::new(
        &serde_json::to_string(&plugin).expect("could not serialize plugin spec!"),
        Span::call_site(),
    );
    let serial_identifier = LitStr::new(
        &serde_json::to_string(&plugin.identifier).expect("could not serialize plugin identifier!"),
        Span::call_site(),
    );

    let attribute_vec: Vec<TokenStream> = plugin
        .attributes
        .iter()
        .map(|a| impl_attribute(a))
        .collect();
    let attributes = &attribute_vec;
    quote! {
        #[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
        #[doc = #description]
        #[doc = "\n\n"]
        #[doc = #documentation]
        pub struct #ident {
            /// Unique plugin instance identifier.
            #[serde(skip)]
            pub id: Uuid,
            #(#attributes),*
        }

        impl From<HEPluginInstance<#ident>> for #ident {
            fn from(mut instance: HEPluginInstance<#ident>) -> Self {
                let mut state = match instance.cells {
                    [cell] => cell.content.state,
                    _ => unreachable!()
                };
                state.id = instance.id;
                state
            }
        }

        impl From<#ident> for HEPluginInstance<#ident> {
            fn from(state: #ident) -> Self {
                HEPluginInstance {
                    id: state.uuid().clone(),
                    cells: [
                        EditorCell {
                            id: inc_uuid(state.uuid()),
                            content: CellContent {
                                plugin: #ident::identifier(),
                                state: state,
                            },
                            rows: None
                        }
                    ]
                }
            }
        }

        impl From<#ident> for Plugins {
            fn from(instance: #ident) -> Self {
                Plugins::#ident(instance)
            }
        }

        impl Plugin for #ident {
            fn specification() -> serlo_he_spec_meta::Plugin {
                serde_json::from_str(#serial_spec)
                    .expect("could not deserialize spec!")
            }

            fn uuid(&self) -> &Uuid {
                &self.id
            }

            fn identifier() -> serlo_he_spec_meta::Identifier {
                serde_json::from_str(#serial_identifier)
                    .expect("could not deserialize identifier!")
            }
        }
    }
}

fn read_spec(path: &PathBuf) -> Specification {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()));
    match serde_yaml::from_str(&{
        let mut input = String::new();
        let mut file = match fs::File::open(root.join(path)) {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        if let Err(e) = file.read_to_string(&mut input) {
            panic!("{}", e)
        }
        input
    }) {
        Ok(spec) => spec,
        Err(err) => {
            panic!("{}", err);
        }
    }
}
