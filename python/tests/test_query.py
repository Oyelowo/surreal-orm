def test_query_bind_and_render():
    from surreal_orm.core import Query, query

    q = query("SELECT * FROM person WHERE name = $name AND age > $age")
    assert isinstance(q, Query)

    q.bind("name", "Alice").bind("age", 20)
    sql, params = q.render()

    assert "$name" in sql and "$age" in sql
    assert params["name"] == "Alice"
    assert params["age"] == 20

    # bind_many convenience
    q2 = Query("SELECT 1")
    q2.bind_many(one=1, two=2)
    s2, p2 = q2.render()
    assert p2 == {"one": 1, "two": 2}
