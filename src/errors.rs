use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use std::fmt;

/// Core error type for datalint operations
#[derive(Debug)]
pub enum DatalintError {
    Io(std::io::Error),
    Database(String),
    Generic(String),
}

impl fmt::Display for DatalintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "IO error: {}", err),
            Self::Database(err) => write!(f, "Database error: {}", err),
            Self::Generic(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<std::io::Error> for DatalintError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<duckdb::Error> for DatalintError {
    fn from(err: duckdb::Error) -> Self {
        Self::Database(err.to_string())
    }
}

impl From<DatalintError> for PyErr {
    fn from(err: DatalintError) -> PyErr {
        PyRuntimeError::new_err(err.to_string())
    }
}

pub type DatalintResult<T> = Result<T, DatalintError>;
