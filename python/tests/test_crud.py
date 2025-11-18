def test_in_memory_crud_basic():
    from surreal_orm import Node
    from surreal_orm.core import Surreal

    @Node
    class Person:
        name: str
        age: int

    db = Surreal()

    # insert two people
    r1 = db.insert(Person(name="Alice", age=30))
    r2 = db.insert(Person(name="Bob", age=25))

    assert r1["id"] == "1"
    assert r2["id"] == "2"

    all_people = db.select("person")
    assert len(all_people) == 2

    # filter by name
    alice = db.select("person", name="Alice")
    assert len(alice) == 1
    assert alice[0]["name"] == "Alice"

    # update
    updated = db.update("person", "1", age=31)
    assert updated["age"] == 31

    # delete
    db.delete("person", "2")
    remaining = db.select("person")
    assert len(remaining) == 1
