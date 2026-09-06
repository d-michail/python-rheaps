import pytest

import rheaps


def test_u32_radix_heap_basic():
    heap = rheaps.U32RadixHeap(0, 100)
    heap.try_push(30)
    heap.try_push(10)
    assert heap.pop() == 10
    assert heap.pop() == 30
    assert heap.pop() is None


def test_u32_radix_heap_rejects_out_of_range_key():
    heap = rheaps.U32RadixHeap(0, 100)
    with pytest.raises(ValueError):
        heap.try_push(200)


def test_u32_radix_heap_rejects_monotonicity_violation():
    heap = rheaps.U32RadixHeap(0, 100)
    heap.try_push(50)
    assert heap.pop() == 50
    with pytest.raises(ValueError):
        heap.try_push(10)


def test_u32_radix_heap_rejects_invalid_bounds():
    with pytest.raises(ValueError):
        rheaps.U32RadixHeap(100, 0)


def test_f64_radix_heap_finite_keys_only():
    heap = rheaps.F64RadixHeap(0.0, 100.0)
    heap.try_push(2.5)
    assert heap.pop() == 2.5
    with pytest.raises(ValueError):
        heap.try_push(float("nan"))


def test_biguint_radix_heap():
    heap = rheaps.BigUintRadixHeap(0, 10**30)
    heap.try_push(10**20)
    heap.try_push(5)
    assert heap.pop() == 5
    assert heap.pop() == 10**20


def test_addressable_radix_heap_decrease_key_and_handles():
    heap = rheaps.U32RadixAddressableHeap(0, 100)
    handle = heap.try_insert(50, "task")
    heap.decrease_key(handle, 10)
    assert heap.key(handle) == 10
    assert heap.peek() == (handle, 10, "task")
    assert heap.delete(handle) == (10, "task")


def test_addressable_radix_heap_decrease_key_violation():
    heap = rheaps.U32RadixAddressableHeap(0, 100)
    handle = heap.try_insert(50, "task")
    with pytest.raises(ValueError):
        heap.decrease_key(handle, 60)
