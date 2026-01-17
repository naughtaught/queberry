use libmpv2::Mpv;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::{async_runtime, AppHandle, Emitter};

#[derive(Serialize, Clone)]
pub struct TimeUpdateEvent {
    pub current_time: u64,
}

#[derive(Clone)]
pub struct PlayerTracker {
    mpv: Arc<Mutex<Mpv>>,
    app_handle: AppHandle,
}

impl PlayerTracker {
    pub fn new(mpv: Arc<Mutex<Mpv>>, app_handle: AppHandle) -> Self {
        Self { mpv, app_handle }
    }

    pub fn start(&self) {
        let mpv_clone = Arc::clone(&self.mpv);
        let app_handle_clone = self.app_handle.clone();

        async_runtime::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

            loop {
                interval.tick().await;

                if let Ok(guard) = mpv_clone.lock() {
                    if let Ok(current_time) = guard.get_property::<f64>("time-pos") {
                        log::debug!("Current time: {}", current_time);

                        let rounded_time = current_time.trunc();

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
