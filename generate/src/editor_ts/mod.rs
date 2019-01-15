//! generate serlo editor plugins, written in typescript.

use crate::files::{GeneratedFile, GenerationError};
use lazy_static::lazy_static;
use serlo_he_spec::Plugins;
use serlo_he_spec_meta::{identifier_from_locator, Plugin, Specification};
use std::collections::HashMap;

mod renderer;
mod templates;

lazy_static! {
    pub static ref TYPESCRIPT_TYPES: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("Plugins".into(), "Editable".into());
        m.insert("MarkdownString".into(), "string".into());
        m.insert("TitleString".into(), "string".into());
        for plugin in Plugins::whole_specification().plugins {
            let ident = identifier_from_locator(&plugin.identifier.name);
            let ident = first_letter_to_uppper_case(&ident);
            m.insert(ident.to_string(), format!("{}PluginState", &ident));
        }
        m
    };
}

lazy_static! {
    pub static ref TYPESCRIPT_IMPORTS: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert(
            "Editable".into(),
            "@splish-me/editor-core/lib/editable.component".into(),
        );
        for plugin in Plugins::whole_specification().plugins {
            let ident = identifier_from_locator(&plugin.identifier.name);
            let ident = first_letter_to_uppper_case(&ident);
            m.insert(format!("{}PluginState", &ident), plugin.identifier.name);
        }
        m
    };
}

pub fn editor_plugin_files(plugin: &Plugin) -> Result<Vec<GeneratedFile>, GenerationError> {
    renderer::generate_plugin_renderer(plugin)
}

pub fn first_letter_to_uppper_case(s1: &str) -> String {
    let mut c = s1.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// find a plugin specification by its type identifier, like "Heading".
pub fn find_plugin_by_typename<'p>(name: &str, spec: &'p Specification) -> Option<&'p Plugin> {
    spec.plugins
        .iter()
        .find(|plugin| identifier_from_locator(&plugin.identifier.name) == name)
}

/// Get the plugins this plugin directly depends on through its attributes' `content_type`.
pub fn get_dependent_plugins<'s>(plugin: &Plugin, spec: &'s Specification) -> Vec<&'s Plugin> {
    plugin
        .attributes
        .iter()
        .filter_map(|a| match find_plugin_by_typename(&a.content_type, &spec) {
            Some(plugin) => (Some(plugin)),
            None => None,
        })
        .collect()
}
