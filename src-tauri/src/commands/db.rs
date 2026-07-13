use crate::{
    db::types::{
        Blacklist, CreateTransfer, CreateUserData, EditableMediaFields, GlobalSettings,
        KeyboardShortcuts, Keys, LocalMediaWithFiles, ScanResult, Transfer, UpdateGlobalSettings,
        UpdateUserData, UserContentRatings,
    },
    errors::{handle_command_async, ApiResponse},
    state::AppState,
    AppError,
};
use tauri::State;

#[tauri::command]
pub async fn get_user_settings(
    app_state: State<'_, AppState>,
    user_id: i32,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("get_user_settings", async || {
        let manager = app_state
            .get_settings_manager()
            .ok_or_else(|| AppError::Runtime("Settings not initialized".to_string()))?;

        let settings = manager.get_settings(user_id).await?;
        let value = serde_json::to_value(&settings)
            .map_err(|e| AppError::Runtime(format!("Failed to serialize settings: {}", e)))?;

        Ok(value)
    })
    .await
}

#[tauri::command]
pub async fn update_user_settings(
    app_state: State<'_, AppState>,
    settings: crate::db::types::UserSettings,
) -> Result<ApiResponse<()>, AppError> {
    handle_command_async("update_user_settings", async || {
        let manager = app_state
            .get_settings_manager()
            .ok_or_else(|| AppError::Runtime("Settings not initialized".to_string()))?;

        manager.update_settings(&settings).await?;
        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn get_all_users(
    app_state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<serde_json::Value>>, AppError> {
    handle_command_async("get_all_users", async || {
        let manager = app_state
            .get_user_manager()
            .ok_or_else(|| AppError::Runtime("User manager not initialized".to_string()))?;

        let users = manager.get_all_users().await?;
        Ok(users.iter().map(|u| u.to_frontend_json()).collect())
    })
    .await
}

#[tauri::command]
pub async fn get_user(
    app_state: State<'_, AppState>,
    user_id: i32,
) -> Result<ApiResponse<Option<serde_json::Value>>, AppError> {
    handle_command_async("get_user", async || {
        let manager = app_state
            .get_user_manager()
            .ok_or_else(|| AppError::Runtime("User manager not initialized".to_string()))?;

        let user = manager.get_user(user_id).await?;
        Ok(user.map(|u| u.to_frontend_json()))
    })
    .await
}

#[tauri::command]
pub async fn get_user_by_postgres_id(
    app_state: State<'_, AppState>,
    postgres_id: &str,
) -> Result<ApiResponse<Option<serde_json::Value>>, AppError> {
    handle_command_async("get_user_by_postgres_id", async || {
        let manager = app_state
            .get_user_manager()
            .ok_or_else(|| AppError::Runtime("User manager not initialized".to_string()))?;

        let user = manager.get_user_by_postgres_id(postgres_id).await?;
        Ok(user.map(|u| u.to_frontend_json()))
    })
    .await
}

#[tauri::command]
pub async fn create_user(
    app_state: State<'_, AppState>,
    user: CreateUserData,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("create_user", async || {
        let user_manager = app_state
            .get_user_manager()
            .ok_or_else(|| AppError::Runtime("User manager not initialized".to_string()))?;

        let settings_manager = app_state
            .get_settings_manager()
            .ok_or_else(|| AppError::Runtime("Settings manager not initialized".to_string()))?;

        let shortcuts_manager = app_state.get_keyboard_shortcuts_manager().ok_or_else(|| {
            AppError::Runtime("Keyboard shortcuts manager not initialized".to_string())
        })?;

        let user = user_manager.create_user(user).await?;

        settings_manager.create_default_settings(user.id).await?;
        shortcuts_manager.create_default_shortcuts(user.id).await?;

        Ok(user.to_frontend_json())
    })
    .await
}

#[tauri::command]
pub async fn update_user(
    app_state: State<'_, AppState>,
    updates: UpdateUserData,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("update_user", async || {
        let manager = app_state
            .get_user_manager()
            .ok_or_else(|| AppError::Runtime("User manager not initialized".to_string()))?;

        let user = manager.update_user(updates).await?;
        Ok(user.to_frontend_json())
    })
    .await
}

#[tauri::command]
pub async fn delete_user(
    app_state: State<'_, AppState>,
    user_id: i32,
) -> Result<ApiResponse<bool>, AppError> {
    handle_command_async("delete_user", async || {
        let user_manager = app_state
            .get_user_manager()
            .ok_or_else(|| AppError::Runtime("User manager not initialized".to_string()))?;

        let global_settings_manager = app_state.get_global_settings_manager().ok_or_else(|| {
            AppError::Runtime("Global settings manager not initialized".to_string())
        })?;

        if let Ok(global_settings) = global_settings_manager.get_global_settings().await {
            if global_settings.primary_user_id == Some(user_id) {
                let updates = UpdateGlobalSettings {
                    parental_controls_are_enabled: Some(false),
                    primary_user_id: None,
                    tv_directory: None,
                    movie_directory: None,
                };
                global_settings_manager
                    .update_global_settings(updates)
                    .await?;
                user_manager.delete_all_content_ratings().await?;
            }
        }

        let deleted = user_manager.delete_user(user_id).await?;
        Ok(deleted)
    })
    .await
}

#[tauri::command]
pub async fn get_keys(app_state: State<'_, AppState>) -> Result<ApiResponse<Vec<Keys>>, AppError> {
    handle_command_async("get_keys", async || {
        let manager = app_state
            .get_keys_manager()
            .ok_or_else(|| AppError::Runtime("Keys manager not initialized".to_string()))?;

        let keys = manager.get_keys().await?;
        Ok(keys)
    })
    .await
}

#[tauri::command]
pub async fn create_key(
    app_state: State<'_, AppState>,
    plugin: String,
    key: String,
    expiration: i64,
) -> Result<ApiResponse<Keys>, AppError> {
    handle_command_async("create_key", async || {
        let manager = app_state
            .get_keys_manager()
            .ok_or_else(|| AppError::Runtime("Keys manager not initialized".to_string()))?;

        let key = manager.create_key(plugin, key, expiration).await?;
        Ok(key)
    })
    .await
}

#[tauri::command]
pub async fn update_key(
    app_state: State<'_, AppState>,
    plugin: Option<String>,
    key: Option<String>,
    expiration: Option<i64>,
) -> Result<ApiResponse<Keys>, AppError> {
    handle_command_async("update_key", async || {
        let manager = app_state
            .get_keys_manager()
            .ok_or_else(|| AppError::Runtime("Keys manager not initialized".to_string()))?;

        let key = manager.update_key(plugin, key, expiration).await?;
        Ok(key)
    })
    .await
}

#[tauri::command]
pub async fn delete_key(
    app_state: State<'_, AppState>,
    plugin: String,
) -> Result<ApiResponse<bool>, AppError> {
    handle_command_async("delete_key", async || {
        let manager = app_state
            .get_keys_manager()
            .ok_or_else(|| AppError::Runtime("Keys manager not initialized".to_string()))?;

        manager.delete_key(&plugin).await?;
        Ok(true)
    })
    .await
}

#[tauri::command]
pub async fn get_users_blacklisted(
    app_state: State<'_, AppState>,
    user_id: i32,
) -> Result<ApiResponse<Vec<Blacklist>>, AppError> {
    handle_command_async("get_users_blacklisted", async || {
        let manager = app_state
            .get_blacklist_manager()
            .ok_or_else(|| AppError::Runtime("Blacklist manager not initialized".to_string()))?;

        let data = manager.get_users_blacklisted(user_id).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn create_blacklisted_hash(
    app_state: State<'_, AppState>,
    user_id: i32,
    hash: String,
) -> Result<ApiResponse<Blacklist>, AppError> {
    handle_command_async("create_blacklisted_hash", async || {
        let manager = app_state
            .get_blacklist_manager()
            .ok_or_else(|| AppError::Runtime("Blacklist manager not initialized".to_string()))?;

        let data = manager.create_blacklisted_hash(user_id, hash).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn delete_users_blacklisted(
    app_state: State<'_, AppState>,
    user_id: i32,
) -> Result<ApiResponse<bool>, AppError> {
    handle_command_async("delete_users_blacklisted", async || {
        let manager = app_state
            .get_blacklist_manager()
            .ok_or_else(|| AppError::Runtime("Blacklist manager not initialized".to_string()))?;

        manager.delete_users_blacklisted(user_id).await?;
        Ok(true)
    })
    .await
}

#[tauri::command]
pub async fn get_global_settings(
    app_state: State<'_, AppState>,
) -> Result<ApiResponse<GlobalSettings>, AppError> {
    handle_command_async("get_global_settings", async || {
        let manager = app_state.get_global_settings_manager().ok_or_else(|| {
            AppError::Runtime("Global settings manager not initialized".to_string())
        })?;

        let data = manager.get_global_settings().await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn update_global_settings(
    app_state: State<'_, AppState>,
    parental_controls_are_enabled: Option<bool>,
    primary_user_id: Option<i32>,
    tv_directory: Option<String>,
    movie_directory: Option<String>,
) -> Result<ApiResponse<GlobalSettings>, AppError> {
    handle_command_async("update_global_settings", async || {
        let manager = app_state.get_global_settings_manager().ok_or_else(|| {
            AppError::Runtime("Global settings manager not initialized".to_string())
        })?;

        let updates = UpdateGlobalSettings {
            parental_controls_are_enabled,
            primary_user_id,
            tv_directory,
            movie_directory,
        };

        let data = manager.update_global_settings(updates).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn get_user_content_ratings(
    app_state: State<'_, AppState>,
    user_id: i32,
) -> Result<ApiResponse<Option<UserContentRatings>>, AppError> {
    handle_command_async("get_user_content_ratings", async || {
        let manager = app_state.get_content_ratings_manager().ok_or_else(|| {
            AppError::Runtime("Content ratings manager not initialized".to_string())
        })?;

        let data = manager.get_user_content_ratings(user_id).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn upsert_user_content_ratings(
    app_state: State<'_, AppState>,
    user_id: i32,
    restricted_movies: Vec<String>,
    restricted_tv: Vec<String>,
) -> Result<ApiResponse<UserContentRatings>, AppError> {
    handle_command_async("upsert_user_content_ratings", async || {
        let manager = app_state.get_content_ratings_manager().ok_or_else(|| {
            AppError::Runtime("Content ratings manager not initialized".to_string())
        })?;

        let data = manager
            .upsert_user_content_ratings(user_id, restricted_movies, restricted_tv)
            .await?;

        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn delete_user_content_ratings(
    app_state: State<'_, AppState>,
    user_id: i32,
) -> Result<ApiResponse<bool>, AppError> {
    handle_command_async("delete_user_content_ratings", async || {
        let manager = app_state.get_content_ratings_manager().ok_or_else(|| {
            AppError::Runtime("Content ratings manager not initialized".to_string())
        })?;

        manager.delete_user_content_ratings(user_id).await?;
        Ok(true)
    })
    .await
}

#[tauri::command]
pub async fn verify_pin(
    app_state: State<'_, AppState>,
    user_id: i32,
    pin: String,
) -> Result<ApiResponse<bool>, AppError> {
    handle_command_async("verify_pin", async || {
        let manager = app_state
            .get_user_manager()
            .ok_or_else(|| AppError::Runtime("User manager not initialized".to_string()))?;

        let valid = manager.verify_pin(user_id, &pin).await?;
        Ok(valid)
    })
    .await
}

#[tauri::command]
pub async fn get_user_keyboard_shortcuts(
    app_state: State<'_, AppState>,
    user_id: i32,
) -> Result<ApiResponse<Option<KeyboardShortcuts>>, AppError> {
    handle_command_async("get_user_keyboard_shortcuts", async || {
        let manager = app_state.get_keyboard_shortcuts_manager().ok_or_else(|| {
            AppError::Runtime("Keyboard shortcuts manager not initialized".to_string())
        })?;

        let shortcuts = manager.get_user_keyboard_shortcuts(user_id).await?;
        Ok(shortcuts)
    })
    .await
}

#[tauri::command]
pub async fn upsert_user_keyboard_shortcuts(
    app_state: State<'_, AppState>,
    shortcuts: KeyboardShortcuts,
) -> Result<ApiResponse<KeyboardShortcuts>, AppError> {
    handle_command_async("upsert_user_keyboard_shortcuts", async || {
        let manager = app_state.get_keyboard_shortcuts_manager().ok_or_else(|| {
            AppError::Runtime("Keyboard shortcuts manager not initialized".to_string())
        })?;

        let data = manager
            .upsert_user_keyboard_shortcuts(shortcuts.user_id, shortcuts)
            .await?;

        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn create_transfer(
    app_state: State<'_, AppState>,
    hash: String,
    transfer: CreateTransfer,
) -> Result<ApiResponse<Transfer>, AppError> {
    handle_command_async("create_transfer", async || {
        let manager = app_state
            .get_transfers_manager()
            .ok_or_else(|| AppError::Runtime("Transfer manager not initialized".to_string()))?;

        let transfer = manager.create_transfer(hash, transfer).await?;
        Ok(transfer)
    })
    .await
}

#[tauri::command]
pub async fn list_transfers(
    app_state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<Transfer>>, AppError> {
    handle_command_async("list_transfers", async || {
        let manager = app_state
            .get_transfers_manager()
            .ok_or_else(|| AppError::Runtime("Transfer manager not initialized".to_string()))?;

        let transfers = manager.list_transfers().await?;
        Ok(transfers)
    })
    .await
}

#[tauri::command]
pub async fn delete_transfer(
    app_state: State<'_, AppState>,
    hash: String,
) -> Result<ApiResponse<bool>, AppError> {
    handle_command_async("delete_transfer", async || {
        let manager = app_state
            .get_transfers_manager()
            .ok_or_else(|| AppError::Runtime("Transfer manager not initialized".to_string()))?;

        let deleted = manager.delete_transfer(&hash).await?;
        Ok(deleted)
    })
    .await
}

#[tauri::command]
pub async fn upsert_transfer(
    app_state: State<'_, AppState>,
    hash: String,
    transfer: CreateTransfer,
) -> Result<ApiResponse<Transfer>, AppError> {
    handle_command_async("upsert_transfer", async || {
        let manager = app_state
            .get_transfers_manager()
            .ok_or_else(|| AppError::Runtime("Transfer manager not initialized".to_string()))?;

        let transfer = manager.upsert_transfer(hash, transfer).await?;
        Ok(transfer)
    })
    .await
}

#[tauri::command]
pub async fn delete_local_media_by_directory(
    app_state: State<'_, AppState>,
    directory: String,
) -> Result<ApiResponse<()>, AppError> {
    handle_command_async("delete_local_media_by_directory", async || {
        let manager = app_state
            .get_local_media_manager()
            .ok_or_else(|| AppError::Runtime("Local media manager not initialized".to_string()))?;

        manager.delete_by_directory(&directory).await?;
        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn scan_local_folder(
    app_state: State<'_, AppState>,
    directory: Option<String>,
) -> Result<ApiResponse<ScanResult>, AppError> {
    handle_command_async("scan_local_folder", async || {
        let manager = app_state
            .get_local_media_manager()
            .ok_or_else(|| AppError::Runtime("Local media manager not initialized".to_string()))?;

        match directory {
            Some(dir) if !dir.is_empty() => {
                let result = manager.scan_folder(&dir).await?;
                Ok(result)
            }
            _ => Ok(ScanResult::default()),
        }
    })
    .await
}

#[tauri::command]
pub async fn find_local_media(
    app_state: State<'_, AppState>,
    imdb_id: String,
    title: String,
    year: Option<i32>,
    season: Option<i64>,
    episode: Option<i64>,
) -> Result<ApiResponse<Option<LocalMediaWithFiles>>, AppError> {
    handle_command_async("find_local_media", async || {
        let manager = app_state
            .get_local_media_manager()
            .ok_or_else(|| AppError::Runtime("Local media manager not initialized".to_string()))?;

        let result = manager
            .find_local_media(&title, year, &imdb_id, season, episode)
            .await?;
        Ok(result.map(|(media, filepaths)| LocalMediaWithFiles { media, filepaths }))
    })
    .await
}

#[tauri::command]
pub async fn get_editable_local_media(
    app_state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<EditableMediaFields>>, AppError> {
    handle_command_async("get_editable_local_media", async || {
        let manager = app_state
            .get_local_media_manager()
            .ok_or_else(|| AppError::Runtime("Local media manager not initialized".to_string()))?;

        let result = manager.get_editable_local_media().await?;
        Ok(result)
    })
    .await
}

#[tauri::command]
pub async fn update_local_media(
    app_state: State<'_, AppState>,
    media: Vec<EditableMediaFields>,
) -> Result<ApiResponse<()>, AppError> {
    handle_command_async("update_local_media", async || {
        let manager = app_state
            .get_local_media_manager()
            .ok_or_else(|| AppError::Runtime("Local media manager not initialized".to_string()))?;

        manager.update_local_media(media).await?;
        Ok(())
    })
    .await
}
