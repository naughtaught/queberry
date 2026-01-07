pub const API_VERSION: &str = "v1.0";
pub const INDEXER_RATE_LIMIT: usize = 10; // Maximum calls per minute for indexer plugins
pub const RESOLVER_RATE_LIMIT: usize = 30; // Maximum calls per minute for resolver plugins
pub const RATE_LIMIT_WINDOW_SECONDS: u64 = 60; // 1 minute window
