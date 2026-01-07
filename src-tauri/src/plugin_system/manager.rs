use crate::plugin_system::runtime::PluginRuntime;
use crate::plugin_system::types::{Plugin, PluginType};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct PluginManager {
    indexer_plugins: HashMap<String, Plugin>,
    resolver_plugins: HashMap<String, Plugin>,
    plugins_dir: PathBuf,
    runtime: PluginRuntime,
    wasm_cache: HashMap<String, Vec<u8>>,
}

// Helper function to get plugin ID as string
fn plugin_id_string(plugin: &Plugin) -> String {
    plugin.id.to_string()
}

impl PluginManager {
    pub fn new(plugins_dir: PathBuf) -> Self {
        let runtime = PluginRuntime::new().expect("Failed to create WASM runtime");

        Self {
            indexer_plugins: HashMap::new(),
            resolver_plugins: HashMap::new(),
            plugins_dir,
            runtime,
            wasm_cache: HashMap::new(),
        }
    }

    pub fn load_plugin(&mut self, plugin: Plugin) {
        let plugin_id_str = plugin_id_string(&plugin);
        let wasm_path = self.plugins_dir.join(&plugin_id_str).join(&plugin.filename);

        if let Ok(wasm_bytes) = fs::read(&wasm_path) {
            self.wasm_cache.insert(plugin_id_str.clone(), wasm_bytes);

            if plugin.types.contains(&PluginType::Indexer) {
                self.indexer_plugins
                    .insert(plugin_id_str.clone(), plugin.clone());
            }

            if plugin.types.contains(&PluginType::Resolver) {
                self.resolver_plugins.insert(plugin_id_str, plugin);
            }
        } else {
            eprintln!("Failed to read WASM file: {}", wasm_path.display());
        }
    }

    pub fn call_plugin_method(
        &self,
        plugin_name: &str,
        interface_method: &str,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        let plugin = self
            .indexer_plugins
            .get(plugin_name)
            .or_else(|| self.resolver_plugins.get(plugin_name))
            .ok_or_else(|| format!("Plugin not found: {}", plugin_name))?;

        let plugin_method = plugin
            .methods
            .iter()
            .find(|m| m.interface_method == interface_method)
            .map(|m| m.plugin_method.as_str())
            .ok_or_else(|| format!("Method not found: {}", interface_method))?;

        let wasm_bytes = self
            .wasm_cache
            .get(&plugin_id_string(plugin))
            .ok_or_else(|| format!("WASM not cached: {}", plugin_id_string(plugin)))?;

        // Pass allowed network hosts from plugin permissions
        self.runtime
            .execute_plugin_method(wasm_bytes, plugin_method, args, &plugin.permissions.network)
            .map_err(|e| format!("Runtime error: {}", e))
    }
}
