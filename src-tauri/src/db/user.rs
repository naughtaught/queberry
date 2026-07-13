use crate::db::types::{CreateUserData, UpdateUserData, User};
use crate::db::Database;
use crate::errors::AppError;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct UserManager {
    db: Arc<Database>,
    pin_attempts: Arc<Mutex<HashMap<i32, PinAttemptTracker>>>,
}

struct PinAttemptTracker {
    attempts: u32,
    first_attempt: Instant,
    locked_until: Option<Instant>,
}

impl PinAttemptTracker {
    fn new() -> Self {
        Self {
            attempts: 0,
            first_attempt: Instant::now(),
            locked_until: None,
        }
    }
}

impl UserManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            pin_attempts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn get_user(&self, user_id: i32) -> Result<Option<User>, AppError> {
        self.query_user_from_db(user_id).await
    }

    pub async fn get_user_by_postgres_id(
        &self,
        postgres_id: &str,
    ) -> Result<Option<User>, AppError> {
        let row = sqlx::query_as::<_, (i32, String, String, Option<String>, Option<String>, String, Option<String>, String)>(
            "SELECT id, username, email, avatar, pin, postgres_id, token, updated_at FROM users WHERE postgres_id = ?"
        )
        .bind(postgres_id)
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            Some((id, username, email, avatar, pin, postgres_id, token, updated_at)) => {
                Ok(Some(User {
                    id,
                    username,
                    email,
                    avatar,
                    pin,
                    postgres_id,
                    token,
                    updated_at,
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        let rows = sqlx::query_as::<_, (i32, String, String, Option<String>, Option<String>, String, Option<String>, String)>(
            "SELECT id, username, email, avatar, pin, postgres_id, token, updated_at FROM users ORDER BY id"
        )
        .fetch_all(&self.db.pool)
        .await?;

        let users: Vec<User> = rows
            .into_iter()
            .map(
                |(id, username, email, avatar, pin, postgres_id, token, updated_at)| User {
                    id,
                    username,
                    email,
                    avatar,
                    pin,
                    postgres_id,
                    token,
                    updated_at,
                },
            )
            .collect();

        Ok(users)
    }

    pub async fn create_user(&self, user_data: CreateUserData) -> Result<User, AppError> {
        if let Some(existing) = self.get_user_by_postgres_id(&user_data.postgres_id).await? {
            return Ok(existing);
        }

        let hashed_pin = user_data
            .pin
            .as_deref()
            .map(|p| hash(p, DEFAULT_COST))
            .transpose()
            .map_err(|e| AppError::Runtime(e.to_string()))?;

        let row = sqlx::query_as::<
            _,
            (
                i32,
                String,
                String,
                Option<String>,
                Option<String>,
                String,
                Option<String>,
                String,
            ),
        >(
            "INSERT INTO users (username, email, avatar, pin, postgres_id, token)
         VALUES (?, ?, ?, ?, ?, ?)
         RETURNING id, username, email, avatar, pin, postgres_id, token, updated_at",
        )
        .bind(&user_data.username)
        .bind(&user_data.email)
        .bind(&user_data.avatar)
        .bind(hashed_pin)
        .bind(&user_data.postgres_id)
        .bind(&user_data.token)
        .fetch_one(&self.db.pool)
        .await?;

        Ok(User {
            id: row.0,
            username: row.1,
            email: row.2,
            avatar: row.3,
            pin: row.4,
            postgres_id: row.5,
            token: row.6,
            updated_at: row.7,
        })
    }

    pub async fn update_user(&self, updates: UpdateUserData) -> Result<User, AppError> {
        if self.get_user(updates.user_id).await?.is_none() {
            return Err(AppError::NotFound(format!(
                "User with id {} not found",
                updates.user_id
            )));
        }

        let hashed_pin = updates
            .pin
            .as_deref()
            .map(|p| hash(p, DEFAULT_COST))
            .transpose()
            .map_err(|e| AppError::Runtime(e.to_string()))?;

        let row = sqlx::query_as::<
            _,
            (
                i32,
                String,
                String,
                Option<String>,
                Option<String>,
                String,
                Option<String>,
                String,
            ),
        >(
            "UPDATE users
         SET username = COALESCE(?, username),
             email = COALESCE(?, email),
             avatar = COALESCE(?, avatar),
             pin = COALESCE(?, pin),
             token = COALESCE(?, token),
             updated_at = CURRENT_TIMESTAMP
         WHERE id = ?
         RETURNING id, username, email, avatar, pin, postgres_id, token, updated_at",
        )
        .bind(&updates.username)
        .bind(&updates.email)
        .bind(&updates.avatar)
        .bind(hashed_pin)
        .bind(&updates.token)
        .bind(updates.user_id)
        .fetch_one(&self.db.pool)
        .await?;

        Ok(User {
            id: row.0,
            username: row.1,
            email: row.2,
            avatar: row.3,
            pin: row.4,
            postgres_id: row.5,
            token: row.6,
            updated_at: row.7,
        })
    }

    pub async fn delete_user(&self, user_id: i32) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(user_id)
            .execute(&self.db.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_all_content_ratings(&self) -> Result<u64, AppError> {
        let result = sqlx::query("DELETE FROM user_content_ratings")
            .execute(&self.db.pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn verify_pin(&self, user_id: i32, pin: &str) -> Result<bool, AppError> {
        const MAX_ATTEMPTS: u32 = 5;
        const ATTEMPT_WINDOW: Duration = Duration::from_secs(300); // 5 minutes
        const LOCKOUT_DURATION: Duration = Duration::from_secs(900); // 15 minutes

        {
            let mut attempts = self
                .pin_attempts
                .lock()
                .map_err(|e| AppError::Runtime(format!("Lock error: {}", e)))?;

            let tracker = attempts
                .entry(user_id)
                .or_insert_with(PinAttemptTracker::new);

            if let Some(locked_until) = tracker.locked_until {
                if Instant::now() < locked_until {
                    let remaining = locked_until.duration_since(Instant::now());
                    return Err(AppError::Validation(format!(
                        "Too many PIN attempts. Try again in {} minutes {} seconds.",
                        remaining.as_secs() / 60,
                        remaining.as_secs() % 60
                    )));
                } else {
                    *tracker = PinAttemptTracker::new();
                }
            }

            if tracker.first_attempt.elapsed() > ATTEMPT_WINDOW {
                *tracker = PinAttemptTracker::new();
            }

            tracker.attempts += 1;

            if tracker.attempts > MAX_ATTEMPTS {
                tracker.locked_until = Some(Instant::now() + LOCKOUT_DURATION);
                return Err(AppError::Validation(format!(
                    "Too many failed PIN attempts. Account locked for {} minutes.",
                    LOCKOUT_DURATION.as_secs() / 60
                )));
            }
        }

        let user = self
            .get_user(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found", user_id)))?;

        let is_valid = match user.pin {
            None => false,
            Some(hashed) => verify(pin, &hashed).map_err(|e| AppError::Runtime(e.to_string()))?,
        };

        if is_valid {
            let mut attempts = self
                .pin_attempts
                .lock()
                .map_err(|e| AppError::Runtime(format!("Lock error: {}", e)))?;
            attempts.remove(&user_id);
        }

        Ok(is_valid)
    }

    async fn query_user_from_db(&self, user_id: i32) -> Result<Option<User>, AppError> {
        let row = sqlx::query_as::<_, (i32, String, String, Option<String>, Option<String>, String, Option<String>, String)>(
            "SELECT id, username, email, avatar, pin, postgres_id, token, updated_at FROM users WHERE id = ?"
        )
        .bind(user_id)
        .fetch_optional(&self.db.pool)
        .await?;

        Ok(row.map(
            |(id, username, email, avatar, pin, postgres_id, token, updated_at)| User {
                id,
                username,
                email,
                avatar,
                pin,
                postgres_id,
                token,
                updated_at,
            },
        ))
    }
}
