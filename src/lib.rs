use pyo3::prelude::*;

mod macros;

mod error;
mod handles;
mod key;
mod heaps {
    pub mod array;
    pub mod dag;
    pub mod monotone;
    pub mod tree;
}

#[pymodule]
fn _rheaps(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<handles::AddressableHandle>()?;
    m.add_class::<handles::TreeHandle>()?;
    m.add_class::<handles::HollowHandle>()?;
    m.add_class::<handles::ReflectedHandle>()?;
    m.add_class::<handles::SoftHandle>()?;
    m.add_class::<handles::RadixHandle>()?;

    m.add_class::<heaps::array::BinaryArrayHeap>()?;
    m.add_class::<heaps::array::DaryArrayHeap>()?;
    m.add_class::<heaps::array::BinaryArrayWeakHeap>()?;
    m.add_class::<heaps::array::BinaryArrayBulkInsertWeakHeap>()?;
    m.add_class::<heaps::array::MinMaxBinaryArrayDoubleEndedHeap>()?;
    m.add_class::<heaps::array::BinaryArrayAddressableHeap>()?;
    m.add_class::<heaps::array::DaryArrayAddressableHeap>()?;

    m.add_class::<heaps::tree::PairingHeap>()?;
    m.add_class::<heaps::tree::PurePairingHeap>()?;
    m.add_class::<heaps::tree::CostlessMeldPairingHeap>()?;
    m.add_class::<heaps::tree::RankPairingHeap>()?;
    m.add_class::<heaps::tree::LeftistHeap>()?;
    m.add_class::<heaps::tree::SkewHeap>()?;
    m.add_class::<heaps::tree::FibonacciHeap>()?;
    m.add_class::<heaps::tree::SimpleFibonacciHeap>()?;
    m.add_class::<heaps::tree::StrictFibonacciHeap>()?;
    m.add_class::<heaps::tree::BinaryTreeAddressableHeap>()?;
    m.add_class::<heaps::tree::DaryTreeAddressableHeap>()?;
    m.add_class::<heaps::tree::ReflectedFibonacciHeap>()?;
    m.add_class::<heaps::tree::ReflectedPairingHeap>()?;
    m.add_class::<heaps::tree::SoftHeap>()?;
    m.add_class::<heaps::tree::SoftAddressableHeap>()?;

    m.add_class::<heaps::dag::HollowHeap>()?;

    m.add_class::<heaps::monotone::U32RadixHeap>()?;
    m.add_class::<heaps::monotone::U64RadixHeap>()?;
    m.add_class::<heaps::monotone::F64RadixHeap>()?;
    m.add_class::<heaps::monotone::BigUintRadixHeap>()?;
    m.add_class::<heaps::monotone::U32RadixAddressableHeap>()?;
    m.add_class::<heaps::monotone::U64RadixAddressableHeap>()?;
    m.add_class::<heaps::monotone::F64RadixAddressableHeap>()?;
    m.add_class::<heaps::monotone::BigUintRadixAddressableHeap>()?;

    Ok(())
}
