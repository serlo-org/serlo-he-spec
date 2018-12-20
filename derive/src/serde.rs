use crate::util::{identifier_from_locator, shadow_identifier};
use proc_macro2::TokenStream;
use quote::quote;
use serlo_he_spec_meta::Plugin;
use syn::Ident;

pub fn impl_serde(plugins: &Vec<Plugin>) -> TokenStream {
    let identifier_vec = plugins
        .iter()
        .map(|p| identifier_from_locator(&p.identifier.name))
        .collect::<Vec<Ident>>();
    let shadows_vec = plugins
        .iter()
        .map(|p| shadow_identifier(&p.identifier.name))
        .collect::<Vec<Ident>>();
    let shadows = &shadows_vec;
    let identifiers = &identifier_vec;
    let identifiers2 = &identifier_vec;
    let identifiers3 = &identifier_vec;

    let mut serialize = quote! {#(
        impl ser::Serialize for #identifiers {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ser::Serializer,
            {
                let mut sv = serializer.serialize_struct(
                    stringify!(#identifiers2),
                    2
                )?;
                sv.serialize_field("plugin", &#identifiers3::specification().identifier)?;
                let shadow = #shadows::from_plugin(self.clone());

                sv.serialize_field("state", &shadow)?;
                sv.end()
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
                struct PluginVisitor;

                impl<'de> Visitor<'de> for PluginVisitor {
                    type Value = #identifiers2;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("a plugin with corresponding state")
                    }

                    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
                    where
                        V: MapAccess<'de>,
                    {

                        let mut identifier: Option<serlo_he_spec_meta::Identifier> = None;
                        let mut state: Option<#shadows> = None;
                        while let Some(key) = map.next_key()? {
                            match key {
                                "plugin" => {
                                    if identifier.is_some() {
                                        return Err(de::Error::duplicate_field("plugin"));
                                    }
                                    identifier = Some(map.next_value()?);
                                }
                                "state" => {
                                    if state.is_some() {
                                        return Err(de::Error::duplicate_field("state"));
                                    }
                                    state = Some(map.next_value()?);
                                },
                                other => return Err(de::Error::unknown_field(other, &["plugin", "state"]))
                            }
                        }
                        let identifier = identifier.ok_or_else(|| de::Error::missing_field("plugin"))?;
                        let state = state.ok_or_else(|| de::Error::missing_field("state"))?.into_plugin();
                        if (identifier.name != Self::Value::specification().identifier.name) {
                            return Err(de::Error::invalid_type(Unexpected::Other("state type for plugin"), &self))
                        }
                        Ok(state)
                    }
                }

                deserializer.deserialize_struct("Plugins", &[#(names),*], PluginVisitor)
            }
        }
    )*};
    serialize.extend(deserialize);
    serialize
}