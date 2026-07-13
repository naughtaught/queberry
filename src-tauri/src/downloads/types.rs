use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Runtime};
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;

use crate::db::downloads::DownloadManager as DbDownloadManager;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileIndex {
    MainFile = 1,
    MediaPoster = 2,
    SeasonPoster = 3,
}

impl From<FileIndex> for u32 {
    fn from(idx: FileIndex) -> Self {
        idx as u32
    }
}

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_backoff: Duration,
    pub max_backoff: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff: Duration::from_secs(1),
            max_backoff: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    pub file_index: u32,
    pub uuid: String,
    pub progress: f64,
    pub speed: f64,
    pub eta: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadMetaData {
    pub folder_path: String,
    pub file_link: String,
    pub resolver_id: String,
    pub imdb_id: String,
    pub title: String,
    pub released: Option<i32>,
    pub season: Option<i32>,
    pub episode: Option<i32>,
    pub file_url: String,
    pub media_poster: Option<String>,
    pub season_poster: Option<String>,
    pub uuid: String,
    pub filename: String,
}

pub struct DownloadInfo {
    pub cancel_flag: Arc<AtomicBool>,
    pub file_paths: Arc<Mutex<Vec<PathBuf>>>,
    pub folder_paths: Arc<Mutex<Vec<PathBuf>>>,
    pub task_handle: Mutex<Option<JoinHandle<()>>>,
}

pub struct WriteCounter<R: Runtime> {
    pub total: u64,
    pub written: u64,
    pub file_index: u32,
    pub uuid: String,
    pub app_handle: AppHandle<R>,
    pub last_emit: Instant,
    pub bytes_in_span: u64,
    pub speed: f64,
    pub last_db_update: u64,
}

impl<R: Runtime> WriteCounter<R> {
    pub fn new(total: u64, file_index: u32, uuid: String, app_handle: AppHandle<R>) -> Self {
        let now = Instant::now();
        Self {
            total,
            written: 0,
            file_index,
            uuid,
            app_handle,
            last_emit: now,
            bytes_in_span: 0,
            speed: 0.0,
            last_db_update: 0,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> usize {
        let n = data.len();
        let now = Instant::now();

        self.written += n as u64;
        self.bytes_in_span += n as u64;

        let percentage = if self.total > 0 {
            (self.written as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };

        if now.duration_since(self.last_emit).as_secs_f64() >= 2.0 {
            let elapsed = now.duration_since(self.last_emit).as_secs_f64();
            let bytes_per_sec = self.bytes_in_span as f64 / elapsed;
            self.speed = (bytes_per_sec * 8.0) / 1_000_000.0;

            let eta = if bytes_per_sec > 0.0 && self.total > 0 {
                let remaining = self.total.saturating_sub(self.written);
                remaining as f64 / bytes_per_sec
            } else {
                0.0
            };

            let _ = self.app_handle.emit(
                "download_progress",
                DownloadProgress {
                    file_index: self.file_index,
                    uuid: self.uuid.clone(),
                    progress: percentage,
                    speed: self.speed,
                    eta,
                },
            );

            self.last_emit = now;
            self.bytes_in_span = 0;
        }

        n
    }

    pub fn emit_completion(&self) {
        let _ = self.app_handle.emit(
            "download_progress",
            DownloadProgress {
                file_index: self.file_index,
                uuid: self.uuid.clone(),
                progress: 100.0,
                speed: 0.0,
                eta: 0.0,
            },
        );
    }
}

pub struct DownloadFile<'a> {
    pub url: &'a str,
    pub path: &'a Path,
    pub file_index: FileIndex,
    pub download_title: &'a str,
    pub uuid: &'a str,
    pub cancel_flag: Arc<AtomicBool>,
    pub temp_path: PathBuf,
    pub final_path: PathBuf,
}

pub struct RateLimiter {
    pub bytes_per_sec: u64,
    pub available_bytes: f64,
    pub last_refill: Instant,
}

impl RateLimiter {
    pub fn new(bytes_per_sec: u64) -> Self {
        Self {
            bytes_per_sec,
            available_bytes: if bytes_per_sec > 0 {
                bytes_per_sec as f64
            } else {
                0.0
            },
            last_refill: Instant::now(),
        }
    }

    pub fn request(&mut self, chunk_size: u64) -> Option<Duration> {
        if self.bytes_per_sec == 0 {
            return None;
        }

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();

        let new_tokens = elapsed * self.bytes_per_sec as f64;
        self.available_bytes += new_tokens;

        self.available_bytes = self.available_bytes.min(self.bytes_per_sec as f64);
        self.last_refill = now;

        self.available_bytes -= chunk_size as f64;

        if self.available_bytes < 0.0 {
            let sleep_secs = (-self.available_bytes) / self.bytes_per_sec as f64;
            Some(Duration::from_secs_f64(sleep_secs))
        } else {
            None
        }
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(0) // 0 means no rate limiting
    }
}

#[derive(Clone)]
pub struct DownloadManager {
    pub downloads: Arc<RwLock<HashMap<String, Arc<DownloadInfo>>>>,
    pub active_downloads: Arc<AtomicUsize>,
    pub http_client: Arc<Client>,
    pub retry_config: RetryConfig,
    pub db: Option<Arc<DbDownloadManager>>,
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self {
            downloads: Arc::new(RwLock::new(HashMap::new())),
            active_downloads: Arc::new(AtomicUsize::new(0)),
            http_client: Arc::new(
                Client::builder()
                    .timeout(Duration::from_secs(3600 * 4))
                    .no_gzip()
                    .no_brotli()
                    .no_deflate()
                    .build()
                    .expect("Failed to build HTTP client"),
            ),
            retry_config: RetryConfig::default(),
            db: None,
        }
    }
}

impl DownloadManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_db(mut self, db: Arc<DbDownloadManager>) -> Self {
        self.db = Some(db);
        self
    }
}
