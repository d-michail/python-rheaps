.. _install:

Install
=======

Not yet published to PyPI. Build and install from source using
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
