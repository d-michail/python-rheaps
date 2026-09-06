//! Array-backed heap wrappers (`rheaps::array`).

use pyo3::prelude::*;

crate::value_heap_pyclass!(BinaryArrayHeap, BinaryArrayHeapRepr, rheaps::array::BinaryArrayHeap);
crate::value_heap_methods!(BinaryArrayHeap, BinaryArrayHeapRepr);

crate::degree_value_heap_pyclass!(
    DaryArrayHeap,
    DaryArrayHeapRepr,
    rheaps::array::DaryArrayHeap,
    crate::error::invalid_degree
);
crate::value_heap_methods!(DaryArrayHeap, DaryArrayHeapRepr);

crate::value_heap_pyclass!(
    BinaryArrayWeakHeap,
    BinaryArrayWeakHeapRepr,
    rheaps::array::BinaryArrayWeakHeap
);
crate::value_heap_methods!(BinaryArrayWeakHeap, BinaryArrayWeakHeapRepr);

crate::value_heap_pyclass!(
    BinaryArrayBulkInsertWeakHeap,
    BinaryArrayBulkInsertWeakHeapRepr,
    rheaps::array::BinaryArrayBulkInsertWeakHeap
);
crate::value_heap_methods!(BinaryArrayBulkInsertWeakHeap, BinaryArrayBulkInsertWeakHeapRepr);

crate::value_heap_pyclass!(
    MinMaxBinaryArrayDoubleEndedHeap,
    MinMaxBinaryArrayDoubleEndedHeapRepr,
    rheaps::array::MinMaxBinaryArrayDoubleEndedHeap
);
crate::value_heap_methods!(MinMaxBinaryArrayDoubleEndedHeap, MinMaxBinaryArrayDoubleEndedHeapRepr);
crate::double_ended_value_heap_methods!(
    MinMaxBinaryArrayDoubleEndedHeap,
    MinMaxBinaryArrayDoubleEndedHeapRepr
);

crate::addressable_heap_pyclass!(
    BinaryArrayAddressableHeap,
    BinaryArrayAddressableHeapRepr,
    rheaps::array::BinaryArrayAddressableHeap
);
crate::addressable_heap_methods!(
    BinaryArrayAddressableHeap,
    BinaryArrayAddressableHeapRepr,
    crate::handles::AddressableHandle
);
crate::decrease_key_methods!(
    BinaryArrayAddressableHeap,
    BinaryArrayAddressableHeapRepr,
    crate::handles::AddressableHandle
);

crate::degree_addressable_heap_pyclass!(
    DaryArrayAddressableHeap,
    DaryArrayAddressableHeapRepr,
    rheaps::array::DaryArrayAddressableHeap,
    crate::error::invalid_degree
);
crate::addressable_heap_methods!(
    DaryArrayAddressableHeap,
    DaryArrayAddressableHeapRepr,
    crate::handles::AddressableHandle
);
crate::decrease_key_methods!(
    DaryArrayAddressableHeap,
    DaryArrayAddressableHeapRepr,
    crate::handles::AddressableHandle
);
