use crate::db::types::UserContentRatings;
use crate::db::Database;
use crate::errors::AppError;
use std::sync::Arc;

#[derive(Clone)]
pub struct ContentRatingsManager {
    db: Arc<Database>,
}

impl ContentRatingsManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_user_content_ratings(
        &self,
        user_id: i32,
    ) -> Result<Option<UserContentRatings>, AppError> {
        let row = sqlx::query_as::<_, (i32, String, String)>(
            "SELECT user_id, restricted_movies, restricted_tv FROM user_content_ratings WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            None => Ok(None),
            Some((uid, movies_json, tv_json)) => {
                let ratings = UserContentRatings {
                    user_id: uid,
                    restricted_movies: serde_json::from_str(&movies_json).unwrap_or_default(),
                    restricted_tv: serde_json::from_str(&tv_json).unwrap_or_default(),
                };
                Ok(Some(ratings))
            }
        }
    }

    pub async fn upsert_user_content_ratings(
        &self,
        user_id: i32,
        restricted_movies: Vec<String>,
        restricted_tv: Vec<String>,
    ) -> Result<UserContentRatings, AppError> {
        let movies_json = serde_json::to_string(&restricted_movies)
            .map_err(|e| AppError::Runtime(e.to_string()))?;
        let tv_json =
            serde_json::to_string(&restricted_tv).map_err(|e| AppError::Runtime(e.to_string()))?;

        let row = sqlx::query_as::<_, (i32, String, String)>(
            "INSERT INTO user_content_ratings (user_id, restricted_movies, restricted_tv)
             VALUES (?, ?, ?)
             ON CONFLICT(user_id) DO UPDATE SET
                restricted_movies = excluded.restricted_movies,
                restricted_tv = excluded.restricted_tv
             RETURNING user_id, restricted_movies, restricted_tv",
        )
        .bind(user_id)
        .bind(&movies_json)
        .bind(&tv_json)
        .fetch_one(&self.db.pool)
        .await?;

        Ok(UserContentRatings {
            user_id: row.0,
            restricted_movies: serde_json::from_str(&row.1).unwrap_or_default(),
            restricted_tv: serde_json::from_str(&row.2).unwrap_or_default(),
        })
    }

    pub async fn delete_user_content_ratings(&self, user_id: i32) -> Result<(), AppError> {
        sqlx::query("DELETE FROM user_content_ratings WHERE user_id = ?")
            .bind(user_id)
            .execute(&self.db.pool)
            .await?;
        Ok(())
    }
}
