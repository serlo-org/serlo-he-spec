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

use serlo_he_spec_meta::{Attribute, Multiplicity, Plugin};

#[proc_macro]
pub fn plugin_spec(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = PathBuf::from(
        syn::parse::<LitStr>(input)
            .expect("could not parse input as string!")
            .value(),
    );
    let mut out = TokenStream::new();

    let spec = read_spec(&path);
    for plugin in &spec {
        out.extend(impl_plugin_struct(&plugin))
    }
    out.extend(impl_plugins_enum(&spec));
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

fn impl_plugins_enum(plugins: &Vec<Plugin>) -> TokenStream {
    let identifier_vec = plugins
        .iter()
        .map(|p| Ident::new(&p.identifier, Span::call_site()))
        .collect::<Vec<Ident>>();
    let identifiers = &identifier_vec;
    let identifiers2 = &identifier_vec;
    let descriptions = plugins
        .iter()
        .map(|p| LitStr::new(&p.description, Span::call_site()));
    quote! {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        /// The specified plugins.
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

            /// Get specification of all variants.
            pub fn specifications() -> Vec<serlo_he_spec_meta::Plugin> {
                vec![#(#identifiers::specification()),*]
            }
        }
    }
}

fn impl_plugin_struct(plugin: &Plugin) -> TokenStream {
    let ident = Ident::new(&plugin.identifier, Span::call_site());
    let description = LitStr::new(&plugin.description, Span::call_site());
    let documentation = LitStr::new(&plugin.documentation, Span::call_site());
    let serial_spec = LitStr::new(
        &serde_json::to_string(plugin).expect("could not serialize plugin spec!"),
        Span::call_site(),
    );
    let attributes: Vec<TokenStream> = plugin
        .attributes
        .iter()
        .map(|a| impl_attribute(a))
        .collect();
    quote! {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[doc = #description]
        #[doc = "\n\n"]
        #[doc = #documentation]
        pub struct #ident {
            #(#attributes),*
        }

        impl #ident {
            /// Specification of this plugin.
            pub fn specification() -> serlo_he_spec_meta::Plugin {
                serde_json::from_str(#serial_spec)
                    .expect("could not deserialize spec!")
            }

            /// Uuid of this plugin.
            pub fn uuid() -> uuid::Uuid {
                serde_json::from_str(#uuid)
                    .expect("could not deserialize uuid!")
            }
        }
    }
}

fn read_spec(path: &PathBuf) -> Vec<Plugin> {
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
