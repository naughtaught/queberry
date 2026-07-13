use serde::{Deserialize, Serialize};
use strum::{EnumString, VariantNames};

impl ApiKeyRequirement {
    pub fn supports_api_keys(&self) -> bool {
        match self {
            ApiKeyRequirement::Always | ApiKeyRequirement::Optional => true,
            ApiKeyRequirement::Never => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ApiKeyRequirement {
    Never,
    Always,
    Optional,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    pub id: String,
    pub name: String,
    #[serde(
        alias = "filename",
        alias = "fileName",
        alias = "file_name",
        alias = "file-name"
    )]
    pub filename: String,
    pub author: String,
    #[serde(
        alias = "homepage",
        alias = "homePage",
        alias = "home_page",
        alias = "home-page"
    )]
    pub homepage: Option<String>,
    pub description: String,
    pub version: String,
    pub sources: Vec<SourceType>,
    pub types: Vec<PluginType>,
    pub cacheless: bool,
    pub permissions: PluginPermissions,
    #[serde(alias = "api_version", alias = "apiVersion", alias = "api-version")]
    pub api_version: String,
    pub methods: Vec<MethodMapping>,
    pub requires_api_key: ApiKeyRequirement,
    #[serde(default)]
    pub rate_limit: Option<PluginRateLimit>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginRateLimit {
    #[serde(alias = "max_calls", alias = "max-calls")]
    pub max_calls: usize,
    #[serde(
        default = "default_window_seconds",
        alias = "window_seconds",
        alias = "window-seconds"
    )]
    pub window_seconds: u64,
}

fn default_window_seconds() -> u64 {
    60
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MethodMapping {
    #[serde(
        alias = "interfaceMethod",
        alias = "interface_method",
        alias = "interface-method"
    )]
    pub interface_method: String,
    #[serde(
        alias = "pluginMethod",
        alias = "plugin_method",
        alias = "plugin-method"
    )]
    pub plugin_method: String,
    #[serde(
        default,
        alias = "requiresApiKey",
        alias = "requires_api_key",
        alias = "requires-apikey"
    )]
    pub requires_api_key: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SourceType {
    #[serde(alias = "torrents", alias = "Torrents")]
    Torrents,
    #[serde(alias = "usenet", alias = "Usenet")]
    Usenet,
    #[serde(alias = "direct", alias = "Direct")]
    Direct,
}

#[derive(Debug, Clone, Deserialize, Serialize, EnumString, VariantNames, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum PluginType {
    #[serde(alias = "indexer", alias = "Indexer")]
    Indexer,
    #[serde(alias = "resolver", alias = "Resolver")]
    Resolver,
    #[serde(alias = "utility", alias = "Utility")]
    Utility,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PluginPermissions {
    #[serde(skip_deserializing)]
    pub validated_hosts: Vec<String>,

    #[serde(alias = "network")]
    pub network_patterns: Vec<String>,

    #[serde(default)]
    pub allow_private_networks: bool,
}
