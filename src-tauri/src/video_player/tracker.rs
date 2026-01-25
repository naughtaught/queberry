use libmpv2::Mpv;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use tauri::{async_runtime, AppHandle, Emitter};

use crate::video_player::types::{CompletionEvent, VideoState};

#[derive(Clone)]
pub struct PlayerTracker {
    mpv: Arc<Mutex<Mpv>>,
    app_handle: AppHandle,
    complete_percent: i32,
    should_stop: Arc<AtomicBool>,
}

impl PlayerTracker {
    pub fn new(mpv: Arc<Mutex<Mpv>>, app_handle: AppHandle, complete_percent: i32) -> Self {
        Self {
            mpv,
            app_handle,
            complete_percent,
            should_stop: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self) {
        self.should_stop.store(true, Ordering::Relaxed);

        std::thread::sleep(std::time::Duration::from_millis(100));

        self.should_stop.store(false, Ordering::Relaxed);

        let mpv_clone = Arc::clone(&self.mpv);
        let app_handle_clone = self.app_handle.clone();
        let complete_percent = self.complete_percent;
        let should_stop = Arc::clone(&self.should_stop);

        async_runtime::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
            let mut completed_event_emitted = false;
            let complete_percent_i64 = complete_percent as i64;

            loop {
                if should_stop.load(Ordering::Relaxed) {
                    break;
                }

                interval.tick().await;

                if let Ok(guard) = mpv_clone.lock() {
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

                    if !completed_event_emitted && current_time >= 30.0 {
                        if let Ok(percent_pos) = guard.get_property::<i64>("percent-pos") {
                            if percent_pos >= complete_percent_i64 {
                                let completion_event = CompletionEvent { is_completed: true };
                                let _ = app_handle_clone.emit("video-completed", completion_event);
                                completed_event_emitted = true;
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
    }
}
