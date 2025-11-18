def test_select_build_and_where():
    from surreal_orm import select, field

    q = select(field("name")).from_("person").where_(field("age").gt(30))
    sql = q.build()
    assert sql == "SELECT name FROM person WHERE age > 30;"


def test_select_value_count():
    from surreal_orm import select_value, count, field

    c = count(field("strength").gte(500))
    q = select_value(c).from_("weapon").group_all()
    sql = q.build()
    assert sql == "SELECT VALUE count FROM (SELECT count(strength >= 500) FROM weapon GROUP ALL);"
