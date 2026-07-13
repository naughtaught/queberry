use crate::db::types::DownloadStatus;
use crate::downloads::types::DownloadMetaData;
use crate::errors::{handle_command_async, ApiResponse, AppError};
use crate::state::AppState;
use tauri::{Emitter, State};

#[tauri::command]
pub async fn download_file(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    params: DownloadMetaData,
    user_id: i32,
) -> Result<ApiResponse<String>, AppError> {
    handle_command_async("download_file", async || {
        let download_manager = state
            .get_download_manager()
            .ok_or_else(|| AppError::Runtime("Download manager not initialized".to_string()))?
            .clone();

        let uuid = params.uuid.clone();
        let app_handle_clone = app_handle.clone();

        download_manager
            .download_file_command(app_handle_clone, params, user_id)
            .await
            .map_err(|e| {
                let _ = app_handle.emit(
                    "download_error",
                    serde_json::json!({
                        "uuid": uuid,
                        "message": e.to_string(),
                    }),
                );
                AppError::Runtime(e.to_string())
            })
    })
    .await
}

#[tauri::command]
pub async fn cancel_download(
    state: State<'_, AppState>,
    uuid: String,
    user_id: i32,
) -> Result<ApiResponse<()>, AppError> {
    handle_command_async("cancel_download", async || {
        let download_manager = state
            .get_download_manager()
            .ok_or_else(|| AppError::Runtime("Download manager not initialized".to_string()))?;
        download_manager.cancel_download(user_id, uuid).await;
        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn process_download_queue(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    user_id: i32,
) -> Result<ApiResponse<()>, AppError> {
    handle_command_async("process_download_queue", async || {
        let download_manager = state
            .get_download_manager()
            .ok_or_else(|| AppError::Runtime("Download manager not initialized".to_string()))?
            .clone();

        let db = state
            .get_db_download_manager()
            .ok_or_else(|| AppError::Runtime("Database download manager not initialized".to_string()))?;

        let stuck_downloads = db.list_downloads_by_status(user_id, DownloadStatus::Downloading).await?;
        for stuck in &stuck_downloads {
            let is_active = {
                let downloads = download_manager.downloads.read().await;
                downloads.contains_key(&stuck.uuid)
            };

            if !is_active {
                let _ = db.update_download_status(user_id, &stuck.uuid, DownloadStatus::Pending).await;
            }
        }

        let pending = db.list_downloads_by_status(user_id, DownloadStatus::Pending).await?;
        let paused = db.list_downloads_by_status(user_id, DownloadStatus::Paused).await?;

        let mut all_downloads = Vec::new();
        all_downloads.extend(pending);
        all_downloads.extend(paused);

        all_downloads.sort_by_key(|d| (d.queue_order, d.added_at));

        for download in &all_downloads {
            {
                let downloads = download_manager.downloads.read().await;
                if downloads.contains_key(&download.uuid) {
                    continue;
                }
            }

            if !download_manager.can_start_new_download(user_id).await {
                break;
            }

            let params = DownloadMetaData {
                folder_path: download.folder_path.clone(),
                file_link: download.file_link.clone(),
                resolver_id: download.resolver_id.clone(),
                imdb_id: download.imdb_id.clone().unwrap_or_default(),
                title: download.title.clone(),
                released: download.released,
                season: download.season,
                episode: download.episode,
                file_url: download.file_url.clone(),
                media_poster: download.media_poster.clone(),
                season_poster: download.season_poster.clone(),
                uuid: download.uuid.clone(),
                filename: download.filename.clone(),
            };

            let _ = download_manager.download_file_command(app_handle.clone(), params, user_id).await;
        }

        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn get_all_downloads(
    state: State<'_, AppState>,
    user_id: i32,
) -> Result<ApiResponse<Vec<serde_json::Value>>, AppError> {
    handle_command_async("get_all_downloads", async || {
        let db = state
            .get_db_download_manager()
            .ok_or_else(|| AppError::Runtime("Database download manager not initialized".to_string()))?;
        
        let all_downloads = db.list_downloads(user_id).await?;
        
        let result: Vec<serde_json::Value> = all_downloads
            .into_iter()
            .map(|d| {
                serde_json::json!({
                    "link": {
                        "uuid": d.uuid,
                        "filename": d.filename,
                        "title": d.title,
                        "folderPath": d.folder_path,
                        "fileLink": d.file_link,
                        "resolverId": d.resolver_id,
                        "fileUrl": d.file_url,
                        "imdbId": d.imdb_id,
                        "released": d.released,
                        "season": d.season,
                        "episode": d.episode,
                        "mediaPoster": d.media_poster,
                        "seasonPoster": d.season_poster,
                    },
                    "progress": if d.total_bytes.is_some() && d.total_bytes.unwrap() > 0 {
                        (d.downloaded_bytes.unwrap_or(0) as f64 / d.total_bytes.unwrap() as f64) * 100.0
                    } else {
                        0.0
                    },
                    "speed": 0,
                    "eta": 0,
                    "fileIndex": 1,
                    "status": d.status.as_str(),
                })
            })
            .collect();
        
        Ok(result)
    })
    .await
}

#[tauri::command]
pub async fn clear_completed_downloads(
    state: State<'_, AppState>,
    user_id: i32,
    hours: Option<i64>,
) -> Result<ApiResponse<u64>, AppError> {
    handle_command_async("clear_completed_downloads", async || {
        let db = state
            .get_db_download_manager()
            .ok_or_else(|| AppError::Runtime("Database download manager not initialized".to_string()))?;
        
        let deleted = if let Some(h) = hours {
            db.clear_completed_downloads_older_than(h).await?
        } else {
            db.clear_completed_downloads(user_id).await?
        };
        
        Ok(deleted)
    })
    .await
}

#[tauri::command]
pub async fn retry_download(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    uuid: String,
    new_file_url: String,
    user_id: i32,
) -> Result<ApiResponse<String>, AppError> {
    handle_command_async("retry_download", async || {
        let download_manager = state
            .get_download_manager()
            .ok_or_else(|| AppError::Runtime("Download manager not initialized".to_string()))?
            .clone();
        
        let db = state
            .get_db_download_manager()
            .ok_or_else(|| AppError::Runtime("Database download manager not initialized".to_string()))?;
        
        let download = db.get_download(user_id, &uuid).await?;
        
        db.update_download_url(user_id, &uuid, &new_file_url).await?;
        db.update_download_status(user_id, &uuid, DownloadStatus::Pending).await?;

        let _ = app_handle.emit(
            "download_queued",
            serde_json::json!({
                "uuid": uuid,
            }),
        );
        
        let params = DownloadMetaData {
            folder_path: download.folder_path,
            file_link: download.file_link,
            resolver_id: download.resolver_id,
            imdb_id: download.imdb_id.unwrap_or_default(),
            title: download.title,
            released: download.released,
            season: download.season,
            episode: download.episode,
            file_url: new_file_url,
            media_poster: download.media_poster,
            season_poster: download.season_poster,
            uuid: download.uuid,
            filename: download.filename,
        };
        
        download_manager
            .download_file_command(app_handle, params, user_id)
            .await
            .map_err(|e| AppError::Runtime(e.to_string()))
    })
    .await
}

#[tauri::command]
pub async fn cleanup_downloads_on_login(
    _app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    user_id: i32,
) -> Result<ApiResponse<()>, AppError> {
    handle_command_async("cleanup_downloads_on_login", async || {
        let download_manager = state
            .get_download_manager()
            .ok_or_else(|| AppError::Runtime("Download manager not initialized".to_string()))?
            .clone();

        let db = state
            .get_db_download_manager()
            .ok_or_else(|| AppError::Runtime("Database download manager not initialized".to_string()))?;

        let stuck_downloads = db.list_downloads_by_status(user_id, DownloadStatus::Downloading).await?;
        for stuck in &stuck_downloads {
            let is_active = {
                let downloads = download_manager.downloads.read().await;
                downloads.contains_key(&stuck.uuid)
            };

            if !is_active {
                let _ = db.update_download_status(user_id, &stuck.uuid, DownloadStatus::Pending).await;
            }
        }

        db.clear_completed_downloads_older_than(24).await?;

        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn clear_completed_download(
    _app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    user_id: i32,
    uuid: String,
) -> Result<ApiResponse<bool>, AppError> {
    handle_command_async("clear_single_completed_download", async || {
        let db = state
            .get_db_download_manager()
            .ok_or_else(|| AppError::Runtime("Database download manager not initialized".to_string()))?;

        db.delete_download(user_id, &uuid).await
    })
    .await
}