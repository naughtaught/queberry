use rusqlite::{params, Connection, Params, Result as SqlResult};
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

#[derive(Debug)]
pub struct Migration {
    pub version: i32,
    pub description: String,
    pub sql: String,
}

impl Database {
    /// Open or create a SQLite database at the given path
    pub fn open<P: AsRef<Path>>(path: P) -> SqlResult<Self> {
        let conn = Connection::open(path)?;
        Ok(Database {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Execute a SQL statement with optional parameters
    pub fn execute<P: Params>(&self, sql: &str, params: P) -> SqlResult<usize> {
        let conn = self.conn.lock().unwrap();
        conn.execute(sql, params)
    }

    /// Query the database and return rows
    pub fn query<T, F, P: Params>(&self, sql: &str, params: P, mapper: F) -> SqlResult<Vec<T>>
    where
        F: Fn(&rusqlite::Row) -> rusqlite::Result<T>,
    {
        let conn = self.conn.lock().unwrap(); // Lock here
        let mut stmt = conn.prepare(sql)?; // Use conn.prepare
        let rows = stmt.query_map(params, mapper)?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }

        Ok(results)
    }

    /// Query a single row from the database
    pub fn query_row<T, F, P: Params>(&self, sql: &str, params: P, mapper: F) -> SqlResult<T>
    where
        F: FnOnce(&rusqlite::Row) -> rusqlite::Result<T>,
    {
        let conn = self.conn.lock().unwrap(); // Lock here
        conn.query_row(sql, params, mapper)
    }

    /// Check if a table exists
    pub fn table_exists(&self, table_name: &str) -> SqlResult<bool> {
        let sql = "SELECT count(*) FROM sqlite_master WHERE type='table' AND name=?";
        let conn = self.conn.lock().unwrap(); // Lock here
        let count: i64 = conn.query_row(sql, params![table_name], |row| row.get(0))?;
        Ok(count > 0)
    }

    /// Initialize the database with migrations table if needed
    pub fn initialize(&self) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS migrations (
                version INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        Ok(())
    }

    /// Apply a migration (requires mutable self for transaction)
    pub fn apply_migration(&self, migration: &Migration) -> SqlResult<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;

        tx.execute_batch(&migration.sql)?;

        tx.execute(
            "INSERT INTO migrations (version, description) VALUES (?, ?)",
            params![migration.version, migration.description],
        )?;

        tx.commit()?;
        Ok(())
    }

    /// Get current database version (highest applied migration)
    pub fn get_current_version(&self) -> SqlResult<i32> {
        if !self.table_exists("migrations")? {
            return Ok(0);
        }

        let conn = self.conn.lock().unwrap(); // Lock here
        let version = conn.query_row(
            "SELECT COALESCE(MAX(version), 0) FROM migrations",
            [],
            |row| row.get::<_, i32>(0),
        )?;

        Ok(version)
    }
}
