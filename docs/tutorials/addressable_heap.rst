.. _tutorials/addressable_heap:

.. currentmodule:: rheaps

Addressable Heap
================

The most classic addressable heap is the pairing heap, so let us create one.

Creating an addressable heap
----------------------------

If not explicitly given a ``key_type``, the heap accepts any comparable
Python object as a key (``key_type=object``); pass ``key_type=int`` or
``key_type=float`` for a native, faster key representation. Values are
always arbitrary Python objects.

.. code-block:: python

   >>> import rheaps
   >>> heap = rheaps.PairingHeap(key_type=float)

Inserting elements
------------------

When inserting an element into an addressable heap, :meth:`.PairingHeap.insert`
returns a *handle* which can later be used to perform operations on that
element, such as decreasing its key (increasing its priority) or deleting it
from the heap.

.. code-block:: python

   >>> handle1 = heap.insert(5.5, 1)
   >>> handle2 = heap.insert(100.5, 2)
   >>> handle3 = heap.insert(3.3, 3)
   >>> handle4 = heap.insert(52.3, 4)
   >>> handle5 = heap.insert(30.0, 5)

Reading an element's key and value
-----------------------------------

Unlike a stateful wrapper object, a handle here is a small, opaque,
`hashable` value; the key and value it addresses are read from the *heap*,
not the handle itself:

.. code-block:: python

   >>> print('Key: {}'.format(heap.key(handle1)))
   Key: 5.5
   >>> print('Value: {}'.format(heap.value(handle1)))
   Value: 1

Values can also be changed directly:

.. code-block:: python

   >>> heap.set_value(handle1, 15)
   >>> heap.value(handle1)
   15

As keys can only be decreased, there is a dedicated method for this,
:meth:`.PairingHeap.decrease_key`. It raises ``ValueError`` if the proposed
key is not lower than the element's current key.

.. code-block:: python

   >>> heap.decrease_key(handle1, 4.5)

Inspecting the size of the heap
--------------------------------

The number of elements in the heap can be found using ``len()``:

.. code-block:: python

   >>> print(len(heap))
   5

Method :meth:`.PairingHeap.is_empty` can be used to check if the heap is
empty.

.. code-block:: python

   >>> heap.is_empty()
   False

Finding the element with the minimum key
------------------------------------------

:meth:`.PairingHeap.peek` returns ``(handle, key, value)`` for a minimum
entry, without removing it -- or ``None`` if the heap is empty.

.. code-block:: python

   >>> handle, key, value = heap.peek()

Removing the element with the minimum key
--------------------------------------------

:meth:`.PairingHeap.pop` removes and returns ``(key, value)`` for a minimum
entry -- or ``None`` if the heap is empty. Unlike ``peek()``, the handle is
no longer valid afterward, since the entry is gone.

.. code-block:: python

   >>> key, value = heap.pop()

Deleting elements
-----------------

Searching in heaps is not possible, but elements can be deleted given their
handle. Given a handle, use :meth:`.PairingHeap.delete`:

.. code-block:: python

   >>> heap.delete(handle4)
   (52.3, 4)

Using ``handle4`` again after this -- for example, calling ``heap.key(handle4)``
-- raises ``ValueError``, since the handle no longer identifies a live entry.

Increasing the priority of an element
--------------------------------------

Sometimes we want to increase the priority of an element in the heap (recall
how Dijkstra's algorithm works). This is exactly what
:meth:`.PairingHeap.decrease_key` is for:

.. code-block:: python

   >>> heap.decrease_key(handle2, 1.5)

Melding two heaps
-----------------

Several addressable heaps (including :class:`.PairingHeap`) support
efficiently absorbing another heap of the *same class and* ``key_type``
via :meth:`.PairingHeap.meld`. The donor is left as a valid, empty heap
afterward; handles it had already issued remain valid, now through the
receiver:

.. code-block:: python

   >>> other = rheaps.PairingHeap(key_type=float)
   >>> deadline = other.insert(2.0, "deadline")
   >>> heap.meld(other)
   >>> len(other)
   0
   >>> heap.decrease_key(deadline, 0.1)

Clearing the heap
-----------------

All elements of the heap can be removed by using method
:meth:`.PairingHeap.clear`. Every handle issued before the call becomes
invalid.

.. code-block:: python

   >>> heap.clear()
   >>> len(heap)
   0
