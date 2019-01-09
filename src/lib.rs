#[cfg(feature = "mfnf")]
mod mfnf;
mod plugins;

pub use crate::plugins::*;

#[cfg(test)]
mod test {
    #[cfg(feature = "mfnf")]
    use crate::mfnf::*;
    use crate::{*};
    #[cfg(feature = "mfnf")]
    use std::fs;
    use uuid::Uuid;

    fn example_heading_doc() -> Plugins {
        Plugins::Heading(Heading {
            id: Uuid::new_v4(),
            caption: Title {
                id: Uuid::new_v4(),
                content: "Hello World".into(),
            },
            content: vec![Plugins::Heading(Heading {
                id: Uuid::new_v4(),
                caption: Title {
                    id: Uuid::new_v4(),
                    content: "Subheading".into(),
                },
                content: vec![Plugins::Markdown(Markdown {
                    id: Uuid::new_v4(),
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

    #[test]
    fn deserialize_single_plugin() {
        let doc = r#"{
          "id": "a6f91bdc-403f-49d3-831d-5c0d09bfc28f",
          "cells": [
            {
              "id": "a6f91bdc-403f-49d3-831d-5c0d09bfc28f",
              "content": {
                "plugin": { "name": "he.serlo.org/markdown", "version": "0.0.0" },
                "state": { "content": "Test" }
              },
              "rows": null
            }
          ]
        }"#;
        let _: Markdown = serde_json::from_str(&doc).unwrap();
    }

    #[test]
    #[cfg(feature = "mfnf")]
    fn simple_mfnf_to_plugins() {
        let root = serde_json::from_reader(
            fs::File::open("src/mfnf_example.json").expect("could not read example!"),
        )
        .expect("could deserialize example!");
        let plugins = plugins_from_element(root).expect("conversion error!");
        let ser = serde_json::to_string_pretty(&plugins).expect("could not serialize!");
        eprintln!("{}", &ser);
    }
}
