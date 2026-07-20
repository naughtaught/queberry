use crate::constants::API_VERSION;
use crate::db::plugin_cache::PluginCacheManager;
use crate::errors::AppError;
use crate::plugin_system::types::{
    ApiKeyRequirement, MethodMapping, Plugin, PluginPermissions, PluginRateLimit, PluginType,
    SourceType,
};
use crate::utils::get_plugins_dir;
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::OnceLock;
use url::Url;

static PLUGIN_ID_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_plugin_id_regex() -> &'static Regex {
    PLUGIN_ID_REGEX.get_or_init(|| {
        Regex::new(r"^[a-z0-9]+(\.[a-z0-9]+)+$").expect("Failed to compile plugin ID regex")
    })
}

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
        #[serde(alias = "cacheless", alias = "cacheLess", alias = "cache-less")]
        cacheless: bool,
        #[serde(
            alias = "requiresApiKey",
            alias = "requires_api_key",
            alias = "requires-apikey"
        )]
        requires_api_key: ApiKeyRequirement,
        permissions: RawPermissions,
        #[serde(alias = "api_version", alias = "apiVersion", alias = "api-version")]
        api_version: String,
        methods: Vec<MethodMapping>,
        #[serde(default)]
        rate_limit: Option<PluginRateLimit>,
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
        cacheless: raw_plugin.cacheless,
        requires_api_key: raw_plugin.requires_api_key,
        permissions: PluginPermissions {
            validated_hosts,
            network_patterns: raw_plugin.permissions.network,
            allow_private_networks: raw_plugin.permissions.allow_private_networks,
        },
        api_version: raw_plugin.api_version,
        methods: raw_plugin.methods,
        rate_limit: raw_plugin.rate_limit,
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

pub fn validate_and_extract_host(pattern: &str, allow_private: bool) -> Result<String, AppError> {
    let trimmed = pattern.trim();

    let (has_wildcard, host_to_validate) = if let Some(domain) = trimmed.strip_prefix("*.") {
        (true, domain.trim_end_matches('.').to_string())
    } else {
        (false, trimmed.trim_end_matches('.').to_string())
    };

    let url_str = match host_to_validate.contains("://") {
        true => host_to_validate,
        false => format!("https://{host_to_validate}"),
    };

    let url = Url::parse(&url_str)
        .map_err(|e| AppError::Validation(format!("Invalid URL '{}': {}", pattern, e)))?;

    if url.scheme() != "http" && url.scheme() != "https" {
        return Err(AppError::Validation(format!(
            "URL must use http:// or https:// scheme: {}",
            pattern
        )));
    }

    let host = url
        .host_str()
        .ok_or_else(|| AppError::Validation(format!("URL must have a host: {}", pattern)))?;

    let host = host.trim_end_matches('.').to_string();

    if let Ok(ip_addr) = host.parse::<IpAddr>() {
        if !allow_private && is_private_ip(&ip_addr) {
            return Err(AppError::Validation(format!(
                "Access to private/local networks not allowed: {}. Set 'allow_private_networks: true' if needed.",
                pattern
            )));
        }
        return Ok(host);
    }

    if has_wildcard {
        if host.is_empty() || host == "*" || !host.contains('.') {
            return Err(AppError::Validation(format!(
                "Invalid wildcard pattern '{}'. Must specify a valid domain (e.g., '*.example.com')",
                pattern
            )));
        }

        if let Some(suffix) = psl::suffix(host.as_bytes()) {
            let suffix_str = std::str::from_utf8(suffix.as_bytes()).map_err(|_| {
                AppError::Validation(format!("Invalid UTF-8 in suffix: {:?}", suffix.as_bytes()))
            })?;

            if suffix_str == host {
                return Err(AppError::Validation(format!(
                    "Wildcard cannot be on a public suffix: {}",
                    pattern
                )));
            }
        }

        if psl::domain(host.as_bytes()).is_none() {
            return Err(AppError::Validation(format!("Invalid domain '{}'", host)));
        }

        if !allow_private && is_private_or_local_host(&host)? {
            return Err(AppError::Validation(format!(
                "Access to private/local networks not allowed: {}. Set 'allow_private_networks: true' if needed.",
                pattern
            )));
        }

        Ok(format!("*.{}", host))
    } else {
        if host.contains('*') {
            return Err(AppError::Validation(format!(
                "Wildcard must be at the beginning of the host: {}",
                pattern
            )));
        }

        if host == "*" || host == "*.*" || host.ends_with(".*") || host.contains("*.*") {
            return Err(AppError::Validation(format!(
                "Invalid wildcard pattern '{}'. Must specify a valid domain (e.g., '*.example.com')",
                pattern
            )));
        }

        if !host.contains('*') && psl::domain(host.as_bytes()).is_none() {
            return Err(AppError::Validation(format!("Invalid domain '{}'", host)));
        }

        if !allow_private && is_private_or_local_host(&host)? {
            return Err(AppError::Validation(format!(
                "Access to private/local networks not allowed: {}. Set 'allow_private_networks: true' if needed.",
                pattern
            )));
        }

        Ok(host)
    }
}

pub fn is_private_or_local_host(host: &str) -> Result<bool, AppError> {
    let host = host.trim_end_matches('.');

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

    if host.ends_with(".test")
        || host.ends_with(".example")
        || host.ends_with(".invalid")
        || host.ends_with(".localhost")
    {
        return Ok(true);
    }

    Ok(false)
}

pub fn is_private_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            // 10.0.0.0/8
            octets[0] == 10
            // 172.16.0.0/12
            || (octets[0] == 172 && (16..=31).contains(&octets[1]))
            // 192.168.0.0/16
            || (octets[0] == 192 && octets[1] == 168)
            // 127.0.0.0/8 (loopback)
            || octets[0] == 127
            // 169.254.0.0/16 (link-local)
            || (octets[0] == 169 && octets[1] == 254)
            // 0.0.0.0/8
            || octets[0] == 0
            // 100.64.0.0/10 (Carrier-grade NAT)
            || (octets[0] == 100 && (64..=127).contains(&octets[1]))
            // 192.0.0.0/24 (IETF Protocol Assignments)
            || (octets[0] == 192 && octets[1] == 0 && octets[2] == 0)
            // 192.0.2.0/24 (TEST-NET-1)
            || (octets[0] == 192 && octets[1] == 0 && octets[2] == 2)
            // 198.51.100.0/24 (TEST-NET-2)
            || (octets[0] == 198 && octets[1] == 51 && octets[2] == 100)
            // 203.0.113.0/24 (TEST-NET-3)
            || (octets[0] == 203 && octets[1] == 0 && octets[2] == 113)
            // 224.0.0.0/4 (Multicast)
            || (224..=239).contains(&octets[0])
            // 240.0.0.0/4 (Reserved)
            || octets[0] >= 240
        }
        IpAddr::V6(ipv6) => {
            // ::1 (loopback)
            ipv6.is_loopback()
            // fe80::/10 (link-local)
            || ((ipv6.segments()[0] & 0xffc0) == 0xfe80)
            // fc00::/7 (unique local)
            || ((ipv6.segments()[0] & 0xfe00) == 0xfc00)
            // ::ffff:0:0/96 (IPv4-mapped)
            || ipv6
                .to_ipv4_mapped()
                .is_some_and(|ipv4| is_private_ip(&IpAddr::V4(ipv4)))
            // ::/128 (unspecified)
            || ipv6.is_unspecified()
            // ff00::/8 (multicast)
            || (ipv6.segments()[0] & 0xff00) == 0xff00
        }
    }
}

pub fn validate_plugin(plugin: &Plugin) -> Result<(), AppError> {
    let id_regex = get_plugin_id_regex();
    if !id_regex.is_match(&plugin.id) {
        return Err(AppError::Validation(format!(
            "Plugin ID '{}' is invalid. Must be in reverse-domain format with lowercase letters/numbers only (e.g., 'com.example.plugin')",
            plugin.id
        )));
    }

    for segment in plugin.id.split('.') {
        if segment.is_empty() {
            return Err(AppError::Validation(
                "Plugin ID cannot have empty segments (e.g., 'com..plugin')".into(),
            ));
        }
        if segment.len() > 63 {
            return Err(AppError::Validation(format!(
                "Plugin ID segment '{}' is too long (max 63 characters per segment)",
                segment
            )));
        }
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

    for method in &plugin.methods {
        if method.interface_method.trim().is_empty() {
            return Err(AppError::Validation(
                "Interface method name cannot be empty".into(),
            ));
        }

        if method.plugin_method.trim().is_empty() {
            return Err(AppError::Validation(
                "Plugin method name cannot be empty".into(),
            ));
        }

        if !is_valid_method_name(&method.plugin_method) {
            return Err(AppError::Validation(format!(
                "Invalid plugin method name '{}'. Must contain only alphanumeric characters and underscores, starting with a letter",
                method.plugin_method
            )));
        }

        if method.requires_api_key && !plugin.requires_api_key.supports_api_keys() {
            return Err(AppError::Validation(format!(
                "Method '{}' requires API key but plugin '{}' has requires_api_key set to 'never'",
                method.interface_method, plugin.id
            )));
        }
    }

    Ok(())
}

fn is_valid_method_name(name: &str) -> bool {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        return false;
    }

    if trimmed.len() > 256 {
        return false;
    }

    if trimmed.contains('\x00') || trimmed.chars().any(|c| c.is_control()) {
        return false;
    }

    if !trimmed.chars().next().unwrap().is_ascii_alphabetic() {
        return false;
    }

    trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
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
                Err(e) => e.log("load_all_plugins"),
            }
        }
    }

    Ok(plugins)
}

pub async fn load_all_plugins_cached(cache: &PluginCacheManager) -> Result<Vec<Plugin>, AppError> {
    let plugins_dir = get_plugins_dir()?;
    let mut plugins = Vec::new();
    let mut present_ids = Vec::new();

    for entry in fs::read_dir(&plugins_dir).map_err(AppError::Io)?.flatten() {
        let plugin_dir = entry.path();
        if !plugin_dir.is_dir() {
            continue;
        }

        let manifest_path = plugin_dir.join("manifest.json");
        let manifest_content = match fs::read_to_string(&manifest_path) {
            Ok(c) => c,
            Err(_) => {
                continue;
            }
        };

        let dir_name = plugin_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let wasm_bytes_opt = quick_parse_filename(&manifest_content)
            .and_then(|fname| fs::read(plugin_dir.join(fname)).ok());

        if let Some(ref wasm_bytes) = wasm_bytes_opt {
            match cache
                .load_if_fresh(&dir_name, &manifest_content, wasm_bytes)
                .await
            {
                Ok(Some(plugin)) => {
                    present_ids.push(plugin.id.clone());
                    plugins.push(plugin);
                    continue;
                }
                Ok(None) => {}
                Err(e) => {
                    AppError::Runtime(format!(
                        "Cache read error for '{}': {} — falling back to disk",
                        dir_name, e
                    ))
                    .log("load_all_plugins_cached");
                }
            }
        }

        match load_plugin_from_dir(plugin_dir) {
            Ok(plugin) => {
                if let Some(ref wasm_bytes) = wasm_bytes_opt {
                    if let Err(e) = cache.upsert(&plugin, &manifest_content, wasm_bytes).await {
                        AppError::Runtime(format!(
                            "Failed to write plugin '{}' to cache: {}",
                            plugin.id, e
                        ))
                        .log("load_all_plugins_cached");
                    }
                }
                present_ids.push(plugin.id.clone());
                plugins.push(plugin);
            }
            Err(e) => e.log("load_all_plugins_cached"),
        }
    }

    if let Err(e) = cache.remove_missing(&present_ids).await {
        AppError::Runtime(format!("Failed to prune stale plugin cache rows: {}", e))
            .log("load_all_plugins_cached");
    }

    Ok(plugins)
}

fn quick_parse_filename(manifest: &str) -> Option<String> {
    let v: serde_json::Value = serde_json::from_str(manifest).ok()?;
    ["filename", "fileName", "file_name", "file-name"]
        .iter()
        .find_map(|&k| v.get(k)?.as_str().map(str::to_string))
}
