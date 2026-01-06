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

        if !self.filename.ends_with(".js") {
            return Err("Plugin filename must end with .js".into());
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

        // TODO TYPES and ensure matching to ENUM
        // for type_str in &self.types {
        //     type_str.parse::<PluginType>().map_err(|_| {
        //         format!(
        //             "Invalid plugin type '{}'. Valid types: {:?}",
        //             type_str,
        //             PluginType::VARIANTS
        //         )
        //     })?;
        // }

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

        let js_path = plugin_dir.join(&plugin.filename);
        if !js_path.exists() {
            return Err(format!(
                "Plugin JS file '{}' not found in {}",
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

    let dir = plugins_dir;

    let mut plugins = vec![];

    let entries =
        fs::read_dir(&dir).map_err(|e| format!("Failed to read plugins directory: {}", e))?;

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
