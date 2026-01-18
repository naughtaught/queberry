use std::path::PathBuf;
use std::{env, fs};

fn is_development_mode() -> bool {
    cfg!(debug_assertions)
}

pub fn get_db_dir() -> Result<PathBuf, String> {
    let db_path = if is_development_mode() {
        env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?
            .join("db")
    } else {
        let exe_path =
            env::current_exe().map_err(|e| format!("Failed to get executable path: {}", e))?;

        let base_dir = exe_path
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or("Failed to get executable directory".to_string())?;

        base_dir.join("db")
    };

    fs::create_dir_all(&db_path).map_err(|e| format!("Failed to create db directory: {}", e))?;

    Ok(db_path)
}
