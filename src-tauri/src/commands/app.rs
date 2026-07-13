use crate::errors::{handle_command, ApiResponse};
use crate::AppError;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn show_window(app: AppHandle) -> ApiResponse<()> {
    handle_command("show_window", || {
        let window = app
            .get_webview_window("main")
            .ok_or_else(|| AppError::NotFound("Main window not found".to_string()))?;

        window
            .show()
            .map_err(|e| AppError::Runtime(format!("Failed to show window: {}", e)))?;

        Ok(())
    })
}

#[tauri::command]
pub async fn log_error(text: String) -> Result<(), String> {
    if let Ok(error_json) = serde_json::from_str::<serde_json::Value>(&text) {
        let code = error_json.get("code").and_then(|c| c.as_u64()).unwrap_or(0);
        let message = error_json
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("Unknown");
        let stack = error_json.get("stack").and_then(|s| s.as_str());
        let context = error_json.get("context").and_then(|c| c.as_str());

        if code >= 500 {
            log::error!(
                "[Frontend Error] Code: {}, Message: {}, Context: {:?}\nStack: {:?}",
                code,
                message,
                context,
                stack
            );
        } else {
            log::warn!(
                "[Frontend Error] Code: {}, Message: {}, Context: {:?}",
                code,
                message,
                context
            );
        }
    } else {
        log::warn!("[Frontend Error] {}", text);
    }

    log::logger().flush();
    Ok(())
}

#[tauri::command]
pub async fn get_log_path() -> Result<String, AppError> {
    let log_dir = dirs::data_local_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("com.queberry.media")
        .join("logs");

    Ok(log_dir.to_string_lossy().to_string())
}
