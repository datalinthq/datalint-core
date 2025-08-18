#![cfg_attr(debug_assertions, allow(dead_code))]

use pyo3::prelude::*;
use std::path::PathBuf;

// Internal modules
mod cache;
pub mod db;
mod enums;
pub mod errors;

use crate::cache::create_cache_db;
use crate::enums::{DatasetTask, DatasetType};

/// Create a cache database for a dataset
///
/// Args:
///     cache_path (str): Path where the cache database will be created
///     dataset_type (DatasetType): Type of dataset (YOLO, COCO, etc.)
///     dataset_task (DatasetTask): Task type (detect, segment, etc.)
///
/// Returns:
///     str: Success message with the cache location
///
/// Raises:
///     RuntimeError: If cache creation fails
#[pyfunction]
fn create_cache(
    cache_path: String,
    dataset_type: DatasetType,
    dataset_task: DatasetTask,
) -> PyResult<String> {
    let path = PathBuf::from(&cache_path);
    create_cache_db(&path, &dataset_type, &dataset_task)?;
    Ok(format!("Cache created at: {}", cache_path))
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
