



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
