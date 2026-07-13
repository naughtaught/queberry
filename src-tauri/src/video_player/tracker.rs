use libmpv2::Mpv;
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};
use tauri::{async_runtime, AppHandle, Emitter};

use crate::video_player::types::{ShutdownState, VideoState};

#[derive(Clone)]
pub struct PlayerTracker {
    mpv: Arc<Mutex<Mpv>>,
    app_handle: AppHandle,
    complete_percent: i32,
    should_stop: Arc<AtomicBool>,
    completed_positions: Arc<Mutex<HashSet<i64>>>,
}

impl PlayerTracker {
    pub fn new(mpv: Arc<Mutex<Mpv>>, app_handle: AppHandle, complete_percent: i32) -> Self {
        Self {
            mpv,
            app_handle,
            complete_percent,
            should_stop: Arc::new(AtomicBool::new(false)),
            completed_positions: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn start(&self, progress: Option<f64>) {
        let mpv_clone = Arc::clone(&self.mpv);
        let app_handle_clone = self.app_handle.clone();
        let complete_percent = self.complete_percent;
        let should_stop = Arc::clone(&self.should_stop);
        let completed_positions = Arc::clone(&self.completed_positions);

        async_runtime::spawn(async move {
            let seek_time = { progress.filter(|&time| time > 0.0) };

            if let Some(time) = seek_time {
                loop {
                    if let Ok(guard) = mpv_clone.lock() {
                        if let Ok(duration) = guard.get_property::<f64>("duration") {
                            if duration > 0.0 {
                                match guard.set_property("time-pos", time) {
                                    Ok(_) => (),
                                    Err(e) => println!("Seek failed: {:?}", e),
                                }
                                break;
                            }
                        }
                    }
                }
            }

            let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
            let complete_percent_i64 = complete_percent as i64;

            loop {
                if should_stop.load(Ordering::Relaxed) {
                    break;
                }

                interval.tick().await;

                if let Ok(guard) = mpv_clone.lock() {
                    let playlist_pos = guard.get_property::<i64>("playlist-pos").unwrap_or(-1);
                    let current_time = guard.get_property::<f64>("time-pos").unwrap_or(0.0);
                    let current_time = current_time.trunc();
                    let duration = guard.get_property::<f64>("duration").unwrap_or(0.0);
                    let is_paused = guard.get_property::<bool>("pause").unwrap_or(false);

                    let demuxer_via_network = guard
                        .get_property::<bool>("demuxer-via-network")
                        .unwrap_or(false);
                    let is_local = !demuxer_via_network;

                    let cache_time = guard
                        .get_property::<f64>("demuxer-cache-time")
                        .unwrap_or(0.0);
                    let effective_cache_time = if is_local && duration > 0.0 {
                        duration
                    } else {
                        cache_time
                    };
                    let cache_time = effective_cache_time.ceil();

                    let cache_speed = guard.get_property::<f64>("cache-speed").unwrap_or(0.0);
                    let cache_speed = (cache_speed * 8.0) / 1024.0;
                    let cache_speed = cache_speed.trunc() as u64;

                    if current_time >= 30.0 {
                        if let Ok(percent_pos) = guard.get_property::<i64>("percent-pos") {
                            if percent_pos >= complete_percent_i64 {
                                let already_completed = completed_positions
                                    .lock()
                                    .map(|set| set.contains(&playlist_pos))
                                    .unwrap_or(true);

                                if !already_completed {
                                    let _ = app_handle_clone.emit("video-completed", ());
                                    if let Ok(mut set) = completed_positions.lock() {
                                        set.insert(playlist_pos);
                                    }
                                }
                            }
                        }
                    }

                    if is_paused {
                        let event_data = VideoState {
                            current_time: current_time as u64,
                            cache_time: cache_time as u64,
                            cache_speed: 0,
                            is_buffering: false,
                            buffering_percent: 0,
                            is_paused: true,
                        };
                        let _ = app_handle_clone.emit("current-video-state", event_data);
                        continue;
                    }

                    let is_buffering = guard
                        .get_property::<bool>("paused-for-cache")
                        .unwrap_or(false);

                    let cache_duration = guard
                        .get_property::<f64>("demuxer-cache-duration")
                        .unwrap_or(0.0);

                    let buffering_percent = if cache_time > 0.0 {
                        ((cache_duration / cache_time) * 100.0).clamp(0.0, 100.0)
                    } else {
                        0.0
                    };
                    let buffering_percent = buffering_percent as u64;

                    let event_data = VideoState {
                        current_time: current_time as u64,
                        cache_time: cache_time as u64,
                        cache_speed,
                        is_buffering,
                        buffering_percent,
                        is_paused,
                    };

                    let _ = app_handle_clone.emit("current-video-state", event_data);
                }
            }
        });
    }

    pub fn stop(&self) {
        self.should_stop.store(true, Ordering::Relaxed);

        let last_state = if let Ok(guard) = self.mpv.lock() {
            let time = guard
                .get_property::<f64>("time-pos")
                .map(|t| t.trunc() as u64)
                .unwrap_or(0);
            let percent_pos = guard.get_property::<i64>("percent-pos").unwrap_or(0);
            let complete_percent_i64 = self.complete_percent as i64;

            let is_completed = percent_pos >= complete_percent_i64;

            Some(ShutdownState {
                current_time: time,
                is_completed,
            })
        } else {
            None
        };

        let _ = self.app_handle.emit("video-shutdown", last_state);
    }
}
