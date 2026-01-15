use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use tauri::WebviewWindow;

pub fn init() {
    use std::env;
    use std::path::PathBuf;

    log::info!("Performing Windows-specific MPV setup");

    let possible_paths = vec![
        PathBuf::from("lib/windows"),
        PathBuf::from("../lib/windows"),
        PathBuf::from("../../lib/windows"),
    ];

    for path in possible_paths {
        let dll_path = path.join("libmpv-2.dll");
        if dll_path.exists() {
            if let Some(path_str) = path.to_str() {
                log::info!("Found MPV DLL at: {}", path_str);
                let current_path = env::var("PATH").unwrap_or_default();
                env::set_var("PATH", format!("{};{}", current_path, path_str));
                break;
            }
        }
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
