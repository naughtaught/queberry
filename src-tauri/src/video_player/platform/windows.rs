use std::{env, fs, path::PathBuf};

use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use tauri::WebviewWindow;

fn is_development_mode() -> bool {
    cfg!(debug_assertions)
}

pub fn init() {
    use std::env;
    use std::path::PathBuf;

    let path_str = "lib/windows";
    let path = PathBuf::from(path_str);
    let dll_path = path.join("libmpv-2.dll");

    if dll_path.exists() {
        let current_path = env::var("PATH").unwrap_or_default();
        env::set_var("PATH", format!("{};{}", current_path, path_str));
    }
}

pub fn get_window_handle_id(window: &WebviewWindow) -> Option<i64> {
    window
        .window_handle()
        .ok()
        .and_then(|handle| match handle.as_raw() {
            RawWindowHandle::Win32(handle) => {
                let hwnd = handle.hwnd.get();
                log::debug!("Got Windows HWND: {:?}", hwnd);
                Some(hwnd as i64)
            }
            _ => {
                log::error!("Expected Win32 handle but got: {:?}", handle.as_raw());
                None
            }
        })
}

pub fn get_mpv_dir() -> Result<PathBuf, String> {
    if is_development_mode() {
        env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?
            .join("../mpv")
            .canonicalize()
            .map_err(|e| format!("Failed to canonicalize mpv path: {}", e))
    } else {
        let exe_path =
            env::current_exe().map_err(|e| format!("Failed to get executable path: {}", e))?;

        let parent = exe_path
            .parent()
            .ok_or("Failed to get executable directory")?;

        let mpv_dir = parent.join("mpv");

        fs::create_dir_all(&mpv_dir)
            .map_err(|e| format!("Failed to create mpv directory: {}", e))?;

        Ok(mpv_dir)
    }
}

pub fn get_mpv_conf_path() -> Result<PathBuf, String> {
    let mpv_dir = get_mpv_dir()?;
    let conf_path = mpv_dir.join("mpv.conf");

    Ok(conf_path)
}
