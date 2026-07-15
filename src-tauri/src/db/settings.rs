use crate::db::types::{row_to_settings, SettingsRow, UserSettings};
use crate::db::Database;
use crate::errors::AppError;
use std::sync::Arc;

const SETTINGS_COLUMNS: &str = "\
    id, user_id, preferred_theme, is_light_mode, image_scaling, autoplay, volume, \
    completion_percent, duration_display, preferred_audio_language, \
    preferred_subtitle_language, subtitle_display, audio_channel, default_resolver, \
    file_size_limit, disabled_plugins, season_completion_required, enable_user_ratings, \
    open_app_fullscreen, update_notification, onscreen_keyboard_enabled, \
    excluded_resolutions, excluded_video_formats, excluded_audio_options, \
    excluded_sources, indexer_sort_criteria, download_rate_limit, screensaver_timeout, \
    max_concurrent_downloads, skip_intro, skip_recap, skip_credits, skip_preview, trailer_volume";

const SETTINGS_INSERT_COLUMNS: &str = "\
    user_id, preferred_theme, is_light_mode, image_scaling, \
    autoplay, volume, completion_percent, duration_display, \
    preferred_audio_language, preferred_subtitle_language, subtitle_display, \
    audio_channel, default_resolver, file_size_limit, disabled_plugins, \
    season_completion_required, enable_user_ratings, open_app_fullscreen, \
    update_notification, onscreen_keyboard_enabled, excluded_resolutions, \
    excluded_video_formats, excluded_audio_options, excluded_sources, \
    indexer_sort_criteria, download_rate_limit, screensaver_timeout, \
    max_concurrent_downloads, skip_intro, skip_recap, skip_credits, skip_preview, trailer_volume";

const SETTINGS_INSERT_VALUES: &str = "\
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?";

const SETTINGS_UPDATE_CONFLICT: &str = "\
    preferred_theme              = excluded.preferred_theme,
    is_light_mode                = excluded.is_light_mode,
    image_scaling                = excluded.image_scaling,
    autoplay                     = excluded.autoplay,
    volume                       = excluded.volume,
    completion_percent           = excluded.completion_percent,
    duration_display             = excluded.duration_display,
    preferred_audio_language     = excluded.preferred_audio_language,
    preferred_subtitle_language  = excluded.preferred_subtitle_language,
    subtitle_display             = excluded.subtitle_display,
    audio_channel                = excluded.audio_channel,
    default_resolver             = excluded.default_resolver,
    file_size_limit              = excluded.file_size_limit,
    disabled_plugins             = excluded.disabled_plugins,
    season_completion_required   = excluded.season_completion_required,
    enable_user_ratings          = excluded.enable_user_ratings,
    open_app_fullscreen          = excluded.open_app_fullscreen,
    update_notification          = excluded.update_notification,
    onscreen_keyboard_enabled    = excluded.onscreen_keyboard_enabled,
    excluded_resolutions         = excluded.excluded_resolutions,
    excluded_video_formats       = excluded.excluded_video_formats,
    excluded_audio_options       = excluded.excluded_audio_options,
    excluded_sources             = excluded.excluded_sources,
    indexer_sort_criteria        = excluded.indexer_sort_criteria,
    download_rate_limit          = excluded.download_rate_limit,
    screensaver_timeout          = excluded.screensaver_timeout,
    max_concurrent_downloads     = excluded.max_concurrent_downloads,
    skip_intro                   = excluded.skip_intro,
    skip_recap                   = excluded.skip_recap,
    skip_credits                 = excluded.skip_credits,
    skip_preview                 = excluded.skip_preview,
    trailer_volume               = excluded.trailer_volume";

#[derive(Clone)]
pub struct SettingsManager {
    db: Arc<Database>,
}

impl SettingsManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_settings(&self, user_id: i32) -> Result<UserSettings, AppError> {
        self.query_settings_from_db(user_id).await
    }

    pub async fn update_settings(&self, settings: &UserSettings) -> Result<(), AppError> {
        sqlx::query(&format!(
            "INSERT INTO settings ({}) VALUES ({})
            ON CONFLICT(user_id) DO UPDATE SET {}",
            SETTINGS_INSERT_COLUMNS, SETTINGS_INSERT_VALUES, SETTINGS_UPDATE_CONFLICT
        ))
        .bind(settings.user_id)
        .bind(&settings.preferred_theme)
        .bind(settings.is_light_mode)
        .bind(settings.image_scaling)
        .bind(settings.autoplay)
        .bind(settings.volume)
        .bind(settings.completion_percent)
        .bind(&settings.duration_display)
        .bind(&settings.preferred_audio_language)
        .bind(&settings.preferred_subtitle_language)
        .bind(&settings.subtitle_display)
        .bind(&settings.audio_channel)
        .bind(&settings.default_resolver)
        .bind(settings.file_size_limit)
        .bind(serde_json::to_string(&settings.disabled_plugins).unwrap())
        .bind(settings.season_completion_required)
        .bind(settings.enable_user_ratings)
        .bind(settings.open_app_fullscreen)
        .bind(settings.update_notification)
        .bind(settings.onscreen_keyboard_enabled)
        .bind(serde_json::to_string(&settings.excluded_resolutions).unwrap())
        .bind(serde_json::to_string(&settings.excluded_video_formats).unwrap())
        .bind(serde_json::to_string(&settings.excluded_audio_options).unwrap())
        .bind(serde_json::to_string(&settings.excluded_sources).unwrap())
        .bind(serde_json::to_string(&settings.indexer_sort_criteria).unwrap())
        .bind(settings.download_rate_limit)
        .bind(settings.screensaver_timeout)
        .bind(settings.max_concurrent_downloads)
        .bind(settings.skip_intro)
        .bind(settings.skip_recap)
        .bind(settings.skip_credits)
        .bind(settings.skip_preview)
        .bind(settings.trailer_volume)
        .execute(&self.db.pool)
        .await?;

        Ok(())
    }

    pub async fn create_default_settings(&self, user_id: i32) -> Result<UserSettings, AppError> {
        let default_settings = UserSettings::default_for_user(user_id);

        let result = sqlx::query(&format!(
            "INSERT INTO settings ({}) VALUES ({})",
            SETTINGS_INSERT_COLUMNS, SETTINGS_INSERT_VALUES
        ))
        .bind(default_settings.user_id)
        .bind(&default_settings.preferred_theme)
        .bind(default_settings.is_light_mode)
        .bind(default_settings.image_scaling)
        .bind(default_settings.autoplay)
        .bind(default_settings.volume)
        .bind(default_settings.completion_percent)
        .bind(&default_settings.duration_display)
        .bind(&default_settings.preferred_audio_language)
        .bind(&default_settings.preferred_subtitle_language)
        .bind(&default_settings.subtitle_display)
        .bind(&default_settings.audio_channel)
        .bind(&default_settings.default_resolver)
        .bind(default_settings.file_size_limit)
        .bind(serde_json::to_string(&default_settings.disabled_plugins).unwrap())
        .bind(default_settings.season_completion_required)
        .bind(default_settings.enable_user_ratings)
        .bind(default_settings.open_app_fullscreen)
        .bind(default_settings.update_notification)
        .bind(default_settings.onscreen_keyboard_enabled)
        .bind(serde_json::to_string(&default_settings.excluded_resolutions).unwrap())
        .bind(serde_json::to_string(&default_settings.excluded_video_formats).unwrap())
        .bind(serde_json::to_string(&default_settings.excluded_audio_options).unwrap())
        .bind(serde_json::to_string(&default_settings.excluded_sources).unwrap())
        .bind(serde_json::to_string(&default_settings.indexer_sort_criteria).unwrap())
        .bind(default_settings.download_rate_limit)
        .bind(default_settings.screensaver_timeout)
        .bind(default_settings.max_concurrent_downloads)
        .bind(default_settings.skip_intro)
        .bind(default_settings.skip_recap)
        .bind(default_settings.skip_credits)
        .bind(default_settings.skip_preview)
        .bind(default_settings.trailer_volume)
        .execute(&self.db.pool)
        .await;

        match result {
            Ok(_) => Ok(default_settings),
            Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {
                self.get_settings(user_id).await
            }
            Err(e) => Err(AppError::from(e)),
        }
    }

    async fn query_settings_from_db(&self, user_id: i32) -> Result<UserSettings, AppError> {
        let row = sqlx::query_as::<_, SettingsRow>(&format!(
            "SELECT {} FROM settings WHERE user_id = ?",
            SETTINGS_COLUMNS
        ))
        .bind(user_id)
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            Some(r) => Ok(row_to_settings(r)),
            None => {
                let default_settings = UserSettings::default_for_user(user_id);
                self.update_settings(&default_settings).await?;
                Ok(default_settings)
            }
        }
    }

    pub async fn get_max_concurrent_downloads(&self, user_id: i32) -> Result<i32, AppError> {
        let max: Option<i32> =
            sqlx::query_scalar("SELECT max_concurrent_downloads FROM settings WHERE user_id = ?")
                .bind(user_id)
                .fetch_optional(&self.db.pool)
                .await?;

        Ok(max.unwrap_or(3))
    }
}
