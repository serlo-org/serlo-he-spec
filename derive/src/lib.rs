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
use syn::{Ident, LitStr};

use serlo_he_spec_meta::{Attribute, Multiplicity, Plugin, Specification};

mod serde;
mod util;

use crate::util::{syn_identifier_from_locator, shadow_identifier};

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
    let content_type = Ident::new(&attribute.content_type, Span::call_site());

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
    let identifier_vec = spec.plugins
        .iter()
        .map(|p| syn_identifier_from_locator(&p.identifier.name))
        .collect::<Vec<Ident>>();
    let identifiers = &identifier_vec;
    let identifiers2 = &identifier_vec;
    let descriptions = spec.plugins
        .iter()
        .map(|p| LitStr::new(&p.description, Span::call_site()));
    let serial_spec = LitStr::new(
        &serde_json::to_string(&spec).expect("could not serialize plugin spec!"),
        Span::call_site(),
    );
    quote! {
        /// The specified plugins.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Plugins {
            #(
                #[doc = #descriptions]
                #identifiers(#identifiers2)
            ),*
        }

        impl Plugins {
            /// Get the specification of a variant.
            pub fn specification(&self) -> serlo_he_spec_meta::Plugin {
                match self {
                    #(
                        Plugins::#identifiers(p) => #identifiers2::specification()
                    ),*
                }
            }

            /// The complete specification object for all plugins.
            pub fn whole_specification() -> serlo_he_spec_meta::Specification {
                serde_json::from_str(#serial_spec).expect("invalid specification in code!")
            }
        }
    }
}

fn impl_plugin_struct(plugin: &Plugin) -> TokenStream {
    let ident = syn_identifier_from_locator(&plugin.identifier.name);
    let shadow = shadow_identifier(&plugin.identifier.name);
    let description = LitStr::new(&plugin.description, Span::call_site());
    let documentation = LitStr::new(&plugin.documentation, Span::call_site());
    let serial_spec = LitStr::new(
        &serde_json::to_string(&plugin).expect("could not serialize plugin spec!"),
        Span::call_site(),
    );
    let attribute_vec: Vec<TokenStream> = plugin
        .attributes
        .iter()
        .map(|a| impl_attribute(a))
        .collect();
    let attributes = &attribute_vec;
    let attribute_names_vec: Vec<Ident> = plugin
        .attributes
        .iter()
        .map(|a| Ident::new(&a.identifier, Span::call_site()))
        .collect();
    let attribute_names = &attribute_names_vec;
    let attribute_names2 = &attribute_names_vec;
    quote! {
        #[derive(Debug, Clone, PartialEq, Default)]
        #[doc = #description]
        #[doc = "\n\n"]
        #[doc = #documentation]
        pub struct #ident {
            #(#attributes),*
        }

        /// Shadow type used in serialization and deserialization.
        /// Represents only the plugin state, without identifier information.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct #shadow {
            #(#attributes),*
        }

        impl #shadow {
            pub fn into_plugin(self) -> #ident {
                #ident {
                    #(#attribute_names: self.#attribute_names2),*
                }
            }

            pub fn from_plugin(plugin: #ident) -> Self {
                Self {
                    #(#attribute_names: plugin.#attribute_names2),*
                }
            }
        }

        impl #ident {
            /// Specification of this plugin.
            pub fn specification() -> serlo_he_spec_meta::Plugin {
                serde_json::from_str(#serial_spec)
                    .expect("could not deserialize spec!")
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
