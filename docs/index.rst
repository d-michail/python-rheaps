.. rheaps documentation master file.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

The rheaps library
===================

Release v\ |version|.

Python bindings for the `rheaps library <https://docs.rs/rheaps>`_.

The rheaps library is an idiomatic Rust port of `JHeaps <https://www.jheaps.org/>`_,
containing state-of-the-art heap implementations. The |Bindings| is a pure
Rust-backed native extension, built with `PyO3 <https://pyo3.rs>`_: the heap
data lives directly inside a Rust struct wrapped by a Python object, with no
separate backend library, JVM, or shared C library to load.

Backend version v\ |BackendVersion| (the `rheaps <https://docs.rs/rheaps>`_
Rust crate embedded in this build).

Development
-----------

Development happens in the following places.

 * https://github.com/d-michail/python-rheaps
 * https://github.com/d-michail/rheaps
 * https://github.com/d-michail/jheaps


Documentation
-------------

.. toctree::
   :maxdepth: 2
   :caption: rheaps

   install
   tutorials/index
   api/index
   license
   credits

.. toctree::
   :maxdepth: 2
   :caption: Example galleries

   auto_examples/index


Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
