"""
Datalint Core - Rust-powered backend for dataset inspection and quality control.

This module provides high-performance functions for dataset processing,
implemented in Rust with Python bindings via PyO3.
"""

from __future__ import annotations

from ._datalint_core import (
    DatasetTask,
    DatasetType,
    __version__,
    create_cache,
)

__all__ = [
    "DatasetTask",
    "DatasetType",
    "__version__",
    "create_cache",
]
