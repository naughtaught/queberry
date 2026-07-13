use std::sync::OnceLock;

use reqwest::Client;

pub const API_VERSION: &str = "v1.0";

// RATE LIMITS
pub const INDEXER_RATE_LIMIT: usize = 10; // Maximum calls per minute
pub const RESOLVER_RATE_LIMIT: usize = 30; // Maximum calls per minute
pub const UTILITY_RATE_LIMIT: usize = 10;
pub const RATE_LIMIT_WINDOW_SECONDS: u64 = 60; // 1 minute window

// TIMEOUTS
pub const DEFAULT_PLUGIN_TIMEOUT_MS: u64 = 30_000; // 30 seconds
pub const INDEXER_PLUGIN_TIMEOUT_MS: u64 = 30_000; // 30 seconds
pub const RESOLVER_PLUGIN_TIMEOUT_MS: u64 = 30_000; // 30 seconds
pub const UTILITY_PLUGIN_TIMEOUT_MS: u64 = 5_000;
pub const CRITICAL_OPERATION_TIMEOUT_MS: u64 = 5_000; // 5 seconds

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

pub static API_CLIENT: OnceLock<Client> = OnceLock::new();
pub static API_BASE: &str = "https://api.litsift.com";
