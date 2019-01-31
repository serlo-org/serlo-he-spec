mod plugins;

pub use crate::plugins::*;

#[cfg(test)]
mod test {
    use crate::*;
    use uuid::Uuid;

    fn example_heading_doc() -> HEPluginInstance<Plugins> {
        Plugins::HeHeading(HeHeading {
            id: Uuid::new_v4(),
            caption: HeTitle {
                id: Uuid::new_v4(),
                content: TitleText::from_str("Hello World"),
            }
            .into(),
            content: vec![Plugins::HeHeading(HeHeading {
                id: Uuid::new_v4(),
                caption: HeTitle {
                    id: Uuid::new_v4(),
                    content: TitleText::from_str("Subheading"),
                }
                .into(),
                content: vec![Plugins::HeMarkdown(HeMarkdown {
                    id: Uuid::new_v4(),
                    content: MarkdownText::from_str("Document content"),
                })
                .into()],
            })
            .into()],
        })
        .into()
    }
    #[test]
    fn serialize_heading() {
        let heading = example_heading_doc();
        let serial = serde_json::to_string_pretty(&heading).expect("could not serialize!");
        eprintln!("{}", &serial);
    }

    #[test]
    fn serde_heading() {
        let doc =
            serde_json::to_string_pretty(&example_heading_doc()).expect("serialization failed");
        let tree: HEPluginInstance<Plugins> =
            serde_json::from_str(&doc).expect("could not deserialize");
        eprintln!(
            "{}",
            &serde_json::to_string_pretty(&tree).expect("could not serialize!")
        );
    }

    #[test]
    fn deserialize_single_markdown() {
        let doc = r#"{
          "type": "@splish-me/editor-core/editable",
          "state": {
            "id": "a6f91bdc-403f-49d3-831d-5c0d09bfc28f",
            "cells": [
              {
                "id": "a6f91bdc-403f-49d3-831d-5c0d09bfc28f",
                "content": {
                  "plugin": { "name": "@serlo/editor-plugin-he-markdown", "version": "0.1.0" },
                  "state": { "content": "Test" }
                },
                "rows": null
              }
            ]
          }
        }"#;
        let _: HEPluginInstance<HeMarkdown> = serde_json::from_str(&doc).unwrap();
    }

    #[test]
    #[should_panic]
    fn deserialize_wrong_plugin() {
        // state is in fact a markdown plugin -> should panic
        let doc = r#"{
          "type": "@splish-me/editor-core/editable",
          "state": {
            "id": "a6f91bdc-403f-49d3-831d-5c0d09bfc28f",
            "cells": [
              {
                "id": "a6f91bdc-403f-49d3-831d-5c0d09bfc28f",
                "content": {
                  "plugin": { "name": "@serlo/editor-plugin-he-title", "version": "0.1.0" },
                  "state": { "content": "Test" }
                },
                "rows": null
              }
            ]
          }
        }"#;
        let _: HEPluginInstance<HeMarkdown> = serde_json::from_str(&doc).unwrap();
    }

    #[test]
    #[should_panic]
    fn deserialize_incompatible_version() {
        let doc = r#"{
          "type": "@splish-me/editor-core/editable",
          "state": {
            "id": "a6f91bdc-403f-49d3-831d-5c0d09bfc28f",
            "cells": [
              {
                "id": "a6f91bdc-403f-49d3-831d-5c0d09bfc28f",
                "content": {
                  "plugin": { "name": "@serlo/editor-plugin-he-title", "version": "10000.1.0" },
                  "state": { "content": "Test" }
                },
                "rows": null
              }
            ]
          }
        }"#;
        let _: HEPluginInstance<HeTitle> = serde_json::from_str(&doc).unwrap();
    }
}
