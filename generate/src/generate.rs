use serlo_he_spec_meta::{identifier_from_locator, Multiplicity, Plugin, Specification};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct GenerationError {
    /// why file generation has failed.
    pub msg: String,
}

impl GenerationError {
    fn new(message: String) -> Self {
        Self { msg: message }
    }
}

impl fmt::Display for GenerationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for GenerationError {
    fn description(&self) -> &str {
        &self.msg
    }
}

/// find a plugin specification by its type identifier, like "Heading".
pub fn find_plugin_by_typename<'p>(name: &str, spec: &'p Specification) -> Option<&'p Plugin> {
    spec.plugins.iter().find(|plugin| identifier_from_locator(&plugin.identifier.name) == name)
}

pub fn typed_attribute_list(
    plugin: &Plugin,
    spec: &Specification,
) -> Result<Vec<String>, GenerationError> {
    plugin.attributes.iter().try_fold(vec![], |mut attrs, a| {
        attrs.push(format!("    {}: {}", a.identifier, {
            let base_type = match spec.editor_types.get(&a.content_type) {
                Some(t) => t.clone(),
                None => match find_plugin_by_typename(&a.content_type, &spec) {
                    Some(plugin) => identifier_from_locator(&plugin.identifier.name),
                    None => return Err(GenerationError::new(format!(
                        "no typescript type defined for {:?}",
                        a.content_type
                    )))
                }
            };
            &match a.multiplicity {
                Multiplicity::Once => base_type,
                Multiplicity::Optional => format!("{} | null", &base_type),
                Multiplicity::Arbitrary | Multiplicity::MinOnce => format!("List<{}>", &base_type),
            }
        }));
        Ok(attrs)
    })
}

/// A generates a list of imports of specified plugins which are used
/// as `content_type` in an attribute of `plugin.
pub fn plugin_package_imports(
    plugin: &Plugin,
    spec: &Specification,
) -> Result<Vec<String>, GenerationError> {
    plugin.attributes.iter().filter(|a| spec.editor_types.get(&a.content_type).is_none()).try_fold(vec![], |mut attrs, a| {
        let plugin = match find_plugin_by_typename(&a.content_type, &spec) {
            Some(plugin) => plugin,
            None => return Err(GenerationError::new(format!(
                "no typescript type defined for {:?}",
                a.content_type
            )))
        };
        attrs.push(format!("import {{ {} }} from '{}'",
            &identifier_from_locator(&plugin.identifier.name),
            &plugin.identifier.name
        ));
        Ok(attrs)
    })
}
