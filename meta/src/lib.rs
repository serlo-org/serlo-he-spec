//! Describes the format (types) used in the plugin specification.

use semver::Version;
use serde_derive::{Deserialize, Serialize};

/// The specification object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Specification {
    /// specification for the plugins.
    pub plugins: Vec<Plugin>,
}

/// Non-exclusive plugin categories.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Category {
    /// Used for document structure, may have no or little semantic meaning.
    Structure,
    /// Cannot contain other block plugins (cannot be nested).
    Block,
    /// Holds self-contained semantic information.
    Semantic,
    /// Is not required for understanding.
    Illustratory,
    /// For author / developer eyes only.
    Debug,
}

/// Plugin attribute multiplicity.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Multiplicity {
    /// Occurs 0-1 times.
    Optional,
    /// Occurs exactly once.
    Once,
    /// Occurs an arbitrary number of times.
    Arbitrary,
    /// Occurs at least once.
    MinOnce,
}

/// Specifies the plugin identity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    /// Plugin resource identifier. Must end with a human-readable plugin identifier,
    /// preferably in kebab-case.
    pub name: String,
    /// Version of the plugin (specification) according to Semantic Versioning.
    #[serde(with = "serde_semver")]
    pub version: Version,
}

/// Specification of a plugin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plugin {
    /// Identifying plugin metadata.
    pub identifier: Identifier,
    /// A list of categories this plugin falls in.
    pub categories: Vec<Category>,
    /// A *short* description of the plugin in CommonMark.
    pub description: String,
    /// A short human readable plugin name. May contain spaces.
    pub name: String,
    /// A (possibly) longer documentation of plugin purpose and usage in CommonMark.
    pub documentation: String,
    /// Plugin attributes.
    pub attributes: Vec<Attribute>,
}

/// Specification of a plugin attribute.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    /// Human-Readable attribute identifier. Must be a valid rust identifier.
    pub identifier: String,
    /// How often the attribute may occur.
    pub multiplicity: Multiplicity,
    /// Constraints the content of the attribute must fulfill.
    /// TODO: This will be typed correctly once there is a contstraint api for plugins!
    pub constraints: Vec<String>,
    /// Content type (must be the rust identifier of a valid type)
    pub content_type: String,
}

mod serde_semver {
    use semver::Version;
    use serde::{
        de,
        de::{Deserializer, Visitor},
        ser::Serializer,
    };
    use std::fmt;
    use std::str::FromStr;

    pub fn serialize<S>(version: &Version, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&version.to_string())
    }

    struct SVVisitor;

    impl<'de> Visitor<'de> for SVVisitor {
        type Value = Version;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid SemVer version number")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match Version::from_str(value) {
                Ok(ver) => Ok(ver),
                Err(e) => Err(E::custom(format!("{}", e))),
            }
        }
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Version, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SVVisitor)
    }
}

/// generate a plugin identifier from a plugin locator
pub fn identifier_from_locator(locator: &str) -> String {
    locator
        .split('/')
        .last()
        .unwrap_or_else(|| panic!("{} is not a valid plugin locator!", locator))
        .trim_start_matches("editor-plugin-")
        .chars()
        .fold((String::new(), true), |mut acc, c| {
            if c == '-' {
                acc.1 = true;
            } else if acc.1 {
                acc.0.push_str(&c.to_uppercase().to_string());
                acc.1 = false;
            } else {
                acc.0.push(c);
            }
            acc
        })
        .0
}
