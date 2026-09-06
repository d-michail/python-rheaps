.. _heaps:

Heaps
*****

.. currentmodule:: rheaps

Every class below (except the radix heaps) accepts a ``key_type`` argument
at construction: ``int`` or ``float`` select a native-key monomorphization
with no per-comparison Python callback; the default, ``object``, accepts any
Python object ordered via its own ``__lt__``/``__eq__``. Values (where
applicable) are always arbitrary Python objects and default to ``None`` when
omitted from ``insert()``.

Value-only heaps
^^^^^^^^^^^^^^^^^

These heaps store keys only -- no associated value, and no handle. This is
the simplest, `heapq`-like interface: ``push()``, ``peek()``, ``pop()``.

.. autoclass:: rheaps.BinaryArrayHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.DaryArrayHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.BinaryArrayWeakHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.BinaryArrayBulkInsertWeakHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.MinMaxBinaryArrayDoubleEndedHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.SoftHeap
   :members:
   :undoc-members:

Addressable heaps
^^^^^^^^^^^^^^^^^

Addressable heaps support key-value pairs. ``insert()`` returns a
:ref:`handle <handles>`, used later to inspect, update, decrease the key of,
or delete that entry -- see :doc:`../tutorials/addressable_heap` for a
worked example.

.. autoclass:: rheaps.PairingHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.PurePairingHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.CostlessMeldPairingHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.RankPairingHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.LeftistHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.SkewHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.FibonacciHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.SimpleFibonacciHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.StrictFibonacciHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.BinaryTreeAddressableHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.DaryTreeAddressableHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.SoftAddressableHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.HollowHeap
   :members:
   :undoc-members:

Double-ended addressable heaps
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

These additionally support ``peek_max()``, ``pop_max()``, and
``increase_key()``.

.. autoclass:: rheaps.ReflectedFibonacciHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.ReflectedPairingHeap
   :members:
   :undoc-members:

Monotone radix heaps
^^^^^^^^^^^^^^^^^^^^

Radix heaps enforce that removed keys are nondecreasing, and require an
inclusive ``[minimum_key, maximum_key]`` bound at construction; violating
either raises ``ValueError``. Unlike every heap above, a radix heap's key
type is fixed by the class itself (there is no ``key_type`` argument).

.. autoclass:: rheaps.U32RadixHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.U64RadixHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.F64RadixHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.BigUintRadixHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.U32RadixAddressableHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.U64RadixAddressableHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.F64RadixAddressableHeap
   :members:
   :undoc-members:

.. autoclass:: rheaps.BigUintRadixAddressableHeap
   :members:
   :undoc-members:
