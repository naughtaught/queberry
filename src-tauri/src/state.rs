use crate::db::settings::SettingsManager;
use crate::db::Database;
use crate::plugin_system::PluginManager;
use crate::video_player::player::MpvPlayer;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub plugin_manager: Arc<Mutex<PluginManager>>,
    pub video_player: Arc<Mutex<Option<MpvPlayer>>>,
    pub database: Arc<Mutex<Option<Database>>>,
    settings_manager: Option<SettingsManager>,
}

impl AppState {
    pub fn new(plugin_manager: PluginManager, database: Option<Database>) -> Self {
        let settings_manager = database.as_ref().map(|db| SettingsManager::new(db.clone()));

        Self {
            plugin_manager: Arc::new(Mutex::new(plugin_manager)),
            video_player: Arc::new(Mutex::new(None)),
            database: Arc::new(Mutex::new(database)),
            settings_manager,
        }
    }

    pub fn get_settings_manager(&self) -> Option<&SettingsManager> {
        self.settings_manager.as_ref()
    }
}
