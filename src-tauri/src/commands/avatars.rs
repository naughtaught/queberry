use crate::errors::{handle_command, ApiResponse};
use crate::utils::avatar_dir::get_avatars_dir;
use crate::AppError;

const VALID_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "webp", "svg", "bmp", "ico", "tiff", "tif", "avif",
];

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
                let is_valid = VALID_EXTENSIONS.iter().any(|ext| name_lower.ends_with(ext));
                if is_valid {
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

#[tauri::command]
pub fn add_avatar(file_path: String) -> ApiResponse<(String, String)> {
    handle_command("add_avatar", || {
        let source = std::path::Path::new(&file_path);

        if !source.exists() {
            return Err(AppError::Runtime(format!(
                "Source file does not exist: {}",
                file_path
            )));
        }

        let extension = source
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
            .ok_or_else(|| AppError::Runtime("File has no extension".to_string()))?;

        if !VALID_EXTENSIONS.contains(&extension.as_str()) {
            return Err(AppError::Runtime(format!(
                "Invalid file type: {}. Allowed types: {}",
                extension,
                VALID_EXTENSIONS.join(", ")
            )));
        }

        let avatars_dir = get_avatars_dir()
            .map_err(|e| AppError::Runtime(format!("Failed to get avatars directory: {}", e)))?;

        if !avatars_dir.exists() {
            return Err(AppError::Runtime(
                "Avatars directory does not exist".to_string(),
            ));
        }

        let file_name = source
            .file_name()
            .ok_or_else(|| AppError::Runtime("Invalid file name".to_string()))?;

        let mut dest_path = avatars_dir.join(file_name);

        if dest_path.exists() {
            let stem = source
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("avatar");
            let mut counter = 1;
            loop {
                let new_name = format!("{}_{}.{}", stem, counter, extension);
                dest_path = avatars_dir.join(&new_name);
                if !dest_path.exists() {
                    break;
                }
                counter += 1;
            }
        }

        std::fs::copy(source, &dest_path)
            .map_err(|e| AppError::Runtime(format!("Failed to copy avatar file: {}", e)))?;

        let file_name = dest_path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("unknown")
            .to_string();
        let full_path = dest_path.to_string_lossy().to_string();

        Ok((file_name, full_path))
    })
}
