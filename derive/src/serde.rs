use crate::util::{syn_identifier_from_locator};
use proc_macro2::TokenStream;
use quote::quote;
use serlo_he_spec_meta::Plugin;
use syn::Ident;

pub fn impl_serde(plugins: &[Plugin]) -> TokenStream {
    let identifier_vec = plugins
        .iter()
        .map(|p| syn_identifier_from_locator(&p.identifier.name))
        .collect::<Vec<Ident>>();
    let identifiers = &identifier_vec;
    let identifiers2 = &identifier_vec;
    let identifiers3 = &identifier_vec;

    let deserialize = quote! {
        #[derive(Serialize, Deserialize)]
        struct ShadowInstance<T> {
            pub id: Uuid,
            pub cells: [EditorCell<T>; 1],
        }

        impl<T> ShadowInstance<T> {
            fn into_instance(mut self, target_identifier: &serlo_he_spec_meta::Identifier) -> Result<HEPluginInstance<T>, String> {
                let local_ident = &self.cells[0].content.plugin;
                if local_ident.name != target_identifier.name {
                    return Err("state type does not match identifier!".into())
                };
                if local_ident.version > target_identifier.version {
                    return Err("plugin version is higher than in specification!".into())
                }
                if local_ident.version.major != target_identifier.version.major || (local_ident.version.major == 0 && local_ident.version.minor != target_identifier.version.minor) {
                    return Err("plugin version is incompatible!".into())
                }

                Ok(HEPluginInstance {
                    id: self.id,
                    cells: self.cells
                })
            }
        }

        #(
        impl<'de> Deserialize<'de> for HEPluginInstance<#identifiers> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                match ShadowInstance::<#identifiers2>::deserialize(deserializer) {
                    Ok(mut result) => {
                        let ident = #identifiers3::identifier();
                        Ok(result.into_instance(&ident).map_err(de::Error::custom)?)
                    },
                    Err(err) => Err(err)
                }
            }
        })*

        impl<'de> Deserialize<'de> for HEPluginInstance<Plugins> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                match ShadowInstance::<Plugins>::deserialize(deserializer) {
                    Ok(mut result) => {
                        let ident = result.cells[0].content.state.identifier();
                        Ok(result.into_instance(&ident).map_err(de::Error::custom)?)
                    },
                    Err(err) => Err(err)
                }
            }
        }
    };
    deserialize
}
