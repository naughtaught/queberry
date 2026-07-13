use std::path::PathBuf;
use std::{env, fs};
use tauri::AppHandle;
use tauri::Manager;

fn is_development_mode() -> bool {
    cfg!(debug_assertions)
}

pub fn get_db_dir(app_handle: &AppHandle) -> Result<PathBuf, String> {
    if is_development_mode() {
        let db_path = env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?
            .join("db");
        fs::create_dir_all(&db_path)
            .map_err(|e| format!("Failed to create db directory: {}", e))?;
        return Ok(db_path);
    }

    let local_data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get local data directory: {}", e))?;

    let db_path = local_data_dir.join("db");
    fs::create_dir_all(&db_path).map_err(|e| format!("Failed to create db directory: {}", e))?;

    Ok(db_path)
}
