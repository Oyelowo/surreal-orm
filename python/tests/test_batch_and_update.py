def test_batch_insert_and_update():
    from surreal_orm import Node
    from surreal_orm.core import Surreal, insert_many

    @Node
    class Item:
        id: str | None = None
        name: str = ""
        qty: int = 0

    db = Surreal()

    objs = [Item(name=f"n{i}", qty=i) for i in range(5)]
    created = insert_many(db, objs)
    assert len(created) == 5
    assert created[0]["id"] == "1"

    # Update one record by classmethod
    Item.update_by_id("1", db, qty=99)
    rows = Item.find_by_id("1", db)
    assert rows[0]["qty"] == 99

    # Instance update
    inst = Item(name="x", qty=1)
    rec = inst.save(db)
    inst.update(db, qty=77)
    rows = Item.find_by_id(rec["id"], db)
    assert rows[0]["qty"] == 77
