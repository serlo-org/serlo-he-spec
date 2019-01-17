use proc_macro2::Span;
pub use serlo_he_spec_meta::identifier_from_locator;
use syn::Ident;

/// generate a plugin identifier from a plugin locator
pub fn syn_identifier_from_locator(locator: &str) -> Ident {
    Ident::new(&identifier_from_locator(locator), Span::call_site())
}
