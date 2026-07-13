use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSettings {
    pub id: i32,
    pub user_id: i32,
    pub preferred_theme: String,
    pub is_light_mode: bool,
    pub image_scaling: i32,
    pub autoplay: bool,
    pub volume: i32,
    pub completion_percent: i32,
    pub duration_display: String,
    pub preferred_audio_language: String,
    pub preferred_subtitle_language: String,
    pub subtitle_display: String,
    pub audio_channel: String,
    pub default_resolver: Option<String>,
    pub file_size_limit: i32,
    pub disabled_plugins: Vec<String>,
    pub season_completion_required: bool,
    pub enable_user_ratings: bool,
    pub open_app_fullscreen: bool,
    pub update_notification: bool,
    pub onscreen_keyboard_enabled: bool,
    pub excluded_resolutions: Vec<String>,
    pub excluded_video_formats: Vec<String>,
    pub excluded_audio_options: Vec<String>,
    pub excluded_sources: Vec<String>,
    pub indexer_sort_criteria: Vec<IndexerSortCriterion>,
    pub download_rate_limit: i32,
    pub screensaver_timeout: i32,
    pub max_concurrent_downloads: i32,
    pub skip_intro: bool,
    pub skip_recap: bool,
    pub skip_credits: bool,
    pub skip_preview: bool,
}

impl UserSettings {
    pub fn default_for_user(user_id: i32) -> Self {
        Self {
            id: 0,
            user_id,
            preferred_theme: "Default".to_string(),
            is_light_mode: false,
            image_scaling: 100,
            autoplay: true,
            volume: 30,
            completion_percent: 80,
            duration_display: "Duration".to_string(),
            preferred_audio_language: "Source".to_string(),
            preferred_subtitle_language: "en".to_string(),
            subtitle_display: "Auto".to_string(),
            audio_channel: "5.1".to_string(),
            default_resolver: None,
            file_size_limit: 20,
            disabled_plugins: vec![],
            season_completion_required: true,
            enable_user_ratings: true,
            open_app_fullscreen: false,
            update_notification: true,
            onscreen_keyboard_enabled: false,
            excluded_resolutions: vec![],
            excluded_video_formats: vec![],
            excluded_audio_options: vec![],
            excluded_sources: vec![],
            indexer_sort_criteria: vec![
                IndexerSortCriterion {
                    key: "Resolution".to_string(),
                    order: SortOrder::Desc,
                },
                IndexerSortCriterion {
                    key: "Quality".to_string(),
                    order: SortOrder::Desc,
                },
                IndexerSortCriterion {
                    key: "Size".to_string(),
                    order: SortOrder::Desc,
                },
                IndexerSortCriterion {
                    key: "Seeders".to_string(),
                    order: SortOrder::Desc,
                },
            ],
            download_rate_limit: 0,
            screensaver_timeout: 30,
            max_concurrent_downloads: 3,
            skip_intro: false,
            skip_recap: false,
            skip_credits: false,
            skip_preview: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexerSortCriterion {
    pub key: String,
    pub order: SortOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserData {
    pub username: String,
    pub email: String,
    pub avatar: Option<String>,
    pub pin: Option<String>,
    pub postgres_id: String,
    pub token: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserData {
    pub user_id: i32,
    pub username: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub pin: Option<String>,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub avatar: Option<String>,
    pub pin: Option<String>,
    pub postgres_id: String,
    pub token: Option<String>,
    pub updated_at: String,
}

impl User {
    pub fn to_frontend_json(&self) -> serde_json::Value {
        let mut val = serde_json::to_value(self).unwrap();
        val.as_object_mut().unwrap().remove("pin");
        val
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keys {
    pub id: i64,
    pub plugin: String,
    pub key: String,
    pub expires_at: i64, // Unix timestamp
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blacklist {
    pub id: i64,
    pub user_id: i32,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalSettings {
    pub id: i32,
    pub parental_controls_are_enabled: bool,
    pub primary_user_id: Option<i32>,
    pub tv_directory: Option<String>,
    pub movie_directory: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGlobalSettings {
    pub parental_controls_are_enabled: Option<bool>,
    pub primary_user_id: Option<i32>,
    pub tv_directory: Option<String>,
    pub movie_directory: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserContentRatings {
    pub user_id: i32,
    pub restricted_movies: Vec<String>,
    pub restricted_tv: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyBinding {
    pub code: String,
    #[serde(rename = "shiftKey")]
    pub shift_key: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct KeyboardShortcutsRow {
    pub user_id: i32,
    pub toggle_play: String,
    pub fullscreen: String,
    pub mute: String,
    pub forward: String,
    pub rewind: String,
    pub playlist_next: String,
    pub playlist_previous: String,
    pub close: String,
    pub volume_up: String,
    pub volume_down: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyboardShortcuts {
    pub user_id: i32,
    pub toggle_play: KeyBinding,
    pub fullscreen: KeyBinding,
    pub mute: KeyBinding,
    pub forward: KeyBinding,
    pub rewind: KeyBinding,
    pub playlist_next: KeyBinding,
    pub playlist_previous: KeyBinding,
    pub close: KeyBinding,
    pub volume_up: KeyBinding,
    pub volume_down: KeyBinding,
}

#[derive(sqlx::FromRow)]
pub struct SettingsRow {
    pub id: i32,
    pub user_id: i32,
    pub preferred_theme: String,
    pub is_light_mode: bool,
    pub image_scaling: i32,
    pub autoplay: bool,
    pub volume: i32,
    pub completion_percent: i32,
    pub duration_display: String,
    pub preferred_audio_language: String,
    pub preferred_subtitle_language: String,
    pub subtitle_display: String,
    pub audio_channel: String,
    pub default_resolver: Option<String>,
    pub file_size_limit: i32,
    pub disabled_plugins: String,
    pub season_completion_required: bool,
    pub enable_user_ratings: bool,
    pub open_app_fullscreen: bool,
    pub update_notification: bool,
    pub onscreen_keyboard_enabled: bool,
    pub excluded_resolutions: String,
    pub excluded_video_formats: String,
    pub excluded_audio_options: String,
    pub excluded_sources: String,
    pub indexer_sort_criteria: String,
    pub download_rate_limit: i32,
    pub screensaver_timeout: i32,
    pub max_concurrent_downloads: i32,
    pub skip_intro: bool,
    pub skip_recap: bool,
    pub skip_credits: bool,
    pub skip_preview: bool,
}

pub fn row_to_settings(row: SettingsRow) -> UserSettings {
    UserSettings {
        id: row.id,
        user_id: row.user_id,
        preferred_theme: row.preferred_theme,
        is_light_mode: row.is_light_mode,
        image_scaling: row.image_scaling,
        autoplay: row.autoplay,
        volume: row.volume,
        completion_percent: row.completion_percent,
        duration_display: row.duration_display,
        preferred_audio_language: row.preferred_audio_language,
        preferred_subtitle_language: row.preferred_subtitle_language,
        subtitle_display: row.subtitle_display,
        audio_channel: row.audio_channel,
        default_resolver: row.default_resolver,
        file_size_limit: row.file_size_limit,
        disabled_plugins: serde_json::from_str(&row.disabled_plugins).unwrap_or_default(),
        season_completion_required: row.season_completion_required,
        enable_user_ratings: row.enable_user_ratings,
        open_app_fullscreen: row.open_app_fullscreen,
        update_notification: row.update_notification,
        onscreen_keyboard_enabled: row.onscreen_keyboard_enabled,
        excluded_resolutions: serde_json::from_str(&row.excluded_resolutions).unwrap_or_default(),
        excluded_video_formats: serde_json::from_str(&row.excluded_video_formats)
            .unwrap_or_default(),
        excluded_audio_options: serde_json::from_str(&row.excluded_audio_options)
            .unwrap_or_default(),
        excluded_sources: serde_json::from_str(&row.excluded_sources).unwrap_or_default(),
        indexer_sort_criteria: serde_json::from_str(&row.indexer_sort_criteria).unwrap_or_default(),
        download_rate_limit: row.download_rate_limit,
        screensaver_timeout: row.screensaver_timeout,
        max_concurrent_downloads: row.max_concurrent_downloads,
        skip_intro: row.skip_intro,
        skip_recap: row.skip_recap,
        skip_credits: row.skip_credits,
        skip_preview: row.skip_preview,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub hash: String,
    pub transfer_id: i32,
    pub progress: i32,
    pub status: String,
    pub resolver: String,
    pub filename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransfer {
    pub transfer_id: i32,
    pub progress: Option<i32>,
    pub status: String,
    pub resolver: String,
    pub filename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTransfer {
    pub progress: Option<i32>,
    pub status: Option<String>,
    pub resolver: Option<String>,
    pub filename: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalMedia {
    pub id: i64,
    pub imdb_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalFilepath {
    pub id: i64,
    pub media_id: i64,
    pub file_path: String,
    pub season: Option<i64>,
    pub episode: Option<i64>,
    pub size: Option<f64>,
    pub resolution: Option<String>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub audio_channels: Option<String>,
    pub video_filters: Option<String>,
    pub tags: Option<String>,
    pub language: Option<String>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalMediaWithFiles {
    pub media: LocalMedia,
    pub filepaths: Vec<LocalFilepath>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
    pub added: u64,
    pub skipped: u64,
    pub errors: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditableMediaFields {
    pub media_id: i64,
    pub imdb_id: Option<String>,
    pub filepath_id: i64,
    pub file_path: String,
    pub season: Option<i64>,
    pub episode: Option<i64>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Download {
    pub uuid: String,
    pub folder_path: String,
    pub file_link: String,
    pub resolver_id: String,
    pub imdb_id: Option<String>,
    pub title: String,
    pub released: Option<i32>,
    pub season: Option<i32>,
    pub episode: Option<i32>,
    pub file_url: String,
    pub media_poster: Option<String>,
    pub season_poster: Option<String>,
    pub filename: String,
    pub part_file_path: String,
    pub final_file_path: String,
    pub status: String,
    pub queue_order: i32,
    pub added_at: DateTime<Utc>,
    pub total_bytes: Option<i64>,
    pub downloaded_bytes: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct CreateDownload {
    pub folder_path: String,
    pub file_link: String,
    pub resolver_id: String,
    pub imdb_id: Option<String>,
    pub title: String,
    pub released: Option<i32>,
    pub season: Option<i32>,
    pub episode: Option<i32>,
    pub file_url: String,
    pub media_poster: Option<String>,
    pub season_poster: Option<String>,
    pub filename: String,
    pub part_file_path: String,
    pub final_file_path: String,
    pub status: DownloadStatus,
    pub queue_order: i32,
    pub total_bytes: Option<i64>,
    pub downloaded_bytes: Option<i64>,
}

impl From<KeyboardShortcutsRow> for KeyboardShortcuts {
    fn from(row: KeyboardShortcutsRow) -> Self {
        Self {
            user_id: row.user_id,
            toggle_play: serde_json::from_str(&row.toggle_play).unwrap_or_default(),
            fullscreen: serde_json::from_str(&row.fullscreen).unwrap_or_default(),
            mute: serde_json::from_str(&row.mute).unwrap_or_default(),
            forward: serde_json::from_str(&row.forward).unwrap_or_default(),
            rewind: serde_json::from_str(&row.rewind).unwrap_or_default(),
            playlist_next: serde_json::from_str(&row.playlist_next).unwrap_or_default(),
            playlist_previous: serde_json::from_str(&row.playlist_previous).unwrap_or_default(),
            close: serde_json::from_str(&row.close).unwrap_or_default(),
            volume_up: serde_json::from_str(&row.volume_up).unwrap_or_default(),
            volume_down: serde_json::from_str(&row.volume_down).unwrap_or_default(),
        }
    }
}

impl KeyboardShortcuts {
    pub fn to_row(&self) -> KeyboardShortcutsRow {
        KeyboardShortcutsRow {
            user_id: self.user_id,
            toggle_play: serde_json::to_string(&self.toggle_play).unwrap(),
            fullscreen: serde_json::to_string(&self.fullscreen).unwrap(),
            mute: serde_json::to_string(&self.mute).unwrap(),
            forward: serde_json::to_string(&self.forward).unwrap(),
            rewind: serde_json::to_string(&self.rewind).unwrap(),
            playlist_next: serde_json::to_string(&self.playlist_next).unwrap(),
            playlist_previous: serde_json::to_string(&self.playlist_previous).unwrap(),
            close: serde_json::to_string(&self.close).unwrap(),
            volume_up: serde_json::to_string(&self.volume_up).unwrap(),
            volume_down: serde_json::to_string(&self.volume_down).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Paused,
    Completed,
    Cancelled,
    Failed,
}

impl DownloadStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DownloadStatus::Pending => "pending",
            DownloadStatus::Downloading => "downloading",
            DownloadStatus::Paused => "paused",
            DownloadStatus::Completed => "completed",
            DownloadStatus::Cancelled => "cancelled",
            DownloadStatus::Failed => "failed",
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            DownloadStatus::Completed | DownloadStatus::Cancelled | DownloadStatus::Failed
        )
    }

    pub fn is_active(&self) -> bool {
        matches!(self, DownloadStatus::Downloading | DownloadStatus::Paused)
    }
}

impl fmt::Display for DownloadStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for DownloadStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(DownloadStatus::Pending),
            "downloading" => Ok(DownloadStatus::Downloading),
            "paused" => Ok(DownloadStatus::Paused),
            "completed" => Ok(DownloadStatus::Completed),
            "cancelled" => Ok(DownloadStatus::Cancelled),
            "failed" => Ok(DownloadStatus::Failed),
            _ => Err(format!("Invalid download status: {}", s)),
        }
    }
}
