use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use tauri::WebviewWindow;

pub fn get_window_handle_id(window: &WebviewWindow) -> Option<i64> {
    if let Ok(handle) = window.window_handle() {
        match handle.as_raw() {
            RawWindowHandle::AppKit(handle) => {
                println!("Got macOS NSView pointer");
                Some(handle.ns_view.as_ptr() as i64)
            }
            _ => {
                eprintln!("Expected AppKit handle but got: {:?}", handle.as_raw());
                None
            }
        }
    } else {
        eprintln!("Failed to get window handle");
        None
    }
}
