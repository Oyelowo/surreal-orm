"""Core runtime stubs for surreal_orm Python package.

This file contains minimal, well-documented placeholders that will be iteratively
implemented to match the Rust crate's functionality.
"""
from typing import Any, Callable, Optional, Dict, List
from dataclasses import asdict, is_dataclass


class Surreal:
    """Connection / client stub.

    Replace with a real async client wrapper for the SurrealDB python driver.
    """

    def __init__(self, url: str = "local", namespace: Optional[str] = None, database: Optional[str] = None):
        self.url = url
        self.namespace = namespace
        self.database = database
        self._connected = False
        # In-memory storage: table -> id -> record
        self._data: Dict[str, Dict[str, Dict[str, Any]]] = {}
        self._id_counters: Dict[str, int] = {}

    async def connect(self) -> None:
        """Open a connection to SurrealDB.

        Currently a stub. Implement actual connection logic using a Python
        SurrealDB client.
        """
        self._connected = True

    async def close(self) -> None:
        """Close the connection."""
        self._connected = False

    async def query(self, sql: str, params: Optional[dict] = None) -> Any:
        """Execute a query string against the database.

        This is a stub and should be replaced with real query execution and
        result deserialization.
        """
        raise NotImplementedError("query execution is not implemented yet")

    # --- Simple synchronous, in-memory CRUD helpers for unit tests ---

    def insert(self, obj: Any) -> Dict[str, Any]:
        """Insert an object (dataclass instance or dict) into the in-memory store.

        Returns the stored record including an auto-generated `id` string.
        """
        if is_dataclass(obj):
            data = asdict(obj)
            cls = obj.__class__
        elif isinstance(obj, dict):
            data = dict(obj)
            cls = None
        else:
            # allow plain class instances with annotations
            data = {k: getattr(obj, k) for k in getattr(obj, "__annotations__", {})}
            cls = obj.__class__

        table = getattr(cls, "__table_name__", data.get("__table_name__", None)) if cls is not None else data.get("__table_name__")
        if table is None:
            raise ValueError("Unable to determine table name for insert")

        # ensure table storage
        if table not in self._data:
            self._data[table] = {}
            self._id_counters[table] = 0

        # simple incremental id
        self._id_counters[table] += 1
        new_id = str(self._id_counters[table])
        record = dict(data)
        record["id"] = new_id
        self._data[table][new_id] = record
        return record

    def select(self, table: str, **filters) -> List[Dict[str, Any]]:
        """Select records from a table applying simple equality filters."""
        if table not in self._data:
            return []
        rows = list(self._data[table].values())
        if not filters:
            return rows
        out = []
        for r in rows:
            ok = True
            for k, v in filters.items():
                if r.get(k) != v:
                    ok = False
                    break
            if ok:
                out.append(r)
        return out

    def update(self, table: str, id: str, **updates) -> Dict[str, Any]:
        if table not in self._data or id not in self._data[table]:
            raise KeyError("record not found")
        self._data[table][id].update(updates)
        return self._data[table][id]

    def delete(self, table: str, id: str) -> None:
        if table in self._data and id in self._data[table]:
            del self._data[table][id]


def insert_many(db: "Surreal", objs: List[Any]) -> List[Dict[str, Any]]:
    """Insert many objects using the in-memory adapter and return created records."""
    out = []
    for o in objs:
        out.append(db.insert(o))
    return out


class ORMQuery:
    """A tiny ORM query helper for in-memory testing."""

    def __init__(self, table: str, filters: Optional[Dict[str, Any]] = None):
        self.table = table
        self.filters = filters or {}

    def return_many(self, db: "Surreal") -> List[Dict[str, Any]]:
        return db.select(self.table, **self.filters)

    def get_one(self, db: "Surreal") -> Optional[Dict[str, Any]]:
        rows = self.return_many(db)
        return rows[0] if rows else None

    def run(self, db: "Surreal") -> List[Dict[str, Any]]:
        return self.return_many(db)


class CountQuery:
    def __init__(self, table: str, filters: Optional[Dict[str, Any]] = None):
        self.table = table
        self.filters = filters or {}

    def get(self, db: "Surreal") -> int:
        return len(db.select(self.table, **self.filters))


# Query builder entrypoints (stubs)

def query(sql: str, /, **params) -> str:
    """Return a query string or a Query object.

    Right now returns the SQL string for illustration. In the future this
    should return a Query object with binding support.
    """
    return Query(sql, params or {})


def transaction(*statements: str) -> str:
    """Wrap statements in a transaction block (stub)."""
    return "BEGIN;\n" + "\n".join(statements) + "\nCOMMIT;"


class Query:
    """Simple Query object with parameter binding and rendering.

    Usage:
        q = Query("SELECT * FROM table WHERE name = $name")
        q.bind("name", "Alice")
        sql, params = q.render()
    """

    def __init__(self, sql: str, params: Optional[Dict[str, Any]] = None):
        self.sql = sql
        # copy to avoid external mutation
        self.params: Dict[str, Any] = dict(params or {})

    def bind(self, name: str, value: Any) -> "Query":
        """Bind a single named parameter and return self for chaining."""
        self.params[name] = value
        return self

    def bind_many(self, **kwargs: Any) -> "Query":
        """Bind multiple parameters at once."""
        self.params.update(kwargs)
        return self

    def render(self) -> (str, Dict[str, Any]):
        """Return the SQL and a parameters dict suitable for execution.

        The SQL is returned as-is; clients should implement placeholder
        replacement according to their driver. We use `$name` placeholders
        conventionally.
        """
        return self.sql, dict(self.params)

    def __str__(self) -> str:
        return self.sql
