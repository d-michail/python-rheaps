import pytest

import rheaps


@pytest.mark.parametrize("key_type,keys", [
    (int, (10, 5)),
    (float, (10.0, 5.0)),
    (object, ("j", "a")),
])
def test_pairing_heap_insert_peek_pop(key_type, keys):
    heap = rheaps.PairingHeap(key_type=key_type)
    first, second = keys
    heap.insert(first, "first")
    handle = heap.insert(second, "second")
    assert len(heap) == 2
    assert heap.peek() == (handle, second, "second")
    assert heap.pop() == (second, "second")
    assert heap.pop() == (first, "first")
    assert heap.pop() is None


def test_pairing_heap_decrease_key_and_delete():
    heap = rheaps.PairingHeap()
    task = heap.insert(10, "compile report")
    heap.insert(5, "answer mail")
    heap.decrease_key(task, 1)
    assert heap.key(task) == 1
    assert heap.value(task) == "compile report"
    assert heap.delete(task) == (1, "compile report")
    assert heap.pop() == (5, "answer mail")


def test_pairing_heap_decrease_key_rejects_increase():
    heap = rheaps.PairingHeap()
    handle = heap.insert(5, "x")
    with pytest.raises(ValueError):
        heap.decrease_key(handle, 10)


def test_pairing_heap_set_value():
    heap = rheaps.PairingHeap()
    handle = heap.insert(1, "old")
    heap.set_value(handle, "new")
    assert heap.value(handle) == "new"


def test_pairing_heap_insert_defaults_value_to_none():
    heap = rheaps.PairingHeap()
    handle = heap.insert(1)
    assert heap.value(handle) is None


def test_pairing_heap_meld_consumes_donor():
    receiver = rheaps.PairingHeap()
    receiver.insert(3)
    receiver.insert(5)

    donor = rheaps.PairingHeap()
    deadline = donor.insert(4, "deadline")
    donor.insert(9)

    receiver.meld(donor)
    receiver.decrease_key(deadline, 1)

    assert [receiver.pop()[0] for _ in range(4)] == [1, 3, 5, 9]
    # donor is left as a valid, empty heap of the same key_type, not garbage.
    assert len(donor) == 0
    assert donor.pop() is None
    donor.insert(42)
    assert len(donor) == 1


def test_meld_rejects_mismatched_key_type():
    a = rheaps.PairingHeap(key_type=int)
    b = rheaps.PairingHeap(key_type=object)
    with pytest.raises(ValueError):
        a.meld(b)


def test_handle_rejected_by_foreign_heap():
    a = rheaps.PairingHeap()
    b = rheaps.PairingHeap()
    handle = a.insert(1, "x")
    with pytest.raises(ValueError):
        b.key(handle)


def test_handle_rejected_after_delete():
    heap = rheaps.PairingHeap()
    handle = heap.insert(1, "x")
    heap.delete(handle)
    with pytest.raises(ValueError):
        heap.key(handle)


def test_handle_rejected_after_clear():
    heap = rheaps.PairingHeap()
    handle = heap.insert(1, "x")
    heap.clear()
    with pytest.raises(ValueError):
        heap.value(handle)


def test_object_key_incomparable_types_raise_typeerror():
    heap = rheaps.PairingHeap(key_type=object)
    heap.insert(1, "int key")
    with pytest.raises(TypeError):
        heap.insert("not comparable to int", "str key")


def test_hollow_heap_matches_addressable_shape():
    heap = rheaps.HollowHeap()
    task = heap.insert(10, "compile report")
    heap.insert(5, "answer mail")
    heap.decrease_key(task, 1)
    assert heap.peek() == (task, 1, "compile report")
    assert heap.delete(task) == (1, "compile report")


def test_reflected_pairing_heap_double_ended():
    heap = rheaps.ReflectedPairingHeap()
    heap.insert(5, "a")
    handle = heap.insert(9, "b")
    heap.insert(1, "c")
    assert heap.peek_max() == (handle, 9, "b")
    heap.increase_key(handle, 20)
    assert heap.peek_max()[1] == 20
    assert heap.pop_max() == (20, "b")


def test_dary_tree_addressable_heap_degree():
    heap = rheaps.DaryTreeAddressableHeap(4)
    handle = heap.insert(5, "x")
    heap.decrease_key(handle, 1)
    assert heap.pop() == (1, "x")


def test_soft_heap_meld_and_rank_limit():
    a = rheaps.SoftHeap(0.1)
    a.push(3)
    b = rheaps.SoftHeap(0.1)
    b.push(1)
    a.meld(b)
    assert len(a) == 2
    assert len(b) == 0
    assert isinstance(a.rank_limit(), int)


def test_soft_addressable_heap_has_no_decrease_key():
    heap = rheaps.SoftAddressableHeap(0.1)
    assert not hasattr(heap, "decrease_key")
