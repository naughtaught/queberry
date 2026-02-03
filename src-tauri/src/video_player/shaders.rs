use std::fs;
use std::path::Path;

use libmpv2::Mpv;

use crate::video_player::platform::windows::get_shaders_dir;
use crate::video_player::types::ShaderInfo;
use crate::AppError;

pub fn get_all_shaders() -> Result<Vec<ShaderInfo>, String> {
    let shaders_dir = get_shaders_dir()?;

    if !shaders_dir.exists() {
        return Ok(Vec::new());
    }

    let mut shaders = Vec::new();

    for entry in fs::read_dir(&shaders_dir)
        .map_err(|e| format!("Failed to read shaders directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if path.is_file() {
            let extension = path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();

            if extension == "glsl" || extension == "frag" || extension == "fs" {
                let name = path
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let full_path = path.canonicalize().unwrap_or(path.clone());

                shaders.push(ShaderInfo {
                    name,
                    filename: path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("")
                        .to_string(),
                    path: normalize_path_for_mpv(&full_path)?,
                });
            }
        }
    }

    shaders.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(shaders)
}

fn normalize_path_for_mpv(path: &Path) -> Result<String, String> {
    let canonical = path
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize path: {}", e))?;

    let mut path_str = canonical.to_string_lossy().to_string();

    #[cfg(windows)]
    let normalized = {
        if path_str.starts_with(r"\\?\") {
            path_str = path_str.trim_start_matches(r"\\?\").to_string();
        }

        if path_str.len() > 1 && path_str.chars().nth(1) == Some(':') {
            let drive_letter = path_str.chars().next().unwrap().to_ascii_uppercase();
            path_str = format!("{}{}", drive_letter, &path_str[1..]);
        }

        path_str
    };

    #[cfg(target_os = "linux")]
    let normalized = { path_str };

    #[cfg(target_os = "macos")]
    let normalized = { path_str };

    Ok(normalized)
}

pub fn toggle_shader(mpv: &Mpv, shader_filepath: &str) -> Result<(), AppError> {
    let _ = mpv.command("change-list", &["glsl-shaders", "toggle", shader_filepath]);

    Ok(())
}
