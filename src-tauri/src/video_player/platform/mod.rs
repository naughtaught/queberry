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
        match crate::video_player::platform::windows::get_mpv_conf_path() {
            Ok(conf_path) => {
                if conf_path.exists() {
                    Some(conf_path)
                } else {
                    None
                }
            }
            Err(e) => {
                log::warn!("Failed to get mpv.conf path from windows module: {}", e);
                None
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        log::debug!("Running on Linux platform");
        // If linux module has get_mpv_conf_path()
        return linux::get_mpv_conf_path();

        // If linux module only has get_mpv_conf() (directory):
        /*
        return match linux::get_mpv_conf() {
            Ok(dir) => {
                let conf_path = dir.join("mpv").join("mpv.conf");
                if conf_path.exists() {
                    Some(conf_path)
                } else {
                    None
                }
            }
            Err(_) => None,
        };
        */
    }

    #[cfg(target_os = "macos")]
    {
        log::debug!("Running on macOS platform");
        // Similar to linux
        return macos::get_mpv_conf_path();
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        log::warn!("Unsupported platform for mpv.conf");
        None
    }
}
