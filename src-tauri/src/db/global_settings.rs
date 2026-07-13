use crate::db::types::{GlobalSettings, UpdateGlobalSettings};
use crate::db::Database;
use crate::errors::AppError;
use std::sync::Arc;

#[derive(Clone)]
pub struct GlobalSettingsManager {
    db: Arc<Database>,
}

impl GlobalSettingsManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_global_settings(&self) -> Result<GlobalSettings, AppError> {
        let row = sqlx::query_as::<_, (i32, bool, Option<i32>, Option<String>, Option<String>)>(
            "SELECT id, parental_controls_are_enabled, primary_user_id, tv_directory, movie_directory
             FROM global_settings WHERE id = 1"
        )
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            Some(r) => Ok(GlobalSettings {
                id: r.0,
                parental_controls_are_enabled: r.1,
                primary_user_id: r.2,
                tv_directory: r.3,
                movie_directory: r.4,
            }),
            None => {
                sqlx::query(
                    "INSERT OR IGNORE INTO global_settings (id, parental_controls_are_enabled, primary_user_id, tv_directory, movie_directory)
                     VALUES (1, 0, NULL, NULL, NULL)"
                )
                .execute(&self.db.pool)
                .await?;

                Ok(GlobalSettings {
                    id: 1,
                    parental_controls_are_enabled: false,
                    primary_user_id: None,
                    tv_directory: None,
                    movie_directory: None,
                })
            }
        }
    }

    pub async fn update_global_settings(
        &self,
        updates: UpdateGlobalSettings,
    ) -> Result<GlobalSettings, AppError> {

        let row = sqlx::query_as::<_, (i32, bool, Option<i32>, Option<String>, Option<String>)>(
        "UPDATE global_settings
         SET parental_controls_are_enabled = ?,
             primary_user_id = ?,
             tv_directory = ?,
             movie_directory = ?
         WHERE id = 1
         RETURNING id, parental_controls_are_enabled, primary_user_id, tv_directory, movie_directory"
    )
    .bind(updates.parental_controls_are_enabled)
    .bind(updates.primary_user_id)
    .bind(updates.tv_directory)
    .bind(updates.movie_directory)
    .fetch_one(&self.db.pool)
    .await?;

        Ok(GlobalSettings {
            id: row.0,
            parental_controls_are_enabled: row.1,
            primary_user_id: row.2,
            tv_directory: row.3,
            movie_directory: row.4,
        })
    }
}
