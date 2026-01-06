use serde_json::Value;
use std::fs;
use std::path::PathBuf;

use crate::plugin_system::types::Plugin;
use crate::utils::get_plugins_dir;  

impl Plugin {
    pub fn new(plugin_dir: PathBuf) -> Result<Self, String> {
        let manifest_path = plugin_dir.join("manifest.json");

        if !manifest_path.exists() || !manifest_path.is_file() {
            return Err(format!("Missing or invalid manifest in {:?}", plugin_dir));
        }

        let manifest_content = fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read manifest: {}", e))?;

        let manifest: Value = serde_json::from_str(&manifest_content)
            .map_err(|e| format!("Failed to parse manifest JSON: {}", e))?;

        let name = manifest["name"]
            .as_str()
            .ok_or_else(|| "Missing 'name' field in manifest".to_string())?
            .to_string();

        let js_filename = match manifest["pathname"].as_str() {
            Some(pathname) => pathname.to_string(),
            None => format!("{}.js", name),
        };

        let filename_for_error = &js_filename;

        let js_path = plugin_dir.join(&js_filename);

        if !js_path.exists() || !js_path.is_file() {
            return Err(format!(
                "Missing or invalid js file '{}' in {:?}",
                filename_for_error, plugin_dir
            ));
        }

        Ok(Plugin {
            name,
            js_path,
            manifest_path,
            plugin_dir,
        })
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
