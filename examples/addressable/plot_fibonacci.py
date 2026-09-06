# -*- coding: utf-8 -*-

"""
Fibonacci Heap
==============

In this example we create a Fibonacci heap.
"""

# %%
# Start by importing the package.

import rheaps

# %%
# Create a Fibonacci heap with ``int`` keys this time.

heap = rheaps.FibonacciHeap(key_type=int)

# %%
# Adding elements can be performed using insert.

for i in range(1, 100):
    heap.insert(i, 1000 + i)

print('Total elements so far: {}'.format(len(heap)))

# %%
# Fibonacci heaps are the classic structure behind an amortized O(1)
# decrease-key, which is what makes Dijkstra's algorithm run in
# O(E + V log V) rather than O(E log V). Insert an element, then decrease
# its key to make it the new minimum.

handle = heap.insert(500, "shortest path candidate")
_, cur_min_key, _ = heap.peek()
heap.decrease_key(handle, cur_min_key - 1)

handle_min, key_min, value_min = heap.peek()
assert handle_min == handle
print('New minimum key: {}, value: {}'.format(key_min, value_min))

# %%
# Deleting a specific element (not necessarily the minimum) is done through
# its handle.

heap.delete(handle)
print('Size of heap after deletion: {}'.format(len(heap)))

# %%
# Popping repeatedly removes elements in nondecreasing key order.

first_five = [heap.pop()[0] for _ in range(5)]
print('Five smallest remaining keys: {}'.format(first_five))

# %%
# Clearing the heap removes every element and invalidates every outstanding
# handle.

heap.clear()
print('Heap is empty: {}'.format(heap.is_empty()))
