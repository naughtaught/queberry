use crate::db::types::UserSettings;
use crate::errors::{AppError, Result};
use crate::video_player::audio::{self, AudioManager};
use crate::video_player::shaders;
use crate::video_player::subtitles::{self, SubtitleManager};
use crate::video_player::types::ShaderInfo;
use crate::video_player::{config::MpvConfig, events::MpvEventHandler, tracker::PlayerTracker};
use libmpv2::Mpv;
use std::sync::{Arc, Mutex, OnceLock};
use tauri::{AppHandle, WebviewWindow};

#[derive(Clone)]
pub struct MpvPlayer {
    pub(crate) mpv: Arc<Mutex<Mpv>>,
    tracker: PlayerTracker,
    pub user_settings: UserSettings,
    pub video_language: Option<String>,
}

static SHADERS_CACHE: OnceLock<Vec<ShaderInfo>> = OnceLock::new();

impl MpvPlayer {
    pub fn new(
        window: WebviewWindow,
        app_handle: AppHandle,
        settings: &UserSettings,
        video_language: Option<String>,
    ) -> Result<Self> {
        let config = MpvConfig::new(settings);

        let mpv =
            Mpv::with_initializer(|initializer| config.apply_during_initialization(&initializer))
                .map_err(|e| {
                AppError::Runtime(format!(
            "Failed to create mpv instance: {}. Please ensure libmpv is in your system PATH",
            e
        ))
            })?;

        mpv.observe_property("playlist-count", libmpv2::Format::Int64, 0)
            .map_err(|e| AppError::Runtime(format!("Failed to observe playlist-count: {}", e)))?;

        mpv.observe_property("playlist-pos", libmpv2::Format::Int64, 0)
            .map_err(|e| AppError::Runtime(format!("Failed to observe playlist-pos: {}", e)))?;

        mpv.observe_property("glsl-shaders", libmpv2::Format::String, 0)
            .map_err(|e| AppError::Runtime(format!("Failed to observe glsl-shaders: {}", e)))?;

        let window_id = crate::video_player::platform::get_window_handle_id(&window)
            .ok_or_else(|| AppError::Runtime("Window handle not available".to_string()))?;

        config.set_window_id(&mpv, window_id)?;
        config.apply_user_settings(&mpv)?;

        let mpv = Arc::new(Mutex::new(mpv));

        let tracker = PlayerTracker::new(
            Arc::clone(&mpv),
            app_handle.clone(),
            settings.completion_percent,
        );

        let _ = Self::initialize_shaders_cache();

        let player = Self {
            mpv,
            tracker,
            user_settings: settings.clone(),
            video_language,
        };

        let player_shared = Arc::new(Mutex::new(player.clone()));
        let mut event_handler = MpvEventHandler::new(Arc::clone(&player.mpv), app_handle.clone());
        event_handler.set_player(Arc::clone(&player_shared));
        event_handler.start();

        Ok(player)
    }

    fn initialize_shaders_cache() -> Result<(), String> {
        SHADERS_CACHE.get_or_init(|| match shaders::get_all_shaders() {
            Ok(shaders) => shaders,
            Err(e) => {
                AppError::Runtime(format!("Failed to cache shaders: {}", e)).log("shaders_cache");
                Vec::new()
            }
        });
        Ok(())
    }

    pub fn load_file(&self, file: String, progress: Option<f64>) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        mpv.command("loadfile", &[&file, "append-play"])
            .map_err(|e| AppError::Runtime(format!("Failed to load file '{}': {}", file, e)))?;

        if !self.user_settings.audio_channel.is_empty() {
            drop(mpv);

            std::thread::sleep(std::time::Duration::from_millis(100));

            audio::set_audio_channel(&self.mpv, &self.user_settings.audio_channel)?;
        }

        self.tracker.start(progress);

        Ok(())
    }

    pub fn toggle_play(&self, paused: bool) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        if paused {
            mpv.set_property("pause", false)
                .map_err(|e| AppError::Runtime(format!("Failed to play: {}", e)))
        } else {
            mpv.set_property("pause", true)
                .map_err(|e| AppError::Runtime(format!("Failed to pause: {}", e)))
        }
    }

    pub fn seek(&self, seek_amount: i8) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        mpv.command("seek", &[&seek_amount.to_string(), "relative"])
            .map_err(|e| AppError::Runtime(format!("Failed to seek {}: {}", seek_amount, e)))?;

        Ok(())
    }

    pub fn set_time(&self, time: f64) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        mpv.command("seek", &[&time.to_string(), "absolute"])
            .map_err(|e| AppError::Runtime(format!("Failed to seek {}: {}", time, e)))?;

        Ok(())
    }

    pub fn set_volume(&self, volume: f64) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        let volume = volume.trunc();

        if volume <= 0.0 {
            mpv.set_property("mute", true)
                .map_err(|e| AppError::Runtime(format!("Failed to mute: {}", e)))?;
        } else {
            mpv.set_property("mute", false)
                .map_err(|e| AppError::Runtime(format!("Failed to unmute: {}", e)))?;

            mpv.set_property("volume", volume).map_err(|e| {
                AppError::Runtime(format!("Failed to set volume {}: {}", volume, e))
            })?;
        }

        Ok(())
    }

    pub fn shutdown(&self) -> Result<()> {
        self.tracker.stop();

        {
            let mpv = self
                .mpv
                .lock()
                .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

            mpv.command("quit", &[])
                .map_err(|e| AppError::Runtime(format!("Failed to quit instance: {}", e)))?;
        }

        Ok(())
    }

    pub fn set_audio_channel(&self, audio_channel: &str) -> Result<()> {
        audio::set_audio_channel(&self.mpv, audio_channel)
    }

    pub fn set_subtitle_track(&self, subtitle_track_id: i64) -> Result<()> {
        let subtitle_manager = SubtitleManager::new(Arc::clone(&self.mpv));
        subtitle_manager.set_subtitle_track(Some(subtitle_track_id))
    }

    pub fn set_audio_track(&self, audio_track_id: i64) -> Result<()> {
        let audio_manager = AudioManager::new(Arc::clone(&self.mpv));
        audio_manager.set_audio_track(Some(audio_track_id))
    }

    pub fn av_sync_adjust(&self, value: f64) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        audio::av_sync_adjust(&mpv, value)
    }

    pub fn center_speaker_level(&self, value: i8) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        audio::center_speaker_level(&mpv, value)
    }

    pub fn set_subtitle_pos(&self, value: i64) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        subtitles::set_subtitle_pos(&mpv, value)
    }

    pub fn set_subtitle_scaling(&self, value: f64) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        subtitles::set_subtitle_scaling(&mpv, value)
    }

    pub fn add_playlist_item(&self, file: String) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        mpv.command("loadfile", &[&file, "insert-next"])
            .map_err(|e| AppError::Runtime(format!("Failed to load file '{}': {}", file, e)))?;

        Ok(())
    }

    pub fn next_playlist_item(&self) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        mpv.command("playlist-next", &[])
            .map_err(|e| AppError::Runtime(format!("Failed to play playlist item : {}", e)))?;

        Ok(())
    }

    pub fn previous_playlist_item(&self) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        mpv.command("playlist-prev", &[])
            .map_err(|e| AppError::Runtime(format!("Failed to play playlist item : {}", e)))?;

        Ok(())
    }

    pub fn get_available_shaders(&self) -> Result<Vec<ShaderInfo>, String> {
        SHADERS_CACHE
            .get()
            .cloned()
            .ok_or_else(|| "Shaders cache not initialized".to_string())
    }

    pub fn get_subtitle_pos(&self) -> Result<i64, String> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| format!("Failed to lock MPV instance: {}", e))?;

        subtitles::get_subtitle_pos(&mpv)
    }

    pub fn toggle_shader(&self, value: &str) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        shaders::toggle_shader(&mpv, value)?;

        Ok(())
    }

    pub fn subtitle_sync_adjust(&self, value: f64) -> Result<()> {
        let mpv = self
            .mpv
            .lock()
            .map_err(|e| AppError::Runtime(format!("Failed to lock MPV instance: {}", e)))?;

        subtitles::subtitle_sync_adjust(&mpv, value)
    }
}
