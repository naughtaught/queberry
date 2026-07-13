use crate::errors::{handle_command, ApiResponse};
use crate::utils::avatar_dir::get_avatars_dir;
use crate::AppError;

#[tauri::command]
pub fn get_avatars() -> ApiResponse<Vec<(String, String)>> {
    handle_command("get_avatars", || {
        let avatars_dir = get_avatars_dir()
            .map_err(|e| AppError::Runtime(format!("Failed to get avatars directory: {}", e)))?;

        if !avatars_dir.exists() {
            std::fs::create_dir_all(&avatars_dir).map_err(|e| {
                AppError::Runtime(format!("Failed to create avatars directory: {}", e))
            })?;
        }

        let entries = std::fs::read_dir(&avatars_dir)
            .map_err(|e| AppError::Runtime(format!("Failed to read avatars directory: {}", e)))?
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                let name_lower = name.to_lowercase();
                if name_lower.ends_with(".png")
                    || name_lower.ends_with(".jpg")
                    || name_lower.ends_with(".jpeg")
                    || name_lower.ends_with(".gif")
                    || name_lower.ends_with(".webp")
                {
                    let full_path = e.path().to_string_lossy().to_string();
                    Some((name, full_path))
                } else {
                    None
                }
            })
            .collect();

        Ok(entries)
    })
}

#[tauri::command]
pub fn get_avatars_dir_path() -> Result<String, String> {
    get_avatars_dir().map(|p| p.to_string_lossy().to_string())
}
