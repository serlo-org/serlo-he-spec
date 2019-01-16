//! generates the edit package.

use crate::editor_ts::{
    first_letter_to_uppper_case, get_dependent_plugins, templates, TYPESCRIPT_IMPORTS,
    TYPESCRIPT_TYPES, package_json_patch
};
use crate::files::{GeneratedFile, GenerationError};
use handlebars::Handlebars;
use serde_json::json;
use serlo_he_spec::Plugins;
use serlo_he_spec_meta::{identifier_from_locator, Multiplicity, Plugin, Specification};
use std::path::PathBuf;
use std::error::Error;

pub fn generate_plugin(plugin: &Plugin) -> Result<Vec<GeneratedFile>, GenerationError> {
    let spec = Plugins::whole_specification();
    Ok(vec![
        package_json_patch(plugin, &spec, false)?,
        index(plugin, &spec)?
    ])
}

fn index(plugin: &Plugin, spec: &Specification) -> Result<GeneratedFile, GenerationError> {
    let mut reg = Handlebars::new();
    reg.set_strict_mode(true);
    reg.register_escape_fn(|s| s.to_string());
    let component_ident = identifier_from_locator(&plugin.identifier.name);
    let content = reg
        .render_template(
            templates::EDIT_INDEX,
            &json!({
                "component_ident": component_ident,
                "plugin_path": plugin.identifier.name,
                "plugin_version": plugin.identifier.version.to_string(),
                "component_description": plugin.description,
                "component_default": "{}",
            }),
        )
        .map_err(|e| GenerationError::new(e.description().to_string()))?;
    Ok(GeneratedFile {
        path: PathBuf::from("src/index.ts"),
        content,
    })
}
