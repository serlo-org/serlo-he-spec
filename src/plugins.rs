use serde::de::{Deserializer, MapAccess, Unexpected, Visitor};
use serde::ser::SerializeStruct;
use serde::{de, ser, Deserialize, Serialize};
use serlo_he_spec_derive::plugin_spec;
use serlo_he_spec_meta;
use uuid::Uuid;
use std::fmt;

const _REFRESHER: &'static str = include_str!("he_plugins.yml");
plugin_spec!("src/he_plugins.yml");

type HEContent = Vec<Plugins>;
type MarkdownString = String;
