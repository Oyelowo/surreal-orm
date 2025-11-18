"""Decorators and helpers that replace Rust derive macros (initial stubs).

The Rust crate exposes `Node`, `Edge`, `Object`, `pick`, and other derive
macros. In Python we provide class decorators and helper functions that mimic
basic behavior (e.g., attaching metadata to classes).
"""
from dataclasses import dataclass, fields, make_dataclass
from typing import Any, Callable, List, Type


def _attach_meta(cls: Type, kind: str) -> Type:
    setattr(cls, "__orm_kind__", kind)
    # Default table name follows the snake_case of the class name unless the
    # class defines `__table_name__` itself.
    if not hasattr(cls, "__table_name__"):
        # Convert CamelCase -> snake_case (simple implementation)
        name = cls.__name__
        snake = ""
        prev_lower = False
        for ch in name:
            if ch.isupper():
                if prev_lower:
                    snake += "_"
                snake += ch.lower()
                prev_lower = False
            else:
                snake += ch
                prev_lower = True
        setattr(cls, "__table_name__", snake)
    return cls


def Node(cls: Type) -> Type:
    """Mark a dataclass as a Node.

    Example:
        @Node
        @dataclass
        class Person:
            name: str
            age: int
    """
    # Ensure the class is a dataclass so it gets an __init__ and asdict support.
    try:
        from dataclasses import is_dataclass, dataclass

        if not is_dataclass(cls):
            cls = dataclass(cls)
    except Exception:
        pass

    cls = _attach_meta(cls, "node")

    # Attach simple ORM helpers for the in-memory test adapter.
    def save(self, db):
        """Save this instance to the provided `Surreal` in-memory adapter.

        This will insert the dataclass into the in-memory store and set the
        `id` attribute on the instance if present in the returned record.
        """
        from surreal_orm.core import Surreal

        if not isinstance(db, Surreal):
            raise TypeError("db must be an instance of surreal_orm.core.Surreal")
        record = db.insert(self)
        try:
            object.__setattr__(self, "id", record.get("id"))
        except Exception:
            try:
                setattr(self, "id", record.get("id"))
            except Exception:
                pass
        return record

    def delete(self, db):
        from surreal_orm.core import Surreal

        if not isinstance(db, Surreal):
            raise TypeError("db must be an instance of surreal_orm.core.Surreal")
        table = getattr(self.__class__, "__table_name__")
        id_val = getattr(self, "id", None)
        if id_val is None:
            raise ValueError("instance has no id")
        db.delete(table, str(id_val))

    def update(self, db, **updates):
        from surreal_orm.core import Surreal

        if not isinstance(db, Surreal):
            raise TypeError("db must be an instance of surreal_orm.core.Surreal")
        table = getattr(self.__class__, "__table_name__")
        id_val = getattr(self, "id", None)
        if id_val is None:
            raise ValueError("instance has no id to update")
        rec = db.update(table, str(id_val), **updates)
        # sync fields back to instance when possible
        for k, v in rec.items():
            if k in getattr(self.__class__, "__annotations__", {}):
                try:
                    object.__setattr__(self, k, v)
                except Exception:
                    try:
                        setattr(self, k, v)
                    except Exception:
                        pass
        return rec

    @classmethod
    def update_by_id(cls, id_val, db, **updates):
        table = getattr(cls, "__table_name__")
        return db.update(table, str(id_val), **updates)

    @classmethod
    def find_by_id(cls, id_val, db):
        from surreal_orm.core import Surreal

        if not isinstance(db, Surreal):
            raise TypeError("db must be an instance of surreal_orm.core.Surreal")
        table = getattr(cls, "__table_name__")
        return db.select(table, id=str(id_val))

    @classmethod
    def find_where(cls, **filters):
        from surreal_orm.core import ORMQuery

        table = getattr(cls, "__table_name__")
        return ORMQuery(table, filters)

    @classmethod
    def count_where(cls, **filters):
        from surreal_orm.core import CountQuery

        table = getattr(cls, "__table_name__")
        return CountQuery(table, filters)

    @classmethod
    def count_all(cls):
        from surreal_orm.core import CountQuery

        table = getattr(cls, "__table_name__")
        return CountQuery(table, {})

    @classmethod
    def delete_by_id(cls, id_val, db):
        table = getattr(cls, "__table_name__")
        db.delete(table, str(id_val))

    @classmethod
    def create_id(cls, val):
        return str(val)

    setattr(cls, "save", save)
    setattr(cls, "delete", delete)
    setattr(cls, "update", update)
    setattr(cls, "update_by_id", update_by_id)
    setattr(cls, "find_by_id", find_by_id)
    setattr(cls, "find_where", find_where)
    setattr(cls, "count_where", count_where)
    setattr(cls, "count_all", count_all)
    setattr(cls, "delete_by_id", delete_by_id)
    setattr(cls, "create_id", create_id)

    return cls


def Edge(cls: Type) -> Type:
    """Mark a dataclass as an Edge."""
    try:
        from dataclasses import is_dataclass, dataclass

        if not is_dataclass(cls):
            cls = dataclass(cls)
    except Exception:
        pass
    return _attach_meta(cls, "edge")


def Object(cls: Type) -> Type:
    """Mark a dataclass as an Object."""
    try:
        from dataclasses import is_dataclass, dataclass

        if not is_dataclass(cls):
            cls = dataclass(cls)
    except Exception:
        pass
    return _attach_meta(cls, "object")


def pick(new_name: str, cls: Type, fields_to_pick: List[str]) -> Type:
    """Create a new dataclass that picks a subset of fields from `cls`.

    This roughly corresponds to the Rust `pick!` macro.
    """
    if not hasattr(cls, "__annotations__"):
        raise TypeError("pick: source must be an annotated class or dataclass")

    annotations = {k: v for k, v in cls.__annotations__.items() if k in fields_to_pick}
    new_cls = make_dataclass(new_name, [(k, v) for k, v in annotations.items()])
    return new_cls


# Backwards-compatible capitalized names (exported by package root)
Node = Node
Edge = Edge
Object = Object
pick = pick
