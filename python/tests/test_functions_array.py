from surreal_orm.query_builder import add, all, any, append, field, Param


def test_add_with_values_and_field_param():
    result = add([1, 2, 3], 4)
    # fine_tune: both arr and val are literals -> two placeholders
    assert result.fine_tune_params() == "array::add($_param_00000001, $_param_00000002)"
    assert result.to_raw().build() == "array::add([1, 2, 3], 4)"

    numbers = field("numbers")
    value = Param("value")
    result = add(numbers, value)
    assert result.fine_tune_params() == "array::add(numbers, $value)"
    assert result.to_raw().build() == "array::add(numbers, $value)"


def test_all_and_any_with_values_and_field():
    res = all([1, 2, 3, 4, 5])
    assert res.fine_tune_params() == "array::all($_param_00000001)"
    assert res.to_raw().build() == "array::all([1, 2, 3, 4, 5])"

    numbers = field("numbers")
    res = all(numbers)
    assert res.fine_tune_params() == "array::all(numbers)"
    assert res.to_raw().build() == "array::all(numbers)"

    res2 = any([1, 2, 3, 4, 5])
    assert res2.fine_tune_params() == "array::any($_param_00000001)"
    assert res2.to_raw().build() == "array::any([1, 2, 3, 4, 5])"

    res2b = any(numbers)
    assert res2b.fine_tune_params() == "array::any(numbers)"
    assert res2b.to_raw().build() == "array::any(numbers)"


def test_append_with_values_field_param():
    res = append([1, 2, 3, 4, 5], 6)
    assert res.fine_tune_params() == "array::append($_param_00000001, $_param_00000002)"
    assert res.to_raw().build() == "array::append([1, 2, 3, 4, 5], 6)"

    numbers = field("numbers")
    value = field("value")
    res2 = append(numbers, value)
    assert res2.fine_tune_params() == "array::append(numbers, value)"
    assert res2.to_raw().build() == "array::append(numbers, value)"

    value_param = Param("value")
    res3 = append(numbers, value_param)
    assert res3.fine_tune_params() == "array::append(numbers, $value)"
    assert res3.to_raw().build() == "array::append(numbers, $value)"
