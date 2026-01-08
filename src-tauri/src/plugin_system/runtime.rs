use extism::{Manifest, Plugin, Wasm};
use serde_json::Value;
use std::collections::HashMap;

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
        let result = plugin.call::<&str, &str>(function_name, &args_json)?;
        let value: Value = serde_json::from_str(result)?;
        Ok(value)
    }
}
