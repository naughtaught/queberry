use serde::{Deserialize, Serialize};
use strum::{EnumString, VariantNames};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    #[serde(alias = "filename", alias = "fileName", alias = "file_name")]
    pub filename: String,
    pub author: String,
    #[serde(alias = "homepage", alias = "homepPage", alias = "home_page")]
    pub homepage: Option<String>,
    pub description: String,
    pub version: String,
    pub sources: Vec<SourceType>,
    pub types: Vec<PluginType>,
    pub permissions: PluginPermissions,
    #[serde(alias = "api_version", alias = "apiVersion")]
    pub api_version: String,
    pub methods: Vec<MethodMapping>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MethodMapping {
    pub interface_method: String,
    pub plugin_method: String,
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
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginPermissions {
    pub network: Vec<String>,
}
