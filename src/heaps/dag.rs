//! DAG-based heap wrappers (`rheaps::dag`).

use pyo3::prelude::*;

crate::addressable_heap_pyclass!(HollowHeap, HollowHeapRepr, rheaps::dag::HollowHeap);
crate::addressable_heap_methods!(HollowHeap, HollowHeapRepr, crate::handles::HollowHandle);
crate::decrease_key_methods!(HollowHeap, HollowHeapRepr, crate::handles::HollowHandle);
crate::meldable_methods!(HollowHeap, HollowHeapRepr, rheaps::dag::HollowHeap);
