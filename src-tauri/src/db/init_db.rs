use crate::db::migrations::get_all_migrations;
use crate::db::Database;
use crate::utils::db_dir::get_db_dir;
use std::path::PathBuf;
use tauri::AppHandle;

pub async fn init_db(app_handle: &AppHandle) -> Result<Database, Box<dyn std::error::Error>> {
    let db_dir = get_db_dir(app_handle)?;
    let db_path = db_dir.join("queberry.db");

    let db = Database::open(&db_path).await?;

    db.initialize().await?;

    let mut migrations = get_all_migrations();

    migrations.sort_by_key(|m| m.version);

    validate_migrations(&migrations)?;

    let current_version = db.get_current_version().await?;

    for migration in migrations {
        if migration.version > current_version {
            db.apply_migration(&migration).await?;
        }
    }

    Ok(db)
}

fn validate_migrations(
    migrations: &[crate::db::Migration],
) -> Result<(), Box<dyn std::error::Error>> {
    if migrations.is_empty() {
        return Ok(());
    }

    if migrations[0].version != 1 {
        return Err(format!(
            "Invalid migration sequence: First migration must be version 1, found version {}",
            migrations[0].version
        )
        .into());
    }

    for window in migrations.windows(2) {
        let current = window[0].version;
        let next = window[1].version;

        if next == current {
            return Err(format!(
                "Duplicate migration version detected: version {} appears more than once",
                current
            )
            .into());
        }

        if next != current + 1 {
            return Err(format!(
                "Migration gap detected: version {} exists but version {} is missing. \
                 Migrations must form a contiguous sequence (1, 2, 3, ...). \
                 Check for missing or misnumbered migration files.",
                current,
                current + 1
            )
            .into());
        }
    }

    Ok(())
}

pub fn get_db_path(app_handle: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let db_dir = get_db_dir(app_handle)?;
    Ok(db_dir.join("queberry.db"))
}
