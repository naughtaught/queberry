use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use tauri::WebviewWindow;

pub fn get_window_handle_id(window: &WebviewWindow) -> Option<i64> {
    if let Ok(handle) = window.window_handle() {
        match handle.as_raw() {
            RawWindowHandle::Xlib(handle) => {
                println!("Got X11 window ID: {}", handle.window);
                Some(handle.window as i64)
            }
            RawWindowHandle::Wayland(handle) => {
                println!("Wayland window surface ID: {:?}", handle.surface);
                None
            }
            _ => {
                eprintln!(
                    "Unsupported Linux window handle type: {:?}",
                    handle.as_raw()
                );
                None
            }
        }
    } else {
        eprintln!("Failed to get window handle");
        None
    }
}
