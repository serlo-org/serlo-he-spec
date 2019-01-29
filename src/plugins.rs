use serde::{de, de::Deserializer, Deserialize, Serialize, Serializer};
use serlo_he_spec_derive::plugin_spec;
use serlo_he_spec_meta;
use std::default::Default;
use std::fmt;
use uuid::Uuid;

const _REFRESHER: &str = include_str!("he_plugins.yml");
plugin_spec!("src/he_plugins.yml");

#[derive(Debug, Serialize, Deserialize, PartialEq, Hash, Clone)]
/// Some verified markdown text.
pub struct MarkdownText(String);

#[derive(Debug, Serialize, Deserialize, PartialEq, Hash, Clone)]
/// Some verified title text.
pub struct TitleText(String);

impl Default for MarkdownText {
    fn default() -> Self {
        Self("use *formatted* text here!".to_string())
    }
}

impl Default for TitleText {
    fn default() -> Self {
        Self("use plain title text here!".to_string())
    }
}

impl fmt::Display for TitleText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for MarkdownText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl MarkdownText {
    pub fn from_str(text: &str) -> Self {
        MarkdownText(text.into())
    }
}

impl TitleText {
    pub fn from_str(text: &str) -> Self {
        TitleText(text.into())
    }
}
