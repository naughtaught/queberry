use extism::{Manifest, Plugin, Wasm};
use serde_json::Value;
use std::collections::HashMap;

pub struct PluginRuntime {
    // Cache compiled plugins by ID
    pub plugins: HashMap<String, Plugin>, // Made public for manager
}

impl PluginRuntime {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            plugins: HashMap::new(),
        })
    }

    /// Load and cache a plugin
    pub fn load_plugin(
        &mut self,
        plugin_id: String,
        wasm_bytes: &[u8],
        allowed_hosts: &[String],
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create manifest with allowed hosts
        let mut manifest = Manifest::new([Wasm::data(wasm_bytes)]);

        // Add all allowed hosts
        for host in allowed_hosts {
            let domain = host
                .trim_start_matches("https://")
                .trim_start_matches("http://")
                .trim_end_matches("/*")
                .trim_end_matches('/');
            manifest = manifest.with_allowed_host(domain);
        }

        // Create and cache the plugin
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
        // Get the cached plugin
        let plugin = self
            .plugins
            .get_mut(plugin_id)
            .ok_or_else(|| format!("Plugin not loaded: {}", plugin_id))?;

        // Serialize args to JSON
        let args_json = serde_json::to_string(&args)?;

        // Call the plugin function
        let result = plugin.call::<&str, &str>(function_name, &args_json)?;

        // Parse result
        let value: Value = serde_json::from_str(result)?;
        Ok(value)
    }
}
