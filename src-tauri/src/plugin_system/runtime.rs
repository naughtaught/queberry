use extism::{Manifest, Plugin, Wasm};
use serde_json::Value;

pub struct PluginRuntime;

impl PluginRuntime {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self)
    }

    pub fn execute_plugin_method(
        &self,
        wasm_bytes: &[u8],
        function_name: &str,
        args: Vec<Value>,
        allowed_hosts: &[String], // Pass allowed hosts from plugin manifest
    ) -> Result<Value, Box<dyn std::error::Error>> {
        // Create manifest with allowed hosts
        let mut manifest = Manifest::new([Wasm::data(wasm_bytes)]);

        // Add all allowed hosts
        for host in allowed_hosts {
            // Extract domain from URL pattern like "https://zilean.elfhosted.com/*"
            let domain = host
                .trim_start_matches("https://")
                .trim_start_matches("http://")
                .trim_end_matches("/*")
                .trim_end_matches('/');
            manifest = manifest.with_allowed_host(domain);
        }

        let mut plugin = Plugin::new(&manifest, [], true)?;

        // Serialize args to JSON
        let args_json = serde_json::to_string(&args)?;

        // Call the plugin function
        let result = plugin.call::<&str, &str>(function_name, &args_json)?;

        // Parse result
        let value: Value = serde_json::from_str(result)?;
        Ok(value)
    }
}
