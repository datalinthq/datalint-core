use crate::db::Database;
use crate::enums::{DatasetTask, DatasetType};
use crate::errors::DatalintResult;
use crate::scanner::{insert_images_batch, scan_images};
use std::fs;
use std::path::Path;

/// Creates a cache database with full schema for dataset caching
pub fn create_cache_db(
    cache_path: &Path,
    dataset_path: &Path,
    dataset_type: &DatasetType,
    dataset_task: &DatasetTask,
) -> DatalintResult<usize> {
    // Create parent directories if they don't exist
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Create database and initialize with metadata
    let mut db = Database::open(cache_path)?;
    db.init_cache_metadata(
        dataset_path.to_str().unwrap_or("unknown"),
        dataset_type.as_str(),
        dataset_task.as_str(),
        env!("CARGO_PKG_VERSION"),
    )?;

    // Scan and insert images
    let images = scan_images(dataset_path)?;
    let image_count = images.len();

    if image_count > 0 {
        println!("Found {} images, caching...", image_count);
        insert_images_batch(&mut db, &images, 10000)?;
    }

    Ok(image_count)
}
