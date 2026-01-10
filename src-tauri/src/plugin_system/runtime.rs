use extism::{Manifest, Plugin, Wasm};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

pub struct PluginRuntime {
    pub plugins: HashMap<String, Plugin>,
    pub default_timeout_ms: u64,
    plugin_timeouts: HashMap<String, u64>,
}

impl PluginRuntime {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            plugins: HashMap::new(),
            default_timeout_ms: 30_000,
            plugin_timeouts: HashMap::new(),
        })
    }

    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.default_timeout_ms = timeout_ms;
        self
    }

    /// Set a custom timeout for a specific plugin
    pub fn set_plugin_timeout(&mut self, plugin_id: &str, timeout_ms: u64) {
        self.plugin_timeouts
            .insert(plugin_id.to_string(), timeout_ms);
    }

    pub fn get_plugin_timeout(&self, plugin_id: &str) -> u64 {
        self.plugin_timeouts
            .get(plugin_id)
            .copied()
            .unwrap_or(self.default_timeout_ms)
    }

    pub fn load_plugin(
        &mut self,
        plugin_id: String,
        wasm_bytes: &[u8],
        allowed_hosts: &[String],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let timeout_ms = self.get_plugin_timeout(&plugin_id);

        let mut manifest =
            Manifest::new([Wasm::data(wasm_bytes)]).with_timeout(Duration::from_millis(timeout_ms));

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

        let plugin = Plugin::new(&manifest, [], true)?;
        self.plugins.insert(plugin_id, plugin);

        Ok(())
    }

    pub fn execute_plugin_method(
        &mut self,
        plugin_id: &str,
        function_name: &str,
        args: Vec<Value>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let plugin = self
            .plugins
            .get_mut(plugin_id)
            .ok_or_else(|| format!("Plugin not loaded: {}", plugin_id))?;

        let args_json = serde_json::to_string(&args)?;

        let result = plugin
            .call::<&str, &str>(function_name, &args_json)
            .map_err(|e| {
                let err_msg = e.to_string();
                if err_msg.contains("timeout") || err_msg.contains("Timeout") {
                    format!(
                        "Plugin '{}' method '{}' exceeded timeout limit",
                        plugin_id, function_name
                    )
                } else {
                    err_msg
                }
            })?;

        let value: Value = serde_json::from_str(result)?;
        Ok(value)
    }
}
