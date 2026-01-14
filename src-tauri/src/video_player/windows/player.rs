use libmpv2::Mpv;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct MpvPlayer {
    mpv: Arc<Mutex<Mpv>>,
}

impl MpvPlayer {
    pub fn new() -> Result<Self, String> {
        #[cfg(target_os = "windows")]
        {
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
                        let current_path = std::env::var("PATH").unwrap_or_default();
                        std::env::set_var("PATH", format!("{};{}", current_path, path_str));
                        break;
                    }
                }
            }
        }

        // Try to create mpv instance
        println!("Attempting to create MPV instance...");
        let mpv = Mpv::new().map_err(|e| {
            format!(
                "Failed to create mpv instance: {}\n\n\
                Please ensure libmpv-2.dll is available in one of these locations:\n\
                - lib/windows/ (relative to executable)\n\
                - resources/lib/windows/ (bundled)\n\
                - In your system PATH\n\
                \n\
                You can download libmpv-2.dll from: https://sourceforge.net/projects/mpv-player-windows/files/libmpv/",
                e
            )
        })?;

        println!("MPV instance created successfully!");

        let _ = mpv.set_property("vo", "gpu-next");
        let _ = mpv.set_property("hwdec", "auto-safe");
        let _ = mpv.set_property("keep-open", "yes");
        let _ = mpv.set_property("force-window", "yes");
        let _ = mpv.set_property("window-scale", 0.5);

        // Set initial volume
        let _ = mpv.set_property("volume", 70.0);

        Ok(Self {
            mpv: Arc::new(Mutex::new(mpv)),
        })
    }

    pub fn load_file(&self, url: &str) -> Result<(), String> {
        println!("Loading file: {}", url);
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| format!("Failed to lock mpv mutex: {}", e))?;

        mpv.command("loadfile", &[url, "replace"])
            .map_err(|e| format!("Failed to load file: {}", e))?;
        Ok(())
    }

    pub fn play(&self) -> Result<(), String> {
        println!("Playing video...");
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| format!("Failed to lock mpv mutex: {}", e))?;

        mpv.set_property("pause", false)
            .map_err(|e| format!("Failed to play: {}", e))?;
        Ok(())
    }

    pub fn pause(&self) -> Result<(), String> {
        println!("Pausing video...");
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| format!("Failed to lock mpv mutex: {}", e))?;

        mpv.set_property("pause", true)
            .map_err(|e| format!("Failed to pause: {}", e))?;
        Ok(())
    }
}
