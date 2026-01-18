use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn test_db(state: State<AppState>) -> Result<String, String> {
    let db_lock = state
        .database
        .lock()
        .map_err(|e| format!("Failed to lock database: {}", e))?;

    if db_lock.is_some() {
        Ok("Database is available".to_string())
    } else {
        Ok("Database is not available".to_string())
    }
}
