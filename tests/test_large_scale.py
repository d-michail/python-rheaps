"""Large-scale conformance tests, ported from the invariants exercised by
`rheaps`' own Rust test suite (`src/{array,tree,dag,monotone}/tests.rs`):
ascending/descending insertion order, randomized insert-then-drain against a
plain `sorted()` oracle, arbitrary-order deletion, bulk `decrease_key`/
`increase_key`, melding, and (for soft heaps) the weaker "every inserted key
is eventually returned, in some order" invariant that corruption permits.

These run thousands of heap operations per parametrized class rather than
the handful used by the other test files, so they take noticeably longer;
that's the point.
"""

import random

import pytest

import rheaps

VALUE_STRESS_SIZE = 20_000
ADDRESSABLE_STRESS_SIZE = 5_000
RADIX_STRESS_SIZE = 10_000
SOFT_STRESS_SIZE = 4_000

KEY_BOUND = 2_000_000


def drain(heap):
    result = []
    while True:
        value = heap.pop()
        if value is None:
            break
        result.append(value)
    return result


def drain_max(heap):
    result = []
    while True:
        value = heap.pop_max()
        if value is None:
            break
        result.append(value)
    return result


VALUE_HEAP_FACTORIES = {
    "BinaryArrayHeap": lambda: rheaps.BinaryArrayHeap(key_type=int),
    "DaryArrayHeap(2)": lambda: rheaps.DaryArrayHeap(2, key_type=int),
    "DaryArrayHeap(3)": lambda: rheaps.DaryArrayHeap(3, key_type=int),
    "DaryArrayHeap(4)": lambda: rheaps.DaryArrayHeap(4, key_type=int),
    "BinaryArrayWeakHeap": lambda: rheaps.BinaryArrayWeakHeap(key_type=int),
    "BinaryArrayBulkInsertWeakHeap": lambda: rheaps.BinaryArrayBulkInsertWeakHeap(key_type=int),
}


@pytest.mark.parametrize("make_heap", VALUE_HEAP_FACTORIES.values(), ids=VALUE_HEAP_FACTORIES.keys())
def test_value_heap_ascending_then_descending(make_heap):
    heap = make_heap()
    for value in range(VALUE_STRESS_SIZE):
        heap.push(value)
        assert heap.peek() == 0
        assert len(heap) == value + 1
    for expected in range(VALUE_STRESS_SIZE):
        assert heap.peek() == expected
        assert heap.pop() == expected
    assert heap.is_empty()

    heap = make_heap()
    for value in reversed(range(VALUE_STRESS_SIZE)):
        heap.push(value)
        assert heap.peek() == value
    assert len(heap) == VALUE_STRESS_SIZE


@pytest.mark.parametrize("make_heap", VALUE_HEAP_FACTORIES.values(), ids=VALUE_HEAP_FACTORIES.keys())
def test_value_heap_random_matches_sorted_oracle(make_heap):
    rng = random.Random(1)
    heap = make_heap()
    values = [rng.randrange(-KEY_BOUND, KEY_BOUND) for _ in range(VALUE_STRESS_SIZE)]
    for value in values:
        heap.push(value)
    assert len(heap) == VALUE_STRESS_SIZE

    result = []
    while not heap.is_empty():
        top = heap.peek()
        popped = heap.pop()
        assert popped == top
        result.append(popped)
    assert result == sorted(values)


def test_minmax_double_ended_heap_random_matches_sorted_oracle():
    rng = random.Random(2)
    heap = rheaps.MinMaxBinaryArrayDoubleEndedHeap(key_type=int)
    values = [rng.randrange(-KEY_BOUND, KEY_BOUND) for _ in range(VALUE_STRESS_SIZE)]
    for value in values:
        heap.push(value)

    assert drain_max(heap) == sorted(values, reverse=True)


def test_minmax_double_ended_heap_pop_min_matches_sorted_oracle():
    rng = random.Random(3)
    heap = rheaps.MinMaxBinaryArrayDoubleEndedHeap(key_type=int)
    values = [rng.randrange(-KEY_BOUND, KEY_BOUND) for _ in range(VALUE_STRESS_SIZE)]
    for value in values:
        heap.push(value)

    assert drain(heap) == sorted(values)


ADDRESSABLE_HEAP_FACTORIES = {
    "PairingHeap": lambda: rheaps.PairingHeap(key_type=int),
    "PurePairingHeap": lambda: rheaps.PurePairingHeap(key_type=int),
    "CostlessMeldPairingHeap": lambda: rheaps.CostlessMeldPairingHeap(key_type=int),
    "RankPairingHeap": lambda: rheaps.RankPairingHeap(key_type=int),
    "LeftistHeap": lambda: rheaps.LeftistHeap(key_type=int),
    "SkewHeap": lambda: rheaps.SkewHeap(key_type=int),
    "FibonacciHeap": lambda: rheaps.FibonacciHeap(key_type=int),
    "SimpleFibonacciHeap": lambda: rheaps.SimpleFibonacciHeap(key_type=int),
    "StrictFibonacciHeap": lambda: rheaps.StrictFibonacciHeap(key_type=int),
    "BinaryTreeAddressableHeap": lambda: rheaps.BinaryTreeAddressableHeap(key_type=int),
    "DaryTreeAddressableHeap(4)": lambda: rheaps.DaryTreeAddressableHeap(4, key_type=int),
    "HollowHeap": lambda: rheaps.HollowHeap(key_type=int),
    "BinaryArrayAddressableHeap": lambda: rheaps.BinaryArrayAddressableHeap(key_type=int),
    "DaryArrayAddressableHeap(4)": lambda: rheaps.DaryArrayAddressableHeap(4, key_type=int),
    "ReflectedFibonacciHeap": lambda: rheaps.ReflectedFibonacciHeap(key_type=int),
    "ReflectedPairingHeap": lambda: rheaps.ReflectedPairingHeap(key_type=int),
}

# Meld is not supported by these -- everything else in
# ADDRESSABLE_HEAP_FACTORIES implements MeldableAddressableHeap.
NOT_MELDABLE = (
    "BinaryTreeAddressableHeap",
    "DaryTreeAddressableHeap(4)",
    "BinaryArrayAddressableHeap",
    "DaryArrayAddressableHeap(4)",
)
MELDABLE_HEAP_FACTORIES = {
    name: factory
    for name, factory in ADDRESSABLE_HEAP_FACTORIES.items()
    if name not in NOT_MELDABLE
}

DOUBLE_ENDED_ADDRESSABLE_HEAP_FACTORIES = {
    "ReflectedFibonacciHeap": rheaps.ReflectedFibonacciHeap,
    "ReflectedPairingHeap": rheaps.ReflectedPairingHeap,
}


@pytest.mark.parametrize(
    "make_heap", ADDRESSABLE_HEAP_FACTORIES.values(), ids=ADDRESSABLE_HEAP_FACTORIES.keys()
)
def test_addressable_heap_ascending_insert_and_ordered_pop(make_heap):
    heap = make_heap()
    handles = []
    for key in range(ADDRESSABLE_STRESS_SIZE):
        handles.append(heap.insert(key, key))
        assert heap.peek()[1] == 0
        assert len(heap) == key + 1

    for expected in range(ADDRESSABLE_STRESS_SIZE):
        assert heap.key(handles[expected]) == expected
        assert heap.pop() == (expected, expected)
    assert heap.is_empty()


@pytest.mark.parametrize(
    "make_heap", ADDRESSABLE_HEAP_FACTORIES.values(), ids=ADDRESSABLE_HEAP_FACTORIES.keys()
)
def test_addressable_heap_random_matches_sorted_oracle(make_heap):
    rng = random.Random(1)
    heap = make_heap()
    keys = [rng.randrange(-KEY_BOUND, KEY_BOUND) for _ in range(ADDRESSABLE_STRESS_SIZE)]
    for key in keys:
        heap.insert(key, key)

    result = []
    while not heap.is_empty():
        key, value = heap.pop()
        assert value == key
        result.append(key)
    assert result == sorted(keys)


@pytest.mark.parametrize(
    "make_heap", ADDRESSABLE_HEAP_FACTORIES.values(), ids=ADDRESSABLE_HEAP_FACTORIES.keys()
)
def test_addressable_heap_arbitrary_order_deletion(make_heap):
    size = 128
    heap = make_heap()
    handles = [heap.insert(key, key) for key in range(size)]
    live = [True] * size

    rng = random.Random(5)
    order = list(range(size))
    rng.shuffle(order)

    for index in order:
        assert heap.delete(handles[index]) == (index, index)
        live[index] = False
        expected = next((i for i, is_live in enumerate(live) if is_live), None)
        top = heap.peek()
        assert (top[1] if top is not None else None) == expected
    assert heap.is_empty()


@pytest.mark.parametrize(
    "make_heap", ADDRESSABLE_HEAP_FACTORIES.values(), ids=ADDRESSABLE_HEAP_FACTORIES.keys()
)
def test_addressable_heap_bulk_decrease_key_matches_sorted_oracle(make_heap):
    heap = make_heap()
    current = []
    handles = []
    for index in range(ADDRESSABLE_STRESS_SIZE):
        key = 2 * index
        current.append(key)
        handles.append(heap.insert(key, index))

    rng = random.Random(1)
    for _ in range(ADDRESSABLE_STRESS_SIZE // 2):
        index = rng.randrange(ADDRESSABLE_STRESS_SIZE)
        old_key = current[index]
        new_key = rng.randrange(old_key) if old_key > 0 else 0
        heap.decrease_key(handles[index], new_key)
        current[index] = new_key

    result = []
    while not heap.is_empty():
        key, _ = heap.pop()
        result.append(key)
    assert len(result) == ADDRESSABLE_STRESS_SIZE
    assert result == sorted(result)
    assert result == sorted(current)


@pytest.mark.parametrize(
    "make_heap", MELDABLE_HEAP_FACTORIES.values(), ids=MELDABLE_HEAP_FACTORIES.keys()
)
def test_addressable_heap_meld_matches_sorted_oracle(make_heap):
    rng = random.Random(7)
    receiver_keys = [rng.randrange(-KEY_BOUND, KEY_BOUND) for _ in range(ADDRESSABLE_STRESS_SIZE // 2)]
    donor_keys = [rng.randrange(-KEY_BOUND, KEY_BOUND) for _ in range(ADDRESSABLE_STRESS_SIZE // 2)]

    receiver = make_heap()
    for key in receiver_keys:
        receiver.insert(key, key)
    donor = make_heap()
    donor_handles = [donor.insert(key, key) for key in donor_keys]

    receiver.meld(donor)
    assert len(receiver) == len(receiver_keys) + len(donor_keys)
    assert len(donor) == 0
    assert donor.pop() is None

    # Handles issued by the donor remain valid, now through the receiver.
    for handle, key in zip(donor_handles, donor_keys):
        assert receiver.key(handle) == key

    result = []
    while not receiver.is_empty():
        key, _ = receiver.pop()
        result.append(key)
    assert result == sorted(receiver_keys + donor_keys)


@pytest.mark.parametrize(
    "make_heap",
    DOUBLE_ENDED_ADDRESSABLE_HEAP_FACTORIES.values(),
    ids=DOUBLE_ENDED_ADDRESSABLE_HEAP_FACTORIES.keys(),
)
def test_double_ended_addressable_heap_random_matches_sorted_oracle(make_heap):
    rng = random.Random(9)
    heap = make_heap()
    keys = [rng.randrange(-KEY_BOUND, KEY_BOUND) for _ in range(ADDRESSABLE_STRESS_SIZE)]
    for key in keys:
        heap.insert(key, key)

    result = []
    while not heap.is_empty():
        key, _ = heap.pop_max()
        result.append(key)
    assert result == sorted(keys, reverse=True)


@pytest.mark.parametrize(
    "make_heap",
    DOUBLE_ENDED_ADDRESSABLE_HEAP_FACTORIES.values(),
    ids=DOUBLE_ENDED_ADDRESSABLE_HEAP_FACTORIES.keys(),
)
def test_double_ended_addressable_heap_bulk_increase_key_matches_sorted_oracle(make_heap):
    heap = make_heap()
    current = []
    handles = []
    for index in range(ADDRESSABLE_STRESS_SIZE):
        key = 2 * index
        current.append(key)
        handles.append(heap.insert(key, index))

    rng = random.Random(2)
    for _ in range(ADDRESSABLE_STRESS_SIZE // 2):
        index = rng.randrange(ADDRESSABLE_STRESS_SIZE)
        old_key = current[index]
        new_key = old_key + rng.randrange(1, 1000)
        heap.increase_key(handles[index], new_key)
        current[index] = new_key

    result = []
    while not heap.is_empty():
        key, _ = heap.pop()
        result.append(key)
    assert len(result) == ADDRESSABLE_STRESS_SIZE
    assert result == sorted(current)


RADIX_HEAP_FACTORIES = {
    "U32RadixHeap": lambda: rheaps.U32RadixHeap(0, KEY_BOUND),
    "U64RadixHeap": lambda: rheaps.U64RadixHeap(0, KEY_BOUND),
    "F64RadixHeap": lambda: rheaps.F64RadixHeap(0.0, float(KEY_BOUND)),
    "BigUintRadixHeap": lambda: rheaps.BigUintRadixHeap(0, KEY_BOUND),
}


@pytest.mark.parametrize("make_heap", RADIX_HEAP_FACTORIES.values(), ids=RADIX_HEAP_FACTORIES.keys())
def test_radix_heap_random_matches_sorted_oracle(make_heap):
    rng = random.Random(11)
    heap = make_heap()
    is_float = isinstance(heap, rheaps.F64RadixHeap)
    values = [
        (rng.uniform(0, KEY_BOUND) if is_float else rng.randrange(0, KEY_BOUND))
        for _ in range(RADIX_STRESS_SIZE)
    ]
    for value in values:
        heap.try_push(value)
    assert len(heap) == RADIX_STRESS_SIZE

    assert drain(heap) == sorted(values)


def test_addressable_radix_heap_decrease_key_respects_monotonicity():
    heap = rheaps.U32RadixAddressableHeap(0, KEY_BOUND)
    current = []
    handles = []
    for index in range(RADIX_STRESS_SIZE):
        key = 2 * index
        current.append(key)
        handles.append(heap.try_insert(key, index))

    rng = random.Random(13)
    for _ in range(RADIX_STRESS_SIZE // 2):
        index = rng.randrange(RADIX_STRESS_SIZE)
        old_key = current[index]
        new_key = rng.randrange(old_key) if old_key > 0 else 0
        heap.decrease_key(handles[index], new_key)
        current[index] = new_key

    result = []
    while not heap.is_empty():
        key, _ = heap.pop()
        result.append(key)
    assert len(result) == RADIX_STRESS_SIZE
    assert result == sorted(current)


SOFT_HEAP_ERROR_RATES = [0.01, 0.25, 0.5, 0.75, 0.99]


@pytest.mark.parametrize("error_rate", SOFT_HEAP_ERROR_RATES)
def test_soft_heap_drains_every_inserted_key(error_rate):
    # A soft heap may corrupt (increase) up to `error_rate` of its keys, so
    # extraction order is not guaranteed to be sorted -- but every key
    # inserted must still come back out exactly once.
    heap = rheaps.SoftHeap(error_rate, key_type=int)
    keys = list(range(SOFT_STRESS_SIZE))
    for key in keys:
        heap.push(key)

    removed = []
    for _ in range(SOFT_STRESS_SIZE // 4):
        removed.append(heap.pop())
    removed.extend(drain(heap))

    assert sorted(removed) == keys


@pytest.mark.parametrize("error_rate", SOFT_HEAP_ERROR_RATES)
def test_soft_heap_meld_drains_every_inserted_key(error_rate):
    small = rheaps.SoftHeap(error_rate, key_type=int)
    large = rheaps.SoftHeap(error_rate, key_type=int)
    for key in range(SOFT_STRESS_SIZE // 3):
        small.push(key)
    for key in range(SOFT_STRESS_SIZE // 3, SOFT_STRESS_SIZE):
        large.push(key)

    small.meld(large)
    assert len(small) == SOFT_STRESS_SIZE
    assert len(large) == 0

    assert sorted(drain(small)) == list(range(SOFT_STRESS_SIZE))
