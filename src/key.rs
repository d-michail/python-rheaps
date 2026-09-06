//! A key wrapper letting arbitrary Python objects be used as heap keys,
//! ordered via their own `__lt__`/`__eq__`.
//!
//! `Ord::cmp` cannot be fallible, but a Python rich comparison can raise (for
//! example, comparing incomparable types). When that happens we stash the
//! `PyErr` in a thread-local sidecar and return a value that keeps the
//! surrounding Rust heap's invariants intact (an arbitrary but consistent
//! ordering for that one call); every pymethod that performs a heap operation
//! must call `check_comparison_error` afterward and propagate it if set, so
//! the Python-visible outcome is still the original exception, not a
//! corrupted heap silently accepting bad data.

use std::cell::RefCell;
use std::cmp::Ordering;

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyFloat, PyInt, PyType};

use rheaps::monotone::FiniteF64;

/// Which concrete key representation a generic heap class was constructed
/// with, chosen once at `__new__` time via `key_type=int|float|object`
/// (default `object`). `Int`/`Float` pick a native `i64`/[`FiniteF64`]
/// monomorphization with zero per-comparison GIL callback; `Object` accepts
/// arbitrary Python objects compared through [`PyObjectKey`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyType {
    Int,
    Float,
    Object,
}

/// Parses the `key_type` constructor argument: Python's own `int`/`float`
/// type objects, the strings `"int"`/`"float"`/`"object"`, or `None`
/// (defaulting to `Object`).
pub fn parse_key_type(py: Python<'_>, key_type: Option<Bound<'_, PyAny>>) -> PyResult<KeyType> {
    let Some(value) = key_type else {
        return Ok(KeyType::Object);
    };
    if let Ok(type_obj) = value.cast::<PyType>() {
        if type_obj.is(&py.get_type::<PyInt>()) {
            return Ok(KeyType::Int);
        }
        if type_obj.is(&py.get_type::<PyFloat>()) {
            return Ok(KeyType::Float);
        }
        if type_obj.is(&py.get_type::<PyAny>()) {
            return Ok(KeyType::Object);
        }
    }
    if let Ok(name) = value.extract::<String>() {
        return match name.as_str() {
            "int" => Ok(KeyType::Int),
            "float" => Ok(KeyType::Float),
            "object" => Ok(KeyType::Object),
            other => Err(PyValueError::new_err(format!(
                "unknown key_type {other:?}; expected int, float, or object"
            ))),
        };
    }
    Err(PyTypeError::new_err(
        "key_type must be int, float, object, a matching type name, or None",
    ))
}

/// Converts an incoming Python key to an `i64` for an `Int`-keyed heap,
/// raising `TypeError` if it isn't an integer.
pub fn to_i64(key: &Bound<'_, PyAny>) -> PyResult<i64> {
    key.extract()
}

/// Converts an incoming Python key to a [`FiniteF64`] for a `Float`-keyed
/// heap, raising `TypeError`/`ValueError` if it isn't a finite float.
pub fn to_finite_f64(key: &Bound<'_, PyAny>) -> PyResult<FiniteF64> {
    let value: f64 = key.extract()?;
    FiniteF64::new(value).map_err(crate::error::non_finite_f64)
}

/// Converts an `i64` key/value back to a Python object.
pub fn i64_to_py(py: Python<'_>, value: i64) -> Py<PyAny> {
    value.into_pyobject(py).unwrap().into_any().unbind()
}

/// Converts a [`FiniteF64`] key back to a Python `float`.
pub fn finite_f64_to_py(py: Python<'_>, value: FiniteF64) -> Py<PyAny> {
    let value: f64 = value.into();
    value.into_pyobject(py).unwrap().into_any().unbind()
}

/// Converts an incoming Python key to a `u32` for a `U32RadixHeap`,
/// raising `TypeError`/`OverflowError` if it isn't a `u32`-sized integer.
pub fn to_u32(key: &Bound<'_, PyAny>) -> PyResult<u32> {
    key.extract()
}

/// Converts a `u32` radix key back to a Python `int`.
pub fn u32_to_py(py: Python<'_>, value: u32) -> Py<PyAny> {
    value.into_pyobject(py).unwrap().into_any().unbind()
}

/// Converts an incoming Python key to a `u64` for a `U64RadixHeap`,
/// raising `TypeError`/`OverflowError` if it isn't a `u64`-sized integer.
pub fn to_u64(key: &Bound<'_, PyAny>) -> PyResult<u64> {
    key.extract()
}

/// Converts a `u64` radix key back to a Python `int`.
pub fn u64_to_py(py: Python<'_>, value: u64) -> Py<PyAny> {
    value.into_pyobject(py).unwrap().into_any().unbind()
}

/// Converts an incoming Python key to a [`num_bigint::BigUint`] for a
/// `BigUintRadixHeap`, raising `TypeError`/`OverflowError` if it isn't a
/// non-negative integer.
pub fn to_biguint(key: &Bound<'_, PyAny>) -> PyResult<num_bigint::BigUint> {
    key.extract()
}

/// Converts a [`num_bigint::BigUint`] radix key back to a Python `int`.
pub fn biguint_to_py(py: Python<'_>, value: num_bigint::BigUint) -> Py<PyAny> {
    value.into_pyobject(py).unwrap().into_any().unbind()
}

thread_local! {
    static COMPARISON_ERROR: RefCell<Option<PyErr>> = const { RefCell::new(None) };
}

fn set_comparison_error(err: PyErr) {
    COMPARISON_ERROR.with(|cell| {
        let mut slot = cell.borrow_mut();
        if slot.is_none() {
            *slot = Some(err);
        }
    });
}

/// Takes and clears any comparison error raised since the last check.
///
/// Call this after every heap operation that might have compared
/// [`PyObjectKey`] values, and propagate the error if one is returned.
pub fn check_comparison_error() -> PyResult<()> {
    let error = COMPARISON_ERROR.with(|cell| cell.borrow_mut().take());
    match error {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

/// A heap key backed by an arbitrary Python object, ordered via `__lt__`.
pub struct PyObjectKey(pub Py<PyAny>);

impl PyObjectKey {
    pub fn new(object: Py<PyAny>) -> Self {
        Self(object)
    }

    pub fn into_inner(self) -> Py<PyAny> {
        self.0
    }
}

impl Clone for PyObjectKey {
    fn clone(&self) -> Self {
        Python::attach(|py| Self(self.0.clone_ref(py)))
    }
}

impl PartialEq for PyObjectKey {
    fn eq(&self, other: &Self) -> bool {
        Python::attach(|py| {
            let a = self.0.bind(py);
            let b = other.0.bind(py);
            match a.eq(b) {
                Ok(equal) => equal,
                Err(error) => {
                    set_comparison_error(error);
                    false
                }
            }
        })
    }
}

impl Eq for PyObjectKey {}

impl PartialOrd for PyObjectKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PyObjectKey {
    fn cmp(&self, other: &Self) -> Ordering {
        Python::attach(|py| {
            let a = self.0.bind(py);
            let b = other.0.bind(py);
            match a.compare(b) {
                Ok(ordering) => ordering,
                Err(error) => {
                    set_comparison_error(error);
                    Ordering::Equal
                }
            }
        })
    }
}
