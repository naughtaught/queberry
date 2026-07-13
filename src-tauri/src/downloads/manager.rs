use futures_util::stream::StreamExt;
use once_cell::sync::Lazy;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Runtime};
use tokio::fs as tokio_fs;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::Mutex;
use tokio::time;

use crate::db::types::{CreateDownload, DownloadStatus};
use crate::downloads::types::{
    DownloadFile, DownloadInfo, DownloadManager, DownloadMetaData, FileIndex, RateLimiter,
    RetryConfig, WriteCounter,
};
use crate::errors::{DownloadError, DownloadResult};

static MEDIA_ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s*\{tt\d+\}").unwrap());

impl DownloadManager {
    fn sanitize_name(name: &str) -> String {
        let invalid_chars = [':', '?', '*', '|', '<', '>', '"', '/', '\\', '\''];
        let mut result = name.to_string();
        for ch in invalid_chars {
            result = result.replace(ch, "");
        }
        result.trim().to_string()
    }

    fn normalize_for_match(s: &str) -> String {
        s.to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect()
    }

    async fn find_existing_media_folder(
        base_dir: &Path,
        imdb_id: &str,
        title: &str,
        released: Option<i32>,
    ) -> Option<PathBuf> {
        let Ok(mut entries) = tokio_fs::read_dir(base_dir).await else {
            return None;
        };
        let mut dirs = Vec::new();
        while let Ok(Some(entry)) = entries.next_entry().await {
            let Ok(metadata) = entry.metadata().await else {
                continue;
            };
            if metadata.is_dir() {
                dirs.push(entry.path());
            }
        }

        if !imdb_id.is_empty() {
            let lower_id = imdb_id.to_lowercase();
            for path in &dirs {
                let folder_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.to_lowercase())
                    .unwrap_or_default();
                if folder_name.contains(&lower_id) {
                    return Some(path.clone());
                }
            }
        }

        let sanitized_title = Self::sanitize_name(title);
        let normalized_title = Self::normalize_for_match(title);
        let normalized_sanitized = Self::normalize_for_match(&sanitized_title);
        let release_year = released.map(|y| y.to_string());

        for path in &dirs {
            let folder_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_owned();
            let normalized_folder = Self::normalize_for_match(&folder_name);
            let matches_title = normalized_folder.contains(&normalized_title)
                || normalized_folder.contains(&normalized_sanitized);
            let matches_year = release_year
                .as_ref()
                .map_or(true, |year| normalized_folder.contains(year));
            if matches_title && matches_year {
                return Some(path.clone());
            }
        }
        None
    }

    async fn get_extension_from_url(url: &str) -> String {
        if let Ok(parsed_url) = url::Url::parse(url) {
            let path = parsed_url.path();
            if let Some(ext) = Path::new(path).extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if matches!(ext_str.as_str(), "jpg" | "jpeg" | "png" | "webp" | "gif") {
                    return format!(".{}", ext_str);
                }
            }
        }
        ".jpg".to_string()
    }

    async fn download_file_once<R: Runtime>(
        &self,
        app_handle: &AppHandle<R>,
        params: &DownloadFile<'_>,
        user_id: i32,
    ) -> Result<(), (DownloadError, u64)> {
        if params.cancel_flag.load(Ordering::Relaxed) {
            return Err((DownloadError::Cancelled, 0));
        }

        let temp_path = &params.temp_path;
        let final_path = &params.final_path;

        let existing_size = tokio_fs::metadata(temp_path)
            .await
            .map(|m| m.len())
            .unwrap_or(0);

        let mut request_builder = self.http_client.get(params.url);
        if existing_size > 0 {
            request_builder = request_builder.header("Range", format!("bytes={}-", existing_size));
        }

        let response = request_builder
            .timeout(Duration::from_secs(3600))
            .send()
            .await
            .map_err(|e| (DownloadError::Http(e), existing_size))?;

        let status = response.status();

        let is_chunked = response
            .headers()
            .get(reqwest::header::TRANSFER_ENCODING)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_ascii_lowercase().contains("chunked"))
            .unwrap_or(false);

        let (total_size, file_start_offset) = match status {
            reqwest::StatusCode::OK => {
                let total = response.content_length().unwrap_or(0);
                if existing_size > 0 {
                    let _ = tokio_fs::remove_file(temp_path).await;
                }
                (total, 0)
            }
            reqwest::StatusCode::PARTIAL_CONTENT => {
                let total = response
                    .headers()
                    .get("content-range")
                    .and_then(|v| {
                        let s = v.to_str().ok()?;
                        s.split('/').nth(1).and_then(|t| t.parse::<u64>().ok())
                    })
                    .or_else(|| response.content_length())
                    .unwrap_or(0);
                (total, existing_size)
            }
            _ if !status.is_success() => {
                return Err((
                    DownloadError::HttpStatus {
                        status: status.as_u16(),
                        url: params.url.to_string(),
                    },
                    existing_size,
                ));
            }
            _ => (response.content_length().unwrap_or(0), existing_size),
        };

        let unverifiable_framing = total_size == 0 && !is_chunked;

        {
            let downloads = self.downloads.read().await;
            if let Some(info) = downloads.get(params.uuid) {
                let mut paths = info.file_paths.lock().await;
                if !paths.contains(final_path) {
                    paths.push(final_path.clone());
                }
            }
        }

        let mut file = if existing_size > 0 && file_start_offset > 0 {
            BufWriter::with_capacity(
                256 * 1024,
                tokio_fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(temp_path)
                    .await
                    .map_err(|e| (DownloadError::Io(e), existing_size))?,
            )
        } else {
            BufWriter::with_capacity(
                256 * 1024,
                tokio_fs::File::create(temp_path)
                    .await
                    .map_err(|e| (DownloadError::Io(e), 0))?,
            )
        };

        let mut stream = response.bytes_stream();
        let mut counter = WriteCounter::new(
            total_size,
            params.file_index as u32,
            params.uuid.to_string(),
            app_handle.clone(),
        );
        counter.written = file_start_offset;
        counter.last_db_update = file_start_offset;

        let speed_limit = self.get_speed_limit_bytes(user_id).await;

        let mut local_limiter = RateLimiter::new(speed_limit);

        if total_size > 0 {
            if let Some(db) = &self.db {
                let _ = db
                    .update_download_total_bytes(user_id, params.uuid, total_size as i64)
                    .await;
            }
        }

        while let Some(chunk_result) = stream.next().await {
            if params.cancel_flag.load(Ordering::Relaxed) {
                let _ = file.flush().await;
                drop(file);
                if let Some(db) = &self.db {
                    let _ = db
                        .update_download_status(user_id, params.uuid, DownloadStatus::Paused)
                        .await;
                    let _ = db
                        .update_download_progress(user_id, params.uuid, counter.written as i64)
                        .await;
                }
                return Err((DownloadError::Cancelled, counter.written));
            }

            let chunk = match chunk_result {
                Ok(chunk) => chunk,
                Err(e) => {
                    if let Some(db) = &self.db {
                        let _ = db
                            .update_download_progress(user_id, params.uuid, counter.written as i64)
                            .await;
                    }
                    return Err((DownloadError::Http(e), counter.written));
                }
            };

            if let Some(sleep_time) = local_limiter.request(chunk.len() as u64) {
                time::sleep(sleep_time).await;
            }

            counter.write(&chunk);
            file.write_all(&chunk)
                .await
                .map_err(|e| (DownloadError::Io(e), counter.written))?;

            if counter.written - counter.last_db_update >= 10 * 1024 * 1024 {
                if let Some(db) = &self.db {
                    let _ = db
                        .update_download_progress(user_id, params.uuid, counter.written as i64)
                        .await;
                }
                counter.last_db_update = counter.written;
            }
        }

        file.flush()
            .await
            .map_err(|e| (DownloadError::Io(e), counter.written))?;

        if total_size > 0 && counter.written != total_size {
            if let Some(db) = &self.db {
                let _ = db
                    .update_download_progress(user_id, params.uuid, counter.written as i64)
                    .await;
            }
            return Err((DownloadError::Incomplete(counter.written), counter.written));
        }

        if unverifiable_framing && counter.written == 0 {
            if let Some(db) = &self.db {
                let _ = db
                    .update_download_progress(user_id, params.uuid, counter.written as i64)
                    .await;
            }
            return Err((DownloadError::Incomplete(counter.written), counter.written));
        }

        if let Err(e) = tokio_fs::rename(temp_path, final_path).await {
            let copied = tokio_fs::copy(temp_path, final_path)
                .await
                .map_err(|_| (DownloadError::Io(e), counter.written))?;

            if copied != counter.written {
                let _ = tokio_fs::remove_file(final_path).await;
                return Err((DownloadError::Incomplete(copied), copied));
            }
            let _ = tokio_fs::remove_file(temp_path).await;
        }

        counter.emit_completion();

        if let Some(db) = &self.db {
            let _ = db
                .update_download_status(user_id, params.uuid, DownloadStatus::Completed)
                .await;
            let _ = db
                .update_download_progress(user_id, params.uuid, counter.written as i64)
                .await;
        }

        let _ = app_handle.emit(
            "download_completed",
            serde_json::json!({
                "uuid": params.uuid,
                "filename": params.download_title
            }),
        );

        Ok(())
    }

    async fn download_file<R: Runtime>(
        &self,
        app_handle: &AppHandle<R>,
        params: DownloadFile<'_>,
        user_id: i32,
    ) -> DownloadResult<()> {
        let RetryConfig {
            max_attempts,
            initial_backoff,
            max_backoff,
        } = self.retry_config.clone();

        let mut per_attempt = 0u32;
        let mut total_resets = 0u32;
        let reset_cap = max_attempts.saturating_mul(2);
        let mut backoff = initial_backoff;

        loop {
            per_attempt += 1;

            match self.download_file_once(app_handle, &params, user_id).await {
                Ok(()) => return Ok(()),

                Err((DownloadError::Cancelled, _)) => {
                    return Err(DownloadError::Cancelled);
                }

                Err((e, _)) if !e.is_retryable() => {
                    self.fail_download(app_handle, &params, user_id, &e).await;
                    return Err(e);
                }

                Err((e, bytes_written)) if per_attempt >= max_attempts => {
                    self.fail_download(app_handle, &params, user_id, &e).await;
                    return Err(DownloadError::RetriesExhausted {
                        attempts: per_attempt,
                        last_error: e.to_string(),
                    });
                }

                Err((e, bytes_written)) => {
                    if bytes_written > 0 && total_resets < reset_cap {
                        total_resets += 1;
                        per_attempt = 0;
                        backoff = initial_backoff;
                    } else if bytes_written > 0 {
                        self.fail_download(app_handle, &params, user_id, &e).await;
                        return Err(DownloadError::RetriesExhausted {
                            attempts: total_resets,
                            last_error: e.to_string(),
                        });
                    } else {
                        time::sleep(backoff).await;
                        backoff = (backoff * 2).min(max_backoff);
                    }
                }
            }
        }
    }

    async fn fail_download<R: Runtime>(
        &self,
        app_handle: &AppHandle<R>,
        params: &DownloadFile<'_>,
        user_id: i32,
        error: &DownloadError,
    ) {
        if matches!(error, DownloadError::Cancelled) {
            let _ = tokio_fs::remove_file(&params.temp_path).await;
            return;
        }

        if let Some(db) = &self.db {
            let _ = db
                .update_download_status(user_id, params.uuid, DownloadStatus::Failed)
                .await;
        }

        let _ = app_handle.emit(
            "download_failed",
            serde_json::json!({
                "uuid": params.uuid,
                "filename": params.download_title,
                "error": error.to_string()
            }),
        );
    }

    pub async fn try_reserve_download_slot(&self, user_id: i32) -> bool {
        let max = self.get_max_concurrent_downloads(user_id).await as usize;
        loop {
            let current = self.active_downloads.load(Ordering::SeqCst);
            if current >= max {
                return false;
            }
            if self
                .active_downloads
                .compare_exchange(current, current + 1, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                return true;
            }
        }
    }

    pub async fn download_file_command<R: Runtime>(
        &self,
        app_handle: AppHandle<R>,
        params: DownloadMetaData,
        user_id: i32,
    ) -> DownloadResult<String> {
        if !self.try_reserve_download_slot(user_id).await {
            let sanitized_filename = Self::sanitize_name(&params.filename);

            if let Some(db) = &self.db {
                let queue_order = db.get_next_queue_order(user_id).await.unwrap_or_default();

                if db.get_download(user_id, &params.uuid).await.is_err() {
                    let create_download = CreateDownload {
                        folder_path: params.folder_path.clone(),
                        file_link: params.file_link.clone(),
                        resolver_id: params.resolver_id.clone(),
                        imdb_id: if params.imdb_id.is_empty() {
                            None
                        } else {
                            Some(params.imdb_id.clone())
                        },
                        title: params.title.clone(),
                        released: params.released,
                        season: params.season,
                        episode: params.episode,
                        file_url: params.file_url.clone(),
                        media_poster: params.media_poster.clone(),
                        season_poster: params.season_poster.clone(),
                        filename: sanitized_filename,
                        part_file_path: String::new(),
                        final_file_path: String::new(),
                        status: DownloadStatus::Pending,
                        queue_order,
                        total_bytes: None,
                        downloaded_bytes: None,
                    };
                    let _ = db
                        .create_download(user_id, params.uuid.clone(), create_download)
                        .await;
                }
            }

            return Ok("Download queued".to_string());
        }

        let cancel_flag = Arc::new(AtomicBool::new(false));
        let uuid = params.uuid.clone();

        {
            let downloads = self.downloads.read().await;
            if downloads.contains_key(&uuid) {
                return Ok("Download already in progress".to_string());
            }
        }

        let sanitized_filename = Self::sanitize_name(&params.filename);

        if let Some(db) = &self.db {
            if db.get_download(user_id, &uuid).await.is_ok() {
                let _ = db
                    .update_download_status(user_id, &uuid, DownloadStatus::Downloading)
                    .await;
            } else {
                let create_download = CreateDownload {
                    folder_path: params.folder_path.clone(),
                    file_link: params.file_link.clone(),
                    resolver_id: params.resolver_id.clone(),
                    imdb_id: if params.imdb_id.is_empty() {
                        None
                    } else {
                        Some(params.imdb_id.clone())
                    },
                    title: params.title.clone(),
                    released: params.released,
                    season: params.season,
                    episode: params.episode,
                    file_url: params.file_url.clone(),
                    media_poster: params.media_poster.clone(),
                    season_poster: params.season_poster.clone(),
                    filename: sanitized_filename,
                    part_file_path: String::new(),
                    final_file_path: String::new(),
                    status: DownloadStatus::Downloading,
                    queue_order: 0,
                    total_bytes: None,
                    downloaded_bytes: None,
                };
                let _ = db
                    .create_download(user_id, uuid.clone(), create_download)
                    .await;
            }
        }

        let this = self.clone();
        let app_handle_clone = app_handle.clone();
        let params_clone = params.clone();
        let cancel_flag_clone = cancel_flag.clone();
        let uuid_clone = uuid.clone();

        let handle = tokio::spawn(async move {
            let _ = this
                .download_internal(app_handle_clone, params_clone, cancel_flag_clone, user_id)
                .await;

            this.active_downloads.fetch_sub(1, Ordering::SeqCst);

            {
                let mut downloads = this.downloads.write().await;
                downloads.remove(&uuid_clone);
            }
        });

        {
            let mut downloads = self.downloads.write().await;
            downloads.insert(
                uuid.clone(),
                Arc::new(DownloadInfo {
                    cancel_flag,
                    file_paths: Arc::new(Mutex::new(Vec::new())),
                    folder_paths: Arc::new(Mutex::new(Vec::new())),
                    task_handle: Mutex::new(Some(handle)),
                }),
            );
        }

        Ok("Download started".to_string())
    }

    async fn download_internal<R: Runtime>(
        &self,
        app_handle: AppHandle<R>,
        params: DownloadMetaData,
        cancel_flag: Arc<AtomicBool>,
        user_id: i32,
    ) -> DownloadResult<String> {
        let base_path = PathBuf::from(&params.folder_path);
        let clean_title = MEDIA_ID_REGEX.replace_all(&params.title, "").to_string();

        let media_folder = if let Some(existing) = Self::find_existing_media_folder(
            &base_path,
            &params.imdb_id,
            &clean_title,
            params.released,
        )
        .await
        {
            existing
        } else {
            let mut folder_name = clean_title.clone();
            if let Some(year) = params.released {
                let year_str = format!("({})", year);
                if !folder_name.contains(&year_str) {
                    folder_name.push_str(&format!(" {}", year_str));
                }
            }
            if !params.imdb_id.is_empty() {
                folder_name.push_str(&format!(" {{{}}}", params.imdb_id));
            }
            let folder_name = Self::sanitize_name(&folder_name);
            let new_path = base_path.join(folder_name);
            tokio_fs::create_dir_all(&new_path)
                .await
                .map_err(DownloadError::Io)?;

            {
                let downloads = self.downloads.read().await;
                if let Some(info) = downloads.get(&params.uuid) {
                    let mut folders = info.folder_paths.lock().await;
                    folders.push(new_path.clone());
                }
            }
            new_path
        };

        if let Some(poster) = &params.media_poster {
            if !poster.is_empty() && poster.starts_with("http") {
                let poster_extension = Self::get_extension_from_url(poster).await;

                let poster_name = if let Some(year) = params.released {
                    format!("{} ({}){}", clean_title, year, poster_extension)
                } else {
                    format!("{}{}", clean_title, poster_extension)
                };
                let sanitized_poster_name = Self::sanitize_name(&poster_name);
                let poster_final = media_folder.join(&sanitized_poster_name);

                if !poster_final.exists() {
                    {
                        let downloads = self.downloads.read().await;
                        if let Some(info) = downloads.get(&params.uuid) {
                            let mut paths = info.file_paths.lock().await;
                            paths.push(poster_final.clone());
                        }
                    }

                    self.download_poster(
                        &app_handle,
                        poster,
                        &media_folder,
                        &sanitized_poster_name,
                        &params.uuid,
                        FileIndex::MediaPoster,
                    )
                    .await;
                }
            }
        }

        let (download_path, download_title) = if let Some(season_num) = params.season {
            let season_folder = if season_num == 0 {
                "Specials".to_string()
            } else {
                format!("Season {:02}", season_num)
            };
            let season_path = media_folder.join(&season_folder);

            if !season_path.exists() {
                tokio_fs::create_dir_all(&season_path)
                    .await
                    .map_err(DownloadError::Io)?;

                {
                    let downloads = self.downloads.read().await;
                    if let Some(info) = downloads.get(&params.uuid) {
                        let mut folders = info.folder_paths.lock().await;
                        if !folders.contains(&season_path) {
                            folders.push(season_path.clone());
                        }
                    }
                }

                if let Some(poster) = &params.season_poster {
                    if !poster.is_empty() && poster.starts_with("http") {
                        let season_title = if season_num == 0 {
                            "Specials".to_string()
                        } else {
                            format!("Season {:02}", season_num)
                        };

                        let poster_extension = Self::get_extension_from_url(poster).await;

                        let poster_name = if let Some(year) = params.released {
                            format!(
                                "{} ({}) - {}{}",
                                params.title, year, season_title, poster_extension
                            )
                        } else {
                            format!("{} - {}{}", params.title, season_title, poster_extension)
                        };
                        let sanitized_poster_name = Self::sanitize_name(&poster_name);
                        let poster_final = season_path.join(&sanitized_poster_name);

                        if !poster_final.exists() {
                            {
                                let downloads = self.downloads.read().await;
                                if let Some(info) = downloads.get(&params.uuid) {
                                    let mut paths = info.file_paths.lock().await;
                                    paths.push(poster_final.clone());
                                }
                            }

                            self.download_poster(
                                &app_handle,
                                poster,
                                &season_path,
                                &sanitized_poster_name,
                                &params.uuid,
                                FileIndex::SeasonPoster,
                            )
                            .await;
                        }
                    }
                }
            }

            let filename = Self::sanitize_name(&params.filename);
            (season_path, filename)
        } else {
            let filename = Self::sanitize_name(&params.filename);
            (media_folder.clone(), filename)
        };

        let final_path = download_path.join(&download_title);
        let temp_path = download_path.join(format!("{}.part", &download_title));

        {
            let downloads = self.downloads.read().await;
            if let Some(info) = downloads.get(&params.uuid) {
                let mut paths = info.file_paths.lock().await;
                if !paths.contains(&final_path) {
                    paths.push(final_path.clone());
                }
            }
        }

        if let Some(db) = &self.db {
            let _ = db
                .update_download_status(user_id, &params.uuid, DownloadStatus::Downloading)
                .await;
            let _ = db
                .update_download_paths(
                    user_id,
                    &params.uuid,
                    &temp_path.to_string_lossy(),
                    &final_path.to_string_lossy(),
                )
                .await;
        }

        self.download_file(
            &app_handle,
            DownloadFile {
                url: &params.file_url,
                path: &download_path,
                file_index: FileIndex::MainFile,
                download_title: &download_title,
                uuid: &params.uuid,
                cancel_flag: cancel_flag.clone(),
                temp_path,
                final_path,
            },
            user_id,
        )
        .await?;

        Ok(format!(
            "Files downloaded successfully to: {}",
            media_folder.display()
        ))
    }

    async fn download_poster_once(&self, params: DownloadFile<'_>) -> Result<(), DownloadError> {
        let final_path = &params.final_path;

        if final_path.exists() {
            return Ok(());
        }

        if let Some(parent) = final_path.parent() {
            tokio_fs::create_dir_all(parent).await?;
        }

        let response = self
            .http_client
            .get(params.url)
            .timeout(Duration::from_secs(30))
            .send()
            .await
            .map_err(DownloadError::Http)?;

        if !response.status().is_success() {
            return Err(DownloadError::HttpStatus {
                status: response.status().as_u16(),
                url: params.url.to_string(),
            });
        }

        let bytes = response.bytes().await.map_err(DownloadError::Http)?;

        tokio_fs::write(final_path, &bytes)
            .await
            .map_err(DownloadError::Io)?;

        Ok(())
    }

    async fn download_poster<R: Runtime>(
        &self,
        app_handle: &AppHandle<R>,
        url: &str,
        path: &Path,
        poster_filename: &str,
        uuid: &str,
        file_index: FileIndex,
    ) {
        let temp_path = path.join(format!("{}.part", poster_filename));
        let final_path = path.join(poster_filename);

        if final_path.exists() {
            return;
        }

        let cancel_flag = Arc::new(AtomicBool::new(false));

        let result = self
            .download_poster_once(DownloadFile {
                url,
                path,
                file_index,
                download_title: poster_filename,
                uuid,
                cancel_flag: cancel_flag.clone(),
                temp_path: temp_path.clone(),
                final_path,
            })
            .await;

        if let Err(e) = result {
            let _ = tokio_fs::remove_file(&temp_path).await;

            let _ = app_handle.emit(
                "download_warning",
                serde_json::json!({
                    "type": "poster",
                    "message": format!("Failed to download poster for {}: {}", poster_filename, e)
                }),
            );
        }
    }

    pub async fn cancel_download(&self, user_id: i32, uuid: String) {
        let handle = {
            let mut downloads = self.downloads.write().await;
            if let Some(info) = downloads.remove(&uuid) {
                info.cancel_flag.store(true, Ordering::Relaxed);
                info.task_handle.lock().await.take()
            } else {
                None
            }
        };

        if let Some(h) = handle {
            let _ = h.await;
        }

        if let Some(db) = &self.db {
            match db.get_download(user_id, &uuid).await {
                Ok(download) => {
                    let mut files_to_remove = Vec::new();
                    let mut folders_to_check = Vec::new();

                    if !download.part_file_path.is_empty() {
                        let part_path = PathBuf::from(&download.part_file_path);
                        if let Some(parent) = part_path.parent() {
                            folders_to_check.push(parent.to_path_buf());
                        }
                        files_to_remove.push(part_path);
                    }

                    let created_files = {
                        let downloads = self.downloads.read().await;
                        if let Some(info) = downloads.get(&uuid) {
                            let paths = info.file_paths.lock().await;
                            let folders = info.folder_paths.lock().await;
                            (paths.clone(), folders.clone())
                        } else {
                            (vec![], vec![])
                        }
                    };

                    for file_path in &created_files.0 {
                        if !files_to_remove.contains(file_path) {
                            if let Some(parent) = file_path.parent() {
                                let parent_path = parent.to_path_buf();
                                if !folders_to_check.contains(&parent_path) {
                                    folders_to_check.push(parent_path);
                                }
                            }
                            files_to_remove.push(file_path.clone());
                        }
                    }

                    for folder_path in &created_files.1 {
                        if !folders_to_check.contains(folder_path) {
                            folders_to_check.push(folder_path.clone());
                        }
                    }

                    if !download.final_file_path.is_empty() {
                        let final_path = PathBuf::from(&download.final_file_path);
                        if !files_to_remove.contains(&final_path) {
                            if let Some(parent) = final_path.parent() {
                                let parent_path = parent.to_path_buf();
                                if !folders_to_check.contains(&parent_path) {
                                    folders_to_check.push(parent_path);
                                }
                            }
                            files_to_remove.push(final_path);
                        }
                    }

                    for file_path in &files_to_remove {
                        if file_path.exists() {
                            let _ = tokio_fs::remove_file(file_path).await;
                        }
                    }

                    for folder in &folders_to_check {
                        self.remove_empty_folder_recursive(folder).await;
                    }

                    let _ = db.delete_download(user_id, &uuid).await;
                }
                Err(_) => {
                    let _ = db
                        .update_download_status(user_id, &uuid, DownloadStatus::Cancelled)
                        .await;
                }
            }
        }
    }

    async fn remove_empty_folder_recursive(&self, folder: &Path) {
        if !folder.exists() {
            return;
        }

        if let Ok(mut entries) = tokio_fs::read_dir(folder).await {
            let mut has_content = false;

            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();

                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if file_name == ".DS_Store" || file_name == "Thumbs.db" {
                        let _ = tokio_fs::remove_file(&path).await;
                        continue;
                    }
                }

                has_content = true;
                break;
            }

            if !has_content {
                let _ = tokio_fs::remove_dir(folder).await;

                if let Some(parent) = folder.parent() {
                    let path_str = parent.to_string_lossy();
                    if !path_str.is_empty()
                        && path_str != "/"
                        && path_str != "C:"
                        && path_str != "C:\\"
                        && path_str.len() > 3
                    {
                        Box::pin(self.remove_empty_folder_recursive(parent)).await;
                    }
                }
            }
        }
    }

    pub async fn can_start_new_download(&self, user_id: i32) -> bool {
        let active = self.active_downloads.load(Ordering::SeqCst);
        let max = self.get_max_concurrent_downloads(user_id).await;
        (active as i32) < max
    }

    pub fn get_active_download_count(&self) -> usize {
        self.active_downloads.load(Ordering::SeqCst)
    }

    pub async fn get_max_concurrent_downloads(&self, user_id: i32) -> i32 {
        if let Some(db) = &self.db {
            if let Ok(max) = db.get_max_concurrent_downloads(user_id).await {
                return max;
            }
        }
        5
    }

    pub async fn get_speed_limit_bytes(&self, user_id: i32) -> u64 {
        if let Some(db) = &self.db {
            if let Ok(rate_limit) = db.get_download_rate_limit(user_id).await {
                if rate_limit > 0 {
                    let bytes = (rate_limit as u64 * 1_000_000) / 8;
                    return bytes;
                }
            }
        }
        u64::MAX
    }
}
