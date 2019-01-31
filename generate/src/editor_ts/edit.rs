//! generates the edit package.

use crate::editor_ts::{
    first_letter_to_lowercase, package_json_patch, plugin_name_suffx, readme, templates,
    STATE_DEFAULTS,
};
use crate::files::{GeneratedFile, GenerationError};
use handlebars::Handlebars;
use serde_json::json;
use serlo_he_spec_meta::{identifier_from_locator, Multiplicity, Plugin};
use std::error::Error;
use std::path::PathBuf;

pub fn generate_plugin(plugin: &Plugin) -> Result<Vec<GeneratedFile>, GenerationError> {
    Ok(vec![
        package_json_patch(plugin, false)?,
        index(plugin)?,
        readme(plugin)?,
    ])
}

fn default_state(plugin: &Plugin) -> Result<String, GenerationError> {
    let mut result = vec![];
    for attr in &plugin.attributes {
        result.push(format!(
            "{}: {}",
            &attr.identifier,
            &STATE_DEFAULTS
                .get(&attr.content_type)
                .map(|r| match attr.multiplicity {
                    Multiplicity::Optional => Ok("null".into()),
                    Multiplicity::Once => Ok(r.clone()),
                    Multiplicity::Arbitrary => Ok("[]".into()),
                    Multiplicity::MinOnce => Ok(format!("[{}]", &r)),
                })
                .unwrap_or(Err(GenerationError::new(format!(
                    "No default defined for {}!",
                    &attr.content_type
                ))))?
        ))
    }
    Ok(format!("{{\n    {}\n  }}", result.join(",\n    ")))
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
                "component_description": plugin.name,
                "plugin_path": plugin.identifier.name,
                "plugin_suffix": first_letter_to_lowercase(&component_ident),
                "plugin_dashed": plugin_name_suffx(plugin),
                "component_default": default_state(plugin)?,
            }),
        )
        .map_err(|e| GenerationError::new(e.description().to_string()))?;
    Ok(GeneratedFile {
        path: PathBuf::from("src/index.ts"),
        content,
    })
}
