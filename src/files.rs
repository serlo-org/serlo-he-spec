use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A generated file with its generated contents.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedFile {
    pub path: PathBuf,
    pub content: String,
}

#[macro_export]
macro_rules! REACT_DEFINITION {
    () => {
        r"import {{ plugin }} from './plugin'
import {{ {} }} from './{}.component'

export default {{
    ...plugin,
    Component: {},
    text: '{}'
}}"
    };
}

#[macro_export]
macro_rules! PACKAGE_JSON_PATCH {
    () => {
        r#"{{
    "name": {:?},
    "version": {:?}
}}
        "#
    };
}

#[macro_export]
macro_rules! README {
    () => {
        r#"# {}

Attributes: `{}`

{}
        "#
    };
}
