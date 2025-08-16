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
#[pyo3(name = "create_cache")]
fn py_create_cache(
    cache_path: String,
    dataset_type: DatasetType,
    dataset_task: DatasetTask,
) -> PyResult<String> {
    let path = PathBuf::from(&cache_path);
    create_cache_db(&path, &dataset_type, &dataset_task)?;
    Ok(format!("Cache created at: {}", cache_path))
}

/// Datalint Core Python module
#[pymodule]
#[pyo3(name = "_datalint_core")]
fn datalint_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add version information
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Add functions
    m.add_function(wrap_pyfunction!(py_create_cache, m)?)?;

    // Add enum classes
    m.add_class::<DatasetTask>()?;
    m.add_class::<DatasetType>()?;

    Ok(())
}
