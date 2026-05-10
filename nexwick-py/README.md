# nexwick-py

This crate contains the `PyO3` bindings for nexwick. So far, only `nexwick_py.parse_nexus_file` and the needed structs are implemented.

## How to build the python package

1. Create a new Python venv environment with `uv run python -m venv .env`.
2. Activate the new environment with `source .env/bin/activate`.
3. Install maturin with `pip install maturin`.
4. Install the python package as an editable package with `maturin develop`.
