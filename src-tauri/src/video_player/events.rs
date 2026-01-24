use libmpv2::{events::Event, Mpv};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

use crate::video_player::player::MpvPlayer;
use crate::video_player::types::Metadata;

pub struct MpvEventHandler {
    mpv: Arc<Mutex<Mpv>>,
    app_handle: AppHandle,
    player: Option<Arc<Mutex<MpvPlayer>>>,
}

enum EventType {
    FileLoaded,
    EndFile,
    Shutdown,
}

impl MpvEventHandler {
    pub fn new(mpv: Arc<Mutex<Mpv>>, app_handle: AppHandle) -> Self {
        Self {
            mpv,
            app_handle,
            player: None,
        }
    }

    pub fn set_player(&mut self, player: Arc<Mutex<MpvPlayer>>) {
        self.player = Some(player);
    }

    pub fn start(&self) {
        let mpv_clone = Arc::clone(&self.mpv);
        let app_handle_clone = self.app_handle.clone();
        let player_clone = self.player.clone();

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
                                if let Err(e) = app_handle_clone.emit("video-metadata", metadata) {
                                    log::error!("Failed to emit video-metadata event: {}", e);
                                }
                            }
                        }
                        EventType::EndFile => {
                            log::info!("MPV Event: EndFile - reason code: {}", reason);
                            // TODO add if not next playlist item
                            if reason == 0 {
                                if let Some(ref player) = player_clone {
                                    if let Ok(p) = player.lock() {
                                        if let Err(e) = p.shutdown() {
                                            log::error!("Failed to shutdown player: {}", e);
                                        }
                                    }
                                }
                            }
                        }
                        EventType::Shutdown => {
                            // TODO
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
        // TODO Title
        let title = "Test Title".to_string();
        let duration = Self::get_duration(&mpv)?;
        let audio_channel = Self::get_audio_channel(&mpv)?;
        Ok(Metadata {
            title,
            duration,
            audio_channel,
        })
    }

    fn get_duration(mpv: &Arc<Mutex<Mpv>>) -> Result<f64, String> {
        let mpv_guard = mpv
            .lock()
            .map_err(|e| format!("Failed to lock MPV mutex: {}", e))?;

        mpv_guard
            .get_property("duration")
            .map_err(|e| format!("Failed to get duration property: {}", e))
    }

    fn get_audio_channel(mpv: &Arc<Mutex<Mpv>>) -> Result<String, String> {
        let mpv_guard = mpv
            .lock()
            .map_err(|e| format!("Failed to lock MPV mutex: {}", e))?;

        mpv_guard
            .get_property("audio-channels")
            .map_err(|e| format!("Failed to get duration property: {}", e))
    }
}
