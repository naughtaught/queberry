use crate::constants::API_VERSION;
use crate::errors::AppError;
use crate::plugin_system::types::{
    MethodMapping, Plugin, PluginPermissions, PluginType, SourceType,
};
use crate::utils::get_plugins_dir;
use serde::Deserialize; // Add this
use std::fs;
use std::net::IpAddr;
use std::path::PathBuf;
use url::Url;

pub fn load_plugin_from_dir(plugin_dir: PathBuf) -> Result<Plugin, AppError> {
    let manifest_path = plugin_dir.join("manifest.json");
    let manifest_content = fs::read_to_string(&manifest_path).map_err(AppError::Io)?;

    #[derive(Deserialize)]
    struct RawPlugin {
        id: String,
        name: String,
        #[serde(
            alias = "filename",
            alias = "fileName",
            alias = "file_name",
            alias = "file-name"
        )]
        filename: String,
        author: String,
        #[serde(
            alias = "homepage",
            alias = "homePage",
            alias = "home_page",
            alias = "home-page"
        )]
        homepage: Option<String>,
        description: String,
        version: String,
        sources: Vec<SourceType>,
        types: Vec<PluginType>,
        permissions: RawPermissions,
        #[serde(alias = "api_version", alias = "apiVersion", alias = "api-version")]
        api_version: String,
        methods: Vec<MethodMapping>,
    }

    #[derive(Deserialize)]
    struct RawPermissions {
        network: Vec<String>,
        #[serde(default)]
        allow_private_networks: bool,
    }

    let raw_plugin: RawPlugin = serde_json::from_str(&manifest_content).map_err(AppError::Json)?;

    let mut validated_hosts = Vec::new();
    for pattern in &raw_plugin.permissions.network {
        let host =
            validate_and_extract_host(pattern, raw_plugin.permissions.allow_private_networks)?;
        validated_hosts.push(host);
    }

    let plugin = Plugin {
        id: raw_plugin.id,
        name: raw_plugin.name,
        filename: raw_plugin.filename,
        author: raw_plugin.author,
        homepage: raw_plugin.homepage,
        description: raw_plugin.description,
        version: raw_plugin.version,
        sources: raw_plugin.sources,
        types: raw_plugin.types,
        permissions: PluginPermissions {
            validated_hosts,
            network_patterns: raw_plugin.permissions.network,
            allow_private_networks: raw_plugin.permissions.allow_private_networks,
        },
        api_version: raw_plugin.api_version,
        methods: raw_plugin.methods,
    };

    validate_plugin(&plugin)?;

    let wasm_path = plugin_dir.join(&plugin.filename);
    if !wasm_path.exists() {
        return Err(AppError::NotFound(format!(
            "Plugin WASM file '{}' not found in {}",
            plugin.filename,
            plugin_dir.display()
        )));
    }

    Ok(plugin)
}

fn validate_and_extract_host(pattern: &str, allow_private: bool) -> Result<String, AppError> {
    let cleaned_pattern = clean_url_pattern(pattern);

    let url = Url::parse(&cleaned_pattern).map_err(AppError::Url)?;

    if url.scheme() != "http" && url.scheme() != "https" {
        return Err(AppError::Validation(format!(
            "URL must use http:// or https:// scheme: {}",
            pattern
        )));
    }

    let host = url
        .host_str()
        .ok_or_else(|| AppError::Validation(format!("URL must have a host: {}", pattern)))?;

    if !allow_private && is_private_or_local_host(host)? {
        return Err(AppError::Validation(format!(
            "Access to private/local networks not allowed: {}. Set 'allow_private_networks: true' if needed.",
            pattern
        )));
    }

    let extracted_host = extract_host_from_pattern(host, pattern)?;

    Ok(extracted_host)
}

fn clean_url_pattern(pattern: &str) -> String {
    let trimmed = pattern.trim();

    let without_wildcard = if let Some(stripped) = trimmed.strip_prefix("*.") {
        stripped
    } else {
        trimmed
    };

    if without_wildcard.starts_with("http://") || without_wildcard.starts_with("https://") {
        without_wildcard.to_string()
    } else {
        format!("https://{}", without_wildcard)
    }
}

fn extract_host_from_pattern(host: &str, original_pattern: &str) -> Result<String, AppError> {
    if original_pattern.contains("*.") {
        let parts: Vec<&str> = host.split('.').collect();
        if parts.len() >= 2 {
            let base_domain = format!("{}.{}", parts[parts.len() - 2], parts[parts.len() - 1]);
            return Ok(base_domain);
        }
    }

    Ok(host.to_string())
}

fn is_private_or_local_host(host: &str) -> Result<bool, AppError> {
    if host.eq_ignore_ascii_case("localhost") || host.eq_ignore_ascii_case("localhost.localdomain")
    {
        return Ok(true);
    }

    if let Ok(ip) = host.parse::<IpAddr>() {
        return Ok(is_private_ip(&ip));
    }

    if host.ends_with(".local") || host.ends_with(".localhost") {
        return Ok(true);
    }

    Ok(false)
}

fn is_private_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => {
            // 10.0.0.0/8
            ipv4.octets()[0] == 10
            // 172.16.0.0/12
            || (ipv4.octets()[0] == 172 && (ipv4.octets()[1] >= 16 && ipv4.octets()[1] <= 31))
            // 192.168.0.0/16
            || (ipv4.octets()[0] == 192 && ipv4.octets()[1] == 168)
            // 127.0.0.0/8 (loopback)
            || ipv4.octets()[0] == 127
            // 169.254.0.0/16 (link-local)
            || (ipv4.octets()[0] == 169 && ipv4.octets()[1] == 254)
            // 0.0.0.0/8
            || ipv4.octets()[0] == 0
        }
        IpAddr::V6(ipv6) => {
            // ::1 (loopback)
            ipv6.is_loopback()
            // fe80::/10 (link-local)
            || ((ipv6.segments()[0] & 0xffc0) == 0xfe80)
            // fc00::/7 (unique local)
            || ((ipv6.segments()[0] & 0xfe00) == 0xfc00)
            // ::ffff:0:0/96 (IPv4-mapped)
            || ipv6.to_ipv4_mapped().is_some()
        }
    }
}

fn validate_plugin(plugin: &Plugin) -> Result<(), AppError> {
    if !plugin.id.contains('.') {
        return Err(AppError::Validation(
            "Plugin ID must be in reverse-domain format (e.g., 'org.example.plugin')".into(),
        ));
    }

    if plugin.name.trim().is_empty() {
        return Err(AppError::Validation("Plugin name cannot be empty".into()));
    }

    if !plugin.filename.ends_with(".wasm") {
        return Err(AppError::Validation(
            "Plugin filename must end with .wasm".into(),
        ));
    }

    if plugin.version.trim().is_empty() {
        return Err(AppError::Validation("Version cannot be empty".into()));
    }

    if plugin.sources.is_empty() {
        return Err(AppError::Validation(
            "Plugin must specify at least one source type".into(),
        ));
    }

    if plugin.types.is_empty() {
        return Err(AppError::Validation(
            "Plugin must specify at least one type".into(),
        ));
    }

    if !plugin.api_version.starts_with('v') {
        return Err(AppError::Validation(
            "API version must start with 'v'".into(),
        ));
    }

    let plugin_major = plugin.api_version.split('.').next().unwrap_or("");
    let expected_major = API_VERSION.split('.').next().unwrap_or("");

    if plugin_major != expected_major {
        return Err(AppError::Validation(format!(
            "Plugin API version '{}' is incompatible. Expected major version '{}'",
            plugin.api_version, API_VERSION
        )));
    }

    Ok(())
}

pub fn load_all_plugins() -> Result<Vec<Plugin>, AppError> {
    let plugins_dir = get_plugins_dir()?;
    let mut plugins = vec![];

    let entries = fs::read_dir(&plugins_dir).map_err(AppError::Io)?;

    for entry in entries.flatten() {
        let plugin_dir = entry.path();

        if plugin_dir.is_dir() {
            match load_plugin_from_dir(plugin_dir) {
                Ok(plugin) => plugins.push(plugin),
                Err(e) => eprintln!("Skipping plugin: {}", e),
            }
        }
    }

    Ok(plugins)
}
