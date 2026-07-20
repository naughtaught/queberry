use crate::db::plugin_cache::PluginCacheManager;
use crate::db::Database;
use crate::plugin_system::loader::load_all_plugins_cached;
use crate::plugin_system::PluginManager;
use crate::utils::plugin_dir::get_plugins_dir;
use crate::AppError;
use std::sync::Arc;

pub async fn init_plugins(db: Arc<Database>) -> Result<PluginManager, Box<dyn std::error::Error>> {
    let plugins_dir = get_plugins_dir()?;
    let cache = PluginCacheManager::new(db);
    let plugin_manager = PluginManager::new(plugins_dir);

    for plugin in load_all_plugins_cached(&cache).await? {
        if let Err(e) = plugin_manager.register_plugin(plugin).await {
            AppError::Runtime(format!("Failed to register plugin: {}", e)).log("init_plugins");
        }
    }

    Ok(plugin_manager)
}
