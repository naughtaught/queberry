use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use tauri::WebviewWindow;

pub fn get_window_handle_id(window: &WebviewWindow) -> Option<i64> {
    if let Ok(handle) = window.window_handle() {
        match handle.as_raw() {
            RawWindowHandle::Win32(handle) => {
                let hwnd = handle.hwnd.get();
                println!("Got Windows HWND: {:?}", hwnd);
                Some(hwnd as i64)
            }
            _ => {
                eprintln!("Expected Win32 handle but got: {:?}", handle.as_raw());
                None
            }
        }
    } else {
        eprintln!("Failed to get window handle");
        None
    }
}

pub fn platform_specific_init() {
    use std::env;
    use std::path::PathBuf;

    println!("Performing Windows-specific MPV setup...");

    let possible_paths = vec![
        PathBuf::from("lib/windows"),
        PathBuf::from("../lib/windows"),
        PathBuf::from("../../lib/windows"),
    ];

    for path in possible_paths {
        let dll_path = path.join("libmpv-2.dll");
        if dll_path.exists() {
            if let Some(path_str) = path.to_str() {
                println!("Found MPV DLL at: {}", path_str);
                let current_path = env::var("PATH").unwrap_or_default();
                env::set_var("PATH", format!("{};{}", current_path, path_str));
                break;
            }
        }
    }
}
