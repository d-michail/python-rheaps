# -*- coding: utf-8 -*-

"""
Radix Heap
==========

In this example we create a monotone radix heap: a heap whose keys must lie
within a fixed range declared up front, and must be removed in nondecreasing
order. This is exactly the access pattern of Dijkstra's algorithm over
non-negative edge weights, where a radix heap can outperform a
comparison-based heap.
"""

# %%
# Start by importing the package.

import rheaps

# %%
# Unlike the other heaps in this library, a radix heap's key type is fixed
# by the class itself -- there is no ``key_type`` argument. Construction
# also requires an inclusive ``[minimum_key, maximum_key]`` bound.

heap = rheaps.U32RadixHeap(0, 1000)

# %%
# Insertion uses `try_push` rather than `push`, since it can fail: a key
# outside the declared bounds raises `ValueError`.

heap.try_push(30)
heap.try_push(10)
heap.try_push(500)

print('Minimum: {}'.format(heap.peek()))
print('Pop order: {}'.format([heap.pop() for _ in range(3)]))

# %%
# The defining restriction is monotonicity: once a key has been removed, no
# smaller key may be inserted afterward. Using a fresh heap to demonstrate:

monotone_demo = rheaps.U32RadixHeap(0, 1000)
monotone_demo.try_push(50)
monotone_demo.pop()
try:
    monotone_demo.try_push(10)
except ValueError as error:
    print('Rejected: {}'.format(error))

# %%
# The addressable variant additionally supports values, handles, and a
# monotonicity-respecting `decrease_key` -- the new key must still be no
# less than the last key removed from the heap.

addressable = rheaps.U32RadixAddressableHeap(0, 1000)
handle = addressable.try_insert(200, "checkpoint")
addressable.decrease_key(handle, 150)
print('key: {}, value: {}'.format(addressable.key(handle), addressable.value(handle)))

# %%
# `F64RadixHeap` and `BigUintRadixHeap` provide the same monotone structure
# for finite floating-point and arbitrary-precision non-negative integer
# keys, respectively.

big = rheaps.BigUintRadixHeap(0, 10**30)
big.try_push(10**20)
big.try_push(5)
print([big.pop(), big.pop()])
