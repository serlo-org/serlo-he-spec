//! generates the edit package.

use crate::editor_ts::{package_json_patch, templates};
use crate::files::{GeneratedFile, GenerationError};
use handlebars::Handlebars;
use serde_json::json;
use serlo_he_spec_meta::{identifier_from_locator, Plugin};
use std::error::Error;
use std::path::PathBuf;

pub fn generate_plugin(plugin: &Plugin) -> Result<Vec<GeneratedFile>, GenerationError> {
    Ok(vec![package_json_patch(plugin, false)?, index(plugin)?])
}

fn index(plugin: &Plugin) -> Result<GeneratedFile, GenerationError> {
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
                "component_description": plugin.name,
                "component_default": "{}",
            }),
        )
        .map_err(|e| GenerationError::new(e.description().to_string()))?;
    Ok(GeneratedFile {
        path: PathBuf::from("src/index.ts"),
        content,
    })
}
