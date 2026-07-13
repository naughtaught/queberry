use crate::api::api_client;
use crate::api::types::{MediaFilters, UpsertUserMediaData};
use crate::errors::{handle_command_async, ApiResponse};
use crate::AppError;

#[tauri::command]
pub async fn api_register(
    email: String,
    username: String,
    password: String,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("api_register", async || {
        let result = api_client::register(email, username, password).await?;
        Ok(serde_json::json!({
            "postgresId": result.user.postgres_id,
            "email": result.user.email,
            "username": result.user.username,
            "token": result.user.token,
        }))
    })
    .await
}

#[tauri::command]
pub async fn api_login(
    email: String,
    password: String,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("api_login", async || {
        let result = api_client::login(email, password).await?;
        Ok(serde_json::json!({
            "postgresId": result.user.postgres_id,
            "email": result.user.email,
            "username": result.user.username,
            "token": result.user.token,
        }))
    })
    .await
}

#[tauri::command]
pub async fn api_fetch_trending(
    postgres_id: String,
    token: String,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("api_fetch_trending", async || {
        let data = api_client::fetch_trending(&postgres_id, &token).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn api_fetch_up_next(
    postgres_id: String,
    token: String,
    local_date: String,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("api_fetch_up_next", async || {
        let data = api_client::fetch_up_next(&postgres_id, &token, &local_date).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn api_fetch_recent_releases(
    postgres_id: String,
    token: String,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("api_fetch_recent_releases", async || {
        let data = api_client::fetch_recent(&postgres_id, &token).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn api_fetch_media_item(
    postgres_id: String,
    token: String,
    id: i32,
    media_type: String,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("api_fetch_media_item", async || {
        let data = api_client::fetch_media_item(&postgres_id, &token, &media_type, id).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn api_fetch_season_data(
    postgres_id: String,
    token: String,
    media_id: i32,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("api_fetch_season_data", async || {
        let data = api_client::fetch_season_data(&postgres_id, &token, media_id).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn api_upsert_user_media(
    postgres_id: String,
    token: String,
    data: UpsertUserMediaData,
) -> Result<ApiResponse<()>, AppError> {
    handle_command_async("api_upsert_user_media", async || {
        api_client::upsert_user_media(&postgres_id, &token, data).await?;
        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn api_upsert_watched_episodes(
    postgres_id: String,
    token: String,
    media_id: i32,
    episode_ids: Vec<i32>,
) -> Result<ApiResponse<()>, AppError> {
    handle_command_async("api_upsert_watched_episodes", async || {
        api_client::upsert_watched_episodes(&postgres_id, &token, media_id, episode_ids).await?;
        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn api_delete_watched_episodes(
    postgres_id: String,
    token: String,
    media_id: i32,
) -> Result<ApiResponse<()>, AppError> {
    handle_command_async("api_delete_watched_episodes", async || {
        api_client::delete_watched_episodes(&postgres_id, &token, media_id).await?;
        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn api_delete_user_media(
    postgres_id: String,
    token: String,
    media_id: i32,
) -> Result<ApiResponse<()>, AppError> {
    handle_command_async("api_delete_user_media", async || {
        api_client::delete_user_media(&postgres_id, &token, media_id).await?;
        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn api_delete_user(
    postgres_id: String,
    token: String,
) -> Result<ApiResponse<()>, AppError> {
    handle_command_async("api_delete_user", async || {
        api_client::delete_user(&postgres_id, &token).await?;
        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn api_reset_user_data(
    postgres_id: String,
    token: String,
) -> Result<ApiResponse<()>, AppError> {
    handle_command_async("api_reset_user_data", async || {
        api_client::reset_user_data(&postgres_id, &token).await?;
        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn api_reset_user_media_state(
    postgres_id: String,
    token: String,
    column_name: String,
) -> Result<ApiResponse<()>, AppError> {
    handle_command_async("api_reset_user_media_state", async || {
        api_client::reset_user_media_state(&postgres_id, &token, &column_name).await?;
        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn api_clear_user_episode_groups(
    postgres_id: String,
    token: String,
) -> Result<ApiResponse<()>, AppError> {
    handle_command_async("api_clear_user_episode_groups", async || {
        api_client::clear_user_episode_groups(&postgres_id, &token).await?;
        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn api_delete_watched_episode_ids(
    postgres_id: String,
    token: String,
    media_id: i32,
    episode_ids: Vec<i32>,
) -> Result<ApiResponse<()>, AppError> {
    handle_command_async("api_delete_watched_episode_ids", async || {
        api_client::delete_watched_episode_ids(&postgres_id, &token, media_id, episode_ids).await?;
        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn api_fetch_data(
    postgres_id: String,
    token: String,
    filters: MediaFilters,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("api_fetch_data", async || {
        let data = api_client::fetch_media_list(&postgres_id, &token, filters).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn api_verify_token(
    postgres_id: String,
    token: String,
) -> Result<ApiResponse<bool>, AppError> {
    handle_command_async("api_verify_token", async || {
        let valid = api_client::verify_token(&postgres_id, &token).await?;
        Ok(valid)
    })
    .await
}

#[tauri::command]
pub async fn api_search_media(
    postgres_id: String,
    token: String,
    search_term: String,
    page: i64,
    page_size: i64,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("api_search_media", || async {
        let data =
            api_client::search_media(&postgres_id, &token, &search_term, page, page_size).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn api_delete_media_request(
    postgres_id: String,
    token: String,
    request_id: i32,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("api_delete_media_request", async || {
        let data = api_client::delete_media_request(&postgres_id, &token, request_id).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn api_request_media(
    postgres_id: String,
    token: String,
    imdb_id: String,
    tmdb_id: Option<i32>,
    tmdb_type: Option<String>,
    tvdb_id: Option<i32>,
    tvdb_type: Option<String>,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("api_request_media", async || {
        let data = api_client::request_media(
            &postgres_id,
            &token,
            &imdb_id,
            tmdb_id,
            tmdb_type,
            tvdb_id,
            tvdb_type,
        )
        .await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn api_get_media_requests(
    postgres_id: String,
    token: String,
    status: Option<String>,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("api_get_media_requests", async || {
        let data = api_client::get_media_requests(&postgres_id, &token, status).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn api_get_blacklist_entry(
    postgres_id: String,
    token: String,
    imdb_id: String,
) -> Result<ApiResponse<Option<serde_json::Value>>, AppError> {
    handle_command_async("api_get_blacklist_entry", async || {
        let data = api_client::get_blacklist_entry(&postgres_id, &token, &imdb_id).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn api_update_user(
    postgres_id: String,
    token: String,
    email: Option<String>,
    username: Option<String>,
    current_password: Option<String>,
    new_password: Option<String>,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("api_update_user", async || {
        let data = api_client::update_user(
            &postgres_id,
            &token,
            email,
            username,
            current_password,
            new_password,
        )
        .await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn api_reset_token(
    postgres_id: String,
    token: String,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("api_reset_token", async || {
        let data = api_client::reset_token(&postgres_id, &token).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn api_fetch_random_media(
    postgres_id: String,
    token: String,
    filters: MediaFilters,
    exclude_media_ids: Option<Vec<i32>>,
) -> Result<ApiResponse<Option<serde_json::Value>>, AppError> {
    handle_command_async("api_fetch_random_media", async || {
        let data = api_client::fetch_random_media(&postgres_id, &token, filters, exclude_media_ids)
            .await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn api_fetch_random_backdrop(
    postgres_id: String,
    token: String,
) -> Result<ApiResponse<Option<String>>, AppError> {
    handle_command_async("api_fetch_random_backdrop", async || {
        let data = api_client::fetch_random_backdrop(Some(&postgres_id), Some(&token)).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn api_fetch_collections(
    postgres_id: String,
    token: String,
    media_id: i32,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("api_fetch_collections", async || {
        let data = api_client::api_fetch_collections(&postgres_id, &token, media_id).await?;
        Ok(data)
    })
    .await
}

#[tauri::command]
pub async fn api_fetch_related_media(
    postgres_id: String,
    token: String,
    media_id: i32,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    handle_command_async("api_fetch_related_media", async || {
        let data = api_client::api_fetch_related_media(&postgres_id, &token, media_id).await?;
        Ok(data)
    })
    .await
}
