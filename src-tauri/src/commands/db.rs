use crate::{errors::ApiResponse, state::AppState, AppError};
use tauri::State;

#[tauri::command]
pub async fn get_user_settings(
    app_state: State<'_, AppState>,
    user_id: i32,
) -> Result<ApiResponse<serde_json::Value>, String> {
    let result = (|| -> Result<serde_json::Value, AppError> {
        let settings_manager = app_state
            .get_settings_manager()
            .ok_or_else(|| AppError::Runtime("Settings not initialized".to_string()))?;

        let settings = settings_manager.get_settings(user_id)?;
        Ok(settings.to_frontend_json())
    })();

    match result {
        Ok(data) => Ok(ApiResponse::ok(data)),
        Err(e) => Ok(ApiResponse::err(e)),
    }
}

#[tauri::command]
pub async fn update_user_settings(
    app_state: State<'_, AppState>,
    settings: crate::db::types::UserSettings,
) -> Result<ApiResponse<()>, String> {
    let result = (|| -> Result<(), AppError> {
        let settings_manager = app_state
            .get_settings_manager()
            .ok_or_else(|| AppError::Runtime("Settings not initialized".to_string()))?;

        settings_manager.update_settings(&settings)?;
        Ok(())
    })();

    match result {
        Ok(_) => Ok(ApiResponse::ok(())),
        Err(e) => Ok(ApiResponse::err(e)),
    }
}
