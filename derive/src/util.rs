use proc_macro2::Span;
pub use serlo_he_spec_meta::identifier_from_locator;
use syn::Ident;

/// generate a plugin identifier from a plugin locator
pub fn syn_identifier_from_locator(locator: &str) -> Ident {
    Ident::new(&identifier_from_locator(locator), Span::call_site())
}

/// Identifier of the shadow struct of a plugin.
pub fn shadow_identifier(locator: &str) -> Ident {
    let ident = identifier_from_locator(locator);
    Ident::new(&format!("_Shadow_{}", ident.to_string()), Span::call_site())
}
