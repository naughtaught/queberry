use crate::constants::{MINIMUM_WATCHED_PERCENT, TIME_UPDATE_INTERVAL};
use crate::errors::{AppError, Result};
use crate::video_player::audio::AudioManager;
use crate::video_player::config::MpvConfig;
use crate::video_player::shaders::ShaderManager;
use crate::video_player::subtitles::SubtitleManager;
use crate::video_player::types::*;
use libmpv2::{events::Event, Mpv};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{Emitter, WebviewWindow};
use tokio::sync::{mpsc, Mutex};
use tokio::time::interval;

/// Main MPV player structure
#[derive(Clone)]
pub struct MpvPlayer {
    pub(crate) mpv: Arc<Mutex<Mpv>>,
    window: WebviewWindow,
    config: MpvConfig,

    // Managers
    audio_manager: AudioManager,
    subtitle_manager: SubtitleManager,
    shader_manager: Arc<Mutex<ShaderManager>>,

    // State
    user_settings: UserSettings,
    playlist_metadata: Arc<Mutex<Vec<PlaylistItemMetadata>>>,
    current_playing_index: Arc<Mutex<usize>>,
    last_known_timestamp: Arc<Mutex<f64>>,
    current_timestamp: Arc<Mutex<i64>>,

    // Tracking
    tracking_cancel: Arc<Mutex<Option<mpsc::Sender<()>>>>,

    // Audio state
    center_boost_level: Arc<Mutex<f64>>,
}

impl MpvPlayer {
    /// Create a new MPV player instance
    pub async fn new(
        window: WebviewWindow,
        user_settings: UserSettings,
        mpv_config_path: PathBuf,
    ) -> Result<Self> {
        #[cfg(target_os = "windows")]
        crate::video_player::platform::windows::init();
        #[cfg(target_os = "linux")]
        crate::video_player::platform::linux::init();
        #[cfg(target_os = "macos")]
        crate::video_player::platform::macos::init();

        log::info!("Creating MPV instance");

        let mpv = Mpv::new().map_err(|e| {
            AppError::Runtime(format!(
                "Failed to create mpv instance: {}. Please ensure libmpv is in your system PATH",
                e
            ))
        })?;

        let window_id = crate::video_player::platform::get_window_handle_id(&window)
            .ok_or_else(|| AppError::Runtime("Window handle not available".to_string()))?;

        let config = MpvConfig::new(mpv_config_path.clone());
        config.set_window_id(&mpv, window_id)?;
        config.apply_to_mpv(&mpv)?;

        let mpv = Arc::new(Mutex::new(mpv));

        let audio_manager = AudioManager::new(Arc::clone(&mpv));
        let subtitle_manager = SubtitleManager::new(Arc::clone(&mpv));

        let shaders_dir = mpv_config_path.join("shaders");
        let shader_manager = Arc::new(Mutex::new(ShaderManager::new(
            Arc::clone(&mpv),
            shaders_dir,
        )));

        Ok(Self {
            mpv,
            window,
            config,
            audio_manager,
            subtitle_manager,
            shader_manager,
            user_settings,
            playlist_metadata: Arc::new(Mutex::new(Vec::new())),
            current_playing_index: Arc::new(Mutex::new(0)),
            last_known_timestamp: Arc::new(Mutex::new(0.0)),
            current_timestamp: Arc::new(Mutex::new(0)),
            tracking_cancel: Arc::new(Mutex::new(None)),
            center_boost_level: Arc::new(Mutex::new(1.0)),
        })
    }

    /// Load and play a video file
    pub async fn load_file(
        &self,
        media_data: serde_json::Value,
        file: String,
        language: String,
        title: String,
        id: i64,
        season_number: i32,
        episode_number: i32,
        timestamp: i64,
    ) -> Result<()> {
        log::info!("Loading file: {}", file);

        // Setup playlist metadata
        let shaders = self.shader_manager.lock().await.get_available_shaders();

        let metadata = PlaylistItemMetadata::new(
            id,
            season_number,
            episode_number,
            file.clone(),
            title,
            language.clone(),
            self.user_settings.audio_channels.clone(),
            media_data,
            shaders,
        );

        let mut playlist = self.playlist_metadata.lock().await;
        playlist.clear();
        playlist.push(metadata);
        drop(playlist);

        *self.current_playing_index.lock().await = 0;
        *self.current_timestamp.lock().await = timestamp;
        *self.center_boost_level.lock().await = 1.0;

        // Load file
        let mpv = self.mpv.lock().await;
        mpv.set_property("volume", self.user_settings.volume as i64)
            .map_err(|e| AppError::Runtime(format!("Failed to set volume: {}", e)))?;

        if !language.is_empty() {
            let _ = mpv.set_property("alang", language.as_str());
        }

        mpv.command("loadfile", &[&file, "append-play"])
            .map_err(|e| AppError::Runtime(format!("Failed to load file '{}': {}", file, e)))?;

        drop(mpv);

        // Set audio configuration
        self.audio_manager
            .set_channels(&self.user_settings.audio_channels)
            .await?;

        // Start tracking and event listener
        self.start_unified_tracking().await;
        self.start_event_listener().await;

        Ok(())
    }

    /// Toggle play/pause
    pub async fn toggle_play(&self, paused: bool) -> Result<()> {
        let mpv = self.mpv.lock().await;
        if paused {
            Ok(mpv
                .set_property("pause", false)
                .map_err(|e| format!("Failed to play: {}", e))?)
        } else {
            Ok(mpv
                .set_property("pause", true)
                .map_err(|e| format!("Failed to play: {}", e))?)
        }
    }

    /// Seek to a position
    pub async fn seek(&self, time: f64, absolute: bool) -> Result<(), AppError> {
        let mode = if absolute { "absolute" } else { "relative" };

        // 1. Lock and execute the command.
        // We convert the error to a String immediately so the !Send libmpv2::Error is dropped.
        let is_paused = {
            let mpv = self.mpv.lock().await;

            mpv.command("seek", &[&time.to_string(), mode])
                .map_err(|e| {
                    AppError::Runtime(format!("Failed to seek {} {}: {}", time, mode, e))
                })?;

            // 2. Check for pause while we still have the lock
            mpv.get_property::<bool>("pause").unwrap_or(false)
        }; // mpv lock and any temporary libmpv2 errors are dropped here

        // 3. Now it is safe to await start_unified_tracking
        if is_paused {
            self.start_unified_tracking().await;
        }

        Ok(())
    }

    /// Set volume
    pub async fn set_volume(&self, volume: f64) -> Result<()> {
        let mpv = self.mpv.lock().await;

        if volume <= 0.0 {
            mpv.set_property("mute", true)
                .map_err(|e| AppError::Runtime(format!("Failed to mute: {}", e)))?;
        } else {
            mpv.set_property("mute", false)
                .map_err(|e| AppError::Runtime(format!("Failed to unmute: {}", e)))?;
            mpv.set_property("volume", volume)
                .map_err(|e| AppError::Runtime(format!("Failed to set volume: {}", e)))?;
        }

        Ok(())
    }

    /// Add to playlist
    pub async fn add_to_playlist(
        &self,
        media_data: serde_json::Value,
        file: String,
        language: String,
        title: String,
        id: i64,
        season_number: i32,
        episode_number: i32,
    ) -> Result<()> {
        let mpv = self.mpv.lock().await;

        mpv.command("loadfile", &[&file, "insert-next"])
            .map_err(|e| AppError::Runtime(format!("Failed to load file '{}': {}", file, e)))?;

        drop(mpv);

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let shaders = self.shader_manager.lock().await.get_available_shaders();

        let metadata = PlaylistItemMetadata::new(
            id,
            season_number,
            episode_number,
            file,
            title,
            language,
            self.user_settings.audio_channels.clone(),
            media_data,
            shaders,
        );

        self.playlist_metadata.lock().await.push(metadata);

        Ok(())
    }

    async fn start_unified_tracking(&self) {
        // Cancel existing tracking
        if let Some(cancel) = self.tracking_cancel.lock().await.take() {
            let _ = cancel.send(()).await;
        }

        let (tx, mut rx) = mpsc::channel::<()>(1);
        *self.tracking_cancel.lock().await = Some(tx);

        let window = self.window.clone();
        let mpv = Arc::clone(&self.mpv);
        let last_timestamp = Arc::clone(&self.last_known_timestamp);
        let complete_percent = self.user_settings.complete_percent;

        tokio::spawn(async move {
            let mut interval = interval(TIME_UPDATE_INTERVAL);
            let mut transparent_emitted = false;
            let mut percent_reached = false;

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        // Clone what we need for this iteration
                        let mpv_clone = Arc::clone(&mpv);
                        let last_timestamp_clone = Arc::clone(&last_timestamp);
                        let window_clone = window.clone();
                        let complete_percent = complete_percent;
                        let transparent_emitted = &mut transparent_emitted;
                        let percent_reached = &mut percent_reached;

                        // Create a Send-safe future by handling errors locally
                        let send_safe_future = async move {
                            let mpv_guard = mpv_clone.lock().await;

                            // Convert libmpv2 errors to string immediately to avoid Rc issues
                            let current_time = match mpv_guard.get_property::<f64>("time-pos") {
                                Ok(t) => Some(t),
                                Err(e) => {
                                    // Convert error to string to avoid Rc issues
                                    log::debug!("Failed to get time-pos: {}", e);
                                    None
                                }
                            };

                            let duration = match mpv_guard.get_property::<f64>("duration") {
                                Ok(d) => Some(d),
                                Err(e) => {
                                    log::debug!("Failed to get duration: {}", e);
                                    None
                                }
                            };

                            if let (Some(current_time), Some(duration)) = (current_time, duration) {
                                let buffered = Self::get_buffered_end(&mpv_guard, current_time, duration);

                                *last_timestamp_clone.lock().await = current_time;

                                if current_time > 0.0 && !*transparent_emitted {
                                    let _ = window_clone.emit("TransparentOn", ());
                                    *transparent_emitted = true;
                                }

                                let _ = window_clone.emit("CurrentTimeUpdated", BufferingInfo {
                                    current_time: current_time.floor(),
                                    buffered,
                                });

                                // Similarly handle percent-pos
                                match mpv_guard.get_property::<i64>("percent-pos") {
                                    Ok(percent_pos) => {
                                        if percent_pos >= complete_percent as i64 && !*percent_reached {
                                            let _ = window_clone.emit("MarkAsComplete", ());
                                            *percent_reached = true;
                                        }
                                    }
                                    Err(e) => {
                                        log::debug!("Failed to get percent-pos: {}", e);
                                    }
                                }
                            }
                        };

                        // Execute the send-safe future
                        send_safe_future.await;
                    }
                    _ = rx.recv() => {
                        log::debug!("Tracking cancelled");
                        break;
                    }
                }
            }
        });
    }

    /// Calculate buffered end time
    fn get_buffered_end(mpv: &Mpv, current_time: f64, duration: f64) -> f64 {
        if let Ok(cache_time) = mpv.get_property::<f64>("demuxer-cache-time") {
            let buffered = current_time + cache_time;
            buffered.min(duration).max(current_time)
        } else {
            current_time
        }
    }

    async fn start_event_listener(&self) {
        let window = self.window.clone();
        let current_timestamp = Arc::clone(&self.current_timestamp);
        let playlist_metadata = Arc::clone(&self.playlist_metadata);
        let current_index = Arc::clone(&self.current_playing_index);
        let audio_manager = self.audio_manager.clone();
        let subtitle_manager = self.subtitle_manager.clone();
        let shader_manager = Arc::clone(&self.shader_manager);
        let user_settings = self.user_settings.clone();

        // We need to clone the MPV Arc to move into the blocking task
        let mpv = Arc::clone(&self.mpv);

        // Create a channel to send commands from the blocking thread to the main thread
        let (command_tx, mut command_rx) = mpsc::channel::<Box<dyn FnOnce() + Send + 'static>>(32);

        // Spawn a blocking thread for MPV operations
        tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Handle::current();

            loop {
                // Lock the MPV instance on this thread
                let mpv_guard_future = mpv.lock();

                // We need to block on this future since we're in a blocking context
                let mut mpv_guard = rt.block_on(mpv_guard_future);

                // Wait for events (this blocks)
                match mpv_guard.wait_event(10.0) {
                    Some(Ok(event)) => {
                        match event {
                            Event::FileLoaded => {
                                log::info!("File loaded event");

                                // We need to execute UI updates on the main thread
                                let current_timestamp_clone = current_timestamp.clone();
                                let audio_manager_clone = audio_manager.clone();
                                let subtitle_manager_clone = subtitle_manager.clone();
                                let shader_manager_clone = shader_manager.clone();
                                let playlist_metadata_clone = playlist_metadata.clone();
                                let current_index_clone = current_index.clone();
                                let user_settings_clone = user_settings.clone();
                                let window_clone = window.clone();

                                // Send command to main thread to handle file loaded
                                let value = rt.clone();
                                let _ = command_tx.try_send(Box::new(move || {
                                    value.block_on(async move {
                                        let timestamp = *current_timestamp_clone.lock().await;
                                        if timestamp > 0 {
                                            // This needs to be done on the MPV thread
                                            // We'll handle this separately
                                        }
                                        *current_timestamp_clone.lock().await = 0;

                                        // Get tracks after file is loaded
                                        if let Ok(audio_tracks) =
                                            audio_manager_clone.get_tracks().await
                                        {
                                            if let Ok(subtitle_tracks) =
                                                subtitle_manager_clone.get_tracks().await
                                            {
                                                let shaders = shader_manager_clone
                                                    .lock()
                                                    .await
                                                    .get_available_shaders();

                                                // Update metadata with tracks
                                                let mut playlist =
                                                    playlist_metadata_clone.lock().await;
                                                let current_idx = *current_index_clone.lock().await;

                                                if current_idx < playlist.len() {
                                                    playlist[current_idx].audio_tracks =
                                                        audio_tracks;
                                                    playlist[current_idx].subtitle_tracks =
                                                        subtitle_tracks;
                                                    playlist[current_idx].available_shaders =
                                                        shaders;

                                                    // Find and set best tracks
                                                    let preferred_lang =
                                                        &user_settings_clone.preferred_language;
                                                    let content_lang =
                                                        &playlist[current_idx].language;

                                                    let best_audio = AudioManager::find_best_track(
                                                        &playlist[current_idx].audio_tracks,
                                                        preferred_lang,
                                                        content_lang,
                                                    );

                                                    let best_subtitle =
                                                        SubtitleManager::find_best_track(
                                                            &playlist[current_idx].subtitle_tracks,
                                                            user_settings_clone.subtitle_setting,
                                                            &user_settings_clone
                                                                .preferred_subtitle_language,
                                                            content_lang,
                                                        );

                                                    playlist[current_idx].current_audio_track =
                                                        best_audio;
                                                    playlist[current_idx].current_subtitle_track =
                                                        best_subtitle;

                                                    // Emit updated metadata
                                                    let _ = window_clone.emit(
                                                        "UpdateVideoMetadata",
                                                        &playlist[current_idx],
                                                    );
                                                }
                                            }
                                        }

                                        let _ = window_clone.emit("VideoLoaded", ());
                                    });
                                }));
                            }
                            Event::EndFile(reason) => {
                                log::info!("File ended: {:?}", reason);
                                let window_clone = window.clone();
                                let value = rt.clone();
                                let _ = command_tx.try_send(Box::new(move || {
                                    value.block_on(async move {
                                        let _ = window_clone.emit("PlaybackEnded", ());
                                    });
                                }));
                            }
                            Event::Shutdown => {
                                log::info!("MPV shutdown event");
                                let window_clone = window.clone();
                                let _ = command_tx.try_send(Box::new(move || {
                                    rt.block_on(async move {
                                        let _ = window_clone.emit("PlayerShutdown", ());
                                    });
                                }));
                                break;
                            }
                            Event::QueueOverflow => {
                                log::warn!("MPV event queue overflow");
                            }
                            _ => {
                                // Ignore other events
                            }
                        }
                    }
                    Some(Err(e)) => {
                        log::error!("Error waiting for MPV event: {}", e);
                        break;
                    }
                    None => {
                        // Timeout, continue
                    }
                }
            }
        });

        // Handle commands from the blocking thread on the main async runtime
        tokio::spawn(async move {
            while let Some(command) = command_rx.recv().await {
                command();
            }
        });
    }

    /// Shutdown the player
    pub async fn shutdown(self) -> Result<()> {
        log::info!("Shutting down player");

        // Stop tracking
        if let Some(cancel) = self.tracking_cancel.lock().await.take() {
            let _ = cancel.send(()).await;
        }

        // Emit events
        let _ = self.window.emit("VideoShutdown", ());
        let _ = self.window.emit("TransparentOff", ());

        // Get last known state for saving
        let last_timestamp = *self.last_known_timestamp.lock().await;
        let playlist = self.playlist_metadata.lock().await;
        let current_idx = *self.current_playing_index.lock().await;

        if current_idx < playlist.len() && last_timestamp > MINIMUM_WATCHED_PERCENT {
            let item = &playlist[current_idx];
            let percent_watched = (last_timestamp / item.duration) * 100.0;

            if percent_watched >= MINIMUM_WATCHED_PERCENT
                && percent_watched < self.user_settings.complete_percent as f64
            {
                let _ = self.window.emit(
                    "IncompleteVideo",
                    serde_json::json!({
                        "timestamp": last_timestamp,
                        "id": item.id,
                        "seasonNumber": item.season_number,
                        "episodeNumber": item.episode_number,
                    }),
                );
            }
        }

        drop(playlist);

        // Quit MPV
        let mpv = self.mpv.lock().await;
        let _ = mpv.command("quit", &[]);

        Ok(())
    }

    /// Get reference to audio manager
    pub fn audio_manager(&self) -> &AudioManager {
        &self.audio_manager
    }

    /// Get reference to subtitle manager
    pub fn subtitle_manager(&self) -> &SubtitleManager {
        &self.subtitle_manager
    }

    /// Get reference to shader manager
    pub fn shader_manager(&self) -> Arc<Mutex<ShaderManager>> {
        Arc::clone(&self.shader_manager)
    }
}

impl Drop for MpvPlayer {
    fn drop(&mut self) {
        log::info!("MpvPlayer dropped");
    }
}
