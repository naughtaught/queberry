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
