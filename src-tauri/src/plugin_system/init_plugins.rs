use crate::plugin_system::loader::load_all_plugins;
use crate::plugin_system::PluginManager;
use crate::utils::plugin_dir::get_plugins_dir;

pub fn init_plugins() -> Result<PluginManager, Box<dyn std::error::Error>> {
    let plugins_dir = get_plugins_dir()?;

    let mut plugin_manager = PluginManager::new(plugins_dir);

    for plugin in load_all_plugins()? {
        if let Err(e) = plugin_manager.register_plugin(plugin) {
            eprintln!("Failed to register plugin: {}", e);
        }
    }

    Ok(plugin_manager)
}
