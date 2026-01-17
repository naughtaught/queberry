use crate::errors::AppError;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct RateLimiter {
    calls: Arc<DashMap<String, (Vec<Instant>, Instant)>>,
    limits: Arc<DashMap<String, usize>>,
    window_seconds: u64,
    last_cleanup: Arc<std::sync::Mutex<Instant>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            calls: Arc::new(DashMap::new()),
            limits: Arc::new(DashMap::new()),
            window_seconds: 60,
            last_cleanup: Arc::new(std::sync::Mutex::new(Instant::now())),
        }
    }

    pub fn with_window_seconds(mut self, seconds: u64) -> Self {
        self.window_seconds = seconds;
        self
    }

    pub fn set_limit(&self, plugin_id: &str, limit: usize) {
        self.limits.insert(plugin_id.to_string(), limit);
    }

    pub fn check_limit(&self, plugin_id: &str) -> Result<(), AppError> {
        {
            let mut last_cleanup = self.last_cleanup.lock().unwrap();
            if last_cleanup.elapsed() > Duration::from_secs(3600) {
                self.cleanup_memory();
                *last_cleanup = Instant::now();
            }
        }

        let limit = match self.limits.get(plugin_id) {
            Some(entry) => *entry.value(),
            None => 0,
        };

        if limit == 0 {
            return Ok(());
        }

        let now = Instant::now();
        let window = Duration::from_secs(self.window_seconds);

        let mut entry = self
            .calls
            .entry(plugin_id.to_string())
            .or_insert_with(|| (Vec::new(), now));

        entry.0.retain(|&time| now.duration_since(time) <= window);

        entry.1 = now;

        if entry.0.len() >= limit {
            return Err(AppError::RateLimit(format!(
                "Rate limit exceeded for plugin '{}'. Maximum {} calls per {} seconds.",
                plugin_id, limit, self.window_seconds
            )));
        }

        entry.0.push(now);
        Ok(())
    }

    pub fn remove_plugin(&self, plugin_id: &str) {
        self.calls.remove(plugin_id);
        self.limits.remove(plugin_id);
    }

    pub fn cleanup_memory(&self) {
        let now = Instant::now();
        let max_idle_time = Duration::from_secs(24 * 3600); // 24 hours

        self.calls.retain(|_plugin_id, (timestamps, last_access)| {
            now.duration_since(*last_access) <= max_idle_time || !timestamps.is_empty()
        });

        let active_plugins: Vec<String> =
            self.calls.iter().map(|entry| entry.key().clone()).collect();

        self.limits
            .retain(|plugin_id, _| active_plugins.contains(plugin_id));
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}
