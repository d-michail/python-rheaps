//! Conversions from `rheaps` error types to Python exceptions.

use pyo3::exceptions::PyValueError;
use pyo3::PyErr;

use rheaps::array::InvalidDegree;
use rheaps::error::{DecreaseKeyError, IncreaseKeyError, InvalidHandle};
use rheaps::monotone::{NonFiniteF64, RadixDecreaseKeyError, RadixHeapError};
use rheaps::tree::{InvalidBranchingFactor, SoftHeapError, SoftMeldError};

pub fn invalid_handle(error: InvalidHandle) -> PyErr {
    PyValueError::new_err(error.to_string())
}

pub fn decrease_key_error(error: DecreaseKeyError) -> PyErr {
    PyValueError::new_err(error.to_string())
}

pub fn increase_key_error(error: IncreaseKeyError) -> PyErr {
    PyValueError::new_err(error.to_string())
}

pub fn radix_heap_error(error: RadixHeapError) -> PyErr {
    PyValueError::new_err(error.to_string())
}

pub fn radix_decrease_key_error(error: RadixDecreaseKeyError) -> PyErr {
    PyValueError::new_err(error.to_string())
}

pub fn non_finite_f64(error: NonFiniteF64) -> PyErr {
    PyValueError::new_err(error.to_string())
}

pub fn invalid_degree(error: InvalidDegree) -> PyErr {
    PyValueError::new_err(error.to_string())
}

pub fn invalid_branching_factor(error: InvalidBranchingFactor) -> PyErr {
    PyValueError::new_err(error.to_string())
}

pub fn soft_heap_error(error: SoftHeapError) -> PyErr {
    PyValueError::new_err(error.to_string())
}

pub fn soft_meld_error(error: SoftMeldError) -> PyErr {
    PyValueError::new_err(error.to_string())
}
