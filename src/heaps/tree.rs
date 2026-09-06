//! Tree-based heap wrappers (`rheaps::tree`).

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

macro_rules! tree_addressable_meldable {
    ($name:ident, $repr:ident, $($path:ident)::+) => {
        crate::addressable_heap_pyclass!($name, $repr, $($path)::+);
        crate::addressable_heap_methods!($name, $repr, crate::handles::TreeHandle);
        crate::decrease_key_methods!($name, $repr, crate::handles::TreeHandle);
        crate::meldable_methods!($name, $repr, $($path)::+);
    };
}

tree_addressable_meldable!(PairingHeap, PairingHeapRepr, rheaps::tree::PairingHeap);
tree_addressable_meldable!(
    PurePairingHeap,
    PurePairingHeapRepr,
    rheaps::tree::PurePairingHeap
);
tree_addressable_meldable!(
    CostlessMeldPairingHeap,
    CostlessMeldPairingHeapRepr,
    rheaps::tree::CostlessMeldPairingHeap
);
tree_addressable_meldable!(
    RankPairingHeap,
    RankPairingHeapRepr,
    rheaps::tree::RankPairingHeap
);
tree_addressable_meldable!(LeftistHeap, LeftistHeapRepr, rheaps::tree::LeftistHeap);
tree_addressable_meldable!(SkewHeap, SkewHeapRepr, rheaps::tree::SkewHeap);
tree_addressable_meldable!(FibonacciHeap, FibonacciHeapRepr, rheaps::tree::FibonacciHeap);
tree_addressable_meldable!(
    SimpleFibonacciHeap,
    SimpleFibonacciHeapRepr,
    rheaps::tree::SimpleFibonacciHeap
);
tree_addressable_meldable!(
    StrictFibonacciHeap,
    StrictFibonacciHeapRepr,
    rheaps::tree::StrictFibonacciHeap
);

crate::addressable_heap_pyclass!(
    BinaryTreeAddressableHeap,
    BinaryTreeAddressableHeapRepr,
    rheaps::tree::BinaryTreeAddressableHeap
);
crate::addressable_heap_methods!(
    BinaryTreeAddressableHeap,
    BinaryTreeAddressableHeapRepr,
    crate::handles::TreeHandle
);
crate::decrease_key_methods!(
    BinaryTreeAddressableHeap,
    BinaryTreeAddressableHeapRepr,
    crate::handles::TreeHandle
);

crate::degree_addressable_heap_pyclass!(
    DaryTreeAddressableHeap,
    DaryTreeAddressableHeapRepr,
    rheaps::tree::DaryTreeAddressableHeap,
    crate::error::invalid_branching_factor
);
crate::addressable_heap_methods!(
    DaryTreeAddressableHeap,
    DaryTreeAddressableHeapRepr,
    crate::handles::TreeHandle
);
crate::decrease_key_methods!(
    DaryTreeAddressableHeap,
    DaryTreeAddressableHeapRepr,
    crate::handles::TreeHandle
);

macro_rules! reflected_heap {
    ($name:ident, $repr:ident, $($path:ident)::+) => {
        crate::addressable_heap_pyclass!($name, $repr, $($path)::+);
        crate::addressable_heap_methods!($name, $repr, crate::handles::ReflectedHandle);
        crate::decrease_key_methods!($name, $repr, crate::handles::ReflectedHandle);
        crate::meldable_methods!($name, $repr, $($path)::+);
        crate::double_ended_addressable_methods!($name, $repr, crate::handles::ReflectedHandle);
    };
}

reflected_heap!(
    ReflectedFibonacciHeap,
    ReflectedFibonacciHeapRepr,
    rheaps::tree::ReflectedFibonacciHeap
);
reflected_heap!(
    ReflectedPairingHeap,
    ReflectedPairingHeapRepr,
    rheaps::tree::ReflectedPairingHeap
);

pub enum SoftHeapRepr {
    Int(rheaps::tree::BinaryTreeSoftHeap<i64>),
    Float(rheaps::tree::BinaryTreeSoftHeap<rheaps::monotone::FiniteF64>),
    Object(rheaps::tree::BinaryTreeSoftHeap<crate::key::PyObjectKey>),
}

/// A Kaplan-Zwick soft heap: a value-only heap that trades a bounded
/// fraction of corrupted (increased) keys for faster operations. Not
/// addressable and does not support `decrease_key`.
#[pyclass(module = "rheaps")]
pub struct SoftHeap {
    repr: SoftHeapRepr,
    error_rate: f64,
}

#[pymethods]
impl SoftHeap {
    #[new]
    #[pyo3(signature = (error_rate, key_type=None))]
    fn new(py: Python<'_>, error_rate: f64, key_type: Option<Bound<'_, PyAny>>) -> PyResult<Self> {
        let repr = match crate::key::parse_key_type(py, key_type)? {
            crate::key::KeyType::Int => SoftHeapRepr::Int(
                rheaps::tree::BinaryTreeSoftHeap::new(error_rate)
                    .map_err(crate::error::soft_heap_error)?,
            ),
            crate::key::KeyType::Float => SoftHeapRepr::Float(
                rheaps::tree::BinaryTreeSoftHeap::new(error_rate)
                    .map_err(crate::error::soft_heap_error)?,
            ),
            crate::key::KeyType::Object => SoftHeapRepr::Object(
                rheaps::tree::BinaryTreeSoftHeap::new(error_rate)
                    .map_err(crate::error::soft_heap_error)?,
            ),
        };
        Ok(Self { repr, error_rate })
    }

    fn push(&mut self, key: Bound<'_, PyAny>) -> PyResult<()> {
        match &mut self.repr {
            SoftHeapRepr::Int(h) => h.push(crate::key::to_i64(&key)?),
            SoftHeapRepr::Float(h) => h.push(crate::key::to_finite_f64(&key)?),
            SoftHeapRepr::Object(h) => h.push(crate::key::PyObjectKey::new(key.unbind())),
        }
        crate::key::check_comparison_error()
    }

    fn peek(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let result = match &self.repr {
            SoftHeapRepr::Int(h) => h.peek().map(|v| crate::key::i64_to_py(py, *v)),
            SoftHeapRepr::Float(h) => h.peek().map(|v| crate::key::finite_f64_to_py(py, *v)),
            SoftHeapRepr::Object(h) => h.peek().map(|v| v.0.clone_ref(py)),
        };
        crate::key::check_comparison_error()?;
        Ok(result)
    }

    fn pop(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let result = match &mut self.repr {
            SoftHeapRepr::Int(h) => h.pop().map(|v| crate::key::i64_to_py(py, v)),
            SoftHeapRepr::Float(h) => h.pop().map(|v| crate::key::finite_f64_to_py(py, v)),
            SoftHeapRepr::Object(h) => h.pop().map(|v| v.0),
        };
        crate::key::check_comparison_error()?;
        Ok(result)
    }

    fn __len__(&self) -> usize {
        match &self.repr {
            SoftHeapRepr::Int(h) => h.len(),
            SoftHeapRepr::Float(h) => h.len(),
            SoftHeapRepr::Object(h) => h.len(),
        }
    }

    fn is_empty(&self) -> bool {
        match &self.repr {
            SoftHeapRepr::Int(h) => h.is_empty(),
            SoftHeapRepr::Float(h) => h.is_empty(),
            SoftHeapRepr::Object(h) => h.is_empty(),
        }
    }

    fn clear(&mut self) {
        match &mut self.repr {
            SoftHeapRepr::Int(h) => h.clear(),
            SoftHeapRepr::Float(h) => h.clear(),
            SoftHeapRepr::Object(h) => h.clear(),
        }
    }

    fn rank_limit(&self) -> usize {
        match &self.repr {
            SoftHeapRepr::Int(h) => h.rank_limit(),
            SoftHeapRepr::Float(h) => h.rank_limit(),
            SoftHeapRepr::Object(h) => h.rank_limit(),
        }
    }

    /// Melds `other` into this heap. `other` is left empty (rebuilt with its
    /// original error rate) afterward, and must have the same `key_type`.
    fn meld(&mut self, other: &mut Self) -> PyResult<()> {
        let error_rate = other.error_rate;
        match (&mut self.repr, &mut other.repr) {
            (SoftHeapRepr::Int(a), SoftHeapRepr::Int(b)) => {
                let placeholder = rheaps::tree::BinaryTreeSoftHeap::new(error_rate)
                    .expect("error_rate was already validated when other was constructed");
                let taken = std::mem::replace(b, placeholder);
                a.meld(taken).map_err(crate::error::soft_meld_error)?;
            }
            (SoftHeapRepr::Float(a), SoftHeapRepr::Float(b)) => {
                let placeholder = rheaps::tree::BinaryTreeSoftHeap::new(error_rate)
                    .expect("error_rate was already validated when other was constructed");
                let taken = std::mem::replace(b, placeholder);
                a.meld(taken).map_err(crate::error::soft_meld_error)?;
            }
            (SoftHeapRepr::Object(a), SoftHeapRepr::Object(b)) => {
                let placeholder = rheaps::tree::BinaryTreeSoftHeap::new(error_rate)
                    .expect("error_rate was already validated when other was constructed");
                let taken = std::mem::replace(b, placeholder);
                a.meld(taken).map_err(crate::error::soft_meld_error)?;
            }
            _ => {
                return Err(PyValueError::new_err(
                    "cannot meld heaps constructed with different key_type",
                ));
            }
        }
        crate::key::check_comparison_error()
    }
}

pub enum SoftAddressableHeapRepr {
    Int(rheaps::tree::BinaryTreeSoftAddressableHeap<i64, Py<PyAny>>),
    Float(rheaps::tree::BinaryTreeSoftAddressableHeap<rheaps::monotone::FiniteF64, Py<PyAny>>),
    Object(rheaps::tree::BinaryTreeSoftAddressableHeap<crate::key::PyObjectKey, Py<PyAny>>),
}

/// The addressable, key-value form of [`SoftHeap`]. Corruption-bounded, so it
/// does not support `decrease_key` (there is no way to guarantee the
/// corruption bound after an arbitrary key decrease).
#[pyclass(module = "rheaps")]
pub struct SoftAddressableHeap {
    repr: SoftAddressableHeapRepr,
    error_rate: f64,
}

#[pymethods]
impl SoftAddressableHeap {
    #[new]
    #[pyo3(signature = (error_rate, key_type=None))]
    fn new(py: Python<'_>, error_rate: f64, key_type: Option<Bound<'_, PyAny>>) -> PyResult<Self> {
        let repr = match crate::key::parse_key_type(py, key_type)? {
            crate::key::KeyType::Int => SoftAddressableHeapRepr::Int(
                rheaps::tree::BinaryTreeSoftAddressableHeap::new(error_rate)
                    .map_err(crate::error::soft_heap_error)?,
            ),
            crate::key::KeyType::Float => SoftAddressableHeapRepr::Float(
                rheaps::tree::BinaryTreeSoftAddressableHeap::new(error_rate)
                    .map_err(crate::error::soft_heap_error)?,
            ),
            crate::key::KeyType::Object => SoftAddressableHeapRepr::Object(
                rheaps::tree::BinaryTreeSoftAddressableHeap::new(error_rate)
                    .map_err(crate::error::soft_heap_error)?,
            ),
        };
        Ok(Self { repr, error_rate })
    }

    fn rank_limit(&self) -> usize {
        match &self.repr {
            SoftAddressableHeapRepr::Int(h) => h.rank_limit(),
            SoftAddressableHeapRepr::Float(h) => h.rank_limit(),
            SoftAddressableHeapRepr::Object(h) => h.rank_limit(),
        }
    }

    /// Melds `other` into this heap. `other` is left empty (rebuilt with its
    /// original error rate) afterward, and must have the same `key_type`.
    fn meld(&mut self, other: &mut Self) -> PyResult<()> {
        let error_rate = other.error_rate;
        match (&mut self.repr, &mut other.repr) {
            (SoftAddressableHeapRepr::Int(a), SoftAddressableHeapRepr::Int(b)) => {
                let placeholder = rheaps::tree::BinaryTreeSoftAddressableHeap::new(error_rate)
                    .expect("error_rate was already validated when other was constructed");
                let taken = std::mem::replace(b, placeholder);
                a.meld(taken).map_err(crate::error::soft_meld_error)?;
            }
            (SoftAddressableHeapRepr::Float(a), SoftAddressableHeapRepr::Float(b)) => {
                let placeholder = rheaps::tree::BinaryTreeSoftAddressableHeap::new(error_rate)
                    .expect("error_rate was already validated when other was constructed");
                let taken = std::mem::replace(b, placeholder);
                a.meld(taken).map_err(crate::error::soft_meld_error)?;
            }
            (SoftAddressableHeapRepr::Object(a), SoftAddressableHeapRepr::Object(b)) => {
                let placeholder = rheaps::tree::BinaryTreeSoftAddressableHeap::new(error_rate)
                    .expect("error_rate was already validated when other was constructed");
                let taken = std::mem::replace(b, placeholder);
                a.meld(taken).map_err(crate::error::soft_meld_error)?;
            }
            _ => {
                return Err(PyValueError::new_err(
                    "cannot meld heaps constructed with different key_type",
                ));
            }
        }
        crate::key::check_comparison_error()
    }
}

crate::addressable_heap_methods!(
    SoftAddressableHeap,
    SoftAddressableHeapRepr,
    crate::handles::SoftHandle
);
