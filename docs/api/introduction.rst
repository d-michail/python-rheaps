.. _introduction:

Introduction
************

.. currentmodule:: rheaps

The |Project| library is a highly efficient library containing state-of-the-art
heap implementations, built with `PyO3 <https://pyo3.rs>`_ directly on top of
the `rheaps <https://docs.rs/rheaps>`_ Rust crate: no separate backend
library, JVM, or shared C library to load.

Unlike some heap bindings, which hide their concrete implementations behind
factory functions returning a common interface, |Bindings| exposes one
concrete Python class per algorithm (:class:`.PairingHeap`,
:class:`.FibonacciHeap`, and so on) -- each is a real, documented,
`isinstance`-able class rather than an opaque object behind a factory.

Every general-purpose heap class accepts a ``key_type`` constructor argument
(``int``, ``float``, or the default ``object``), letting a single class
support both a fast native-key path and arbitrary Python objects ordered via
their own ``__lt__``/``__eq__``. Values (where applicable) are always
arbitrary Python objects.

Available heaps:

  * Tree-based.

    * Fibonacci mergeable and addressable heaps (:class:`.FibonacciHeap`,
      :class:`.SimpleFibonacciHeap`, :class:`.StrictFibonacciHeap`).

    * Pairing mergeable and addressable heaps (:class:`.PairingHeap`,
      :class:`.PurePairingHeap`, :class:`.CostlessMeldPairingHeap`).

    * Rank-Pairing mergeable and addressable heaps (:class:`.RankPairingHeap`).

    * Leftist mergeable and addressable heaps (:class:`.LeftistHeap`).

    * Skew heaps (:class:`.SkewHeap`).

    * Explicit binary and d-ary tree addressable heaps
      (:class:`.BinaryTreeAddressableHeap`, :class:`.DaryTreeAddressableHeap`).

    * Binary tree soft heaps (:class:`.SoftHeap`, :class:`.SoftAddressableHeap`).

  * Dag-based.

    * Hollow mergeable and addressable heaps (:class:`.HollowHeap`).

  * Double-ended mergeable and addressable heaps.

    * Reflected Fibonacci heaps (:class:`.ReflectedFibonacciHeap`).

    * Reflected Pairing heaps (:class:`.ReflectedPairingHeap`).

  * Array-based.

    * Binary heaps and binary addressable heaps
      (:class:`.BinaryArrayHeap`, :class:`.BinaryArrayAddressableHeap`).

    * D-ary heaps and d-ary addressable heaps
      (:class:`.DaryArrayHeap`, :class:`.DaryArrayAddressableHeap`).

    * Binary weak heaps, and a variant supporting bulk insertion
      (:class:`.BinaryArrayWeakHeap`, :class:`.BinaryArrayBulkInsertWeakHeap`).

  * Double-ended array-based.

    * Binary MinMax heaps (:class:`.MinMaxBinaryArrayDoubleEndedHeap`).

  * Monotone heaps.

    * Addressable and non-addressable radix heaps with ``u32``, ``u64``,
      finite ``float``, and arbitrary-precision non-negative integer keys
      (:class:`.U32RadixHeap`, :class:`.U64RadixHeap`, :class:`.F64RadixHeap`,
      :class:`.BigUintRadixHeap`, and their ``*RadixAddressableHeap``
      counterparts).
