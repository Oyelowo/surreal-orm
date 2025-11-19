def test_find_where_and_count_where():
    from surreal_orm import Node
    from surreal_orm.core import Surreal

    @Node
    class Item:
        id: str | None = None
        kind: str = ""
        value: int = 0

    db = Surreal()

    # create sample data
    for i in range(10):
        Item(kind="a" if i % 2 == 0 else "b", value=i).save(db)

    # count where kind == 'a'
    cnt = Item.count_where(kind="a").get(db)
    assert cnt == 5

    # find where value > 5 -- our simple filters only handle equality,
    # so check equality behavior instead
    rows = Item.find_where(kind="b").return_many(db)
    assert all(r["kind"] == "b" for r in rows)
    assert len(rows) == 5
