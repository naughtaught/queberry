use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::api::types::{LoginResponse, MediaFilters, UpsertUserMediaData};
use crate::constants::{API_BASE, API_CLIENT};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

fn api_error(e: impl std::fmt::Display) -> String {
    e.to_string().replace(API_BASE, "API")
}

pub fn get_client() -> &'static Client {
    API_CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client")
    })
}

pub async fn login(email: String, password: String) -> Result<LoginResponse, String> {
    let client = get_client();
    let response = client
        .post(format!("{}/api/login", API_BASE))
        .json(&serde_json::json!({ "email": email, "password": password }))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    let data = api_response
        .data
        .ok_or_else(|| "No data returned".to_string())?;

    Ok(LoginResponse {
        user: serde_json::from_value(data["user"].clone()).map_err(api_error)?,
        token: data["user"]["token"].as_str().unwrap_or("").to_string(),
    })
}

pub async fn register(
    email: String,
    username: String,
    password: String,
) -> Result<LoginResponse, String> {
    let client = get_client();
    let response = client
        .post(format!("{}/api/register", API_BASE))
        .json(&serde_json::json!({
            "email": email,
            "username": username,
            "password": password
        }))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    let data = api_response
        .data
        .ok_or_else(|| "No data returned".to_string())?;

    Ok(LoginResponse {
        user: serde_json::from_value(data["user"].clone()).map_err(api_error)?,
        token: data["user"]["token"].as_str().unwrap_or("").to_string(),
    })
}

pub async fn fetch_trending(postgres_id: &str, token: &str) -> Result<serde_json::Value, String> {
    let client = get_client();
    let response = client
        .get(format!("{}/api/trending", API_BASE))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    api_response
        .data
        .ok_or_else(|| "No data returned".to_string())
}

pub async fn fetch_up_next(
    postgres_id: &str,
    token: &str,
    local_date: &str,
) -> Result<serde_json::Value, String> {
    let client = get_client();
    let response = client
        .get(format!("{}/api/up-next", API_BASE))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .query(&[("local_date", local_date)])
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    api_response
        .data
        .ok_or_else(|| "No data returned".to_string())
}

pub async fn fetch_recent(postgres_id: &str, token: &str) -> Result<serde_json::Value, String> {
    let client = get_client();
    let response = client
        .get(format!("{}/api/recent", API_BASE))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    api_response
        .data
        .ok_or_else(|| "No data returned".to_string())
}

pub async fn fetch_media_item(
    postgres_id: &str,
    token: &str,
    media_type: &str,
    id: i32,
) -> Result<serde_json::Value, String> {
    let client = get_client();
    let response = client
        .get(format!("{}/api/media/{}/{}", API_BASE, media_type, id))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    api_response
        .data
        .ok_or_else(|| "No data returned".to_string())
}

pub async fn fetch_season_data(
    postgres_id: &str,
    token: &str,
    media_id: i32,
) -> Result<serde_json::Value, String> {
    let client = get_client();
    let response = client
        .get(format!("{}/api/season/{}", API_BASE, media_id))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    api_response
        .data
        .ok_or_else(|| "No data returned".to_string())
}

pub async fn upsert_user_media(
    postgres_id: &str,
    token: &str,
    data: UpsertUserMediaData,
) -> Result<(), String> {
    let client = get_client();
    let response = client
        .post(format!("{}/api/user-media", API_BASE))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .json(&data)
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<()> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    Ok(())
}

pub async fn verify_token(postgres_id: &str, token: &str) -> Result<bool, String> {
    let client = get_client();
    let response = client
        .get(format!("{}/api/verify", API_BASE))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(api_error)?;

    if response.status() != 200 {
        return Err(format!(
            "Verification failed with status: {}",
            response.status()
        ));
    }

    let api_response: ApiResponse<bool> = response
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Verification failed".to_string()));
    }

    Ok(api_response.data.unwrap_or(false))
}

pub async fn fetch_media_list(
    postgres_id: &str,
    token: &str,
    filters: MediaFilters,
) -> Result<serde_json::Value, String> {
    let client = get_client();

    let response = client
        .post(format!("{}/api/media-list", API_BASE))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .json(&filters)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read body: {}", e))?;

    let text = String::from_utf8_lossy(&bytes).into_owned();

    let api_response: ApiResponse<serde_json::Value> =
        serde_json::from_str(&text).map_err(|e| {
            format!(
                "JSON parse error: {} - Body: {}",
                e,
                &text[..text.len().min(200)]
            )
        })?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    api_response
        .data
        .ok_or_else(|| "No data returned".to_string())
}

pub async fn upsert_watched_episodes(
    postgres_id: &str,
    token: &str,
    media_id: i32,
    episode_ids: Vec<i32>,
) -> Result<(), String> {
    let client = get_client();
    let response = client
        .post(format!("{}/api/watched", API_BASE))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "media_id": media_id,
            "episode_ids": episode_ids,
        }))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<()> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    Ok(())
}

pub async fn delete_watched_episodes(
    postgres_id: &str,
    token: &str,
    media_id: i32,
) -> Result<(), String> {
    let client = get_client();
    let response = client
        .delete(format!("{}/api/watched/{}", API_BASE, media_id))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<()> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    Ok(())
}

pub async fn delete_user_media(
    postgres_id: &str,
    token: &str,
    media_id: i32,
) -> Result<(), String> {
    let client = get_client();
    let response = client
        .delete(format!("{}/api/user-media/{}", API_BASE, media_id))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<()> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    Ok(())
}

pub async fn delete_user(postgres_id: &str, token: &str) -> Result<(), String> {
    let client = get_client();
    let response = client
        .delete(format!("{}/api/user", API_BASE))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(api_error)?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_default();
        return Err(format!("HTTP {}: {}", status.as_u16(), error_body));
    }

    Ok(())
}

pub async fn reset_user_media_state(
    postgres_id: &str,
    token: &str,
    column_name: &str,
) -> Result<(), String> {
    let client = get_client();
    let response = client
        .post(format!("{}/api/user-media/reset", API_BASE))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "postgres_id": postgres_id,
            "column_name": column_name,
        }))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<()> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    Ok(())
}

pub async fn clear_user_episode_groups(postgres_id: &str, token: &str) -> Result<(), String> {
    let client = get_client();
    let response = client
        .post(format!("{}/api/user-media/clear-episode-groups", API_BASE))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "postgres_id": postgres_id,
        }))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<()> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    Ok(())
}

pub async fn delete_watched_episode_ids(
    postgres_id: &str,
    token: &str,
    media_id: i32,
    episode_ids: Vec<i32>,
) -> Result<(), String> {
    let client = get_client();
    let response = client
        .delete(format!("{}/api/watched/batch/{}", API_BASE, media_id))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "episode_ids": episode_ids,
        }))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<()> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    Ok(())
}

pub async fn search_media(
    postgres_id: &str,
    token: &str,
    search_term: &str,
    page: i64,
    page_size: i64,
) -> Result<serde_json::Value, String> {
    let client = get_client();
    let response = client
        .get(format!("{}/api/search-media", API_BASE))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .query(&[
            ("search_term", search_term),
            ("page", &page.to_string()),
            ("page_size", &page_size.to_string()),
        ])
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    api_response
        .data
        .ok_or_else(|| "No data returned".to_string())
}

pub async fn request_media(
    postgres_id: &str,
    token: &str,
    imdb_id: &str,
    tmdb_id: Option<i32>,
    tmdb_type: Option<String>,
    tvdb_id: Option<i32>,
    tvdb_type: Option<String>,
) -> Result<serde_json::Value, String> {
    let client = get_client();

    let request_body = serde_json::json!({
        "external_ids": {
            "imdb": { "id": imdb_id },
            "tmdb": { "id": tmdb_id, "type": tmdb_type },
            "tvdb": { "id": tvdb_id, "type": tvdb_type }
        }
    });

    let response = client
        .post(format!("{}/api/media/request", API_BASE))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .json(&request_body)
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    api_response
        .data
        .ok_or_else(|| "No data returned".to_string())
}

pub async fn get_media_requests(
    postgres_id: &str,
    token: &str,
    status: Option<String>,
) -> Result<serde_json::Value, String> {
    let client = get_client();

    let mut url = format!("{}/api/media/requests", API_BASE);
    if let Some(s) = &status {
        url = format!("{}?status={}", url, s);
    }

    let response = client
        .get(&url)
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    api_response
        .data
        .ok_or_else(|| "No data returned".to_string())
}

pub async fn delete_media_request(
    postgres_id: &str,
    token: &str,
    request_id: i32,
) -> Result<serde_json::Value, String> {
    let client = get_client();

    let response = client
        .delete(format!("{}/api/media/request/{}", API_BASE, request_id))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    api_response
        .data
        .ok_or_else(|| "No data returned".to_string())
}

pub async fn get_blacklist_entry(
    postgres_id: &str,
    token: &str,
    imdb_id: &str,
) -> Result<Option<serde_json::Value>, String> {
    let client = get_client();
    let response = client
        .get(format!("{}/api/blacklist/{}", API_BASE, imdb_id))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    Ok(api_response.data)
}

pub async fn reset_user_data(postgres_id: &str, token: &str) -> Result<(), String> {
    let client = get_client();
    let response = client
        .delete(format!("{}/api/user/reset", API_BASE))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(api_error)?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_default();
        return Err(format!("HTTP {}: {}", status.as_u16(), error_body));
    }

    Ok(())
}

pub async fn update_user(
    postgres_id: &str,
    token: &str,
    email: Option<String>,
    username: Option<String>,
    current_password: Option<String>,
    new_password: Option<String>,
) -> Result<serde_json::Value, String> {
    let client = get_client();

    let mut body = serde_json::json!({});

    if let Some(e) = email {
        body["email"] = serde_json::json!(e);
    }
    if let Some(u) = username {
        body["username"] = serde_json::json!(u);
    }
    if let Some(cp) = current_password {
        body["current_password"] = serde_json::json!(cp);
    }
    if let Some(np) = new_password {
        body["new_password"] = serde_json::json!(np);
    }

    let response = client
        .put(format!("{}/api/user", API_BASE))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .json(&body)
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    api_response
        .data
        .ok_or_else(|| "No data returned".to_string())
}

pub async fn reset_token(postgres_id: &str, token: &str) -> Result<serde_json::Value, String> {
    let client = get_client();

    let response = client
        .post(format!("{}/api/user/token/reset", API_BASE))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    api_response
        .data
        .ok_or_else(|| "No data returned".to_string())
}

pub async fn fetch_random_media(
    postgres_id: &str,
    token: &str,
    filters: MediaFilters,
    exclude_media_ids: Option<Vec<i32>>,
) -> Result<Option<serde_json::Value>, String> {
    let client = get_client();

    let mut request_body = serde_json::to_value(&filters).map_err(api_error)?;

    if let Some(ids) = exclude_media_ids {
        request_body["excludeMediaIds"] = serde_json::json!(ids);
    }

    let response = client
        .post(format!("{}/api/random", API_BASE))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .json(&request_body)
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    Ok(api_response.data)
}

pub async fn fetch_random_backdrop(
    postgres_id: Option<&str>,
    token: Option<&str>,
) -> Result<Option<String>, String> {
    let client = get_client();

    let mut request = client.get(format!("{}/api/random-backdrop", API_BASE));

    if let (Some(id), Some(tok)) = (postgres_id, token) {
        request = request
            .header("X-User-Id", id)
            .header("Authorization", format!("Bearer {}", tok));
    }

    let response = request.send().await.map_err(api_error)?;

    let api_response: ApiResponse<Option<String>> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    Ok(api_response.data.flatten())
}

pub async fn api_fetch_collections(
    postgres_id: &str,
    token: &str,
    media_id: i32,
) -> Result<serde_json::Value, String> {
    let client = get_client();
    let response = client
        .get(format!("{}/api/collections/{}", API_BASE, media_id))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    api_response
        .data
        .ok_or_else(|| "No data returned".to_string())
}

pub async fn api_fetch_related_media(
    postgres_id: &str,
    token: &str,
    media_id: i32,
) -> Result<serde_json::Value, String> {
    let client = get_client();
    let response = client
        .get(format!("{}/api/related/{}", API_BASE, media_id))
        .header("X-User-Id", postgres_id)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(api_error)?;

    let api_response: ApiResponse<serde_json::Value> = response.json().await.map_err(api_error)?;

    if !api_response.success {
        return Err(api_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()));
    }

    api_response
        .data
        .ok_or_else(|| "No data returned".to_string())
}
