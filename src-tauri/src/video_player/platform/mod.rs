#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

use std::path::PathBuf;

use tauri::WebviewWindow;

pub fn get_window_handle_id(window: &WebviewWindow) -> Option<i64> {
    #[cfg(target_os = "windows")]
    return crate::video_player::platform::windows::get_window_handle_id(window);

    #[cfg(target_os = "linux")]
    return linux::get_window_handle_id(window);

    #[cfg(target_os = "macos")]
    return macos::get_window_handle_id(window);

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        log::warn!("Unsupported platform for window handle");
        None
    }
}

pub fn get_mpv_conf_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        match crate::video_player::platform::windows::get_mpv_dir() {
            Ok(mpv_dir) => {
                let shaders_dir = mpv_dir.join("shaders");
                if let Err(e) = std::fs::create_dir_all(&shaders_dir) {
                    log::warn!("Failed to create mpv/shaders directory: {}", e);
                }
                let conf_path = mpv_dir.join("mpv.conf");
                if conf_path.exists() {
                    Some(conf_path)
                } else {
                    None
                }
            }
            Err(e) => {
                log::warn!("Failed to get mpv dir: {}", e);
                None
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        log::debug!("Running on Linux platform");
        return linux::get_mpv_conf_path();
    }

    #[cfg(target_os = "macos")]
    {
        log::debug!("Running on macOS platform");
        return macos::get_mpv_conf_path();
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        log::warn!("Unsupported platform for mpv.conf");
        None
    }
}
