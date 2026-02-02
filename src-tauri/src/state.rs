use crate::db::settings::SettingsManager;
use crate::db::Database;
use crate::plugin_system::PluginManager;
use crate::video_player::player::MpvPlayer;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub plugin_manager: Arc<Mutex<PluginManager>>,
    pub video_player: Arc<Mutex<Option<MpvPlayer>>>,
    // TODO remove? needed?
    _database: Option<Arc<Database>>,
    settings_manager: Option<SettingsManager>,
}

impl AppState {
    pub fn new(plugin_manager: PluginManager, database: Option<Database>) -> Self {
        let database_arc = database.map(Arc::new);
        let settings_manager = database_arc
            .as_ref()
            .map(|db_arc| SettingsManager::new(db_arc.clone()));

        Self {
            plugin_manager: Arc::new(Mutex::new(plugin_manager)),
            video_player: Arc::new(Mutex::new(None)),
            _database: database_arc,
            settings_manager,
        }
    }

    pub fn get_settings_manager(&self) -> Option<&SettingsManager> {
        self.settings_manager.as_ref()
    }
}
