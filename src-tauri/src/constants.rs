use std::time::Duration;

pub const API_VERSION: &str = "v1.0";

// RATE LIMITS
pub const INDEXER_RATE_LIMIT: usize = 10; // Maximum calls per minute for indexer plugins
pub const RESOLVER_RATE_LIMIT: usize = 30; // Maximum calls per minute for resolver plugins
pub const RATE_LIMIT_WINDOW_SECONDS: u64 = 60; // 1 minute window

// TIMEOUTS
pub const DEFAULT_PLUGIN_TIMEOUT_MS: u64 = 30_000; // 30 seconds
pub const INDEXER_PLUGIN_TIMEOUT_MS: u64 = 60_000; // 60 seconds (longer for network)
pub const RESOLVER_PLUGIN_TIMEOUT_MS: u64 = 10_000; // 10 seconds (should be fast)
pub const CRITICAL_OPERATION_TIMEOUT_MS: u64 = 5_000; // 5 seconds (for critical paths)

// MEMORY
pub const DEFAULT_MAX_MEMORY_BYTES: usize = 32 * 1024 * 1024; // 32 MB
pub const DEFAULT_MAX_MEMORY_PAGES: u32 = 512; // 32 MB / 64 KB
pub const CONSERVATIVE_MAX_MEMORY_PAGES: u32 = 256; // 16 MB / 64 KB
pub const HIGH_MAX_MEMORY_PAGES: u32 = 1024; // 64 MB / 64 KB

// HELPERS
pub const fn bytes_to_pages(bytes: usize) -> u32 {
    const PAGE_SIZE: usize = 64 * 1024; // 64 KB
    bytes.div_ceil(PAGE_SIZE) as u32
}
pub const fn pages_to_bytes(pages: u32) -> usize {
    const PAGE_SIZE: usize = 64 * 1024; // 64 KB
    pages as usize * PAGE_SIZE
}

// VIDEO
pub const TIMESTAMP_TRACKING_INTERVAL: Duration = Duration::from_secs(1);
pub const TIME_UPDATE_INTERVAL: Duration = Duration::from_secs(1);
pub const MINIMUM_WATCHED_PERCENT: f64 = 10.0;
pub const SUBTITLE_DEFAULT_POSITION: i64 = 95;
pub const SUBTITLE_DEFAULT_SIZE: f64 = 1.2;
pub const SUBTITLE_SIZE_STEP: f64 = 0.1;
pub const SUBTITLE_SIZE_MIN: f64 = 0.1;
pub const SUBTITLE_SIZE_MAX: f64 = 2.0;
pub const AUDIO_OFFSET_STEP: f64 = 0.05;
pub const SUBTITLE_OFFSET_STEP: f64 = 0.05;
pub const CENTER_SPEAKER_STEP: f64 = 0.1;
pub const CENTER_BOOST_MIN: f64 = 0.1;
pub const CENTER_BOOST_MAX: f64 = 5.0;
pub const SUBTITLE_OFFSET: i64 = 10;
