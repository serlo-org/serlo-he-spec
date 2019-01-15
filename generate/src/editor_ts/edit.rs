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

pub fn generate_plugin(plugin: &Plugin) -> Result<Vec<GeneratedFile>, GenerationError> {
    let spec = Plugins::whole_specification();
    Ok(vec![
        package_json_patch(plugin, &spec, false)?,
    ])
}
