use libmpv2::{events::Event, Mpv};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

use crate::video_player::audio::AudioManager;
use crate::video_player::player::MpvPlayer;
use crate::video_player::subtitles::SubtitleManager;
use crate::video_player::types::VideoProperties;

pub struct MpvEventHandler {
    mpv: Arc<Mutex<Mpv>>,
    app_handle: AppHandle,
    player: Option<Arc<Mutex<MpvPlayer>>>,
    current_video_properties: Arc<Mutex<Option<VideoProperties>>>,
}

struct CoreVideoProps {
    duration: f64,
    audio_channel: String,
    av_sync: f64,
    subtitle_sync: f64,
    playlist_count: i64,
    playlist_position: i64,
}

#[derive(Debug)]
enum EventType {
    FileLoaded,
    EndFile,
    Shutdown,
    PropertyChange(String),
    PlaybackStarted,
    PlaybackError(i32),
}

impl MpvEventHandler {
    pub fn new(mpv: Arc<Mutex<Mpv>>, app_handle: AppHandle) -> Self {
        Self {
            mpv,
            app_handle,
            player: None,
            current_video_properties: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_player(&mut self, player: Arc<Mutex<MpvPlayer>>) {
        self.player = Some(player);
    }

    pub fn start(&self) {
        let mpv_clone = Arc::clone(&self.mpv);
        let app_handle_clone = self.app_handle.clone();
        let player_clone = self.player.clone();
        let current_video_properties_clone = Arc::clone(&self.current_video_properties);

        {
            let mpv_guard = self.mpv.lock().unwrap();
            mpv_guard
                .observe_property("playback-time", libmpv2::Format::Double, 1)
                .unwrap();
        }

        thread::spawn(move || {
            let mut video_started = false;
            loop {
                let event_data = {
                    let mut mpv_guard = match mpv_clone.lock() {
                        Ok(guard) => guard,
                        Err(e) => {
                            log::error!("Failed to lock MPV mutex: {}", e);
                            break;
                        }
                    };

                    match mpv_guard.wait_event(0.0) {
                        Some(Ok(Event::EndFile(reason))) => Some((EventType::EndFile, reason)),
                        Some(Ok(Event::FileLoaded)) => Some((EventType::FileLoaded, 0)),
                        Some(Ok(Event::Shutdown)) => Some((EventType::Shutdown, 0)),

                        Some(Ok(Event::PropertyChange { name, change, .. })) => {
                            if name == "playback-time" && !video_started {
                                if let libmpv2::events::PropertyData::Double(time) = change {
                                    if time > 0.0 {
                                        Some((EventType::PlaybackStarted, 0))
                                    } else {
                                        Some((EventType::PropertyChange(name.to_string()), 0))
                                    }
                                } else {
                                    Some((EventType::PropertyChange(name.to_string()), 0))
                                }
                            } else {
                                Some((EventType::PropertyChange(name.to_string()), 0))
                            }
                        }
                        Some(Ok(event)) => {
                            log::debug!("MPV Event: {:?}", event);
                            None
                        }
                        Some(Err(e)) => {
                            log::error!("Error from MPV event: {}", e);
                            match e {
                                libmpv2::Error::Raw(code) => {
                                    Some((EventType::PlaybackError(code), 0))
                                }
                                _ => None,
                            }
                        }
                        None => None,
                    }
                };

                if let Some((event_type, reason)) = event_data {
                    match event_type {
                        EventType::FileLoaded => {
                            if let Ok(video_properties) = Self::get_video_properties(
                                Arc::clone(&mpv_clone),
                                player_clone.clone(),
                            ) {
                                {
                                    let mut video_properties_guard =
                                        current_video_properties_clone.lock().unwrap();
                                    *video_properties_guard = Some(video_properties.clone());
                                }
                                if let Err(e) =
                                    app_handle_clone.emit("video-properties", video_properties)
                                {
                                    log::error!("Failed to emit video-properties event: {}", e);
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
                        EventType::PlaybackStarted => {
                            video_started = true;
                            if let Err(e) = app_handle_clone.emit("video-started", ()) {
                                log::error!("Failed to emit video-started event: {}", e);
                            }

                            if let Ok(guard) = mpv_clone.lock() {
                                if let Err(e) = guard.unobserve_property(1) {
                                    log::debug!("Failed to unobserve playback-time: {}", e);
                                }
                            }
                        }

                        EventType::PropertyChange(property_name) => {
                            if property_name == "glsl-shaders" {
                                let mut video_properties_guard =
                                    current_video_properties_clone.lock().unwrap();
                                if let Some(ref mut video_properties) = *video_properties_guard {
                                    if let Ok(active) = Self::get_active_shaders(&mpv_clone) {
                                        video_properties.active_shaders = active;
                                        if let Err(e) = app_handle_clone
                                            .emit("video-properties", video_properties.clone())
                                        {
                                            log::error!(
                                                "Failed to emit video_properties event: {}",
                                                e
                                            );
                                        }
                                    }
                                }
                            }
                            if property_name == "playlist-count" || property_name == "playlist-pos"
                            {
                                let mut video_properties_guard =
                                    current_video_properties_clone.lock().unwrap();

                                if let Some(ref mut video_properties) = *video_properties_guard {
                                    if let Ok(count) = Self::get_playlist_count(&mpv_clone) {
                                        video_properties.playlist_count = count;
                                    }
                                    if let Ok(position) = Self::get_playlist_position(&mpv_clone) {
                                        video_properties.playlist_position = position;
                                    }

                                    if let Ok(active) = Self::get_active_shaders(&mpv_clone) {
                                        video_properties.active_shaders = active;
                                    }

                                    if let Err(e) = app_handle_clone
                                        .emit("video-properties", video_properties.clone())
                                    {
                                        log::error!("Failed to emit video_properties event: {}", e);
                                    }
                                } else if let Ok(video_properties) = Self::get_video_properties(
                                    Arc::clone(&mpv_clone),
                                    player_clone.clone(),
                                ) {
                                    *video_properties_guard = Some(video_properties.clone());
                                    if let Err(e) =
                                        app_handle_clone.emit("video-properties", video_properties)
                                    {
                                        log::error!("Failed to emit video_properties event: {}", e);
                                    }
                                }
                            }
                        }
                        EventType::Shutdown => {
                            break;
                        }
                        EventType::PlaybackError(code) => {
                            log::error!("MPV playback error: {}", code);
                            let _ = app_handle_clone.emit("playback-error", code);
                        }
                    }
                }

                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    fn get_core_properties(mpv: &Arc<Mutex<Mpv>>) -> Result<CoreVideoProps, String> {
        let guard = mpv
            .lock()
            .map_err(|e| format!("Failed to lock MPV mutex: {}", e))?;

        let duration: f64 = guard
            .get_property("duration")
            .map_err(|e| format!("Failed to get duration property: {}", e))?;

        let raw_channel: String = guard
            .get_property("audio-channels")
            .map_err(|e| format!("Failed to get audio channel property: {}", e))?;
        let audio_channel = match raw_channel.to_lowercase().as_str() {
            "stereo" => "2.0".to_string(),
            "mono" => "1.0".to_string(),
            _ => raw_channel,
        };

        let av_sync: f64 = guard
            .get_property("audio-delay")
            .map_err(|e| format!("Failed to get av sync property: {}", e))?;

        let subtitle_sync: f64 = guard
            .get_property("sub-delay")
            .map_err(|e| format!("Failed to get subtitle sync property: {}", e))?;

        let playlist_count: i64 = guard
            .get_property("playlist-count")
            .map_err(|e| format!("Failed to get playlist count: {}", e))?;

        let playlist_position: i64 = guard
            .get_property("playlist-pos")
            .map_err(|e| format!("Failed to get playlist position: {}", e))?;

        Ok(CoreVideoProps {
            duration,
            audio_channel,
            av_sync,
            subtitle_sync,
            playlist_count,
            playlist_position,
        })
    }

    fn get_video_properties(
        mpv: Arc<Mutex<Mpv>>,
        player: Option<Arc<Mutex<MpvPlayer>>>,
    ) -> Result<VideoProperties, String> {
        let core = Self::get_core_properties(&mpv)?;

        let (available_shaders, active_shaders) = if let Some(player_arc) = &player {
            match player_arc.lock() {
                Ok(player_guard) => {
                    let available = player_guard.get_available_shaders().unwrap_or_else(|e| {
                        log::warn!("Failed to get available shaders: {}", e);
                        Vec::new()
                    });

                    let active = Self::get_active_shaders(&mpv).unwrap_or_else(|e| {
                        log::warn!("Failed to get active shaders: {}", e);
                        Vec::new()
                    });

                    (available, active)
                }
                Err(e) => {
                    log::warn!("Failed to lock player for shaders: {}", e);
                    (Vec::new(), Vec::new())
                }
            }
        } else {
            (Vec::new(), Vec::new())
        };

        let video_language = match &player {
            Some(player_arc) => match player_arc.lock() {
                Ok(player_guard) => player_guard.video_language.clone(),
                Err(_) => None,
            },
            None => None,
        };

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
            if let Some(lang) = &video_language {
                audio_manager
                    .auto_select_audio_track(lang.as_str(), settings)
                    .map_err(|e| format!("Failed to auto set audio track: {}", e))?;
            }
        }
        let current_audio_track = audio_manager
            .get_current_audio_track()
            .map_err(|e| format!("Failed to get current audio track: {}", e))?;

        let subtitle_manager = SubtitleManager::new(Arc::clone(&mpv));
        let subtitle_tracks = subtitle_manager
            .get_all_subtitle_tracks()
            .map_err(|e| format!("Failed to get subtitle tracks: {}", e))?;
        if let Some(settings) = &user_settings {
            if let Some(lang) = &video_language {
                subtitle_manager
                    .auto_select_subtitle_track(lang.as_str(), settings)
                    .map_err(|e| format!("Failed to auto set subtitle: {}", e))?;
            } else {
                subtitle_manager
                    .set_subtitle_track(None)
                    .map_err(|e| format!("Failed to disable subtitles: {}", e))?;
            }
        } else {
            subtitle_manager
                .set_subtitle_track(None)
                .map_err(|e| format!("Failed to disable subtitles: {}", e))?;
        }
        let current_subtitle_track = subtitle_manager
            .get_current_subtitle_track()
            .map_err(|e| format!("Failed to get current subtitle: {}", e))?;

        Ok(VideoProperties {
            duration: core.duration,
            audio_channel: core.audio_channel,
            subtitle_tracks,
            current_subtitle_track,
            audio_tracks,
            current_audio_track,
            av_sync: core.av_sync,
            subtitle_sync: core.subtitle_sync,
            playlist_position: core.playlist_position,
            playlist_count: core.playlist_count,
            available_shaders,
            active_shaders,
        })
    }

    fn get_active_shaders(mpv: &Arc<Mutex<Mpv>>) -> Result<Vec<String>, String> {
        let mpv_guard = mpv
            .lock()
            .map_err(|e| format!("Failed to lock MPV mutex: {}", e))?;

        let shaders_str: String = mpv_guard
            .get_property("glsl-shaders")
            .map_err(|e| format!("Failed to get shaders property: {}", e))?;

        if shaders_str.is_empty() {
            Ok(Vec::new())
        } else {
            Ok(shaders_str
                .split(';')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|full_path| {
                    std::path::Path::new(full_path)
                        .file_name()
                        .and_then(|name| name.to_str())
                        .map(|name| name.to_string())
                        .unwrap_or_else(|| full_path.to_string())
                })
                .collect())
        }
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
