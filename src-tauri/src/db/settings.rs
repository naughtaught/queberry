use crate::db::types::UserSettings;
use crate::db::Database;
use crate::errors::AppError;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct SettingsManager {
    db: Arc<Database>,
    cache: Arc<RwLock<std::collections::HashMap<i32, UserSettings>>>,
}

impl SettingsManager {
    pub fn new(db: Database) -> Self {
        Self {
            db: Arc::new(db),
            cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub fn get_settings(&self, user_id: i32) -> Result<UserSettings, AppError> {
        // Check cache first
        {
            let cache = self.cache.read().unwrap();
            if let Some(settings) = cache.get(&user_id) {
                return Ok(settings.clone());
            }
        }

        // Query database
        let settings = self.query_settings_from_db(user_id)?;

        // Update cache
        {
            let mut cache = self.cache.write().unwrap();
            cache.insert(user_id, settings.clone());
        }

        Ok(settings)
    }

    pub fn update_settings(&self, settings: &UserSettings) -> Result<(), AppError> {
        let sql = "
            INSERT OR REPLACE INTO settings (
                id, user_id, preferred_theme, is_light_mode, image_size,
                autoplay, volume, complete_percent, duration_display,
                preferred_language, preferred_subtitle_language, subtitle_setting,
                audio_channels, preferred_resolver, size_limit, disabled_plugins,
                stop_on_incomplete_season, enable_user_ratings, always_open_app_fullscreen,
                update_notification, keyboard_enabled, exclude, sort, speed_limit, screensaver
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ";

        self.db.execute(
            sql,
            rusqlite::params![
                settings.id,
                settings.user_id,
                &settings.preferred_theme,
                settings.is_light_mode,
                settings.image_size,
                settings.autoplay,
                settings.volume,
                settings.complete_percent,
                &settings.duration_display,
                &settings.preferred_language,
                &settings.preferred_subtitle_language,
                &settings.subtitle_setting,
                &settings.audio_channels,
                &settings.preferred_resolver,
                settings.size_limit,
                &settings.disabled_plugins,
                settings.stop_on_incomplete_season,
                settings.enable_user_ratings,
                settings.always_open_app_fullscreen,
                settings.update_notification,
                settings.keyboard_enabled,
                &settings.exclude,
                &settings.sort,
                settings.speed_limit,
                settings.screensaver,
            ],
        )?;

        // Update cache
        {
            let mut cache = self.cache.write().unwrap();
            cache.insert(settings.user_id, settings.clone());
        }

        Ok(())
    }

    pub fn invalidate_cache(&self, user_id: i32) {
        let mut cache = self.cache.write().unwrap();
        cache.remove(&user_id);
    }

    pub fn invalidate_all_cache(&self) {
        let mut cache = self.cache.write().unwrap();
        cache.clear();
    }

    fn query_settings_from_db(&self, user_id: i32) -> Result<UserSettings, AppError> {
        let sql = "SELECT * FROM settings WHERE user_id = ?";

        match self.db.query_row(sql, [user_id], |row| {
            Ok(UserSettings {
                id: row.get(0)?,
                user_id: row.get(1)?,
                preferred_theme: row.get(2)?,
                is_light_mode: row.get(3)?,
                image_size: row.get(4)?,
                autoplay: row.get(5)?,
                volume: row.get(6)?,
                complete_percent: row.get(7)?,
                duration_display: row.get(8)?,
                preferred_language: row.get(9)?,
                preferred_subtitle_language: row.get(10)?,
                subtitle_setting: row.get(11)?,
                audio_channels: row.get(12)?,
                preferred_resolver: row.get(13)?,
                size_limit: row.get(14)?,
                disabled_plugins: row.get(15)?,
                stop_on_incomplete_season: row.get(16)?,
                enable_user_ratings: row.get(17)?,
                always_open_app_fullscreen: row.get(18)?,
                update_notification: row.get(19)?,
                keyboard_enabled: row.get(20)?,
                exclude: row.get(21)?,
                sort: row.get(22)?,
                speed_limit: row.get(23)?,
                screensaver: row.get(24)?,
            })
        }) {
            Ok(settings) => Ok(settings),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                // Create default settings for user
                let default_settings = UserSettings::default_for_user(user_id);
                self.update_settings(&default_settings)?;
                Ok(default_settings)
            }
            Err(e) => Err(AppError::from(e)),
        }
    }
}
