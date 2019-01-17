use serde::{de, de::Deserializer, Deserialize, Serialize};
use serlo_he_spec_derive::plugin_spec;
use serlo_he_spec_meta;
use uuid::Uuid;
use std::default::Default;

const _REFRESHER: &str = include_str!("he_plugins.yml");
plugin_spec!("src/he_plugins.yml");

type MarkdownString = String;
type TitleString = String;
