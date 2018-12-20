use serde::de::{Deserialize, Deserializer, MapAccess, Unexpected, Visitor};
use serde::ser::SerializeStruct;
use serde::{de, ser};
use serde_derive::{Deserialize, Serialize};
use serlo_he_spec_derive::plugin_spec;
use serlo_he_spec_meta;
use std::fmt;

const _REFRESHER: &'static str = include_str!("he_plugins.yml");
plugin_spec!("src/he_plugins.yml");

type HEContent = Vec<Plugins>;
