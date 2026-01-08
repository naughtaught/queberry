use crate::constants::{INDEXER_RATE_LIMIT, RATE_LIMIT_WINDOW_SECONDS, RESOLVER_RATE_LIMIT};
use crate::errors::AppError;
use crate::plugin_system::rate_limiter::RateLimiter;
use crate::plugin_system::runtime::PluginRuntime;
use crate::plugin_system::types::{Plugin, PluginType};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub struct PluginManager {
    indexer_plugins: HashMap<String, Plugin>,
    resolver_plugins: HashMap<String, Plugin>,
    plugins_dir: PathBuf,
    runtime: Arc<RwLock<PluginRuntime>>,
    rate_limiter: RateLimiter,
}

fn plugin_id_string(plugin: &Plugin) -> String {
    plugin.id.to_string()
}

impl PluginManager {
    pub fn new(plugins_dir: PathBuf) -> Self {
        let runtime = PluginRuntime::new().expect("Failed to create WASM runtime");
        let rate_limiter = RateLimiter::new().with_window_seconds(RATE_LIMIT_WINDOW_SECONDS);

        Self {
            indexer_plugins: HashMap::new(),
            resolver_plugins: HashMap::new(),
            plugins_dir,
            runtime: Arc::new(RwLock::new(runtime)),
            rate_limiter,
        }
    }

    pub fn register_plugin(&mut self, plugin: Plugin) {
        let plugin_id_str = plugin_id_string(&plugin);

        let rate_limit = if plugin.types.contains(&PluginType::Indexer) {
            INDEXER_RATE_LIMIT
        } else {
            RESOLVER_RATE_LIMIT
        };

        self.rate_limiter.set_limit(&plugin_id_str, rate_limit);

        let is_indexer = plugin.types.contains(&PluginType::Indexer);
        let is_resolver = plugin.types.contains(&PluginType::Resolver);

        match (is_indexer, is_resolver) {
            (true, true) => {
                self.indexer_plugins
                    .insert(plugin_id_str.clone(), plugin.clone());
                self.resolver_plugins.insert(plugin_id_str, plugin);
            }
            (true, false) => {
                self.indexer_plugins.insert(plugin_id_str, plugin);
            }
            (false, true) => {
                self.resolver_plugins.insert(plugin_id_str, plugin);
            }
            (false, false) => {
                eprintln!("Warning: Plugin '{}' has no valid types", plugin_id_str);
            }
        }
    }

    fn ensure_plugin_loaded(&mut self, plugin_id: &str) -> Result<(), AppError> {
        {
            let runtime_guard = self
                .runtime
                .read()
                .unwrap_or_else(|poisoned| poisoned.into_inner());

            if runtime_guard.plugins.contains_key(plugin_id) {
                return Ok(());
            }
        }

        let plugin = self
            .indexer_plugins
            .get(plugin_id)
            .or_else(|| self.resolver_plugins.get(plugin_id))
            .ok_or_else(|| AppError::NotFound(format!("Plugin not found: {}", plugin_id)))?;

        let wasm_path = self.plugins_dir.join(plugin_id).join(&plugin.filename);
        let wasm_bytes = fs::read(&wasm_path).map_err(AppError::Io)?;

        let mut runtime_guard = self
            .runtime
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if !runtime_guard.plugins.contains_key(plugin_id) {
            runtime_guard
                .load_plugin(
                    plugin_id.to_string(),
                    &wasm_bytes,
                    &plugin.permissions.network,
                )
                .map_err(|e| AppError::Runtime(e.to_string()))?;
        }

        Ok(())
    }

    pub fn call_plugin_method(
        &mut self,
        plugin_name: &str,
        interface_method: &str,
        args: Vec<Value>,
    ) -> Result<Value, AppError> {
        self.rate_limiter
            .check_limit(plugin_name)
            .map_err(|e| AppError::RateLimit(e.to_string()))?;

        // Get the plugin ID and method name first before mutable operations
        let (plugin_id, plugin_method_name) = {
            let plugin = self
                .indexer_plugins
                .get(plugin_name)
                .or_else(|| self.resolver_plugins.get(plugin_name))
                .ok_or_else(|| AppError::NotFound(format!("Plugin not found: {}", plugin_name)))?;

            let plugin_method = plugin
                .methods
                .iter()
                .find(|m| m.interface_method == interface_method)
                .map(|m| m.plugin_method.as_str())
                .ok_or_else(|| {
                    AppError::NotFound(format!("Method not found: {}", interface_method))
                })?;

            (plugin_id_string(plugin), plugin_method.to_string())
        };

        self.ensure_plugin_loaded(&plugin_id)?;

        let mut runtime_guard = self
            .runtime
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        runtime_guard
            .execute_plugin_method(&plugin_id, &plugin_method_name, args)
            .map_err(|e| AppError::Runtime(e.to_string()))
    }
}
