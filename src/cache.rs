use crate::db::Database;
use crate::enums::{DatasetTask, DatasetType};
use crate::errors::DatalintResult;
use std::fs;
use std::path::Path;

/// Creates a cache database with full schema
///
/// This creates a DuckDB database with all required tables for dataset caching.
///
/// # Arguments
/// * `cache_path` - Path where the cache database will be created
/// * `dataset_type` - Type of the dataset (YOLO, COCO, etc.)
/// * `dataset_task` - Task type (detect, segment, etc.)
///
/// # Returns
/// * `Ok(())` - If the database and tables were created successfully
/// * `Err(DatalintError)` - If database creation fails
pub fn create_cache_db(
    cache_path: &Path,
    dataset_type: &DatasetType,
    dataset_task: &DatasetTask,
) -> DatalintResult<()> {
    // Create parent directories if they don't exist
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Create database with full schema
    let mut db = Database::open(cache_path)?;

    // Initialize cache metadata
    db.init_cache_metadata(
        cache_path.to_str().unwrap_or("unknown"),
        dataset_type.as_str(),
        dataset_task.as_str(),
        env!("CARGO_PKG_VERSION"),
    )?;

    Ok(())
}
