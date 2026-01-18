use libmpv2::Mpv;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::{async_runtime, AppHandle, Emitter};

#[derive(Serialize, Clone)]
pub struct TimeUpdateEvent {
    pub current_time: u64,
}

#[derive(Serialize, Clone)]
pub struct CompletionEvent {
    pub is_completed: bool,
}

#[derive(Clone)]
pub struct PlayerTracker {
    mpv: Arc<Mutex<Mpv>>,
    app_handle: AppHandle,
    complete_percent: i32,
}

impl PlayerTracker {
    pub fn new(mpv: Arc<Mutex<Mpv>>, app_handle: AppHandle, complete_percent: i32) -> Self {
        Self {
            mpv,
            app_handle,
            complete_percent,
        }
    }

    pub fn start(&self) {
        let mpv_clone = Arc::clone(&self.mpv);
        let app_handle_clone = self.app_handle.clone();
        let complete_percent = self.complete_percent;
        async_runtime::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
            let mut completed_event_emitted = false;
            let complete_percent_i64 = complete_percent as i64;

            loop {
                interval.tick().await;

                if let Ok(guard) = mpv_clone.lock() {
                    if let Ok(current_time) = guard.get_property::<f64>("time-pos") {
                        let rounded_time = current_time.trunc();

                        if !completed_event_emitted && rounded_time >= 30.0 {
                            if let Ok(percent_pos) = guard.get_property::<i64>("percent-pos") {
                                if percent_pos >= complete_percent_i64 {
                                    let completion_event = CompletionEvent { is_completed: true };
                                    let _ =
                                        app_handle_clone.emit("video-completed", completion_event);
                                    completed_event_emitted = true;
                                }
                            }
                        }

                        let event_data = TimeUpdateEvent {
                            current_time: rounded_time as u64,
                        };

                        let _ = app_handle_clone.emit("current-time-update", event_data);
                    }
                }
            }
        });
    }
}
