use extism::{Manifest, Plugin, Wasm};
use serde_json::Value;
use std::collections::HashMap;
use url::Url;

pub struct PluginRuntime {
    pub plugins: HashMap<String, Plugin>,
}

impl PluginRuntime {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            plugins: HashMap::new(),
        })
    }

    pub fn load_plugin(
        &mut self,
        plugin_id: String,
        wasm_bytes: &[u8],
        allowed_hosts: &[String],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut manifest = Manifest::new([Wasm::data(wasm_bytes)]);

        // Process each allowed host from the manifest
        for host_pattern in allowed_hosts {
            // Extract the domain from the URL pattern
            let domain = self.extract_domain_from_url_pattern(host_pattern)?;

            // Allow the domain
            manifest = manifest.with_allowed_host(&domain);

            // Log for debugging
            println!(
                "Plugin '{}' allowed to access: {} (from pattern: {})",
                plugin_id, domain, host_pattern
            );
        }

        // If no network permissions are specified, the plugin gets no network access
        if allowed_hosts.is_empty() {
            println!(
                "Plugin '{}' has no network permissions - network access disabled",
                plugin_id
            );
        }

        let plugin = Plugin::new(&manifest, [], true)?;
        self.plugins.insert(plugin_id, plugin);

        Ok(())
    }

    fn extract_domain_from_url_pattern(
        &self,
        pattern: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Remove wildcard suffix if present (e.g., "/*" or "*")
        let mut cleaned = pattern.trim();
        if cleaned.ends_with("/*") {
            cleaned = &cleaned[..cleaned.len() - 2];
        } else if cleaned.ends_with('*') {
            cleaned = &cleaned[..cleaned.len() - 1];
        }

        // Ensure it ends with / for parsing
        let to_parse = if cleaned.ends_with('/') {
            cleaned.to_string()
        } else {
            format!("{}/", cleaned)
        };

        // Parse as URL
        let url = Url::parse(&to_parse)
            .map_err(|e| format!("Failed to parse URL pattern '{}': {}", pattern, e))?;

        // Extract host
        let host = url
            .host_str()
            .ok_or_else(|| format!("No host found in URL pattern: {}", pattern))?
            .to_string();

        Ok(host)
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
        let result = plugin.call::<&str, &str>(function_name, &args_json)?;
        let value: Value = serde_json::from_str(result)?;
        Ok(value)
    }
}
