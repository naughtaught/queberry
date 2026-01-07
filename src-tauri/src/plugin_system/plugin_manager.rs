use std::fs;
use std::path::PathBuf;

use crate::constants::API_VERSION;
use crate::plugin_system::types::Plugin;
use crate::utils::get_plugins_dir;

impl Plugin {
    fn validate_plugin(&self) -> Result<(), String> {
        if !self.id.contains('.') {
            return Err(
                "Plugin ID must be in reverse-domain format (e.g., 'org.example.plugin')".into(),
            );
        }

        if self.name.trim().is_empty() {
            return Err("Plugin name cannot be empty".into());
        }

        // Check for WASM file extension instead of JS
        if !self.filename.ends_with(".wasm") {
            return Err("Plugin filename must end with .wasm".into());
        }

        if self.version.trim().is_empty() {
            return Err("Version cannot be empty".into());
        }

        if self.sources.is_empty() {
            return Err("Plugin must specify at least one source type".into());
        }

        if self.types.is_empty() {
            return Err("Plugin must specify at least one type".into());
        }

        if !self.api_version.starts_with('v') {
            return Err("API version must start with 'v'".into());
        }

        let plugin_major = self.api_version.split('.').next().unwrap_or("");
        let expected_major = API_VERSION.split('.').next().unwrap_or("");

        if plugin_major != expected_major {
            return Err(format!(
                "Plugin API version '{}' is incompatible. Expected major version '{}'",
                self.api_version, API_VERSION
            ));
        }

        Ok(())
    }

    pub fn new(plugin_dir: PathBuf) -> Result<Self, String> {
        let manifest_path = plugin_dir.join("manifest.json");

        let manifest_content = fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read manifest: {}", e))?;

        let plugin: Plugin = serde_json::from_str(&manifest_content)
            .map_err(|e| format!("Failed to parse manifest: {}", e))?;

        // Check for WASM file instead of JS
        let wasm_path = plugin_dir.join(&plugin.filename);
        if !wasm_path.exists() {
            return Err(format!(
                "Plugin WASM file '{}' not found in {}",
                plugin.filename,
                plugin_dir.display()
            ));
        }

        plugin.validate_plugin()?;

        Ok(plugin)
    }
}

pub fn load_all_plugins() -> Result<Vec<Plugin>, String> {
    let plugins_dir = get_plugins_dir()?;

    let mut plugins = vec![];

    let entries = fs::read_dir(&plugins_dir)
        .map_err(|e| format!("Failed to read plugins directory: {}", e))?;

    for entry in entries.flatten() {
        let plugin_dir = entry.path();

        if plugin_dir.is_dir() {
            match Plugin::new(plugin_dir) {
                Ok(plugin) => plugins.push(plugin),
                Err(e) => eprintln!("Skipping plugin: {}", e),
            }
        }
    }

    Ok(plugins)
}
