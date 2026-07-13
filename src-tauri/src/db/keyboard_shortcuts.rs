use crate::db::types::{KeyboardShortcuts, KeyboardShortcutsRow};
use crate::db::Database;
use crate::errors::AppError;
use std::sync::Arc;

const KEYBOARD_SHORTCUTS_COLUMNS: &str = "\
    user_id, toggle_play, fullscreen, mute, forward, rewind, \
    playlist_next, playlist_previous, close, volume_up, volume_down";

const KEYBOARD_SHORTCUTS_INSERT_COLUMNS: &str = "\
    user_id, toggle_play, fullscreen, mute, forward, rewind, \
    playlist_next, playlist_previous, close, volume_up, volume_down";

const KEYBOARD_SHORTCUTS_INSERT_VALUES: &str = "?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?";

const KEYBOARD_SHORTCUTS_UPDATE_CONFLICT: &str = "\
    toggle_play       = excluded.toggle_play,
    fullscreen        = excluded.fullscreen,
    mute              = excluded.mute,
    forward           = excluded.forward,
    rewind            = excluded.rewind,
    playlist_next     = excluded.playlist_next,
    playlist_previous = excluded.playlist_previous,
    close             = excluded.close,
    volume_up         = excluded.volume_up,
    volume_down       = excluded.volume_down";

#[derive(Clone)]
pub struct KeyboardShortcutsManager {
    db: Arc<Database>,
}

impl KeyboardShortcutsManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_user_keyboard_shortcuts(
        &self,
        user_id: i32,
    ) -> Result<Option<KeyboardShortcuts>, AppError> {
        let row = sqlx::query_as::<_, KeyboardShortcutsRow>(&format!(
            "SELECT {} FROM keyboard_shortcuts WHERE user_id = ?",
            KEYBOARD_SHORTCUTS_COLUMNS
        ))
        .bind(user_id)
        .fetch_optional(&self.db.pool)
        .await?;

        Ok(row.map(KeyboardShortcuts::from))
    }

    pub async fn upsert_user_keyboard_shortcuts(
        &self,
        user_id: i32,
        shortcuts: KeyboardShortcuts,
    ) -> Result<KeyboardShortcuts, AppError> {
        let row_data = shortcuts.to_row();

        let row = sqlx::query_as::<_, KeyboardShortcutsRow>(&format!(
            "INSERT INTO keyboard_shortcuts ({}) VALUES ({})
             ON CONFLICT(user_id) DO UPDATE SET
                {}
             RETURNING {}",
            KEYBOARD_SHORTCUTS_INSERT_COLUMNS,
            KEYBOARD_SHORTCUTS_INSERT_VALUES,
            KEYBOARD_SHORTCUTS_UPDATE_CONFLICT,
            KEYBOARD_SHORTCUTS_COLUMNS
        ))
        .bind(user_id)
        .bind(&row_data.toggle_play)
        .bind(&row_data.fullscreen)
        .bind(&row_data.mute)
        .bind(&row_data.forward)
        .bind(&row_data.rewind)
        .bind(&row_data.playlist_next)
        .bind(&row_data.playlist_previous)
        .bind(&row_data.close)
        .bind(&row_data.volume_up)
        .bind(&row_data.volume_down)
        .fetch_one(&self.db.pool)
        .await?;

        Ok(KeyboardShortcuts::from(row))
    }

    pub async fn create_default_shortcuts(
        &self,
        user_id: i32,
    ) -> Result<KeyboardShortcuts, AppError> {
        sqlx::query("INSERT OR IGNORE INTO keyboard_shortcuts (user_id) VALUES (?)")
            .bind(user_id)
            .execute(&self.db.pool)
            .await?;

        self.get_user_keyboard_shortcuts(user_id)
            .await?
            .ok_or_else(|| {
                AppError::Runtime("Failed to retrieve keyboard shortcuts after insert".to_string())
            })
    }
}
