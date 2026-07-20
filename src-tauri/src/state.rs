use crate::db::blacklist::BlacklistManager;
use crate::db::content_ratings::ContentRatingsManager;
use crate::db::downloads::DownloadManager as DbDownloadManager;
use crate::db::global_settings::GlobalSettingsManager;
use crate::db::keyboard_shortcuts::KeyboardShortcutsManager;
use crate::db::keys::KeysManager;
use crate::db::local_media::LocalMediaManager;
use crate::db::plugin_cache::PluginCacheManager;
use crate::db::transfers::TransferManager;
use crate::db::Database;
use crate::db::{settings::SettingsManager, user::UserManager};
use crate::downloads::types::DownloadManager;
use crate::plugin_system::PluginManager;
use crate::video_player::player::MpvPlayer;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub plugin_manager: Arc<PluginManager>,
    pub video_player: Arc<Mutex<Option<MpvPlayer>>>,
    pub download_manager: Option<DownloadManager>,
    pub db_download_manager: Option<DbDownloadManager>,
    pub plugin_cache: Option<PluginCacheManager>,
    _database: Option<Arc<Database>>,
    settings_manager: Option<SettingsManager>,
    user_manager: Option<UserManager>,
    keys_manager: Option<KeysManager>,
    blacklist_manager: Option<BlacklistManager>,
    global_settings_manager: Option<GlobalSettingsManager>,
    content_ratings_manager: Option<ContentRatingsManager>,
    keyboard_shortcuts_manager: Option<KeyboardShortcutsManager>,
    transfers_manager: Option<TransferManager>,
    local_media_manager: Option<LocalMediaManager>,
}

impl AppState {
    pub fn new(plugin_manager: PluginManager, database: Option<Arc<Database>>) -> Self {
        let settings_manager = database.as_ref().map(|db| SettingsManager::new(db.clone()));
        let user_manager = database.as_ref().map(|db| UserManager::new(db.clone()));
        let keys_manager = database.as_ref().map(|db| KeysManager::new(db.clone()));
        let blacklist_manager = database
            .as_ref()
            .map(|db| BlacklistManager::new(db.clone()));
        let global_settings_manager = database
            .as_ref()
            .map(|db| GlobalSettingsManager::new(db.clone()));
        let content_ratings_manager = database
            .as_ref()
            .map(|db| ContentRatingsManager::new(db.clone()));
        let keyboard_shortcuts_manager = database
            .as_ref()
            .map(|db| KeyboardShortcutsManager::new(db.clone()));
        let transfers_manager = database.as_ref().map(|db| TransferManager::new(db.clone()));
        let local_media_manager = database
            .as_ref()
            .map(|db| LocalMediaManager::new(db.clone()));
        let db_download_manager = database
            .as_ref()
            .map(|db| DbDownloadManager::new(db.clone()));
        let download_manager = db_download_manager
            .as_ref()
            .map(|db| DownloadManager::new().with_db(Arc::new(db.clone())));
        let plugin_cache = database
            .as_ref()
            .map(|db| PluginCacheManager::new(db.clone()));

        Self {
            plugin_manager: Arc::new(plugin_manager),
            video_player: Arc::new(Mutex::new(None)),
            download_manager,
            db_download_manager,
            plugin_cache,
            _database: database,
            settings_manager,
            user_manager,
            keys_manager,
            blacklist_manager,
            global_settings_manager,
            content_ratings_manager,
            keyboard_shortcuts_manager,
            transfers_manager,
            local_media_manager,
        }
    }

    pub fn get_settings_manager(&self) -> Option<&SettingsManager> {
        self.settings_manager.as_ref()
    }

    pub fn get_user_manager(&self) -> Option<&UserManager> {
        self.user_manager.as_ref()
    }

    pub fn get_keys_manager(&self) -> Option<&KeysManager> {
        self.keys_manager.as_ref()
    }

    pub fn get_blacklist_manager(&self) -> Option<&BlacklistManager> {
        self.blacklist_manager.as_ref()
    }

    pub fn get_global_settings_manager(&self) -> Option<&GlobalSettingsManager> {
        self.global_settings_manager.as_ref()
    }

    pub fn get_content_ratings_manager(&self) -> Option<&ContentRatingsManager> {
        self.content_ratings_manager.as_ref()
    }

    pub fn get_keyboard_shortcuts_manager(&self) -> Option<&KeyboardShortcutsManager> {
        self.keyboard_shortcuts_manager.as_ref()
    }

    pub fn get_transfers_manager(&self) -> Option<&TransferManager> {
        self.transfers_manager.as_ref()
    }

    pub fn get_local_media_manager(&self) -> Option<&LocalMediaManager> {
        self.local_media_manager.as_ref()
    }

    pub fn get_download_manager(&self) -> Option<&DownloadManager> {
        self.download_manager.as_ref()
    }

    pub fn get_db_download_manager(&self) -> Option<&DbDownloadManager> {
        self.db_download_manager.as_ref()
    }
}
