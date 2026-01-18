use crate::db::migrations::get_all_migrations;
use crate::db::Database;
use crate::utils::db_dir::get_db_dir;
use std::path::PathBuf;

pub fn init_db() -> Result<Database, Box<dyn std::error::Error>> {
    let db_dir = get_db_dir()?;
    let db_path = db_dir.join("app.db");

    let db = Database::open(&db_path)?;

    db.initialize()?;

    let current_version = db.get_current_version()?;

    for migration in get_all_migrations() {
        if migration.version > current_version {
            println!(
                "Applying migration v{}: {}",
                migration.version, migration.description
            );
            db.apply_migration(&migration)?;
        }
    }

    println!("Database initialized");

    Ok(db)
}

pub fn get_db_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let db_dir = get_db_dir()?;
    Ok(db_dir.join("app.db"))
}
