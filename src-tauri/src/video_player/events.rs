use libmpv2::{events::Event, Mpv};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

use crate::video_player::audio::AudioManager;
use crate::video_player::player::MpvPlayer;
use crate::video_player::subtitles::SubtitleManager;
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
                            if let Ok(metadata) =
                                Self::get_metadata(Arc::clone(&mpv_clone), player_clone.clone())
                            {
                                if let Err(e) = app_handle_clone.emit("video-metadata", metadata) {
                                    log::error!("Failed to emit video-metadata event: {}", e);
                                }
                            }
                        }
                        EventType::EndFile => {
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
                            break;
                        }
                    }
                }

                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    fn get_metadata(
        mpv: Arc<Mutex<Mpv>>,
        player: Option<Arc<Mutex<MpvPlayer>>>,
    ) -> Result<Metadata, String> {
        // TODO Title
        let title = "Test Title".to_string();

        let duration = Self::get_duration(&mpv)?;
        let audio_channel = Self::get_audio_channel(&mpv)?;

        // TODO pass video langauge
        let video_language = "th";
        let user_settings = match player {
            Some(player_arc) => match player_arc.lock() {
                Ok(player) => Some(player.user_settings.clone()),
                Err(_) => None,
            },
            None => None,
        };

        let audio_manager = AudioManager::new(Arc::clone(&mpv));
        let audio_tracks = audio_manager
            .get_all_audio_tracks()
            .map_err(|e| format!("Failed to get audio tracks: {}", e))?;
        if let Some(settings) = &user_settings {
            audio_manager
                .auto_select_audio_track(video_language, settings)
                .map_err(|e| format!("Failed to auto set audio track: {}", e))?;
        }
        let current_audio_track = audio_manager
            .get_current_audio_track()
            .map_err(|e| format!("Failed to get current audio track: {}", e))?;

        let subtitle_manager = SubtitleManager::new(Arc::clone(&mpv));
        let subtitle_tracks = subtitle_manager
            .get_all_subtitle_tracks()
            .map_err(|e| format!("Failed to get subtitle tracks: {}", e))?;
        if let Some(settings) = user_settings {
            subtitle_manager
                .auto_select_subtitle_track(video_language, &settings)
                .map_err(|e| format!("Failed to auto set subtitle: {}", e))?;
        } else {
            subtitle_manager
                .set_subtitle_track(None)
                .map_err(|e| format!("Failed to disable subtitles: {}", e))?;
        }
        let current_subtitle_track = subtitle_manager
            .get_current_subtitle_track()
            .map_err(|e| format!("Failed to get current subtitle: {}", e))?;

        Ok(Metadata {
            title,
            duration,
            audio_channel,
            subtitle_tracks,
            current_subtitle_track,
            audio_tracks,
            current_audio_track,
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
