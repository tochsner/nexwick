# nexwick-py

This crate contains the `PyO3` bindings for nexwick. So far, teh following has been implemented:

- Bindings for `nexwick_py.parse_nexus_file` and the needed structs.
- Type stubs in `nexwick_py.pyi`.

## How to build the python package

1. Create a new Python venv environment with `uv run python -m venv .env`.
2. Activate the new environment with `source .env/bin/activate`.
3. Install maturin with `pip install maturin`.
4. Install the python package as an editable package with `maturin develop`.

## How to use this package locally in another (uv) project

1. Navigate into the project where you want to use nexwick.
2. Install nexwick with `uv add /Users/ochsneto/Documents/Tools/nexwick/nexwick-py`.
3. Try it out:
    ```python
    import nexwick_py
    trees, lm = nexwick_py.parse_nexus_file('yule.trees')
    print(trees[0].to_newick(lm))
    ```
