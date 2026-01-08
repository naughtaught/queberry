use crate::errors::AppError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct RateLimiter {
    calls: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    limits: HashMap<String, usize>,
    window_seconds: u64,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            calls: Arc::new(Mutex::new(HashMap::new())),
            limits: HashMap::new(),
            window_seconds: 60,
        }
    }

    pub fn with_window_seconds(mut self, seconds: u64) -> Self {
        self.window_seconds = seconds;
        self
    }

    pub fn set_limit(&mut self, plugin_id: &str, limit: usize) {
        self.limits.insert(plugin_id.to_string(), limit);
    }

    pub fn check_limit(&self, plugin_id: &str) -> Result<(), AppError> {
        let limit = self.limits.get(plugin_id).copied().unwrap_or(0);

        if limit == 0 {
            return Ok(());
        }

        // Handle poison error by recovering the data
        let mut calls_map = self.calls.lock().unwrap_or_else(|poisoned| {
            eprintln!("Rate limiter mutex was poisoned, recovering...");
            poisoned.into_inner()
        });

        let now = Instant::now();
        let window = Duration::from_secs(self.window_seconds);

        let calls = calls_map.entry(plugin_id.to_string()).or_default();
        calls.retain(|&time| now.duration_since(time) <= window);

        if calls.len() >= limit {
            return Err(AppError::RateLimit(format!(
                "Rate limit exceeded for plugin '{}'. Maximum {} calls per {} seconds.",
                plugin_id, limit, self.window_seconds
            )));
        }

        calls.push(now);
        Ok(())
    }
}
