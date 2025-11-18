"""Migration helpers and embedding (initial stubs).

This module will host migration utilities ported from the Rust `migrator` crate.
"""
import os
from typing import List


def discover_migrations(path: str) -> List[str]:
    """Discover migration files under `path` (surql files).

    Returns list of file paths sorted by name.
    """
    if not os.path.isdir(path):
        return []
    files = [os.path.join(path, f) for f in os.listdir(path) if f.endswith(".surql") or f.endswith(".sql")]
    files.sort()
    return files


def embed_migrations(path: str) -> List[str]:
    """Read and return migration file contents. Stub for `embed_migrations` macro."""
    found = discover_migrations(path)
    out = []
    for fp in found:
        with open(fp, "r", encoding="utf-8") as fh:
            out.append(fh.read())
    return out
