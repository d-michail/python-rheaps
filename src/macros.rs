//! Shared codegen for every generic (`key_type=int|float|object`) heap
//! wrapper.
//!
//! Every non-radix heap algorithm in `rheaps` exposes the same Python
//! surface, differing only in which trait capabilities (`DecreaseKeyHeap`,
//! `MeldableAddressableHeap`, `DoubleEndedAddressableHeap`, ...) the concrete
//! type implements. Each capability gets its own macro expanding to a
//! `#[pymethods] impl` block (the `multiple-pymethods` pyo3 feature lets
//! several such blocks coexist on one `#[pyclass]`), matching on a
//! per-algorithm `Int`/`Float`/`Object` repr enum and delegating to the
//! wrapped `rheaps` type's inherent methods. A concrete heap module composes
//! exactly the macros matching what that algorithm implements.
//!
//! Radix heaps (`monotone`) are unrelated: their key type is fixed per class
//! (no `key_type` choice), so they get their own two macros at the bottom of
//! this file instead of an `Int`/`Float`/`Object` enum.

/// Declares the `Repr` enum and `#[pyclass]` struct for a `Heap<T>`-shaped
/// (value-only) algorithm with a parameterless `new()` constructor, plus its
/// `key_type`-dispatching `#[new]`.
#[macro_export]
macro_rules! value_heap_pyclass {
    ($name:ident, $repr:ident, $($path:ident)::+) => {
        pub enum $repr {
            Int($($path)::+<i64>),
            Float($($path)::+<rheaps::monotone::FiniteF64>),
            Object($($path)::+<crate::key::PyObjectKey>),
        }

        #[pyclass(module = "rheaps")]
        pub struct $name {
            repr: $repr,
        }

        #[pymethods]
        impl $name {
            #[new]
            #[pyo3(signature = (key_type=None))]
            fn new(py: Python<'_>, key_type: Option<Bound<'_, PyAny>>) -> PyResult<Self> {
                let repr = match crate::key::parse_key_type(py, key_type)? {
                    crate::key::KeyType::Int => $repr::Int(<$($path)::+<i64>>::new()),
                    crate::key::KeyType::Float => {
                        $repr::Float(<$($path)::+<rheaps::monotone::FiniteF64>>::new())
                    }
                    crate::key::KeyType::Object => {
                        $repr::Object(<$($path)::+<crate::key::PyObjectKey>>::new())
                    }
                };
                Ok(Self { repr })
            }
        }
    };
}

/// `push`/`peek`/`pop`/`__len__`/`is_empty`/`clear` for a `Heap<T>`-shaped
/// pyclass declared by [`value_heap_pyclass!`].
#[macro_export]
macro_rules! value_heap_methods {
    ($name:ident, $repr:ident) => {
        #[pymethods]
        impl $name {
            fn push(&mut self, key: Bound<'_, PyAny>) -> PyResult<()> {
                match &mut self.repr {
                    $repr::Int(h) => rheaps::Heap::push(h, crate::key::to_i64(&key)?),
                    $repr::Float(h) => rheaps::Heap::push(h, crate::key::to_finite_f64(&key)?),
                    $repr::Object(h) => {
                        rheaps::Heap::push(h, crate::key::PyObjectKey::new(key.unbind()))
                    }
                }
                crate::key::check_comparison_error()
            }

            fn peek(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
                let result = match &self.repr {
                    $repr::Int(h) => rheaps::Heap::peek(h).map(|v| crate::key::i64_to_py(py, *v)),
                    $repr::Float(h) => {
                        rheaps::Heap::peek(h).map(|v| crate::key::finite_f64_to_py(py, *v))
                    }
                    $repr::Object(h) => rheaps::Heap::peek(h).map(|v| v.0.clone_ref(py)),
                };
                crate::key::check_comparison_error()?;
                Ok(result)
            }

            fn pop(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
                let result = match &mut self.repr {
                    $repr::Int(h) => rheaps::Heap::pop(h).map(|v| crate::key::i64_to_py(py, v)),
                    $repr::Float(h) => {
                        rheaps::Heap::pop(h).map(|v| crate::key::finite_f64_to_py(py, v))
                    }
                    $repr::Object(h) => rheaps::Heap::pop(h).map(|v| v.0),
                };
                crate::key::check_comparison_error()?;
                Ok(result)
            }

            fn __len__(&self) -> usize {
                match &self.repr {
                    $repr::Int(h) => rheaps::Heap::len(h),
                    $repr::Float(h) => rheaps::Heap::len(h),
                    $repr::Object(h) => rheaps::Heap::len(h),
                }
            }

            fn is_empty(&self) -> bool {
                match &self.repr {
                    $repr::Int(h) => rheaps::Heap::is_empty(h),
                    $repr::Float(h) => rheaps::Heap::is_empty(h),
                    $repr::Object(h) => rheaps::Heap::is_empty(h),
                }
            }

            fn clear(&mut self) {
                match &mut self.repr {
                    $repr::Int(h) => rheaps::Heap::clear(h),
                    $repr::Float(h) => rheaps::Heap::clear(h),
                    $repr::Object(h) => rheaps::Heap::clear(h),
                }
            }
        }
    };
}

/// Adds `peek_max`/`pop_max` to a [`value_heap_pyclass!`] whose wrapped type
/// implements `DoubleEndedHeap`.
#[macro_export]
macro_rules! double_ended_value_heap_methods {
    ($name:ident, $repr:ident) => {
        #[pymethods]
        impl $name {
            fn peek_max(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
                let result = match &self.repr {
                    $repr::Int(h) => {
                        rheaps::DoubleEndedHeap::peek_max(h).map(|v| crate::key::i64_to_py(py, *v))
                    }
                    $repr::Float(h) => rheaps::DoubleEndedHeap::peek_max(h)
                        .map(|v| crate::key::finite_f64_to_py(py, *v)),
                    $repr::Object(h) => {
                        rheaps::DoubleEndedHeap::peek_max(h).map(|v| v.0.clone_ref(py))
                    }
                };
                crate::key::check_comparison_error()?;
                Ok(result)
            }

            fn pop_max(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
                let result = match &mut self.repr {
                    $repr::Int(h) => {
                        rheaps::DoubleEndedHeap::pop_max(h).map(|v| crate::key::i64_to_py(py, v))
                    }
                    $repr::Float(h) => rheaps::DoubleEndedHeap::pop_max(h)
                        .map(|v| crate::key::finite_f64_to_py(py, v)),
                    $repr::Object(h) => rheaps::DoubleEndedHeap::pop_max(h).map(|v| v.0),
                };
                crate::key::check_comparison_error()?;
                Ok(result)
            }
        }
    };
}

/// Like [`value_heap_pyclass!`], but for a d-ary algorithm whose constructor
/// takes a `degree: usize` and returns `Result<Self, $err>` (`$err_conv`
/// converts `$err` to a `PyErr`).
#[macro_export]
macro_rules! degree_value_heap_pyclass {
    ($name:ident, $repr:ident, $($path:ident)::+, $err_conv:path) => {
        pub enum $repr {
            Int($($path)::+<i64>),
            Float($($path)::+<rheaps::monotone::FiniteF64>),
            Object($($path)::+<crate::key::PyObjectKey>),
        }

        #[pyclass(module = "rheaps")]
        pub struct $name {
            repr: $repr,
        }

        #[pymethods]
        impl $name {
            #[new]
            #[pyo3(signature = (degree, key_type=None))]
            fn new(
                py: Python<'_>,
                degree: usize,
                key_type: Option<Bound<'_, PyAny>>,
            ) -> PyResult<Self> {
                let repr = match crate::key::parse_key_type(py, key_type)? {
                    crate::key::KeyType::Int => {
                        $repr::Int(<$($path)::+<i64>>::new(degree).map_err($err_conv)?)
                    }
                    crate::key::KeyType::Float => $repr::Float(
                        <$($path)::+<rheaps::monotone::FiniteF64>>::new(degree).map_err($err_conv)?,
                    ),
                    crate::key::KeyType::Object => $repr::Object(
                        <$($path)::+<crate::key::PyObjectKey>>::new(degree).map_err($err_conv)?,
                    ),
                };
                Ok(Self { repr })
            }

            fn degree(&self) -> usize {
                match &self.repr {
                    $repr::Int(h) => h.degree(),
                    $repr::Float(h) => h.degree(),
                    $repr::Object(h) => h.degree(),
                }
            }
        }
    };
}

/// Like [`addressable_heap_pyclass!`], but for a d-ary algorithm whose
/// constructor takes a `degree: usize` and returns `Result<Self, $err>`.
#[macro_export]
macro_rules! degree_addressable_heap_pyclass {
    ($name:ident, $repr:ident, $($path:ident)::+, $err_conv:path) => {
        pub enum $repr {
            Int($($path)::+<i64, Py<PyAny>>),
            Float($($path)::+<rheaps::monotone::FiniteF64, Py<PyAny>>),
            Object($($path)::+<crate::key::PyObjectKey, Py<PyAny>>),
        }

        #[pyclass(module = "rheaps")]
        pub struct $name {
            repr: $repr,
        }

        #[pymethods]
        impl $name {
            #[new]
            #[pyo3(signature = (degree, key_type=None))]
            fn new(
                py: Python<'_>,
                degree: usize,
                key_type: Option<Bound<'_, PyAny>>,
            ) -> PyResult<Self> {
                let repr = match crate::key::parse_key_type(py, key_type)? {
                    crate::key::KeyType::Int => {
                        $repr::Int(<$($path)::+<i64, Py<PyAny>>>::new(degree).map_err($err_conv)?)
                    }
                    crate::key::KeyType::Float => $repr::Float(
                        <$($path)::+<rheaps::monotone::FiniteF64, Py<PyAny>>>::new(degree)
                            .map_err($err_conv)?,
                    ),
                    crate::key::KeyType::Object => $repr::Object(
                        <$($path)::+<crate::key::PyObjectKey, Py<PyAny>>>::new(degree)
                            .map_err($err_conv)?,
                    ),
                };
                Ok(Self { repr })
            }

            fn degree(&self) -> usize {
                match &self.repr {
                    $repr::Int(h) => h.degree(),
                    $repr::Float(h) => h.degree(),
                    $repr::Object(h) => h.degree(),
                }
            }
        }
    };
}

/// Declares the `Repr` enum and `#[pyclass]` struct for an
/// `AddressableHeap<K, V>`-shaped algorithm with a parameterless `new()`
/// constructor (value is always `Py<PyAny>`), plus its `key_type`-dispatching
/// `#[new]`.
#[macro_export]
macro_rules! addressable_heap_pyclass {
    ($name:ident, $repr:ident, $($path:ident)::+) => {
        pub enum $repr {
            Int($($path)::+<i64, Py<PyAny>>),
            Float($($path)::+<rheaps::monotone::FiniteF64, Py<PyAny>>),
            Object($($path)::+<crate::key::PyObjectKey, Py<PyAny>>),
        }

        #[pyclass(module = "rheaps")]
        pub struct $name {
            repr: $repr,
        }

        #[pymethods]
        impl $name {
            #[new]
            #[pyo3(signature = (key_type=None))]
            fn new(py: Python<'_>, key_type: Option<Bound<'_, PyAny>>) -> PyResult<Self> {
                let repr = match crate::key::parse_key_type(py, key_type)? {
                    crate::key::KeyType::Int => $repr::Int(<$($path)::+<i64, Py<PyAny>>>::new()),
                    crate::key::KeyType::Float => {
                        $repr::Float(<$($path)::+<rheaps::monotone::FiniteF64, Py<PyAny>>>::new())
                    }
                    crate::key::KeyType::Object => {
                        $repr::Object(<$($path)::+<crate::key::PyObjectKey, Py<PyAny>>>::new())
                    }
                };
                Ok(Self { repr })
            }
        }
    };
}

/// `insert`/`peek`/`pop`/`key`/`value`/`set_value`/`delete`/`__len__`/
/// `is_empty`/`clear` for an `AddressableHeap`-shaped pyclass declared by
/// [`addressable_heap_pyclass!`]. `$handle` is the handle wrapper pyclass
/// (from `crate::handles`) shared by every key-type variant of `$name`.
#[macro_export]
macro_rules! addressable_heap_methods {
    ($name:ident, $repr:ident, $handle:ty) => {
        #[pymethods]
        impl $name {
            #[pyo3(signature = (key, value=None))]
            fn insert(
                &mut self,
                py: Python<'_>,
                key: Bound<'_, PyAny>,
                value: Option<Py<PyAny>>,
            ) -> PyResult<$handle> {
                let value = value.unwrap_or_else(|| py.None());
                let handle = match &mut self.repr {
                    $repr::Int(h) => {
                        <$handle>::from(h.insert(crate::key::to_i64(&key)?, value))
                    }
                    $repr::Float(h) => {
                        <$handle>::from(h.insert(crate::key::to_finite_f64(&key)?, value))
                    }
                    $repr::Object(h) => <$handle>::from(
                        h.insert(crate::key::PyObjectKey::new(key.unbind()), value),
                    ),
                };
                crate::key::check_comparison_error()?;
                Ok(handle)
            }

            fn peek(&self, py: Python<'_>) -> PyResult<Option<($handle, Py<PyAny>, Py<PyAny>)>> {
                let result = match &self.repr {
                    $repr::Int(h) => h.peek_entry().map(|(handle, k, v)| {
                        (<$handle>::from(handle), crate::key::i64_to_py(py, *k), v.clone_ref(py))
                    }),
                    $repr::Float(h) => h.peek_entry().map(|(handle, k, v)| {
                        (
                            <$handle>::from(handle),
                            crate::key::finite_f64_to_py(py, *k),
                            v.clone_ref(py),
                        )
                    }),
                    $repr::Object(h) => h.peek_entry().map(|(handle, k, v)| {
                        (<$handle>::from(handle), k.0.clone_ref(py), v.clone_ref(py))
                    }),
                };
                crate::key::check_comparison_error()?;
                Ok(result)
            }

            fn pop(&mut self, py: Python<'_>) -> PyResult<Option<(Py<PyAny>, Py<PyAny>)>> {
                let result = match &mut self.repr {
                    $repr::Int(h) => h.pop_entry().map(|(k, v)| (crate::key::i64_to_py(py, k), v)),
                    $repr::Float(h) => {
                        h.pop_entry().map(|(k, v)| (crate::key::finite_f64_to_py(py, k), v))
                    }
                    $repr::Object(h) => h.pop_entry().map(|(k, v)| (k.into_inner(), v)),
                };
                crate::key::check_comparison_error()?;
                Ok(result)
            }

            fn key(&self, py: Python<'_>, handle: $handle) -> PyResult<Py<PyAny>> {
                match &self.repr {
                    $repr::Int(h) => h
                        .key(handle.0)
                        .map(|k| crate::key::i64_to_py(py, *k))
                        .map_err(crate::error::invalid_handle),
                    $repr::Float(h) => h
                        .key(handle.0)
                        .map(|k| crate::key::finite_f64_to_py(py, *k))
                        .map_err(crate::error::invalid_handle),
                    $repr::Object(h) => h
                        .key(handle.0)
                        .map(|k| k.0.clone_ref(py))
                        .map_err(crate::error::invalid_handle),
                }
            }

            fn value(&self, py: Python<'_>, handle: $handle) -> PyResult<Py<PyAny>> {
                match &self.repr {
                    $repr::Int(h) => {
                        h.value(handle.0).map(|v| v.clone_ref(py)).map_err(crate::error::invalid_handle)
                    }
                    $repr::Float(h) => {
                        h.value(handle.0).map(|v| v.clone_ref(py)).map_err(crate::error::invalid_handle)
                    }
                    $repr::Object(h) => {
                        h.value(handle.0).map(|v| v.clone_ref(py)).map_err(crate::error::invalid_handle)
                    }
                }
            }

            fn set_value(&mut self, handle: $handle, value: Py<PyAny>) -> PyResult<()> {
                match &mut self.repr {
                    $repr::Int(h) => {
                        *h.value_mut(handle.0).map_err(crate::error::invalid_handle)? = value;
                    }
                    $repr::Float(h) => {
                        *h.value_mut(handle.0).map_err(crate::error::invalid_handle)? = value;
                    }
                    $repr::Object(h) => {
                        *h.value_mut(handle.0).map_err(crate::error::invalid_handle)? = value;
                    }
                }
                Ok(())
            }

            fn delete(&mut self, py: Python<'_>, handle: $handle) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
                let result = match &mut self.repr {
                    $repr::Int(h) => h
                        .delete(handle.0)
                        .map(|(k, v)| (crate::key::i64_to_py(py, k), v))
                        .map_err(crate::error::invalid_handle),
                    $repr::Float(h) => h
                        .delete(handle.0)
                        .map(|(k, v)| (crate::key::finite_f64_to_py(py, k), v))
                        .map_err(crate::error::invalid_handle),
                    $repr::Object(h) => h
                        .delete(handle.0)
                        .map(|(k, v)| (k.into_inner(), v))
                        .map_err(crate::error::invalid_handle),
                }?;
                crate::key::check_comparison_error()?;
                Ok(result)
            }

            fn __len__(&self) -> usize {
                match &self.repr {
                    $repr::Int(h) => h.len(),
                    $repr::Float(h) => h.len(),
                    $repr::Object(h) => h.len(),
                }
            }

            fn is_empty(&self) -> bool {
                match &self.repr {
                    $repr::Int(h) => h.is_empty(),
                    $repr::Float(h) => h.is_empty(),
                    $repr::Object(h) => h.is_empty(),
                }
            }

            fn clear(&mut self) {
                match &mut self.repr {
                    $repr::Int(h) => h.clear(),
                    $repr::Float(h) => h.clear(),
                    $repr::Object(h) => h.clear(),
                }
            }
        }
    };
}

/// Adds `decrease_key` to an [`addressable_heap_pyclass!`] whose wrapped type
/// implements `DecreaseKeyHeap`.
#[macro_export]
macro_rules! decrease_key_methods {
    ($name:ident, $repr:ident, $handle:ty) => {
        #[pymethods]
        impl $name {
            fn decrease_key(&mut self, handle: $handle, key: Bound<'_, PyAny>) -> PyResult<()> {
                let result = match &mut self.repr {
                    $repr::Int(h) => h.decrease_key(handle.0, crate::key::to_i64(&key)?),
                    $repr::Float(h) => {
                        h.decrease_key(handle.0, crate::key::to_finite_f64(&key)?)
                    }
                    $repr::Object(h) => {
                        h.decrease_key(handle.0, crate::key::PyObjectKey::new(key.unbind()))
                    }
                };
                crate::key::check_comparison_error()?;
                result.map_err(crate::error::decrease_key_error)
            }
        }
    };
}

/// Adds `meld(other)` to an [`addressable_heap_pyclass!`] whose wrapped type
/// implements `MeldableAddressableHeap` with an infallible meld and a
/// parameterless `new()` constructor. `other` is left as a valid, empty heap
/// of the same key type afterward — Python has no move semantics, so the
/// donor's real contents are swapped out via `mem::replace` and consumed by
/// value on the Rust side, matching `rheaps`' "donor moved into the call"
/// contract as closely as a shared reference allows.
#[macro_export]
macro_rules! meldable_methods {
    ($name:ident, $repr:ident, $($path:ident)::+) => {
        #[pymethods]
        impl $name {
            fn meld(&mut self, other: &mut Self) -> PyResult<()> {
                match (&mut self.repr, &mut other.repr) {
                    ($repr::Int(a), $repr::Int(b)) => {
                        let taken = std::mem::replace(b, <$($path)::+<i64, Py<PyAny>>>::new());
                        a.meld(taken);
                    }
                    ($repr::Float(a), $repr::Float(b)) => {
                        let taken =
                            std::mem::replace(b, <$($path)::+<rheaps::monotone::FiniteF64, Py<PyAny>>>::new());
                        a.meld(taken);
                    }
                    ($repr::Object(a), $repr::Object(b)) => {
                        let taken =
                            std::mem::replace(b, <$($path)::+<crate::key::PyObjectKey, Py<PyAny>>>::new());
                        a.meld(taken);
                    }
                    _ => {
                        return Err(pyo3::exceptions::PyValueError::new_err(
                            "cannot meld heaps constructed with different key_type",
                        ));
                    }
                }
                crate::key::check_comparison_error()
            }
        }
    };
}

/// Adds `peek_max`/`pop_max`/`increase_key` to an [`addressable_heap_pyclass!`]
/// whose wrapped type implements `DoubleEndedAddressableHeap`.
#[macro_export]
macro_rules! double_ended_addressable_methods {
    ($name:ident, $repr:ident, $handle:ty) => {
        #[pymethods]
        impl $name {
            fn peek_max(
                &self,
                py: Python<'_>,
            ) -> PyResult<Option<($handle, Py<PyAny>, Py<PyAny>)>> {
                let result = match &self.repr {
                    $repr::Int(h) => h.peek_max_entry().map(|(handle, k, v)| {
                        (<$handle>::from(handle), crate::key::i64_to_py(py, *k), v.clone_ref(py))
                    }),
                    $repr::Float(h) => h.peek_max_entry().map(|(handle, k, v)| {
                        (
                            <$handle>::from(handle),
                            crate::key::finite_f64_to_py(py, *k),
                            v.clone_ref(py),
                        )
                    }),
                    $repr::Object(h) => h.peek_max_entry().map(|(handle, k, v)| {
                        (<$handle>::from(handle), k.0.clone_ref(py), v.clone_ref(py))
                    }),
                };
                crate::key::check_comparison_error()?;
                Ok(result)
            }

            fn pop_max(&mut self, py: Python<'_>) -> PyResult<Option<(Py<PyAny>, Py<PyAny>)>> {
                let result = match &mut self.repr {
                    $repr::Int(h) => {
                        h.pop_max_entry().map(|(k, v)| (crate::key::i64_to_py(py, k), v))
                    }
                    $repr::Float(h) => h
                        .pop_max_entry()
                        .map(|(k, v)| (crate::key::finite_f64_to_py(py, k), v)),
                    $repr::Object(h) => h.pop_max_entry().map(|(k, v)| (k.into_inner(), v)),
                };
                crate::key::check_comparison_error()?;
                Ok(result)
            }

            fn increase_key(&mut self, handle: $handle, key: Bound<'_, PyAny>) -> PyResult<()> {
                let result = match &mut self.repr {
                    $repr::Int(h) => h.increase_key(handle.0, crate::key::to_i64(&key)?),
                    $repr::Float(h) => {
                        h.increase_key(handle.0, crate::key::to_finite_f64(&key)?)
                    }
                    $repr::Object(h) => {
                        h.increase_key(handle.0, crate::key::PyObjectKey::new(key.unbind()))
                    }
                };
                crate::key::check_comparison_error()?;
                result.map_err(crate::error::increase_key_error)
            }
        }
    };
}

/// Declares a non-generic `#[pyclass]` wrapping a monotone radix heap
/// (`rheaps::monotone::{U32,U64,F64,BigUint}RadixHeap`), whose key type is
/// fixed by the class itself rather than chosen via `key_type`. `$to_rust`
/// converts an incoming Python key to `$key`; `$to_py` converts an owned
/// `$key` back to a Python object.
#[macro_export]
macro_rules! radix_heap_pyclass {
    ($name:ident, $path:ty, $key:ty, $to_rust:path, $to_py:path) => {
        #[pyclass(module = "rheaps")]
        pub struct $name {
            inner: $path,
        }

        #[pymethods]
        impl $name {
            #[new]
            fn new(minimum_key: Bound<'_, PyAny>, maximum_key: Bound<'_, PyAny>) -> PyResult<Self> {
                let minimum_key: $key = $to_rust(&minimum_key)?;
                let maximum_key: $key = $to_rust(&maximum_key)?;
                let inner = <$path>::new(minimum_key, maximum_key)
                    .map_err(crate::error::radix_heap_error)?;
                Ok(Self { inner })
            }

            fn try_push(&mut self, key: Bound<'_, PyAny>) -> PyResult<()> {
                let key: $key = $to_rust(&key)?;
                self.inner.try_push(key).map_err(crate::error::radix_heap_error)
            }

            fn peek(&self, py: Python<'_>) -> Option<Py<PyAny>> {
                self.inner.peek().map(|value| $to_py(py, value.clone()))
            }

            fn pop(&mut self, py: Python<'_>) -> Option<Py<PyAny>> {
                self.inner.pop().map(|value| $to_py(py, value))
            }

            fn __len__(&self) -> usize {
                self.inner.len()
            }

            fn is_empty(&self) -> bool {
                self.inner.is_empty()
            }

            fn clear(&mut self) {
                self.inner.clear();
            }

            fn bucket_count(&self) -> usize {
                self.inner.bucket_count()
            }
        }
    };
}

/// Declares a non-generic `#[pyclass]` wrapping an addressable monotone radix
/// heap (`rheaps::monotone::{U32,U64,F64,BigUint}RadixAddressableHeap<V>`,
/// with `V` fixed to `Py<PyAny>`).
#[macro_export]
macro_rules! addressable_radix_heap_pyclass {
    ($name:ident, $path:ty, $key:ty, $to_rust:path, $to_py:path) => {
        #[pyclass(module = "rheaps")]
        pub struct $name {
            inner: $path,
        }

        #[pymethods]
        impl $name {
            #[new]
            fn new(minimum_key: Bound<'_, PyAny>, maximum_key: Bound<'_, PyAny>) -> PyResult<Self> {
                let minimum_key: $key = $to_rust(&minimum_key)?;
                let maximum_key: $key = $to_rust(&maximum_key)?;
                let inner = <$path>::new(minimum_key, maximum_key)
                    .map_err(crate::error::radix_heap_error)?;
                Ok(Self { inner })
            }

            #[pyo3(signature = (key, value=None))]
            fn try_insert(
                &mut self,
                py: Python<'_>,
                key: Bound<'_, PyAny>,
                value: Option<Py<PyAny>>,
            ) -> PyResult<crate::handles::RadixHandle> {
                let key: $key = $to_rust(&key)?;
                let value = value.unwrap_or_else(|| py.None());
                self.inner
                    .try_insert(key, value)
                    .map(crate::handles::RadixHandle::from)
                    .map_err(crate::error::radix_heap_error)
            }

            fn peek(
                &self,
                py: Python<'_>,
            ) -> Option<(crate::handles::RadixHandle, Py<PyAny>, Py<PyAny>)> {
                self.inner.peek().map(|(handle, key, value)| {
                    (
                        crate::handles::RadixHandle::from(handle),
                        $to_py(py, key.clone()),
                        value.clone_ref(py),
                    )
                })
            }

            fn pop(&mut self, py: Python<'_>) -> Option<(Py<PyAny>, Py<PyAny>)> {
                self.inner.pop().map(|(key, value)| ($to_py(py, key), value))
            }

            fn key(&self, py: Python<'_>, handle: crate::handles::RadixHandle) -> PyResult<Py<PyAny>> {
                self.inner
                    .key(handle.0)
                    .map(|key| $to_py(py, key.clone()))
                    .map_err(crate::error::invalid_handle)
            }

            fn value(&self, py: Python<'_>, handle: crate::handles::RadixHandle) -> PyResult<Py<PyAny>> {
                self.inner
                    .value(handle.0)
                    .map(|value| value.clone_ref(py))
                    .map_err(crate::error::invalid_handle)
            }

            fn set_value(
                &mut self,
                handle: crate::handles::RadixHandle,
                value: Py<PyAny>,
            ) -> PyResult<()> {
                *self.inner.value_mut(handle.0).map_err(crate::error::invalid_handle)? = value;
                Ok(())
            }

            fn decrease_key(
                &mut self,
                handle: crate::handles::RadixHandle,
                key: Bound<'_, PyAny>,
            ) -> PyResult<()> {
                let key: $key = $to_rust(&key)?;
                self.inner
                    .decrease_key(handle.0, key)
                    .map_err(crate::error::radix_decrease_key_error)
            }

            fn delete(
                &mut self,
                py: Python<'_>,
                handle: crate::handles::RadixHandle,
            ) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
                self.inner
                    .delete(handle.0)
                    .map(|(key, value)| ($to_py(py, key), value))
                    .map_err(crate::error::invalid_handle)
            }

            fn __len__(&self) -> usize {
                self.inner.len()
            }

            fn is_empty(&self) -> bool {
                self.inner.is_empty()
            }

            fn clear(&mut self) {
                self.inner.clear();
            }

            fn bucket_count(&self) -> usize {
                self.inner.bucket_count()
            }
        }
    };
}
