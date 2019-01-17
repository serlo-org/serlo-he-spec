//! generate serlo editor plugins, written in typescript.

use crate::files::{GeneratedFile, GenerationError};
use handlebars::Handlebars;
use lazy_static::lazy_static;
use serde_json::json;
use serlo_he_spec::Plugins;
use serlo_he_spec_meta::{identifier_from_locator, Plugin, Specification};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

mod edit;
mod renderer;
mod templates;

lazy_static! {
    pub static ref PLUGIN_SPEC: Specification = Plugins::whole_specification();
}

lazy_static! {
    pub static ref TYPESCRIPT_TYPES: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("Plugins".into(), "Editable".into());
        m.insert("MarkdownString".into(), "string".into());
        m.insert("TitleString".into(), "string".into());
        for plugin in &PLUGIN_SPEC.plugins {
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
        for plugin in &PLUGIN_SPEC.plugins {
            let ident = identifier_from_locator(&plugin.identifier.name);
            let ident = first_letter_to_uppper_case(&ident);
            m.insert(
                format!("{}PluginState", &ident),
                plugin.identifier.name.to_string(),
            );
        }
        m
    };
}

pub fn editor_plugin_files(plugin: &Plugin) -> Result<Vec<GeneratedFile>, GenerationError> {
    let dashed_name = plugin
        .identifier
        .name
        .split('/')
        .last()
        .unwrap_or_else(|| panic!(format!("invalid plugin name: {}!", plugin.identifier.name)))
        .trim_right_matches("editor-plugin-he-");

    let mut result = vec![];
    for mut file in edit::generate_plugin(plugin)?.drain(..) {
        file.path = PathBuf::from(dashed_name).join(file.path);
        result.push(file);
    }
    for mut file in renderer::generate_plugin_renderer(plugin)?.drain(..) {
        file.path = PathBuf::from(&format!("{}-renderer", dashed_name)).join(file.path);
        result.push(file);
    }
    Ok(result)
}

pub fn first_letter_to_uppper_case(s1: &str) -> String {
    let mut c = s1.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// find a plugin specification by its type identifier, like "Heading".
pub fn find_plugin_by_typename(name: &str) -> Option<&Plugin> {
    PLUGIN_SPEC
        .plugins
        .iter()
        .find(|plugin| identifier_from_locator(&plugin.identifier.name) == name)
}

/// Get the plugins this plugin directly depends on through its attributes' `content_type`.
pub fn get_dependent_plugins(plugin: &Plugin) -> Vec<&Plugin> {
    plugin
        .attributes
        .iter()
        .filter_map(|a| match find_plugin_by_typename(&a.content_type) {
            Some(plugin) => (Some(plugin)),
            None => None,
        })
        .collect()
}

/// generate a patch to transform the template package.json
pub fn package_json_patch(
    plugin: &Plugin,
    renderer: bool,
) -> Result<GeneratedFile, GenerationError> {
    let mut reg = Handlebars::new();
    reg.set_strict_mode(true);
    reg.register_escape_fn(|s| s.to_string());
    let content = reg
        .render_template(
            templates::RENDERER_PACKAGE,
            &json!({
                "name": plugin.identifier.name,
                "name_suffix": if renderer { "-renderer" } else { "" },
                "version": plugin.identifier.version.to_string(),
                "dependencies": (if renderer { vec![] } else { vec![plugin] }).iter().chain(get_dependent_plugins(plugin)
                    .iter())
                    .map(|p| format!(
                        "    \"{}-renderer\": \"^{}\"",
                        p.identifier.name,
                        &p.identifier.version.to_string())
                    ).collect::<Vec<String>>()
                    .join(",\n")
            }),
        )
        .map_err(|e| GenerationError::new(e.description().to_string()))?;
    Ok(GeneratedFile {
        path: PathBuf::from("package_json.patch"),
        content,
    })
}
