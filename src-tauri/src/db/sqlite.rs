use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: SqlitePool,
}

#[derive(Debug)]
pub struct Migration {
    pub version: i32,
    pub description: String,
    pub sql: String,
}

impl Database {
    pub async fn open<P: AsRef<Path>>(path: P) -> Result<Self, sqlx::Error> {
        let path_str = path.as_ref().to_str().expect("Invalid DB path");
        let options = SqliteConnectOptions::from_str(&format!("sqlite:{}", path_str))?
            .create_if_missing(true)
            .foreign_keys(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .busy_timeout(std::time::Duration::from_secs(5));

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        Ok(Database { pool })
    }

    pub async fn initialize(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS migrations (
                version INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_current_version(&self) -> Result<i32, sqlx::Error> {
        let row: (i32,) = sqlx::query_as("SELECT COALESCE(MAX(version), 0) FROM migrations")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0)
    }

    pub async fn apply_migration(&self, migration: &Migration) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(&migration.sql).execute(&mut *tx).await?;
        sqlx::query("INSERT INTO migrations (version, description) VALUES (?, ?)")
            .bind(migration.version)
            .bind(&migration.description)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn run_migrations(&self, mut migrations: Vec<Migration>) -> Result<(), sqlx::Error> {
        migrations.sort_by_key(|m| m.version);

        self.validate_migration_sequence(&migrations);

        let current_version = self.get_current_version().await?;

        for migration in migrations {
            if migration.version > current_version {
                println!(
                    "Applying migration v{}: {}",
                    migration.version, migration.description
                );
                self.apply_migration(&migration).await?;
            }
        }

        Ok(())
    }

    fn validate_migration_sequence(&self, migrations: &[Migration]) {
        if migrations.is_empty() {
            return;
        }

        debug_assert!(
            migrations[0].version == 1,
            "First migration must be version 1, found version {}",
            migrations[0].version
        );

        for window in migrations.windows(2) {
            let current = window[0].version;
            let next = window[1].version;

            debug_assert!(
                next == current + 1,
                "Migration gap detected: version {} exists but version {} is missing. \
                 Migrations must be contiguous.",
                current,
                current + 1
            );
        }

        let has_duplicates = migrations.windows(2).any(|w| w[0].version == w[1].version);

        debug_assert!(!has_duplicates, "Duplicate migration versions detected!");
    }
}
