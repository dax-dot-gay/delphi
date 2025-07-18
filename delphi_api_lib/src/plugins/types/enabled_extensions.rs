use std::{ collections::HashSet, sync::Arc };

use parking_lot::RwLock;
use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };
use specta::Type;

#[derive(
    Type,
    JsonSchema,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
#[serde(rename_all = "snake_case")]
pub enum EnabledExtension {
    BroadcastChannel,
    Cache,
    Console,
    Cron,
    Crypto,
    #[serde(alias = "FFI")]
    Ffi,

    #[serde(alias = "fs", alias = "FS")]
    Filesystem,

    #[serde(alias = "HTTP")]
    Http,

    #[serde(alias = "IO")]
    Io,

    #[serde(alias = "KV", alias = "kv")]
    KeyValue,

    #[serde(alias = "URL")]
    Url,

    #[serde(alias = "web_gpu")]
    Webgpu,

    #[serde(alias = "web_socket", alias = "ws")]
    Websocket,

    #[serde(alias = "webstorage")]
    WebStorage,

    #[serde(alias = "TLS")]
    Tls,
}

#[derive(Clone, Debug, Type, JsonSchema, Serialize, Deserialize)]
struct SerEnabledExtensions(pub(self) Vec<EnabledExtension>);

impl From<EnabledExtensions> for SerEnabledExtensions {
    fn from(value: EnabledExtensions) -> Self {
        let exts = value.0.read().clone();
        Self(Vec::from_iter(exts))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from = "SerEnabledExtensions", into = "SerEnabledExtensions")]
pub struct EnabledExtensions(pub(self) Arc<RwLock<HashSet<EnabledExtension>>>);

impl From<SerEnabledExtensions> for EnabledExtensions {
    fn from(value: SerEnabledExtensions) -> Self {
        Self(Arc::new(RwLock::new(HashSet::from_iter(value.0))))
    }
}

impl FromIterator<EnabledExtension> for EnabledExtensions {
    fn from_iter<T: IntoIterator<Item = EnabledExtension>>(iter: T) -> Self {
        Self::new(iter)
    }
}

impl Type for EnabledExtensions {
    fn inline(
        opts: specta::datatype::DefOpts,
        generics: &[specta::datatype::DataType]
    ) -> Result<specta::datatype::DataType, specta::r#type::ExportError> {
        SerEnabledExtensions::inline(opts, generics)
    }

    fn definition_generics() -> Vec<specta::datatype::GenericType> {
        SerEnabledExtensions::definition_generics()
    }

    fn category_impl(
        opts: specta::datatype::DefOpts,
        generics: &[specta::datatype::DataType]
    ) -> Result<specta::r#type::TypeCategory, specta::r#type::ExportError> {
        SerEnabledExtensions::category_impl(opts, generics)
    }
}

impl JsonSchema for EnabledExtensions {
    fn schema_name() -> String {
        "EnabledExtensions".to_owned()
    }

    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        SerEnabledExtensions::json_schema(generator)
    }
}

impl EnabledExtensions {
    pub fn new(items: impl IntoIterator<Item = EnabledExtension>) -> Self {
        SerEnabledExtensions(Vec::from_iter(items)).into()
    }

    pub fn add_extension(&self, extension: EnabledExtension) {
        let mut exts = self.0.write();
        exts.insert(extension);
    }

    pub fn with(self, extension: EnabledExtension) -> Self {
        self.add_extension(extension);
        self
    }

    pub fn extensions(&self) -> Vec<EnabledExtension> {
        SerEnabledExtensions::from(self.clone()).0
    }

    pub fn is_enabled(&self, extension: EnabledExtension) -> bool {
        self.extensions().contains(&extension)
    }
}

impl Default for EnabledExtensions {
    fn default() -> Self {
        Self::new(vec![EnabledExtension::Console, EnabledExtension::Crypto, EnabledExtension::Url])
    }
}
