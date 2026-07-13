use crate::db::types::Keys;
use crate::db::Database;
use crate::errors::AppError;
use std::sync::Arc;

#[derive(Clone)]
pub struct KeysManager {
    db: Arc<Database>,
}

impl KeysManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_keys(&self) -> Result<Vec<Keys>, AppError> {
        let rows = sqlx::query_as::<_, (i64, String, String, i64)>(
            "SELECT id, plugin, key, expires_at FROM keys ORDER BY expires_at ASC",
        )
        .fetch_all(&self.db.pool)
        .await?;

        let keys: Vec<Keys> = rows
            .into_iter()
            .map(|(id, plugin, key, expires_at)| Keys {
                id,
                plugin,
                key,
                expires_at,
            })
            .collect();

        Ok(keys)
    }

    pub async fn create_key(
        &self,
        plugin: String,
        key: String,
        expires_at: i64,
    ) -> Result<Keys, AppError> {
        let row = sqlx::query_as::<_, (i64, String, String, i64)>(
            "INSERT INTO keys (plugin, key, expires_at)
             VALUES (?, ?, ?)
             RETURNING id, plugin, key, expires_at",
        )
        .bind(&plugin)
        .bind(&key)
        .bind(expires_at)
        .fetch_one(&self.db.pool)
        .await?;

        Ok(Keys {
            id: row.0,
            plugin: row.1,
            key: row.2,
            expires_at: row.3,
        })
    }

    pub async fn update_key(
        &self,
        plugin: Option<String>,
        key: Option<String>,
        expires_at: Option<i64>,
    ) -> Result<Keys, AppError> {
        let plugin =
            plugin.ok_or_else(|| AppError::Validation("Plugin is required".to_string()))?;

        let row = sqlx::query_as::<_, (i64, String, String, i64)>(
            "UPDATE keys
             SET key = COALESCE(?, key),
                 expires_at = COALESCE(?, expires_at)
             WHERE plugin = ?
             RETURNING id, plugin, key, expires_at",
        )
        .bind(key)
        .bind(expires_at)
        .bind(&plugin)
        .fetch_one(&self.db.pool)
        .await?;

        Ok(Keys {
            id: row.0,
            plugin: row.1,
            key: row.2,
            expires_at: row.3,
        })
    }

    pub async fn delete_key(&self, plugin: &str) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM keys WHERE plugin = ?")
            .bind(plugin)
            .execute(&self.db.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "Key with plugin '{}' not found",
                plugin
            )));
        }

        Ok(())
    }
}
