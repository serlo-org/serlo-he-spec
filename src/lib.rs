use serde_derive::{Deserialize, Serialize};
use serlo_he_spec_derive::plugin_spec;
use serlo_he_spec_meta;

const _REFRESHER: &'static str = include_str!("test_spec.yml");
plugin_spec!("src/test_spec.yml");

type HEContent = Vec<Plugins>;

#[cfg(test)]
mod test {
    use crate::{Heading, Markdown, Plugins};

    #[test]
    fn serialize_heading() {
        let heading = Plugins::Heading(Heading {
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
        });
        let serial = serde_json::to_string_pretty(&heading).expect("could not serialize!");
        eprintln!("{}", serial);
    }
    #[test]
    fn deserialize_heading() {
        let doc = r#"{
            Heading: {
                caption: [ Markdown: { content: "Hello World" } ],
                content: [
                    Heading: {
                        caption: [ Markdown: { content: "Subheading" } ],
                        content: [ Markdown: { content: "Document Content" } ]
                    }
                ]
            }
        }"#;
        let tree: Plugins = serde_json::from_str(doc).expect("could not deserialize");
        eprintln!(
            "{}",
            &serde_json::to_string_pretty(&tree).expect("could not serialize!")
        );
    }
}
