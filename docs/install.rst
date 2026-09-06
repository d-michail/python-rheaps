.. _install:

Install
=======

Prebuilt wheels are published on `PyPI <https://pypi.org/project/rheaps/>`_
for Linux, Windows, and macOS (Python 3.10-3.14)::

  $ pip install rheaps

Building from source
---------------------

To work on the bindings themselves, build with
`maturin <https://www.maturin.rs/>`_, which requires a Rust toolchain
(stable, edition 2024 support -- 1.88+) in addition to Python::

  $ python -m venv venv
  $ source venv/bin/activate
  $ pip install --upgrade pip
  $ pip install maturin
  $ maturin develop

``maturin develop`` builds the extension and installs it into the active
virtual environment in one step. Use ``maturin develop --release`` for an
optimized build, or ``maturin build --release`` to produce a wheel in
``target/wheels/`` without installing it.
