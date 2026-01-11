use crate::constants::{pages_to_bytes, DEFAULT_MAX_MEMORY_PAGES};
use extism::{Manifest, Plugin, Wasm};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

pub struct PluginRuntime {
    pub plugins: HashMap<String, Plugin>,
    pub default_timeout_ms: u64,
    plugin_timeouts: HashMap<String, u64>,
    default_max_memory_pages: Option<u32>,
    plugin_memory_limits: HashMap<String, u32>,
}

impl PluginRuntime {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            plugins: HashMap::new(),
            default_timeout_ms: 30_000,
            plugin_timeouts: HashMap::new(),
            default_max_memory_pages: Some(DEFAULT_MAX_MEMORY_PAGES),
            plugin_memory_limits: HashMap::new(),
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

    pub fn set_plugin_timeout(&mut self, plugin_id: &str, timeout_ms: u64) {
        self.plugin_timeouts
            .insert(plugin_id.to_string(), timeout_ms);
    }

    pub fn set_plugin_memory_limit(&mut self, plugin_id: &str, max_pages: u32) {
        self.plugin_memory_limits
            .insert(plugin_id.to_string(), max_pages);
    }

    pub fn get_plugin_timeout(&self, plugin_id: &str) -> u64 {
        self.plugin_timeouts
            .get(plugin_id)
            .copied()
            .unwrap_or(self.default_timeout_ms)
    }

    pub fn get_plugin_memory_limit(&self, plugin_id: &str) -> Option<u32> {
        self.plugin_memory_limits
            .get(plugin_id)
            .copied()
            .or(self.default_max_memory_pages)
    }

    pub fn load_plugin(
        &mut self,
        plugin_id: String,
        wasm_bytes: &[u8],
        allowed_hosts: &[String],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let timeout_ms = self.get_plugin_timeout(&plugin_id);
        let memory_limit = self.get_plugin_memory_limit(&plugin_id);

        // Create manifest with WASM data
        let mut manifest =
            Manifest::new([Wasm::data(wasm_bytes)]).with_timeout(Duration::from_millis(timeout_ms));

        // Apply memory limits if configured
        if let Some(max_pages) = memory_limit {
            manifest = manifest.with_memory_max(max_pages);

            let max_bytes = pages_to_bytes(max_pages);
            let max_mb = max_bytes as f64 / (1024.0 * 1024.0);

            println!(
                "Plugin '{}' loaded with {}MB memory limit ({} pages)",
                plugin_id, max_mb, max_pages
            );
        } else {
            println!(
                "Plugin '{}' loaded with NO memory limit (WARNING: may cause OOM)",
                plugin_id
            );
        }

        // Configure network permissions
        for host in allowed_hosts {
            manifest = manifest.with_allowed_host(host);
            println!("Plugin '{}' allowed to access: {}", plugin_id, host);
        }

        if allowed_hosts.is_empty() {
            println!(
                "Plugin '{}' has no network permissions - network access disabled",
                plugin_id
            );
        }

        println!(
            "Plugin '{}' loaded with {}ms timeout",
            plugin_id, timeout_ms
        );

        // Create the plugin instance
        let plugin = Plugin::new(&manifest, [], true).map_err(|e| {
            format!(
                "Failed to load plugin '{}': {}. This may indicate an invalid WASM module or incompatible memory configuration.",
                plugin_id, e
            )
        })?;

        self.plugins.insert(plugin_id, plugin);

        Ok(())
    }

    pub fn execute_plugin_method(
        &mut self,
        plugin_id: &str,
        function_name: &str,
        args: Vec<Value>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let memory_limit = self.get_plugin_memory_limit(plugin_id);
        let timeout = self.get_plugin_timeout(plugin_id);

        let plugin = self
            .plugins
            .get_mut(plugin_id)
            .ok_or_else(|| format!("Plugin not loaded: {}", plugin_id))?;

        let args_json = serde_json::to_string(&args)?;

        let result = plugin
            .call::<&str, &str>(function_name, &args_json)
            .map_err(|e| {
                let err_msg = e.to_string();

                // Categorize errors for better user feedback
                if err_msg.contains("timeout") || err_msg.contains("Timeout") {
                    format!(
                        "Plugin '{}' method '{}' exceeded timeout limit ({}ms). \
                        Consider optimizing the plugin or increasing the timeout.",
                        plugin_id,
                        function_name,
                        timeout // Use captured timeout value
                    )
                } else if err_msg.contains("memory")
                    || err_msg.contains("Memory")
                    || err_msg.contains("out of memory")
                    || err_msg.contains("OOM")
                {
                    let limit_str = memory_limit
                        .map(|pages| {
                            let mb = pages_to_bytes(pages) as f64 / (1024.0 * 1024.0);
                            format!("{}MB", mb)
                        })
                        .unwrap_or_else(|| "unlimited".to_string());

                    format!(
                        "Plugin '{}' exceeded memory limit ({}). \
                        This plugin may be malicious or poorly written. \
                        Consider uninstalling it.",
                        plugin_id, limit_str
                    )
                } else if err_msg.contains("unreachable") || err_msg.contains("trap") {
                    format!(
                        "Plugin '{}' encountered a fatal error (trap/unreachable). \
                        This plugin is likely buggy or malicious.",
                        plugin_id
                    )
                } else {
                    format!(
                        "Plugin '{}' method '{}' failed: {}",
                        plugin_id, function_name, err_msg
                    )
                }
            })?;

        let value: Value = serde_json::from_str(result).map_err(|e| {
            format!(
                "Plugin '{}' returned invalid JSON: {}. Raw output: {}",
                plugin_id, e, result
            )
        })?;

        Ok(value)
    }
}
