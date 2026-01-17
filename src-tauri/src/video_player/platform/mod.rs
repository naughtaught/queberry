#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

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
