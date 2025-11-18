from surreal_orm.query_builder import field, Param, meta_id, meta_tb


def test_meta_id_and_tb():
    # tuple thing
    record_id = ("person", "oyelowo")
    result = meta_id(record_id)
    assert result.to_raw().build() == "meta::id(person:oyelowo)"

    result = meta_tb(record_id)
    assert result.to_raw().build() == "meta::tb(person:oyelowo)"

    # field
    id_field = field("id_field")
    result = meta_id(id_field)
    assert result.to_raw().build() == "meta::id(id_field)"

    result = meta_tb(id_field)
    assert result.to_raw().build() == "meta::tb(id_field)"

    # param
    id_param = Param("id_param")
    result = meta_id(id_param)
    assert result.to_raw().build() == "meta::id($id_param)"

    result = meta_tb(id_param)
    assert result.to_raw().build() == "meta::tb($id_param)"
