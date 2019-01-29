//! generates the renderer package

use crate::editor_ts::{
    first_letter_to_lowercase, package_json_patch, plugin_name_suffx, templates,
    TYPESCRIPT_IMPORTS, TYPESCRIPT_TYPES,
};
use crate::files::{GeneratedFile, GenerationError};
use handlebars::Handlebars;
use serde_json::json;
use serlo_he_spec_meta::{identifier_from_locator, Multiplicity, Plugin};
use std::error::Error;
use std::path::PathBuf;

pub fn generate_plugin_renderer(plugin: &Plugin) -> Result<Vec<GeneratedFile>, GenerationError> {
    Ok(vec![index(plugin)?, package_json_patch(plugin, true)?])
}

fn state_attributes(plugin: &Plugin) -> Result<Vec<String>, GenerationError> {
    plugin.attributes.iter().try_fold(vec![], |mut res, a| {
        match TYPESCRIPT_TYPES.get(&a.content_type) {
            Some(t) => {
                let t = match a.multiplicity {
                    Multiplicity::Once => t.to_string(),
                    Multiplicity::Optional => format!("{} | null", &t),
                    Multiplicity::Arbitrary | Multiplicity::MinOnce => format!("Array<{}>", &t),
                };
                res.push(format!("{}: {}", a.identifier, t))
            }
            None => {
                return Err(GenerationError::new(format!(
                    "no typescript type defined for \"{}\"!",
                    &a.content_type
                )));
            }
        };
        Ok(res)
    })
}

fn index(plugin: &Plugin) -> Result<GeneratedFile, GenerationError> {
    let mut reg = Handlebars::new();
    reg.set_strict_mode(true);
    reg.register_escape_fn(|s| s.to_string());
    let component_ident = identifier_from_locator(&plugin.identifier.name);
    let content = reg
        .render_template(
            templates::RENDERER_INDEX,
            &json!({
                "imports": state_type_imports(plugin),
                "component_ident": component_ident,
                "attributes": state_attributes(plugin)?,
                "plugin_suffix": first_letter_to_lowercase(&component_ident),
                "plugin_dashed": plugin_name_suffx(plugin)
            }),
        )
        .map_err(|e| GenerationError::new(e.description().to_string()))?;
    Ok(GeneratedFile {
        path: PathBuf::from("src/index.ts"),
        content,
    })
}

/// A generates a list of imports for types used in the plugin state.
pub fn state_type_imports(plugin: &Plugin) -> Vec<String> {
    let mut imports: Vec<String> = plugin
        .attributes
        .iter()
        .map(|a| {
            TYPESCRIPT_TYPES
                .get(&a.content_type)
                .unwrap_or(&a.content_type)
        })
        .filter_map(|t| {
            TYPESCRIPT_IMPORTS
                .get(t)
                .map(|p| format!("import {{ {} }} from '{}'", t, &p))
        })
        .collect();
    imports.sort();
    imports.dedup();
    imports
}
