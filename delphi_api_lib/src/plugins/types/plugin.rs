use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };
use specta::Type;

use crate::plugins::types::EnabledExtensions;

#[derive(Clone, Debug, Serialize, Deserialize, Type, JsonSchema)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,

    #[serde(default)]
    pub link: Option<String>,

    #[serde(default)]
    pub about: Option<String>,
    pub extensions: EnabledExtensions,
}
