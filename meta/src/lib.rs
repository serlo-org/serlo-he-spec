//! Describes the format (types) used in the plugin specification.

use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

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

/// Specification of a plugin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plugin {
    /// Universally unique identifier of the plugin.
    pub uuid: Uuid,
    /// Human-readable identifier of the plugin. Must be a valid rust identifier.
    pub identifier: String,
    /// A list of categories this plugin falls in.
    pub categories: Vec<Category>,
    /// A *short* description of the plugin in CommonMark.
    pub description: String,
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
