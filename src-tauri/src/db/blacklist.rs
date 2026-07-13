use crate::db::types::Blacklist;
use crate::db::Database;
use crate::errors::AppError;
use std::sync::Arc;

#[derive(Clone)]
pub struct BlacklistManager {
    db: Arc<Database>,
}

impl BlacklistManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_users_blacklisted(&self, user_id: i32) -> Result<Vec<Blacklist>, AppError> {
        let rows = sqlx::query_as::<_, (i64, i32, String)>(
            "SELECT id, user_id, hash FROM blacklist WHERE user_id = ?",
        )
        .bind(user_id)
        .fetch_all(&self.db.pool)
        .await?;

        let blacklist: Vec<Blacklist> = rows
            .into_iter()
            .map(|(id, user_id, hash)| Blacklist { id, user_id, hash })
            .collect();

        Ok(blacklist)
    }

    pub async fn create_blacklisted_hash(
        &self,
        user_id: i32,
        hash: String,
    ) -> Result<Blacklist, AppError> {
        let row = sqlx::query_as::<_, (i64, i32, String)>(
            "INSERT INTO blacklist (user_id, hash) VALUES (?, ?) RETURNING id, user_id, hash",
        )
        .bind(user_id)
        .bind(&hash)
        .fetch_one(&self.db.pool)
        .await?;

        Ok(Blacklist {
            id: row.0,
            user_id: row.1,
            hash: row.2,
        })
    }

    pub async fn delete_users_blacklisted(&self, user_id: i32) -> Result<(), AppError> {
        sqlx::query("DELETE FROM blacklist WHERE user_id = ?")
            .bind(user_id)
            .execute(&self.db.pool)
            .await?;
        Ok(())
    }
}
