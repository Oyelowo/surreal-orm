from surreal_orm.query_builder import len, first, last, field


def test_array_len_first_last():
    res = len([1, 2, 3])
    assert res.fine_tune_params() == "array::len($_param_00000001)"
    assert res.to_raw().build() == "array::len([1, 2, 3])"

    f = field("own_goals")
    r2 = len(f)
    assert r2.fine_tune_params() == "array::len(own_goals)"
    assert r2.to_raw().build() == "array::len(own_goals)"

    r3 = first([1, 2, 3, 4, 5])
    assert r3.fine_tune_params() == "array::first($_param_00000001)"
    assert r3.to_raw().build() == "array::first([1, 2, 3, 4, 5])"

    f2 = field("numbers")
    r4 = first(f2)
    assert r4.fine_tune_params() == "array::first(numbers)"
    assert r4.to_raw().build() == "array::first(numbers)"

    r5 = last([1, 2, 3])
    assert r5.fine_tune_params() == "array::last($_param_00000001)"
    assert r5.to_raw().build() == "array::last([1, 2, 3])"

    r6 = last(f2)
    assert r6.fine_tune_params() == "array::last(numbers)"
    assert r6.to_raw().build() == "array::last(numbers)"
