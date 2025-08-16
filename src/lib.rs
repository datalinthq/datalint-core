#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use pyo3::prelude::*;
use std::path::PathBuf;

// Internal modules
mod cache;
mod enums;
mod errors;

use crate::cache::create_cache_db;
use crate::enums::{DatasetTask, DatasetType};
use crate::errors::DatalintResult;

/// Get the version of datalint-core
fn get_datalint_core_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Create a cache database for a dataset at the specified path
///
/// Args:
///     cache_path (str): Path where the cache database will be created
///
/// Returns:
///     str: Success message indicating where the database was created
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

    // Create the cache database
    create_cache_db(&path, &dataset_type, &dataset_task)?;

    // Return success message
    Ok(format!("Cache created at: {}", cache_path))
}

/// Main Python module definition
///
/// This follows pydantic-core's pattern of having a clean module initialization
/// with version information and exported functions.
#[pymodule]
#[pyo3(name = "_datalint_core")]
fn datalint_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add version information
    m.add("__version__", get_datalint_core_version())?;

    // Add functions
    m.add_function(wrap_pyfunction!(py_create_cache, m)?)?;

    // Add enum classes
    m.add_class::<DatasetTask>()?;
    m.add_class::<DatasetType>()?;

    Ok(())
}
