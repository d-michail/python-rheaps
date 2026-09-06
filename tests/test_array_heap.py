import pytest

import rheaps


def test_binary_array_heap_object_keys():
    heap = rheaps.BinaryArrayHeap()
    heap.push(4)
    heap.push(1)
    heap.push(3)
    assert len(heap) == 3
    assert heap.peek() == 1
    assert heap.pop() == 1
    assert heap.pop() == 3
    assert heap.pop() == 4
    assert heap.pop() is None
    assert heap.is_empty()


def test_binary_array_heap_int_keys():
    heap = rheaps.BinaryArrayHeap(key_type=int)
    for value in (5, 2, 8, 1):
        heap.push(value)
    assert [heap.pop() for _ in range(4)] == [1, 2, 5, 8]


def test_binary_array_heap_float_keys():
    heap = rheaps.BinaryArrayHeap(key_type=float)
    heap.push(2.5)
    heap.push(1.5)
    assert heap.pop() == 1.5
    assert heap.pop() == 2.5


def test_binary_array_heap_object_key_ordering_uses_lt():
    heap = rheaps.BinaryArrayHeap(key_type=object)
    heap.push((2, "b"))
    heap.push((1, "a"))
    heap.push((1, "z"))
    assert heap.pop() == (1, "a")
    assert heap.pop() == (1, "z")
    assert heap.pop() == (2, "b")


def test_dary_array_heap_degree_and_clear():
    heap = rheaps.DaryArrayHeap(4)
    assert heap.degree() == 4
    heap.push(3)
    heap.push(1)
    heap.clear()
    assert len(heap) == 0
    assert heap.peek() is None


def test_dary_array_heap_invalid_degree_raises():
    with pytest.raises(ValueError):
        rheaps.DaryArrayHeap(1)


def test_minmax_double_ended_heap():
    heap = rheaps.MinMaxBinaryArrayDoubleEndedHeap()
    for value in (5, 1, 9, 3):
        heap.push(value)
    assert heap.peek() == 1
    assert heap.peek_max() == 9
    assert heap.pop_max() == 9
    assert heap.pop() == 1
    assert sorted([heap.pop(), heap.pop()]) == [3, 5]
