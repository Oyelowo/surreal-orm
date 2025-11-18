def test_model_save_find_delete():
    from surreal_orm import Node
    from surreal_orm.core import Surreal

    @Node
    class Person:
        id: str | None = None
        name: str = ""
        age: int = 0

    db = Surreal()

    p = Person(name="Carol", age=28)
    rec = p.save(db)
    assert "id" in rec
    assert getattr(p, "id") == rec["id"]

    found = Person.find_by_id(rec["id"], db)
    assert found is not None
    # find_by_id now returns a list of rows; pick first element
    if isinstance(found, list):
        row = found[0]
        assert row["name"] == "Carol"
    else:
        # keep backward-compatible support for instances
        assert getattr(found, "name") == "Carol"

    # delete by id
    Person.delete_by_id(rec["id"], db)
    after = Person.find_by_id(rec["id"], db)
    assert after == []
