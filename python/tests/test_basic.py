import pytest
from surreal_orm import Node, pick


def test_node_decorator_and_pick():
    @Node
    class Person:
        name: str
        age: int

    P = pick("PersonPick", Person, ["name"])

    p = P(name="Alice")
    assert hasattr(Person, "__orm_kind__")
    assert getattr(Person, "__orm_kind__") == "node"
    assert p.name == "Alice"
