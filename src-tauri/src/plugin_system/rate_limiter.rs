use crate::errors::AppError;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct RateLimiter {
    calls: Arc<DashMap<String, Vec<Instant>>>,
    limits: DashMap<String, usize>,
    window_seconds: u64,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            calls: Arc::new(DashMap::new()),
            limits: DashMap::new(),
            window_seconds: 60,
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
        let limit = self.limits.get(plugin_id).map(|v| *v).unwrap_or(0);

        if limit == 0 {
            return Ok(());
        }

        let now = Instant::now();
        let window = Duration::from_secs(self.window_seconds);

        let mut entry = self.calls.entry(plugin_id.to_string()).or_default();

        entry.retain(|&time| now.duration_since(time) <= window);

        if entry.len() >= limit {
            return Err(AppError::RateLimit(format!(
                "Rate limit exceeded for plugin '{}'. Maximum {} calls per {} seconds.",
                plugin_id, limit, self.window_seconds
            )));
        }

        entry.push(now);
        Ok(())
    }
}
