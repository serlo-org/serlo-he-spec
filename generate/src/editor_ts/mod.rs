//! generate serlo editor plugins, written in typescript.

use crate::files::{GeneratedFile, GenerationError};
use lazy_static::lazy_static;
use serlo_he_spec::Plugins;
use serlo_he_spec_meta::{identifier_from_locator, Plugin};
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
