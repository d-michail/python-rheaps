# -*- coding: utf-8 -*-

"""
Binary Array Heap
==================

In this example we create a plain, value-only binary heap -- the simplest
heap in the library, comparable to the standard library's ``heapq`` module
but available with a choice of ``key_type`` and several other algorithms.
"""

# %%
# Start by importing the package.

import rheaps

# %%
# A value-only heap has no separate key/value pair, no handle, and no
# `insert` method -- just `push`, `peek`, and `pop`, like `heapq`. By
# default it accepts any comparable Python object; here we ask for the
# fast, native `int` representation instead.

heap = rheaps.BinaryArrayHeap(key_type=int)

for value in (5, 2, 8, 1, 9, 3):
    heap.push(value)

print('Total elements: {}'.format(len(heap)))
print('Minimum: {}'.format(heap.peek()))

# %%
# Popping repeatedly removes elements in nondecreasing order.

order = [heap.pop() for _ in range(len(heap))]
print('Pop order: {}'.format(order))
print('Heap is empty: {}'.format(heap.is_empty()))

# %%
# `key_type=object` (the default) accepts any comparable Python object,
# ordered the same way plain `<` would order them -- for example, tuples
# compare element-by-element.

names = rheaps.BinaryArrayHeap()
names.push((2, "banana"))
names.push((1, "apple"))
names.push((1, "avocado"))

print([names.pop() for _ in range(3)])

# %%
# `DaryArrayHeap` is the same idea with a configurable branching factor:
# a larger degree makes insertion cheaper but removal costlier.

wide = rheaps.DaryArrayHeap(4, key_type=int)
print('Degree: {}'.format(wide.degree()))
for value in (7, 3, 5, 1):
    wide.push(value)
print([wide.pop() for _ in range(4)])
