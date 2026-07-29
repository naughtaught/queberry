use crate::db::types::ResolvedCache;
use crate::db::Database;
use crate::errors::AppError;
use std::sync::Arc;

#[derive(Clone)]
pub struct ResolvedCacheManager {
    db: Arc<Database>,
}

impl ResolvedCacheManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_by_imdb_id(&self, imdb_id: &str) -> Result<Vec<ResolvedCache>, AppError> {
        let rows = sqlx::query_as::<_, (i64, String, String, String, String, i64)>(
            "SELECT id, imdb_id, infohash, plugin_id, files_json, created_at
             FROM resolved_cache
             WHERE imdb_id = ?
             ORDER BY created_at DESC",
        )
        .bind(imdb_id)
        .fetch_all(&self.db.pool)
        .await?;

        let results: Vec<ResolvedCache> = rows
            .into_iter()
            .map(
                |(id, imdb_id, infohash, plugin_id, files_json, created_at)| ResolvedCache {
                    id,
                    imdb_id,
                    infohash,
                    plugin_id,
                    files_json,
                    created_at,
                },
            )
            .collect();

        Ok(results)
    }

    pub async fn create(
        &self,
        imdb_id: String,
        infohash: String,
        plugin_id: String,
        files_json: String,
    ) -> Result<ResolvedCache, AppError> {
        let row = sqlx::query_as::<_, (i64, String, String, String, String, i64)>(
            "INSERT INTO resolved_cache (imdb_id, infohash, plugin_id, files_json)
             VALUES (?, ?, ?, ?)
             ON CONFLICT(imdb_id, infohash, plugin_id) DO UPDATE SET
                files_json = excluded.files_json,
                created_at = unixepoch()
             RETURNING id, imdb_id, infohash, plugin_id, files_json, created_at",
        )
        .bind(&imdb_id)
        .bind(&infohash)
        .bind(&plugin_id)
        .bind(&files_json)
        .fetch_one(&self.db.pool)
        .await?;

        Ok(ResolvedCache {
            id: row.0,
            imdb_id: row.1,
            infohash: row.2,
            plugin_id: row.3,
            files_json: row.4,
            created_at: row.5,
        })
    }

    pub async fn delete_all(&self) -> Result<(), AppError> {
        sqlx::query("DELETE FROM resolved_cache")
            .execute(&self.db.pool)
            .await?;

        Ok(())
    }

    pub async fn delete_by_infohash(&self, infohash: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM resolved_cache WHERE infohash = ?")
            .bind(infohash)
            .execute(&self.db.pool)
            .await?;

        Ok(())
    }
}
