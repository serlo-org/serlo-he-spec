//! conversion of an mfnf syntax tree to plugins

use crate::plugins::*;
use mediawiki_parser::{Element, TransformationError};

fn plugin_vec_from_elements(
    mut content: Vec<Element>,
) -> Result<Vec<Plugins>, TransformationError> {
    let mut new = vec![];
    for e in content.drain(..) {
        // elements to unfold
        match e {
            Element::Paragraph(par) => new.extend(plugin_vec_from_elements(par.content)?),
            _ => new.push(plugins_from_element(e)?),
        }
    }
    Ok(new)
}

pub fn plugins_from_element(root: Element) -> Result<Plugins, TransformationError> {
    Ok(match root {
        Element::Heading(heading) => Plugins::Heading(Heading {
            caption: plugin_vec_from_elements(heading.caption)?,
            content: plugin_vec_from_elements(heading.content)?,
        }),
        Element::Text(text) => Plugins::Markdown(Markdown { content: text.text }),
        Element::Document(doc) => Plugins::Heading(Heading {
            caption: vec![Plugins::Markdown(Markdown {
                content: "Root".into(),
            })],
            content: plugin_vec_from_elements(doc.content)?,
        }),
        other => Plugins::Markdown(Markdown {
            content: other.get_variant_name().to_string(),
        }),
    })
}
