

/// find a plugin specification by its type identifier, like "Heading".
pub fn find_plugin_by_typename<'p>(name: &str, spec: &'p Specification) -> Option<&'p Plugin> {
    spec.plugins
        .iter()
        .find(|plugin| identifier_from_locator(&plugin.identifier.name) == name)
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
                    Some(plugin) => {
                        format!("{}State", identifier_from_locator(&plugin.identifier.name))
                    }
                    None => {
                        return Err(GenerationError::new(format!(
                            "no typescript type defined for {:?}",
                            a.content_type
                        )))
                    }
                },
            };
            &match a.multiplicity {
                Multiplicity::Once => base_type,
                Multiplicity::Optional => format!("{} | null", &base_type),
                Multiplicity::Arbitrary | Multiplicity::MinOnce => format!("Array<{}>", &base_type),
            }
        }));
        Ok(attrs)
    })
}

/// Get the plugins this plugin directly depends on through its attributes' `content_type`.
pub fn get_dependent_plugins<'s>(
    plugin: &Plugin,
    spec: &'s Specification
) -> Vec<&'s Plugin> {
    plugin.attributes.iter().filter_map(|a| {
        match find_plugin_by_typename(&a.content_type, &spec) {
            Some(plugin) => (Some(plugin)),
            None => None
        }
    }).collect()
}

/// A generates a list of imports of specified plugins which are used
/// as `content_type` in an attribute of `plugin.
pub fn plugin_package_imports(
    plugin: &Plugin,
    spec: &Specification,
) -> Result<Vec<String>, GenerationError> {
    let mut additional_types: HashMap<String, String> = HashMap::new();
    additional_types.insert(
        "Editable".into(),
        "@splish-me/editor-core/lib/editable.component".into(),
    );
    let mut result: Vec<(String, String)> = plugin.attributes.iter().filter_map(|a| {
        if let Some(editor_type) = spec.editor_types.get(&a.content_type) {
            if let Some(path) = additional_types.get(editor_type) {
                return Some((editor_type.clone(), path.clone()))
            }
        };
        None
    }).collect();
    result.extend(get_dependent_plugins(plugin, spec).iter().filter_map(|p| {
        Some((format!("{}State", &identifier_from_locator(&p.identifier.name)), p.identifier.name.clone()))
    }));

    Ok(result.iter().map(|(name, path)| format!("import {{ {} }} from '{}'", name, path)).collect())
}
