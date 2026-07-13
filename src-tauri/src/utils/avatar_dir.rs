use std::{env, fs, path::PathBuf};

pub fn get_avatars_dir() -> Result<PathBuf, String> {
    let avatars_dir = if cfg!(debug_assertions) {
        env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?
            .join("avatars")
    } else {
        let exe_path =
            env::current_exe().map_err(|e| format!("Failed to get executable path: {}", e))?;
        let base_dir = exe_path
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or("Failed to get executable directory".to_string())?;
        base_dir.join("avatars")
    };
    fs::create_dir_all(&avatars_dir)
        .map_err(|e| format!("Failed to create avatars directory: {}", e))?;
    Ok(avatars_dir)
}
