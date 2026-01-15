// utils/mpv_config.rs
use std::path::PathBuf;
use std::{env, fs};

pub fn get_mpv_config_dir() -> Result<PathBuf, String> {
    let config_dir = if cfg!(debug_assertions) {
        // Development: use project directory
        env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?
            .join(".mpv")
    } else {
        // Production: use system config directory
        let config_dir = dirs::config_dir()
            .ok_or("Failed to get config directory".to_string())?
            .join("mpv");
        config_dir
    };

    // Create directory if it doesn't exist
    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create MPV config directory: {}", e))?;

    // Also create shaders subdirectory
    let shaders_dir = config_dir.join("shaders");
    let _ = fs::create_dir_all(&shaders_dir);

    Ok(config_dir)
}
