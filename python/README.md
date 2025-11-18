surreal_orm Python port (scaffold)

This folder contains a minimal scaffold for a Python package that will evolve
into a feature-parity port of the `surreal_orm` Rust workspace.

Quickstart (dev):

- Create a virtualenv and install `pytest` (for now):

```bash
python -m venv .venv
source .venv/bin/activate
python -m pip install -U pip
python -m pip install pytest
```

- Run tests:

```bash
cd python
pytest -q
```

What's next:
- Implement the Query builder API in `core.py`
- Port procedural macros into Python decorators in `decorators.py`
- Implement actual SurrealDB client integration for query execution
- Port migration CLI functionality from `migrator`
