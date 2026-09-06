.. _handles:

Handles
*******

.. currentmodule:: rheaps

A handle is an opaque, checked capability returned by an addressable heap's
``insert()``. It is used to inspect, update, decrease the key of, or delete
that specific entry later, and is rejected with ``ValueError`` once it is
stale (its entry was removed, or the heap was cleared) or belongs to a
different heap instance.

Handles are plain values (comparable and hashable), not owning references --
there is nothing to close or clean up. Each family of heap algorithms shares
one handle type:

.. autoclass:: rheaps.AddressableHandle
   :members:
   :undoc-members:

.. autoclass:: rheaps.TreeHandle
   :members:
   :undoc-members:

.. autoclass:: rheaps.HollowHandle
   :members:
   :undoc-members:

.. autoclass:: rheaps.ReflectedHandle
   :members:
   :undoc-members:

.. autoclass:: rheaps.SoftHandle
   :members:
   :undoc-members:

.. autoclass:: rheaps.RadixHandle
   :members:
   :undoc-members:
