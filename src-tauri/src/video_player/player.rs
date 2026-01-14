use crate::video_player::utils::*;
use libmpv2::Mpv;
use std::sync::{Arc, Mutex};
use tauri::WebviewWindow;

pub struct MpvPlayer {
    mpv: Arc<Mutex<Mpv>>,
}

impl MpvPlayer {
    pub fn new(window: Option<&WebviewWindow>) -> Result<Self, String> {
        // Perform platform-specific initialization
        platform_specific_init();

        println!("Attempting to create MPV instance...");
        let  mpv = Mpv::new().map_err(|e| {
            format!(
                "Failed to create mpv instance: {}\n\n\
                Please ensure libmpv is available in your system PATH\n\
                \n\
                For Windows: Download libmpv-2.dll from: https://sourceforge.net/projects/mpv-player-windows/files/libmpv/",
                e
            )
        })?;

        println!("MPV instance created successfully!");

        // Configure MPV with universal settings
        let _ = mpv.set_property("vo", "gpu-next");
        let _ = mpv.set_property("hwdec", "auto-safe");
        let _ = mpv.set_property("keep-open", "yes");

        // Set initial volume
        let _ = mpv.set_property("volume", 70.0);

        // If we have a window, embed MPV into it
        if let Some(window) = window {
            // Get window handle ID (platform-specific)
            let window_id = get_window_handle_id(window);

            if let Some(window_id) = window_id {
                // Set window ID for embedding
                if let Err(e) = mpv.set_property("wid", window_id) {
                    eprintln!("Failed to set window ID for embedding: {}", e);
                    // Fallback to windowed mode if embedding fails
                    let _ = mpv.set_property("force-window", "yes");
                } else {
                    // Successfully embedded, don't force separate window
                    let _ = mpv.set_property("force-window", "no");
                }
            } else {
                // Couldn't get window ID, create separate window
                let _ = mpv.set_property("force-window", "yes");
            }
        } else {
            // No window provided, create separate window
            let _ = mpv.set_property("force-window", "yes");
        }

        Ok(Self {
            mpv: Arc::new(Mutex::new(mpv)),
        })
    }

    pub fn load_file(&self, url: &str) -> Result<(), String> {
        // Validate URL

        let mpv = self
            .mpv
            .lock()
            .map_err(|e| format!("Failed to lock mpv mutex: {}", e))?;

        mpv.command("loadfile", &[url, "replace"])
            .map_err(|e| format!("Failed to load file: {}", e))?;
        Ok(())
    }

    pub fn play(&self) -> Result<(), String> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| format!("Failed to lock mpv mutex: {}", e))?;

        mpv.set_property("pause", false)
            .map_err(|e| format!("Failed to play: {}", e))?;
        Ok(())
    }
}
