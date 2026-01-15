// src/state.rs
use crate::plugin_system::PluginManager;
use crate::video_player::player::MpvPlayer;
use crate::video_player::types::UserSettings;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub plugin_manager: Arc<Mutex<PluginManager>>,
    pub video_player: Arc<Mutex<Option<MpvPlayer>>>,
    pub user_settings: Arc<Mutex<UserSettings>>,
    pub mpv_config_path: PathBuf,
}

impl AppState {
    pub fn new(plugin_manager: PluginManager, mpv_config_path: PathBuf) -> Self {
        Self {
            plugin_manager: Arc::new(Mutex::new(plugin_manager)),
            video_player: Arc::new(Mutex::new(None)),
            user_settings: Arc::new(Mutex::new(UserSettings::default())),
            mpv_config_path,
        }
    }
}
