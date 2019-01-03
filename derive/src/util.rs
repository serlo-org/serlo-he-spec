use proc_macro2::Span;
use syn::Ident;

/// generate a plugin identifier from a plugin locator
pub fn identifier_from_locator(locator: &str) -> String {
    locator
        .split("/")
        .last()
        .unwrap_or_else(|| panic!("{} is not a valid plugin locator!", locator))
        .chars()
        .fold((String::new(), true), |mut acc, c| {
            if c == '-' {
                acc.1 = true;
            } else {
                if acc.1 {
                    acc.0.push_str(&c.to_uppercase().to_string());
                    acc.1 = false;
                } else {
                    acc.0.push(c);
                }
            }
            acc
        })
        .0
}

/// generate a plugin identifier from a plugin locator
pub fn syn_identifier_from_locator(locator: &str) -> Ident {
    Ident::new(&identifier_from_locator(locator), Span::call_site())
}

/// Identifier of the shadow struct of a plugin.
pub fn shadow_identifier(locator: &str) -> Ident {
    let ident = identifier_from_locator(locator);
    Ident::new(&format!("_Shadow_{}", ident.to_string()), Span::call_site())
}
