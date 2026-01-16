// src/state.rs
use crate::plugin_system::PluginManager;
use crate::video_player::player::MpvPlayer;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub plugin_manager: Arc<Mutex<PluginManager>>,
    pub video_player: Arc<Mutex<Option<MpvPlayer>>>,
}

impl AppState {
    pub fn new(plugin_manager: PluginManager) -> Self {
        Self {
            plugin_manager: Arc::new(Mutex::new(plugin_manager)),
            video_player: Arc::new(Mutex::new(None)),
        }
    }
}
