def test_model_save_find_delete():
    from surreal_orm import Node
    from surreal_orm.core import Surreal

    @Node
    class Ship:
        name: str
        id: str = None

    db = Surreal()

    s = Ship(name="Enterprise")
    rec = s.save(db)

    assert "id" in rec
    assert rec["name"] == "Enterprise"

    # find by id
    found = Ship.find_by_id(rec["id"], db)
    assert len(found) == 1
    assert found[0]["name"] == "Enterprise"

    # delete instance
    s_with_id = Ship(name="Enterprise", id=rec["id"])
    s_with_id.delete(db)
    after = Ship.find_by_id(rec["id"], db)
    assert len(after) == 0
