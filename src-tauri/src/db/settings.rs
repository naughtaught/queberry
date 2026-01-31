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
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
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
                id, user_id, preferred_theme, is_light_mode, image_scaling,
                autoplay, volume, completion_percent, duration_display,
                preferred_audio_language, preferred_subtitle_language, subtitle_display,
                subtitle_margin, audio_channel, default_resolver, file_size_limit, disabled_plugins,
                season_completion_required, enable_user_ratings, open_app_fullscreen,
                update_notification, onscreen_keyboard_enabled, excluded_resolutions, 
                excluded_video_formats, excluded_audio_options, excluded_sources, 
                indexer_sort_criteria, download_rate_limit, screensaver_timeout
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ";

        self.db.execute(
            sql,
            rusqlite::params![
                settings.id,
                settings.user_id,
                &settings.preferred_theme,
                settings.is_light_mode,
                settings.image_scaling,
                settings.autoplay,
                settings.volume,
                settings.completion_percent,
                &settings.duration_display,
                &settings.preferred_audio_language,
                &settings.preferred_subtitle_language,
                &settings.subtitle_display,
                settings.subtitle_margin,
                &settings.audio_channel,
                &settings.default_resolver,
                settings.file_size_limit,
                &settings.disabled_plugins,
                settings.season_completion_required,
                settings.enable_user_ratings,
                settings.open_app_fullscreen,
                settings.update_notification,
                settings.onscreen_keyboard_enabled,
                &settings.excluded_resolutions,
                &settings.excluded_video_formats,
                &settings.excluded_audio_options,
                &settings.excluded_sources,
                &settings.indexer_sort_criteria,
                settings.download_rate_limit,
                settings.screensaver_timeout,
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
                image_scaling: row.get(4)?,
                autoplay: row.get(5)?,
                volume: row.get(6)?,
                completion_percent: row.get(7)?,
                duration_display: row.get(8)?,
                preferred_audio_language: row.get(9)?,
                preferred_subtitle_language: row.get(10)?,
                subtitle_display: row.get(11)?,
                subtitle_margin: row.get(12)?,
                audio_channel: row.get(13)?,
                default_resolver: row.get(14)?,
                file_size_limit: row.get(15)?,
                disabled_plugins: row.get(16)?,
                season_completion_required: row.get(17)?,
                enable_user_ratings: row.get(18)?,
                open_app_fullscreen: row.get(19)?,
                update_notification: row.get(20)?,
                onscreen_keyboard_enabled: row.get(21)?,
                excluded_resolutions: row.get(22)?,
                excluded_video_formats: row.get(23)?,
                excluded_audio_options: row.get(24)?,
                excluded_sources: row.get(25)?,
                indexer_sort_criteria: row.get(26)?,
                download_rate_limit: row.get(27)?,
                screensaver_timeout: row.get(28)?,
            })
        }) {
            Ok(settings) => Ok(settings),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                let default_settings = UserSettings::default_for_user(user_id);
                self.update_settings(&default_settings)?;
                Ok(default_settings)
            }
            Err(e) => Err(AppError::from(e)),
        }
    }
}
