//! Monotone radix heap wrappers (`rheaps::monotone`).
//!
//! Unlike every other heap family, a radix heap's key type is fixed by
//! construction bounds (its `RadixKey` is sealed to `u32`/`u64`/`FiniteF64`/
//! `BigUint`), so there is no `key_type` choice here — each class below is
//! one fixed native key type, matching the four `rheaps::monotone` key
//! families.

use pyo3::prelude::*;

crate::radix_heap_pyclass!(
    U32RadixHeap,
    rheaps::monotone::U32RadixHeap,
    u32,
    crate::key::to_u32,
    crate::key::u32_to_py
);
crate::radix_heap_pyclass!(
    U64RadixHeap,
    rheaps::monotone::U64RadixHeap,
    u64,
    crate::key::to_u64,
    crate::key::u64_to_py
);
crate::radix_heap_pyclass!(
    F64RadixHeap,
    rheaps::monotone::F64RadixHeap,
    rheaps::monotone::FiniteF64,
    crate::key::to_finite_f64,
    crate::key::finite_f64_to_py
);
crate::radix_heap_pyclass!(
    BigUintRadixHeap,
    rheaps::monotone::BigUintRadixHeap,
    rheaps::monotone::BigUint,
    crate::key::to_biguint,
    crate::key::biguint_to_py
);

crate::addressable_radix_heap_pyclass!(
    U32RadixAddressableHeap,
    rheaps::monotone::U32RadixAddressableHeap<Py<PyAny>>,
    u32,
    crate::key::to_u32,
    crate::key::u32_to_py
);
crate::addressable_radix_heap_pyclass!(
    U64RadixAddressableHeap,
    rheaps::monotone::U64RadixAddressableHeap<Py<PyAny>>,
    u64,
    crate::key::to_u64,
    crate::key::u64_to_py
);
crate::addressable_radix_heap_pyclass!(
    F64RadixAddressableHeap,
    rheaps::monotone::F64RadixAddressableHeap<Py<PyAny>>,
    rheaps::monotone::FiniteF64,
    crate::key::to_finite_f64,
    crate::key::finite_f64_to_py
);
crate::addressable_radix_heap_pyclass!(
    BigUintRadixAddressableHeap,
    rheaps::monotone::BigUintRadixAddressableHeap<Py<PyAny>>,
    rheaps::monotone::BigUint,
    crate::key::to_biguint,
    crate::key::biguint_to_py
);
