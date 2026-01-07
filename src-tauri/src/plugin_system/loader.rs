use crate::constants::API_VERSION;
use crate::errors::AppError;
use crate::plugin_system::types::Plugin;
use crate::utils::get_plugins_dir;
use std::fs;
use std::path::PathBuf;
use url::Url;

pub fn load_plugin_from_dir(plugin_dir: PathBuf) -> Result<Plugin, AppError> {
    let manifest_path = plugin_dir.join("manifest.json");

    let manifest_content = fs::read_to_string(&manifest_path).map_err(AppError::Io)?;

    let plugin: Plugin = serde_json::from_str(&manifest_content).map_err(AppError::Json)?;

    validate_plugin(&plugin)?;

    // Validate network permissions URLs
    for url in &plugin.permissions.network {
        validate_url(url)?;
    }

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

fn validate_url(url_str: &str) -> Result<(), AppError> {
    let url = Url::parse(url_str).map_err(AppError::Url)?;

    if url.scheme() != "http" && url.scheme() != "https" {
        return Err(AppError::Validation(format!(
            "URL must use http:// or https:// scheme: {}",
            url_str
        )));
    }

    if url.host_str().is_none() {
        return Err(AppError::Validation(format!(
            "URL must have a host: {}",
            url_str
        )));
    }

    Ok(())
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
