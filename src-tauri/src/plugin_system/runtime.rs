use crate::constants::DEFAULT_MAX_MEMORY_PAGES;
use crate::errors::classify_plugin_error;
use crate::AppError;
use dashmap::DashMap;
use extism::{Manifest, Plugin, Wasm};
use serde_json::Value;
use std::panic::{self, AssertUnwindSafe};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct PluginRuntime {
    pub plugins: DashMap<String, Arc<Mutex<Plugin>>>,
    pub default_timeout_ms: u64,
    plugin_timeouts: DashMap<String, u64>,
    default_max_memory_pages: Option<u32>,
    plugin_memory_limits: DashMap<String, u32>,
}

impl PluginRuntime {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            plugins: DashMap::new(),
            default_timeout_ms: 30_000,
            plugin_timeouts: DashMap::new(),
            default_max_memory_pages: Some(DEFAULT_MAX_MEMORY_PAGES),
            plugin_memory_limits: DashMap::new(),
        })
    }

    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.default_timeout_ms = timeout_ms;
        self
    }

    pub fn with_memory_limit(mut self, max_pages: Option<u32>) -> Self {
        self.default_max_memory_pages = max_pages;
        self
    }

    pub fn set_plugin_timeout(&self, plugin_id: &str, timeout_ms: u64) {
        self.plugin_timeouts
            .insert(plugin_id.to_string(), timeout_ms);
    }

    pub fn set_plugin_memory_limit(&self, plugin_id: &str, max_pages: u32) {
        self.plugin_memory_limits
            .insert(plugin_id.to_string(), max_pages);
    }

    pub fn get_plugin_timeout(&self, plugin_id: &str) -> u64 {
        self.plugin_timeouts
            .get(plugin_id)
            .map(|v| *v)
            .unwrap_or(self.default_timeout_ms)
    }

    pub fn get_plugin_memory_limit(&self, plugin_id: &str) -> Option<u32> {
        self.plugin_memory_limits
            .get(plugin_id)
            .map(|v| *v)
            .or(self.default_max_memory_pages)
    }

    pub fn load_plugin(
        &self,
        plugin_id: String,
        wasm_bytes: &[u8],
        allowed_hosts: &[String],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let plugin_id_clone = plugin_id.clone();

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            let timeout_ms = self.get_plugin_timeout(&plugin_id);
            let memory_limit = self.get_plugin_memory_limit(&plugin_id);

            let mut manifest = Manifest::new([Wasm::data(wasm_bytes)])
                .with_timeout(Duration::from_millis(timeout_ms));

            if let Some(max_pages) = memory_limit {
                manifest = manifest.with_memory_max(max_pages);
            }

            for host in allowed_hosts {
                manifest = manifest.with_allowed_host(host);
            }

            let plugin = Plugin::new(&manifest, [], true).map_err(|e| {
                format!(
                    "Failed to load plugin '{}': {}. This may indicate an invalid WASM module.",
                    plugin_id, e
                )
            })?;

            self.plugins.insert(plugin_id, Arc::new(Mutex::new(plugin)));

            Ok(())
        }));

        match result {
            Ok(inner_result) => inner_result,
            Err(panic_err) => {
                let panic_msg = if let Some(s) = panic_err.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = panic_err.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Unknown panic".to_string()
                };

                Err(format!(
                    "Plugin '{}' crashed during loading. This plugin is incompatible or corrupt. Error: {}",
                    plugin_id_clone, panic_msg
                ).into())
            }
        }
    }

    pub fn execute_plugin_method(
        &self,
        plugin_id: &str,
        function_name: &str,
        args: Vec<Value>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let plugin_id_owned = plugin_id.to_string();
        let function_name_owned = function_name.to_string();

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            let memory_limit = self.get_plugin_memory_limit(plugin_id);
            let timeout = self.get_plugin_timeout(plugin_id);

            let plugin_arc = self
                .plugins
                .get(plugin_id)
                .ok_or_else(|| format!("Plugin not loaded: {}", plugin_id))?;

            let mut plugin = plugin_arc
                .lock()
                .map_err(|e| format!("Failed to acquire plugin lock for '{}': {}", plugin_id, e))?;

            let args_json = serde_json::to_string(&args)?;

            let result = plugin
                .call::<&str, &str>(function_name, &args_json)
                .map_err(|e| {
                    classify_plugin_error(
                        plugin_id,
                        function_name,
                        &e.to_string(),
                        timeout,
                        memory_limit,
                    )
                })?;

            let value: Value =
                serde_json::from_str(result).map_err(|e| AppError::PluginInvalidOutput {
                    plugin_id: plugin_id.to_string(),
                    details: format!("Invalid JSON: {}. Raw output: {}", e, result),
                })?;

            Ok(value)
        }));

        match result {
            Ok(inner_result) => inner_result,
            Err(panic_err) => {
                let panic_msg = if let Some(s) = panic_err.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = panic_err.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Unknown panic".to_string()
                };

                self.plugins.remove(&plugin_id_owned);

                Err(Box::new(AppError::PluginCrashed {
                    plugin_id: plugin_id_owned,
                    details: format!(
                        "Plugin panicked during execution of '{}': {}. The plugin has been unloaded for safety.",
                        function_name_owned, panic_msg
                    ),
                }))
            }
        }
    }

    pub fn unload_plugin(&self, plugin_id: &str) {
        self.plugins.remove(plugin_id);
        self.plugin_timeouts.remove(plugin_id);
        self.plugin_memory_limits.remove(plugin_id);
    }
}
