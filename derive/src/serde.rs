use crate::util::{shadow_identifier, syn_identifier_from_locator};
use proc_macro2::TokenStream;
use quote::quote;
use serlo_he_spec_meta::Plugin;
use syn::Ident;

pub fn impl_serde(plugins: &Vec<Plugin>) -> TokenStream {
    let identifier_vec = plugins
        .iter()
        .map(|p| syn_identifier_from_locator(&p.identifier.name))
        .collect::<Vec<Ident>>();
    let shadows_vec = plugins
        .iter()
        .map(|p| shadow_identifier(&p.identifier.name))
        .collect::<Vec<Ident>>();
    let shadows = &shadows_vec;
    let identifiers = &identifier_vec;
    let identifiers2 = &identifier_vec;

    let mut serialize = quote! {#(
        impl ser::Serialize for #identifiers {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ser::Serializer,
            {
                let shadow = PluginInstance {
                    id: self.id,
                    cells: vec![
                        EditorCell {
                            id: Uuid::new_v4(),
                            content: CellContent {
                                plugin: #identifiers2::specification().identifier.clone(),
                                state: #shadows::from_plugin(self.clone()),
                            },
                            rows: None
                        }
                    ],
                };
                shadow.serialize(serializer)
            }
        }
        )*
    };
    let deserialize = quote! {#(
        impl<'de> Deserialize<'de> for #identifiers {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                match PluginInstance::<#shadows>::deserialize(deserializer) {
                    Ok(mut result) => {
                        let id = result.id;
                        match result.cells.pop() {
                            Some(cell) => Ok(cell.content.state.into_plugin(id)),
                            _ => return Err(de::Error::custom("expected one cell!"))
                        }
                    },
                    Err(err) => Err(err)
                }
            }
        }
    )*};
    serialize.extend(deserialize);
    serialize
}
