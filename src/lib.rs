use serde::de::{Deserialize, Deserializer, MapAccess, Unexpected, Visitor};
use serde::ser::SerializeStruct;
use serde::{de, ser};
use serde_derive::{Deserialize, Serialize};
use serlo_he_spec_derive::plugin_spec;
use serlo_he_spec_meta;
use std::fmt;

const _REFRESHER: &'static str = include_str!("test_spec.yml");
plugin_spec!("src/test_spec.yml");

type HEContent = Vec<Plugins>;

#[cfg(test)]
mod test {
    use crate::{Heading, Markdown, Plugins};

    fn example_heading_doc() -> Plugins {
        Plugins::Heading(Heading {
            caption: vec![Plugins::Markdown(Markdown {
                content: "Hello World".into(),
            })],
            content: vec![Plugins::Heading(Heading {
                caption: vec![Plugins::Markdown(Markdown {
                    content: "Subheading".into(),
                })],
                content: vec![Plugins::Markdown(Markdown {
                    content: "Document content".into(),
                })],
            })],
        })
    }
    #[test]
    fn serialize_heading() {
        let heading = example_heading_doc();
        let serial = serde_json::to_string_pretty(&heading).expect("could not serialize!");
        eprintln!("{}", &serial);
    }

    #[test]
    fn serde_heading() {
        let doc = serde_json::to_string(&example_heading_doc()).expect("serialization failed");
        let tree: Plugins = serde_json::from_str(&doc).expect("could not deserialize");
        eprintln!(
            "{}",
            &serde_json::to_string_pretty(&tree).expect("could not serialize!")
        );
    }
}
