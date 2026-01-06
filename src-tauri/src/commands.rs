use crate::plugin_system;
use plugin_system::{load_all_plugins, types::Plugin};
use std::collections::HashMap;

#[tauri::command]
pub fn get_plugins() -> Result<Vec<Plugin>, String> {
    load_all_plugins()
}

#[tauri::command]
pub async fn http_get(
    url: String,
    headers: Option<HashMap<String, String>>,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let mut request = client.get(&url);

    if let Some(headers_map) = headers {
        for (key, value) in headers_map {
            request = request.header(&key, &value);
        }
    } else {
        request = request
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36")
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("Connection", "keep-alive")
            .header("Upgrade-Insecure-Requests", "1")
            .header("Sec-Fetch-Dest", "document")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-User", "?1");
    }

    match request.send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(text) => Ok(text),
                    Err(e) => Err(format!("Failed to read response text: {}", e)),
                }
            } else {
                Err(format!(
                    "HTTP request failed with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => Err(format!("HTTP request failed: {}", e)),
    }
}
