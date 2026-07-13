use crate::db::types::{Download, DownloadStatus};
use crate::db::{types::CreateDownload, Database};
use crate::errors::AppError;
use std::sync::Arc;

const DOWNLOAD_COLUMNS: &str = "\
    uuid, user_id, folder_path, file_link, resolver_id, imdb_id, title, released, \
    season, episode, file_url, media_poster, season_poster, filename, part_file_path, \
    final_file_path, status, queue_order, added_at, total_bytes, downloaded_bytes";

const DOWNLOAD_INSERT_COLUMNS: &str = "\
    uuid, user_id, folder_path, file_link, resolver_id, imdb_id, title, released, \
    season, episode, file_url, media_poster, season_poster, filename, part_file_path, \
    final_file_path, status, queue_order, total_bytes, downloaded_bytes";

const DOWNLOAD_INSERT_VALUES: &str = "\
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?";

#[derive(Clone)]
pub struct DownloadManager {
    db: Arc<Database>,
}

impl DownloadManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn create_download(
        &self,
        user_id: i32,
        uuid: String,
        download: CreateDownload,
    ) -> Result<Download, AppError> {
        let download = sqlx::query_as::<_, Download>(&format!(
            "INSERT INTO downloads ({}) VALUES ({})
             RETURNING {}",
            DOWNLOAD_INSERT_COLUMNS, DOWNLOAD_INSERT_VALUES, DOWNLOAD_COLUMNS
        ))
        .bind(&uuid)
        .bind(user_id)
        .bind(&download.folder_path)
        .bind(&download.file_link)
        .bind(&download.resolver_id)
        .bind(&download.imdb_id)
        .bind(&download.title)
        .bind(download.released)
        .bind(download.season)
        .bind(download.episode)
        .bind(&download.file_url)
        .bind(&download.media_poster)
        .bind(&download.season_poster)
        .bind(&download.filename)
        .bind(&download.part_file_path)
        .bind(&download.final_file_path)
        .bind(download.status.as_str())
        .bind(download.queue_order)
        .bind(download.total_bytes)
        .bind(download.downloaded_bytes)
        .fetch_one(&self.db.pool)
        .await?;

        Ok(download)
    }

    pub async fn get_download(&self, user_id: i32, uuid: &str) -> Result<Download, AppError> {
        let download = sqlx::query_as::<_, Download>(&format!(
            "SELECT {} FROM downloads WHERE uuid = ? AND user_id = ?",
            DOWNLOAD_COLUMNS
        ))
        .bind(uuid)
        .bind(user_id)
        .fetch_optional(&self.db.pool)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "Download not found: uuid={}, user_id={}",
                uuid, user_id
            ))
        })?;

        Ok(download)
    }

    pub async fn list_downloads(&self, user_id: i32) -> Result<Vec<Download>, AppError> {
        let downloads = sqlx::query_as::<_, Download>(&format!(
            "SELECT {} FROM downloads WHERE user_id = ? ORDER BY queue_order ASC, added_at ASC",
            DOWNLOAD_COLUMNS
        ))
        .bind(user_id)
        .fetch_all(&self.db.pool)
        .await?;

        Ok(downloads)
    }

    pub async fn list_downloads_by_status(
        &self,
        user_id: i32,
        status: DownloadStatus,
    ) -> Result<Vec<Download>, AppError> {
        let downloads = sqlx::query_as::<_, Download>(&format!(
            "SELECT {} FROM downloads WHERE status = ? AND user_id = ? ORDER BY queue_order ASC, added_at ASC",
            DOWNLOAD_COLUMNS
        ))
        .bind(status.as_str())
        .bind(user_id)
        .fetch_all(&self.db.pool)
        .await?;

        Ok(downloads)
    }

    pub async fn update_download_status(
        &self,
        user_id: i32,
        uuid: &str,
        status: DownloadStatus,
    ) -> Result<bool, AppError> {
        let result = sqlx::query("UPDATE downloads SET status = ? WHERE uuid = ? AND user_id = ?")
            .bind(status.as_str())
            .bind(uuid)
            .bind(user_id)
            .execute(&self.db.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn update_download_progress(
        &self,
        user_id: i32,
        uuid: &str,
        downloaded_bytes: i64,
    ) -> Result<bool, AppError> {
        let result =
            sqlx::query("UPDATE downloads SET downloaded_bytes = ? WHERE uuid = ? AND user_id = ?")
                .bind(downloaded_bytes)
                .bind(uuid)
                .bind(user_id)
                .execute(&self.db.pool)
                .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn update_download_paths(
        &self,
        user_id: i32,
        uuid: &str,
        part_file_path: &str,
        final_file_path: &str,
    ) -> Result<bool, AppError> {
        let result = sqlx::query(
            "UPDATE downloads SET part_file_path = ?, final_file_path = ? WHERE uuid = ? AND user_id = ?",
        )
        .bind(part_file_path)
        .bind(final_file_path)
        .bind(uuid)
        .bind(user_id)
        .execute(&self.db.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn update_download_queue_order(
        &self,
        user_id: i32,
        uuid: &str,
        queue_order: i32,
    ) -> Result<bool, AppError> {
        let result =
            sqlx::query("UPDATE downloads SET queue_order = ? WHERE uuid = ? AND user_id = ?")
                .bind(queue_order)
                .bind(uuid)
                .bind(user_id)
                .execute(&self.db.pool)
                .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_download(&self, user_id: i32, uuid: &str) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM downloads WHERE uuid = ? AND user_id = ?")
            .bind(uuid)
            .bind(user_id)
            .execute(&self.db.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn upsert_download(
        &self,
        user_id: i32,
        uuid: String,
        download: CreateDownload,
    ) -> Result<Download, AppError> {
        let download = sqlx::query_as::<_, Download>(&format!(
            "INSERT INTO downloads ({}) VALUES ({})
            ON CONFLICT(uuid) DO UPDATE SET
                user_id = excluded.user_id,
                folder_path = excluded.folder_path,
                file_link = excluded.file_link,
                resolver_id = excluded.resolver_id,
                imdb_id = excluded.imdb_id,
                title = excluded.title,
                released = excluded.released,
                season = excluded.season,
                episode = excluded.episode,
                file_url = excluded.file_url,
                media_poster = excluded.media_poster,
                season_poster = excluded.season_poster,
                filename = excluded.filename,
                part_file_path = excluded.part_file_path,
                final_file_path = excluded.final_file_path,
                status = excluded.status,
                queue_order = excluded.queue_order,
                total_bytes = excluded.total_bytes,
                downloaded_bytes = excluded.downloaded_bytes
            RETURNING {}",
            DOWNLOAD_INSERT_COLUMNS, DOWNLOAD_INSERT_VALUES, DOWNLOAD_COLUMNS
        ))
        .bind(&uuid)
        .bind(user_id)
        .bind(&download.folder_path)
        .bind(&download.file_link)
        .bind(&download.resolver_id)
        .bind(&download.imdb_id)
        .bind(&download.title)
        .bind(download.released)
        .bind(download.season)
        .bind(download.episode)
        .bind(&download.file_url)
        .bind(&download.media_poster)
        .bind(&download.season_poster)
        .bind(&download.filename)
        .bind(&download.part_file_path)
        .bind(&download.final_file_path)
        .bind(download.status.as_str())
        .bind(download.queue_order)
        .bind(download.total_bytes)
        .bind(download.downloaded_bytes)
        .fetch_one(&self.db.pool)
        .await?;

        Ok(download)
    }

    pub async fn clear_completed_downloads(&self, user_id: i32) -> Result<u64, AppError> {
        let result = sqlx::query("DELETE FROM downloads WHERE status IN (?, ?, ?) AND user_id = ?")
            .bind(DownloadStatus::Completed.as_str())
            .bind(DownloadStatus::Cancelled.as_str())
            .bind(DownloadStatus::Failed.as_str())
            .bind(user_id)
            .execute(&self.db.pool)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn get_next_pending_download(
        &self,
        user_id: i32,
    ) -> Result<Option<Download>, AppError> {
        let download = sqlx::query_as::<_, Download>(&format!(
            "SELECT {} FROM downloads 
            WHERE status = ? AND user_id = ?
            ORDER BY queue_order ASC, added_at ASC 
            LIMIT 1",
            DOWNLOAD_COLUMNS
        ))
        .bind(DownloadStatus::Pending.as_str())
        .bind(user_id)
        .fetch_optional(&self.db.pool)
        .await?;

        Ok(download)
    }

    pub async fn clear_completed_downloads_older_than(&self, hours: i64) -> Result<u64, AppError> {
        let downloads = sqlx::query_as::<_, Download>(&format!(
            "SELECT {} FROM downloads 
         WHERE 
            status = ?
            OR (
                status = ? 
                AND julianday('now') - julianday(added_at) >= ?
            )
            OR (
                status = ? 
                AND julianday('now') - julianday(added_at) >= ?
            )",
            DOWNLOAD_COLUMNS
        ))
        .bind(DownloadStatus::Cancelled.as_str())
        .bind(DownloadStatus::Completed.as_str())
        .bind(hours as f64 / 24.0)
        .bind(DownloadStatus::Failed.as_str())
        .bind((hours * 2) as f64 / 24.0)
        .fetch_all(&self.db.pool)
        .await?;

        for download in &downloads {
            if !download.part_file_path.is_empty() {
                let part_path = std::path::PathBuf::from(&download.part_file_path);
                if part_path.exists() {
                    let _ = tokio::fs::remove_file(&part_path).await;
                }
            }
        }

        let result = sqlx::query(
            "DELETE FROM downloads 
         WHERE 
            status = ?
            OR (
                status = ? 
                AND julianday('now') - julianday(added_at) >= ?
            )
            OR (
                status = ? 
                AND julianday('now') - julianday(added_at) >= ?
            )",
        )
        .bind(DownloadStatus::Cancelled.as_str())
        .bind(DownloadStatus::Completed.as_str())
        .bind(hours as f64 / 24.0)
        .bind(DownloadStatus::Failed.as_str())
        .bind((hours * 2) as f64 / 24.0)
        .execute(&self.db.pool)
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn get_next_queue_order(&self, user_id: i32) -> Result<i32, AppError> {
        let result: Option<i32> = sqlx::query_scalar(
            "SELECT COALESCE(MAX(queue_order), -1) + 1 FROM downloads WHERE user_id = ?",
        )
        .bind(user_id)
        .fetch_one(&self.db.pool)
        .await?;

        Ok(result.unwrap_or(0))
    }

    pub async fn update_download_url(
        &self,
        user_id: i32,
        uuid: &str,
        file_url: &str,
    ) -> Result<bool, AppError> {
        let result =
            sqlx::query("UPDATE downloads SET file_url = ? WHERE uuid = ? AND user_id = ?")
                .bind(file_url)
                .bind(uuid)
                .bind(user_id)
                .execute(&self.db.pool)
                .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_max_concurrent_downloads(&self, user_id: i32) -> Result<i32, AppError> {
        let result: Option<i32> =
            sqlx::query_scalar("SELECT max_concurrent_downloads FROM settings WHERE user_id = ?")
                .bind(user_id)
                .fetch_optional(&self.db.pool)
                .await?;

        Ok(result.unwrap_or(5))
    }

    pub async fn get_download_rate_limit(&self, user_id: i32) -> Result<i64, AppError> {
        let rate_limit: i64 =
            sqlx::query_scalar("SELECT download_rate_limit FROM settings WHERE user_id = ?")
                .bind(user_id)
                .fetch_one(&self.db.pool)
                .await?;

        Ok(rate_limit)
    }

    pub async fn update_download_total_bytes(
        &self,
        user_id: i32,
        uuid: &str,
        total_bytes: i64,
    ) -> Result<bool, AppError> {
        let result =
            sqlx::query("UPDATE downloads SET total_bytes = ? WHERE uuid = ? AND user_id = ?")
                .bind(total_bytes)
                .bind(uuid)
                .bind(user_id)
                .execute(&self.db.pool)
                .await?;

        Ok(result.rows_affected() > 0)
    }
}
