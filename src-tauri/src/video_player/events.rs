// video_player/events.rs
use libmpv2::{events::Event, Mpv};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::video_player::types::Metadata;

pub struct MpvEventHandler {
    mpv: Arc<Mutex<Mpv>>,
}

enum EventType {
    FileLoaded,
    EndFile,
    StartFile,
    Shutdown,
}

impl MpvEventHandler {
    pub fn new(mpv: Arc<Mutex<Mpv>>) -> Self {
        Self { mpv }
    }

    pub fn start(&self) {
        let mpv_clone = Arc::clone(&self.mpv);

        thread::spawn(move || {
            log::info!("MPV event logger started");

            loop {
                let event_data = {
                    let mut mpv_guard = match mpv_clone.lock() {
                        Ok(guard) => guard,
                        Err(e) => {
                            log::error!("Failed to lock MPV mutex: {}", e);
                            break;
                        }
                    };

                    match mpv_guard.wait_event(0.1) {
                        Some(Ok(Event::EndFile(reason))) => Some((EventType::EndFile, reason)),
                        Some(Ok(Event::FileLoaded)) => Some((EventType::FileLoaded, 0)),
                        Some(Ok(Event::StartFile)) => Some((EventType::StartFile, 0)),
                        Some(Ok(Event::Shutdown)) => Some((EventType::Shutdown, 0)),
                        Some(Ok(event)) => {
                            log::debug!("MPV Event: {:?}", event);
                            None
                        }
                        Some(Err(e)) => {
                            log::error!("Error from MPV event: {}", e);
                            None
                        }
                        None => None, // Timeout
                    }
                };

                if let Some((event_type, reason)) = event_data {
                    match event_type {
                        EventType::FileLoaded => {
                            log::info!("MPV Event: FileLoaded");
                            if let Ok(metadata) = Self::get_metadata(Arc::clone(&mpv_clone)) {
                                log::info!("Loaded file metadata: {:?}", metadata);
                            }
                        }
                        EventType::EndFile => {
                            log::info!("MPV Event: EndFile - reason code: {}", reason)
                        }
                        EventType::StartFile => log::info!("MPV Event: StartFile"),
                        EventType::Shutdown => {
                            log::info!("MPV Event: Shutdown");
                            break;
                        }
                    }
                }

                thread::sleep(Duration::from_millis(10));
            }

            log::info!("MPV event logger stopped");
        });
    }

    fn get_metadata(mpv: Arc<Mutex<Mpv>>) -> Result<Metadata, String> {
        let duration = Self::get_duration(mpv)?;
        Ok(Metadata { duration })
    }

    fn get_duration(mpv: Arc<Mutex<Mpv>>) -> Result<f64, String> {
        let mpv_guard = mpv
            .lock()
            .map_err(|e| format!("Failed to lock MPV mutex: {}", e))?;

        mpv_guard
            .get_property("duration")
            .map_err(|e| format!("Failed to get duration property: {}", e))
    }
}
