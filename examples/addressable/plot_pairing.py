# -*- coding: utf-8 -*-

"""
Pairing Heap
============

In this example we create a pairing heap.
"""

# %%
# Start by importing the package.

import rheaps

# %%
# Create a pairing heap. By default, an addressable heap accepts any
# comparable Python object as a key (``key_type=object``); pass
# ``key_type=int`` or ``key_type=float`` for a native, faster key
# representation. We use ``float`` keys here.

heap = rheaps.PairingHeap(key_type=float)

# %%
# Adding elements can be performed using insert. We next add an element with
# key equal to 3.14 and value 100. Moreover, we add a few more elements.

heap.insert(3.14, 100)

for i in range(1, 100):
    heap.insert(float(i), 1000 + i)

# %%
# Now our heap has 100 elements.

print('Total elements so far: {}'.format(len(heap)))

# %%
# If we never need to refer to that element again, except from possibly
# accessing it when its key is minimum in the heap, we are done. Otherwise,
# when inserting an element we are given back a `handle` which we can later
# use to refer to that particular element.

handle1 = heap.insert(15.3, 200)

# %%
# Using the handle, together with the heap, we can read the key and value of
# the element -- unlike a stateful wrapper object, the handle is just a
# small opaque value; the heap holds the actual data.

print('key: {}, value: {}'.format(heap.key(handle1), heap.value(handle1)))

# %%
# We can also adjust its value.

heap.set_value(handle1, 250)
print('key: {}, value: {}'.format(heap.key(handle1), heap.value(handle1)))

# %%
# Method `peek` returns a `(handle, key, value)` triple for the current
# minimum element, without removing it. Right now that's the element we
# inserted with key 1.0.

before_handle, before_key, before_value = heap.peek()
print('current minimum -- key: {}, value: {}'.format(before_key, before_value))

# %%
# Adjusting a key is more limited than setting a value: we can only increase
# an element's priority, thus decrease its key. Decreasing `handle1`'s key
# below the current minimum makes it the new minimum.

heap.decrease_key(handle1, before_key - 1.0)
after_handle, after_key, after_value = heap.peek()
print('new minimum -- key: {}, value: {}'.format(after_key, after_value))
assert after_handle == handle1

# %%
# Method `pop` removes the minimum element and returns its `(key, value)`
# pair. The handle that used to address it (`handle1`) is no longer valid
# afterward. With `handle1`'s entry gone, the original minimum from before
# is back on top.

popped_key, popped_value = heap.pop()
print('popped -- key: {}, value: {}'.format(popped_key, popped_value))
_, restored_key, restored_value = heap.peek()
print('new current minimum -- key: {}, value: {}'.format(restored_key, restored_value))

# %%
# Except for decreasing the key, handles are also useful when we want to
# delete an element which is not the element with the minimum key. We next
# insert an element and then remove it.

print('Size of heap before insertion: {}'.format(len(heap)))
handle4 = heap.insert(50.5, 103)
print('Size of heap after insertion: {}'.format(len(heap)))
heap.delete(handle4)
print('Size of heap after deletion: {}'.format(len(heap)))

# %%
# Two pairing heaps of the same key type can be efficiently melded together.
# The donor is left as a valid, empty heap afterward -- handles it had
# already issued remain valid, now addressed through the receiver.

other = rheaps.PairingHeap(key_type=float)
other.insert(0.5, "urgent")
heap.meld(other)
print('Size of heap after meld: {}, size of donor: {}'.format(len(heap), len(other)))

# %%
# Clearing the heap can be done using method `clear`.

heap.clear()

print('Size of heap: {}'.format(len(heap)))
print('Heap is empty: {}'.format(heap.is_empty()))
