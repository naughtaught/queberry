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
    current_metadata: Arc<Mutex<Option<Metadata>>>,
}

#[derive(Debug)]
enum EventType {
    FileLoaded,
    EndFile,
    Shutdown,
    PropertyChange(String),
}

impl MpvEventHandler {
    pub fn new(mpv: Arc<Mutex<Mpv>>, app_handle: AppHandle) -> Self {
        Self {
            mpv,
            app_handle,
            player: None,
            current_metadata: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_player(&mut self, player: Arc<Mutex<MpvPlayer>>) {
        self.player = Some(player);
    }

    pub fn start(&self) {
        let mpv_clone = Arc::clone(&self.mpv);
        let app_handle_clone = self.app_handle.clone();
        let player_clone = self.player.clone();
        let current_metadata_clone = Arc::clone(&self.current_metadata);

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
                        Some(Ok(Event::PropertyChange { name, .. })) => {
                            Some((EventType::PropertyChange(name.to_string()), 0))
                        }
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
                                {
                                    let mut metadata_guard = current_metadata_clone.lock().unwrap();
                                    *metadata_guard = Some(metadata.clone());
                                }
                                if let Err(e) = app_handle_clone.emit("video-metadata", metadata) {
                                    log::error!("Failed to emit video-metadata event: {}", e);
                                }
                            }
                        }
                        EventType::EndFile => {
                            let _ = Self::handle_end_of_file(
                                Arc::clone(&mpv_clone),
                                player_clone.clone(),
                                reason,
                            );
                        }
                        EventType::PropertyChange(property_name) => {
                            if property_name == "playlist-count" || property_name == "playlist-pos"
                            {
                                let mut metadata_guard = current_metadata_clone.lock().unwrap();

                                if let Some(ref mut metadata) = *metadata_guard {
                                    if let Ok(count) = Self::get_playlist_count(&mpv_clone) {
                                        metadata.playlist_count = count;
                                    }
                                    if let Ok(position) = Self::get_playlist_position(&mpv_clone) {
                                        metadata.playlist_position = position;
                                    }

                                    if let Err(e) =
                                        app_handle_clone.emit("video-metadata", metadata.clone())
                                    {
                                        log::error!("Failed to emit video-metadata event: {}", e);
                                    }
                                } else if let Ok(metadata) =
                                    Self::get_metadata(Arc::clone(&mpv_clone), player_clone.clone())
                                {
                                    *metadata_guard = Some(metadata.clone());
                                    if let Err(e) =
                                        app_handle_clone.emit("video-metadata", metadata)
                                    {
                                        log::error!("Failed to emit video-metadata event: {}", e);
                                    }
                                }
                            }
                        }
                        EventType::Shutdown => {
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
        let av_sync = Self::get_audio_delay(&mpv)?;
        let subtitle_margin = Self::get_subtitle_margin(&mpv)?;
        let playlist_count = Self::get_playlist_count(&mpv)?;
        let playlist_position = Self::get_playlist_position(&mpv)?;

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
            av_sync,
            subtitle_margin,
            playlist_position,
            playlist_count,
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

    fn get_audio_delay(mpv: &Arc<Mutex<Mpv>>) -> Result<f64, String> {
        let mpv_guard = mpv
            .lock()
            .map_err(|e| format!("Failed to lock MPV mutex: {}", e))?;

        mpv_guard
            .get_property("audio-delay")
            .map_err(|e| format!("Failed to get av sync property: {}", e))
    }

    fn get_subtitle_margin(mpv: &Arc<Mutex<Mpv>>) -> Result<i64, String> {
        let mpv_guard = mpv
            .lock()
            .map_err(|e| format!("Failed to lock MPV mutex: {}", e))?;

        mpv_guard
            .get_property("sub-margin-y")
            .map_err(|e| format!("Failed to get av sync property: {}", e))
    }

    fn get_playlist_position(mpv: &Arc<Mutex<Mpv>>) -> Result<i64, String> {
        let mpv_guard = mpv
            .lock()
            .map_err(|e| format!("Failed to lock MPV mutex: {}", e))?;

        mpv_guard
            .get_property("playlist-pos")
            .map_err(|e| format!("Failed to get playlist position: {}", e))
    }

    fn get_playlist_count(mpv: &Arc<Mutex<Mpv>>) -> Result<i64, String> {
        let mpv_guard = mpv
            .lock()
            .map_err(|e| format!("Failed to lock MPV mutex: {}", e))?;

        mpv_guard
            .get_property("playlist-count")
            .map_err(|e| format!("Failed to get playlist count: {}", e))
    }

    fn handle_end_of_file(
        mpv: Arc<Mutex<Mpv>>,
        player: Option<Arc<Mutex<MpvPlayer>>>,
        reason: u32,
    ) -> Result<(), String> {
        let playlist_position = Self::get_playlist_position(&mpv);

        if reason == 0 && playlist_position == Ok(-1) {
            if let Some(player_arc) = player {
                let player_guard = player_arc
                    .lock()
                    .map_err(|e| format!("Failed to lock player mutex: {}", e))?;

                player_guard
                    .shutdown()
                    .map_err(|e| format!("Failed to shutdown player: {}", e))?;
            }
        }

        Ok(())
    }
}
