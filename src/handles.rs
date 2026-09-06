//! Python wrappers for the crate's opaque, `Copy` heap handles.
//!
//! A handle is a small value type, not an owning pointer, so wrapping it only
//! needs a plain newtype `#[pyclass]` -- no `__del__`/refcounting dance is
//! needed. The same wrapper is reused by every heap algorithm in its family
//! that returns that particular Rust handle type.

use pyo3::prelude::*;

macro_rules! handle_pyclass {
    ($name:ident, $inner:path) => {
        #[pyclass(module = "rheaps", frozen, eq, hash, from_py_object)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(pub $inner);

        #[pymethods]
        impl $name {
            fn __repr__(&self) -> String {
                format!(concat!(stringify!($name), "({:?})"), self.0)
            }
        }

        impl From<$inner> for $name {
            fn from(inner: $inner) -> Self {
                Self(inner)
            }
        }
    };
}

handle_pyclass!(AddressableHandle, rheaps::array::AddressableHandle);
handle_pyclass!(TreeHandle, rheaps::tree::TreeHandle);
handle_pyclass!(HollowHandle, rheaps::dag::HollowHandle);
handle_pyclass!(ReflectedHandle, rheaps::tree::ReflectedHandle);
handle_pyclass!(SoftHandle, rheaps::tree::SoftHandle);
handle_pyclass!(RadixHandle, rheaps::monotone::RadixHandle);
