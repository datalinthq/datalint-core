#![cfg_attr(debug_assertions, allow(dead_code))]

use pyo3::prelude::*;
use std::path::PathBuf;

// Internal modules
pub mod cache;
pub mod db;
pub mod enums;
pub mod errors;
pub mod scanner;

use crate::cache::create_cache_db;
use crate::enums::{DatasetTask, DatasetType};

/// Create a cache database for a dataset
///
/// Args:
///     cache_path (str): Path where the cache database will be created
///     dataset_path (str): Path to the dataset directory to scan
///     dataset_type (DatasetType): Type of dataset (YOLO, COCO, etc.)
///     dataset_task (DatasetTask): Task type (detect, segment, etc.)
///
/// Returns:
///     str: Success message with the cache location and image count
///
/// Raises:
///     RuntimeError: If cache creation fails
#[pyfunction]
fn create_cache(
    cache_path: String,
    dataset_path: String,
    dataset_type: DatasetType,
    dataset_task: DatasetTask,
) -> PyResult<String> {
    let cache = PathBuf::from(&cache_path);
    let dataset = PathBuf::from(&dataset_path);
    let image_count = create_cache_db(&cache, &dataset, &dataset_type, &dataset_task)?;
    Ok(format!(
        "Cache created at: {} with {} images",
        cache_path, image_count
    ))
}

/// Datalint Core Python module
#[pymodule(gil_used = false)]
mod _datalint_core {
    use super::*;

    // Export functions and classes
    #[pymodule_export]
    use crate::{create_cache, DatasetTask, DatasetType};

    // Module initialization
    #[pymodule_init]
    fn module_init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add("__version__", env!("CARGO_PKG_VERSION"))?;
        Ok(())
    }
}
