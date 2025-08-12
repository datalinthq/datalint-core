use rusqlite::{Connection, Result as SqliteResult};
use std::fs;
use std::path::Path;

use crate::errors::{DatalintError, DatalintResult};

/// Creates a cache database with a basic metadata table
///
/// This creates an SQLite database with a simple table structure
/// that can be expanded in future iterations.
///
/// # Arguments
/// * `cache_path` - Path where the cache database will be created
///
/// # Returns
/// * `Ok(())` - If the database and table were created successfully
/// * `Err(DatalintError)` - If database creation fails
pub fn create_cache_db(cache_path: &Path) -> DatalintResult<()> {
    // Create parent directories if they don't exist
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Create or open the SQLite database
    let conn = Connection::open(cache_path)
        .map_err(|e| DatalintError::Generic(format!("Failed to create database: {}", e)))?;

    // Create a simple metadata table
    // This is a minimal starting point that can be expanded
    conn.execute(
        "CREATE TABLE IF NOT EXISTS dataset_metadata (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL UNIQUE,
            value TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .map_err(|e| DatalintError::Generic(format!("Failed to create table: {}", e)))?;

    // Set some pragmas for better performance
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA synchronous = NORMAL;",
    )
    .map_err(|e| DatalintError::Generic(format!("Failed to set pragmas: {}", e)))?;

    Ok(())
}
