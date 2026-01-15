use crate::errors::{AppError, Result};
use crate::video_player::types::ShaderInfo;
use libmpv2::Mpv;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ShaderManager {
    mpv: Arc<Mutex<Mpv>>,
    shaders_dir: PathBuf,
    available_shaders: Vec<ShaderInfo>,
}

impl ShaderManager {
    pub fn new(mpv: Arc<Mutex<Mpv>>, shaders_dir: PathBuf) -> Self {
        let available_shaders = Self::discover_shaders(&shaders_dir);

        Self {
            mpv,
            shaders_dir,
            available_shaders,
        }
    }

    pub fn get_available_shaders(&self) -> Vec<ShaderInfo> {
        self.available_shaders.clone()
    }

    pub async fn set_shader(&mut self, shader_path: &str) -> Result<()> {
        let mpv = self.mpv.lock().await;

        for shader in &mut self.available_shaders {
            shader.is_active = false;
        }

        mpv.set_property("glsl-shaders", shader_path).map_err(|e| {
            AppError::Runtime(format!("Failed to set shader to '{}': {}", shader_path, e))
        })?;

        if let Some(shader) = self
            .available_shaders
            .iter_mut()
            .find(|s| s.path == shader_path)
        {
            shader.is_active = true;
        }

        Ok(())
    }

    pub async fn clear_shaders(&mut self) -> Result<()> {
        let mpv = self.mpv.lock().await;

        // Deactivate all shaders
        for shader in &mut self.available_shaders {
            shader.is_active = false;
        }

        mpv.set_property("glsl-shaders", "")
            .map_err(|e| AppError::Runtime(format!("Failed to clear shaders: {}", e)))?;

        Ok(())
    }

    pub async fn toggle_shader(&mut self, shader_path: &str) -> Result<bool> {
        let is_active = self
            .available_shaders
            .iter()
            .find(|s| s.path == shader_path)
            .map(|s| s.is_active)
            .unwrap_or(false);

        if is_active {
            self.clear_shaders().await?;
            Ok(false)
        } else {
            self.set_shader(shader_path).await?;
            Ok(true)
        }
    }

    fn discover_shaders(shaders_dir: &PathBuf) -> Vec<ShaderInfo> {
        let mut shaders = vec![ShaderInfo::none()];

        if !shaders_dir.exists() {
            log::warn!(
                "Shaders directory does not exist: {}",
                shaders_dir.display()
            );
            return shaders;
        }

        let entries = match std::fs::read_dir(shaders_dir) {
            Ok(entries) => entries,
            Err(e) => {
                log::error!("Failed to read shaders directory: {}", e);
                return shaders;
            }
        };

        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    let path = entry.path();

                    if Self::is_shader_file(&path) {
                        if let Some(file_name) = path.file_name() {
                            let file_name_str = file_name.to_string_lossy().to_string();
                            let friendly_name = Self::extract_shader_name(&file_name_str);
                            let full_path = path.to_string_lossy().to_string();

                            shaders.push(ShaderInfo::new(friendly_name, file_name_str, full_path));
                        }
                    }
                }
            }
        }

        log::info!("Discovered {} shaders", shaders.len() - 1);
        shaders
    }

    fn is_shader_file(path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            matches!(ext_str.as_str(), "glsl" | "hook" | "frag" | "vert")
        } else {
            false
        }
    }

    fn extract_shader_name(filename: &str) -> String {
        let name = filename
            .trim_end_matches(".glsl")
            .trim_end_matches(".hook")
            .trim_end_matches(".frag")
            .trim_end_matches(".vert");

        let name = name.replace(['_', '-'], " ");

        name.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn refresh(&mut self) {
        self.available_shaders = Self::discover_shaders(&self.shaders_dir);
    }
}
