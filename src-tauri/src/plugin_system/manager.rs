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
}

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
        }
    }

    pub fn register_plugin(&mut self, plugin: Plugin) {
        let plugin_id_str = plugin_id_string(&plugin);

        if plugin.types.contains(&PluginType::Indexer) {
            self.indexer_plugins
                .insert(plugin_id_str.clone(), plugin.clone());
        }

        if plugin.types.contains(&PluginType::Resolver) {
            self.resolver_plugins.insert(plugin_id_str, plugin);
        }
    }

    fn ensure_plugin_loaded(&mut self, plugin_id: &str) -> Result<(), String> {
        if self.runtime.plugins.contains_key(plugin_id) {
            return Ok(());
        }

        let plugin = self
            .indexer_plugins
            .get(plugin_id)
            .or_else(|| self.resolver_plugins.get(plugin_id))
            .ok_or_else(|| format!("Plugin not found: {}", plugin_id))?;

        let wasm_path = self.plugins_dir.join(plugin_id).join(&plugin.filename);
        let wasm_bytes =
            fs::read(&wasm_path).map_err(|e| format!("Failed to read WASM file: {}", e))?;

        self.runtime
            .load_plugin(
                plugin_id.to_string(),
                &wasm_bytes,
                &plugin.permissions.network,
            )
            .map_err(|e| format!("Failed to load plugin into runtime: {}", e))?;

        Ok(())
    }

    pub fn call_plugin_method(
        &mut self,
        plugin_name: &str,
        interface_method: &str,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        self.ensure_plugin_loaded(plugin_name)?;

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

        let plugin_id = plugin_id_string(plugin);

        self.runtime
            .execute_plugin_method(&plugin_id, plugin_method, args)
            .map_err(|e| format!("Runtime error: {}", e))
    }
}
