# python-rheaps

[![Python rheaps](https://github.com/d-michail/python-rheaps/actions/workflows/CI.yml/badge.svg)](https://github.com/d-michail/python-rheaps/actions/workflows/CI.yml)
[![PyPI](https://img.shields.io/pypi/v/rheaps.svg)](https://pypi.org/project/rheaps/)
[![Documentation Status](https://readthedocs.org/projects/python-rheaps/badge/?version=latest)](https://python-rheaps.readthedocs.io/en/latest/?badge=latest)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Python bindings for [`rheaps`](https://docs.rs/rheaps), a Rust library of heap /
priority-queue data structures, built with [PyO3](https://pyo3.rs) and
[maturin](https://www.maturin.rs/). `rheaps` is itself a Rust port of
[JHeaps](https://github.com/d-michail/jheaps); this package gives Python the
same broad selection of heap algorithms as native, object-oriented classes.

## Features

- **31 heap classes** across four families: array-backed, tree-based,
  DAG-based, and monotone radix heaps.
- **Flexible keys.** Every general-purpose heap accepts `key_type=int`,
  `key_type=float`, or the default `key_type=object` (any Python object,
  ordered via its own `__lt__`/`__eq__`, exactly like `heapq`). `int`/`float`
  keys use Rust's native comparisons with no per-comparison Python callback.
- **Arbitrary values.** Every entry can carry an arbitrary Python object as
  its value (defaults to `None` if omitted), independent of the key type.
- **Checked handles.** Addressable heaps return a handle from `insert()` used
  to inspect, update, decrease, or delete that entry later. A handle is
  rejected with `ValueError` if it's stale (its entry was removed, or the
  heap was cleared) or belongs to a different heap instance.
- **No garbage collection surprises.** Heap state lives inside the Rust
  object; there's no manual cleanup, `close()`, or context manager to
  remember.

## Installation

Not yet published to PyPI. Build and install from source with
[maturin](https://www.maturin.rs/):

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install maturin
maturin develop          # builds the extension and installs it into the venv
# maturin develop --release   # for an optimized build
```

Requires a Rust toolchain (stable, edition 2024 support — 1.88+) to build.

## Quick start

A plain, value-only heap (like `heapq`, but with more algorithm choices):

```python
import rheaps

heap = rheaps.BinaryArrayHeap()
heap.push(4)
heap.push(1)
heap.push(3)

assert heap.peek() == 1
assert heap.pop() == 1
assert heap.pop() == 3
```

An addressable heap, where `insert()` returns a handle you can use later:

```python
heap = rheaps.PairingHeap()
task = heap.insert(10, "compile report")
heap.insert(5, "answer mail")

heap.decrease_key(task, 1)
assert heap.peek() == (task, 1, "compile report")
assert heap.delete(task) == (1, "compile report")
assert heap.pop() == (5, "answer mail")
```

Melding combines two heaps of the same class and `key_type`. The donor is
left as a valid, empty heap afterward — Python has no move semantics, so
unlike Rust's `rheaps` (which makes reusing a melded-away heap a compile
error), reusing the donor here is well-defined, just empty:

```python
a = rheaps.PairingHeap()
a.insert(3)
a.insert(5)

b = rheaps.PairingHeap()
deadline = b.insert(4)
b.insert(9)

a.meld(b)               # b's entries move into a; handles b issued stay valid
a.decrease_key(deadline, 1)

assert [a.pop()[0] for _ in range(4)] == [1, 3, 5, 9]
assert len(b) == 0      # b is still a usable, empty PairingHeap
```

Note that `insert()` (not `push()`) is the entry point for every addressable
heap in the table below — `push`/`peek`/`pop` (no handle) are reserved for
the plain, value-only heaps.

### Choosing a key type

```python
rheaps.BinaryArrayHeap()                    # key_type=object (default): any comparable Python object
rheaps.BinaryArrayHeap(key_type=int)        # native i64 keys, fastest
rheaps.BinaryArrayHeap(key_type=float)      # native finite f64 keys, fastest
```

`key_type` is fixed for the lifetime of the heap (chosen once at
construction) and two heaps must share it to `meld()`. Comparing
incompatible objects under `key_type=object` raises `TypeError`, the same as
it would from plain `a < b`.

## Choosing an implementation

| Class | Family | Addressable | Decrease key | Meld | Notes |
|---|---|---|---|---|---|
| `BinaryArrayHeap` | array | no | – | no | smallest, cache-friendly default |
| `DaryArrayHeap(degree)` | array | no | – | no | larger `degree` = cheaper insert, pricier removal |
| `BinaryArrayWeakHeap` | array | no | – | no | relaxed invariant, fewer comparisons |
| `BinaryArrayBulkInsertWeakHeap` | array | no | – | no | weak heap tuned for bulk insertion |
| `MinMaxBinaryArrayDoubleEndedHeap` | array | no | – | no | `peek_max()`/`pop_max()` too |
| `BinaryArrayAddressableHeap` | array | yes | yes | no | array-backed with handles |
| `DaryArrayAddressableHeap(degree)` | array | yes | yes | no | as above, d-ary |
| `PairingHeap` | tree | yes | yes | yes | good all-round default for meld + decrease-key |
| `PurePairingHeap` | tree | yes | yes | yes | pairing-heap variant |
| `CostlessMeldPairingHeap` | tree | yes | yes | yes | pairing-heap variant |
| `RankPairingHeap` | tree | yes | yes | yes | pairing-heap variant |
| `LeftistHeap` | tree | yes | yes | yes | classic leftist heap |
| `SkewHeap` | tree | yes | yes | yes | self-adjusting, no extra per-node state |
| `FibonacciHeap` | tree | yes | yes | yes | amortized O(1) decrease-key |
| `SimpleFibonacciHeap` | tree | yes | yes | yes | Fibonacci-heap variant |
| `StrictFibonacciHeap` | tree | yes | yes | yes | worst-case (not just amortized) bounds |
| `BinaryTreeAddressableHeap` | tree | yes | yes | no | node-based binary heap with handles |
| `DaryTreeAddressableHeap(degree)` | tree | yes | yes | no | `degree` must be a power of two, ≥ 2 |
| `ReflectedFibonacciHeap` | tree | yes | yes | yes | double-ended (`peek_max`/`pop_max`/`increase_key`) |
| `ReflectedPairingHeap` | tree | yes | yes | yes | double-ended (`peek_max`/`pop_max`/`increase_key`) |
| `SoftHeap(error_rate)` | tree | no | no | yes (fallible) | corruption-bounded; trades some wrong keys for speed |
| `SoftAddressableHeap(error_rate)` | tree | yes | **no** | yes (fallible) | can't decrease-key: corruption bound isn't tracked per-entry |
| `HollowHeap` | dag | yes | yes | yes | decrease-key/meld without cutting nodes from a parent |
| `U32RadixHeap(min, max)` | monotone | no | – | no | keys removed in nondecreasing order (e.g. Dijkstra) |
| `U64RadixHeap(min, max)` | monotone | no | – | no | as above, 64-bit keys |
| `F64RadixHeap(min, max)` | monotone | no | – | no | as above, finite float keys |
| `BigUintRadixHeap(min, max)` | monotone | no | – | no | as above, arbitrary-precision non-negative integer keys |
| `U32RadixAddressableHeap(min, max)` | monotone | yes | yes | no | addressable counterpart |
| `U64RadixAddressableHeap(min, max)` | monotone | yes | yes | no | addressable counterpart |
| `F64RadixAddressableHeap(min, max)` | monotone | yes | yes | no | addressable counterpart |
| `BigUintRadixAddressableHeap(min, max)` | monotone | yes | yes | no | addressable counterpart |

Every class above except the radix heaps takes `key_type=int|float|object`
(default `object`); radix heaps have a fixed native key type baked into the
class name instead, since their bucket structure depends on it.

## API reference

### Value-only heaps (`Heap`-shaped: `BinaryArrayHeap`, `DaryArrayHeap`, the weak heaps, `MinMaxBinaryArrayDoubleEndedHeap`, `SoftHeap`)

| Method | Description |
|---|---|
| `push(key)` | Insert `key`. |
| `peek()` | Return the minimum key, or `None` if empty. |
| `pop()` | Remove and return the minimum key, or `None` if empty. |
| `len(heap)` | Number of entries. |
| `is_empty()` | Whether the heap has no entries. |
| `clear()` | Remove every entry. |

`MinMaxBinaryArrayDoubleEndedHeap` additionally has `peek_max()`/`pop_max()`.
`SoftHeap` additionally has `rank_limit()` and a fallible `meld(other)` (see
[Soft heaps](#soft-heaps) below).

### Addressable heaps (everything else, except radix heaps)

| Method | Description |
|---|---|
| `insert(key, value=None)` | Insert an entry, returning a handle. |
| `peek()` | Return `(handle, key, value)` for a minimum entry, or `None`. |
| `pop()` | Remove and return `(key, value)` for a minimum entry, or `None`. |
| `key(handle)` | The key addressed by `handle`. Raises `ValueError` if stale/foreign. |
| `value(handle)` | The value addressed by `handle`. Raises `ValueError` if stale/foreign. |
| `set_value(handle, value)` | Replace the value addressed by `handle`. |
| `delete(handle)` | Remove and return `(key, value)` for `handle`. |
| `len(heap)` / `is_empty()` / `clear()` | As above. |
| `decrease_key(handle, key)`* | Decrease the key addressed by `handle`. Raises `ValueError` if `key` isn't lower, or the handle is invalid. |
| `meld(other)`* | Absorb `other` (same class and `key_type`); `other` is left empty. |
| `peek_max()` / `pop_max()` / `increase_key(handle, key)`* | Only on double-ended heaps (`ReflectedFibonacciHeap`, `ReflectedPairingHeap`). |

\* Only on classes that support that capability — see the table above
(`SoftAddressableHeap` has no `decrease_key`; the non-meldable classes have no
`meld`).

### Radix heaps (`monotone`)

Radix heaps enforce that removed keys are **nondecreasing**: once a key is
popped, no smaller key may be inserted or `decrease_key`'d below it. They
also require a fixed inclusive `[minimum_key, maximum_key]` range at
construction. Violating either raises `ValueError`.

| Method | Description |
|---|---|
| `Cls(minimum_key, maximum_key)` | Construct with inclusive key bounds. |
| `try_push(key)` (non-addressable) / `try_insert(key, value=None)` (addressable) | Insert, raising `ValueError` on an out-of-range or non-monotone key. |
| `peek()`, `pop()`, `len(heap)`, `is_empty()`, `clear()` | As above. |
| `bucket_count()` | Number of radix buckets the heap allocated. |
| *(addressable only)* `key()`, `value()`, `set_value()`, `delete()`, `decrease_key()` | As above; `decrease_key` still enforces monotonicity. |

### Soft heaps

`SoftHeap`/`SoftAddressableHeap` implement a Kaplan-Zwick soft heap: in
exchange for faster operations, up to `error_rate` (a fraction between 0 and
1, exclusive) of keys may be *corrupted* (silently increased) at any time.
Use them when an approximately-correct minimum is acceptable — for example,
as a building block inside a minimum spanning tree algorithm. Neither
supports `decrease_key`, since corruption means the heap no longer tracks
each entry's exact position.

## Error handling

| Situation | Exception |
|---|---|
| Stale or foreign handle | `ValueError` |
| `decrease_key`/`increase_key` with a key that doesn't strictly move that direction | `ValueError` |
| Invalid construction parameter (non-power-of-two/too-small `degree`, out-of-`(0,1)` `error_rate`, invalid radix bounds) | `ValueError` |
| Radix heap key out of range, or lower than the last removed key | `ValueError` |
| Non-finite (`nan`/`inf`) key under `key_type=float` or in an `F64Radix*` heap | `ValueError` |
| Wrong Python type for `key_type=int`/`float` (e.g. a string) | `TypeError` |
| Incomparable objects under `key_type=object` | `TypeError` (same as the underlying `<`) |
| `meld()` between heaps of different `key_type` | `ValueError` |
| Empty heap (`peek`/`pop`/`pop_max`) | not an error — returns `None` |

## Development

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install maturin pytest
maturin develop
pytest
```

`tests/test_large_scale.py` ports the large-scale conformance invariants
`rheaps`' own Rust test suite exercises against each implementation
(thousands of ascending/random/decreasing-key operations checked against a
plain `sorted()` oracle, arbitrary-order deletion, melding, and — for soft
heaps — the weaker "every inserted key eventually comes back out, in some
order" invariant that corruption permits) across every parametrized heap
class. It runs as part of the same `pytest` invocation (a few extra seconds),
not a separate opt-in suite.

The Rust source lives in `src/`:

- `key.rs` — the `int`/`float`/`object` key representations and conversions.
- `error.rs` — `rheaps` error types → Python exceptions.
- `handles.rs` — one `#[pyclass]` per `rheaps` handle type.
- `macros.rs` — shared codegen: each heap-trait shape (`Heap`,
  `AddressableHeap` + `DecreaseKeyHeap`, `MeldableAddressableHeap`,
  `DoubleEndedAddressableHeap`, and the radix-heap shapes) gets one macro
  that a concrete heap module composes.
- `heaps/{array,tree,dag,monotone}.rs` — the 31 concrete heap classes,
  mostly a handful of lines each invoking the shared macros.

## Documentation

Full documentation (API reference, tutorials, and an example gallery) is
built with [Sphinx](https://www.sphinx-doc.org/):

```bash
source .venv/bin/activate
pip install -e ".[docs]"     # sphinx, sphinx-rtd-theme, sphinx-gallery, matplotlib, pillow
make -C docs html
```

The rendered pages land in `docs/_build/html/index.html`. The source lives
in `docs/`:

- `docs/api/` — `introduction.rst` (heap inventory by family), `heaps.rst`
  and `handles.rst` (`autoclass` reference for every heap and handle class).
- `docs/tutorials/` — a walkthrough of the addressable-heap API
  (`insert`/`peek`/`decrease_key`/`delete`/`meld`).
- `docs/install.rst`, `docs/license.rst`, `docs/credits.rst`.

`examples/` holds the [sphinx-gallery](https://sphinx-gallery.github.io/)
scripts rendered into that documentation build (`addressable/`, `array/`,
`monotone/`, one plain, runnable `.py` file per example) — run any of them
directly with `python3 examples/addressable/plot_pairing.py` without
building the docs.

## Relationship to `rheaps` and JHeaps

This package is a thin PyO3 wrapper: it doesn't reimplement any algorithm,
it just exposes the [`rheaps`](https://docs.rs/rheaps) crate — an idiomatic
Rust port of [JHeaps](https://github.com/d-michail/jheaps) — as Python
classes. If you use this library, consider citing the paper describing the
algorithms and implementation set it's derived from:

> D. Michail. **JHeaps: An open-source library of priority queues.**
> SoftwareX, 16:100869, 2021. <https://doi.org/10.1016/j.softx.2021.100869>

## License

Copyright 2024-2026 [Dimitrios Michail](https://github.com/d-michail)

Licensed under the Apache License, Version 2.0 (the "License"); you may not
use this project's files except in compliance with the License. You may
obtain a copy of the License at
<https://www.apache.org/licenses/LICENSE-2.0>, or see the [`LICENSE`](LICENSE)
file in this repository.

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
License for the specific language governing permissions and limitations
under the License.

SPDX-License-Identifier: Apache-2.0
