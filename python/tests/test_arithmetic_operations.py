def test_arithmetic_operations_basic():
    from surreal_orm.query_builder import Field

    strength = Field("strength")
    op = strength + 5
    assert op.to_raw().build() == "strength + 5"
    assert op.fine_tune_params() == "strength + $_param_00000001"


def test_arithmetic_operations_complex():
    from surreal_orm.query_builder import Field

    strength = Field("strength")
    bunch = Field("bunchOfOtherFields")
    operation = ((strength + 5) / (strength + bunch)) * 32

    assert operation.to_raw().build() == "((strength + 5) / (strength + bunchOfOtherFields)) * 32"
    assert operation.fine_tune_params() == "((strength + $_param_00000001) / (strength + bunchOfOtherFields)) * $_param_00000002"


def test_query_string_from_fields():
    from surreal_orm.query_builder import Field

    age = Field("age")
    name = Field("name")
    email = Field("email")
    surname = Field("surname")

    operation = (age + name) + (email + surname)
    assert operation.query_string == "(age + name) + (email + surname)"
