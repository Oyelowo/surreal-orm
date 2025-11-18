from surreal_orm.query_builder import (
    field,
    abs,
    ceil,
    floor,
    round,
    sqrt,
    mean,
    median,
    mode,
    min,
    max,
    product,
    sum,
    Param,
)


def test_single_number_functions_with_field_and_literal_and_param():
    f = field("temperature")

    # field
    assert abs(f).to_raw().build() == "math::abs(temperature)"
    assert abs(f).fine_tune_params() == "math::abs(temperature)"

    # literal float
    assert ceil(45.23).to_raw().build() == "math::ceil(45.23f)"
    assert ceil(45.23).fine_tune_params() == "math::ceil($_param_00000001)"

    # literal negative int
    assert floor(-454).to_raw().build() == "math::floor(-454)"
    assert floor(-454).fine_tune_params() == "math::floor($_param_00000001)"

    # param
    p = Param("temperature")
    assert round(p).to_raw().build() == "math::round($temperature)"
    assert round(p).fine_tune_params() == "math::round($temperature)"

    # sqrt
    assert sqrt(16).to_raw().build() == "math::sqrt(16)"
    assert sqrt(16).fine_tune_params() == "math::sqrt($_param_00000001)"


def test_array_functions_numeric_array_and_mixed_and_field_and_param():
    # field as arg
    f = field("size_list")
    assert mean(f).to_raw().build() == "math::mean(size_list)"
    assert mean(f).fine_tune_params() == "math::mean(size_list)"

    # numeric array literal -> single placeholder in fine_tune, expanded in raw
    arr1 = [1, 2, 3, 3.5]
    assert median(arr1).to_raw().build() == "math::median([1, 2, 3, 3.5f])"
    assert median(arr1).fine_tune_params() == "math::median($_param_00000001)"

    # mixed array
    age = field("age")
    arr2 = [1, 2, "4334", "Oyelowo", age]
    assert mode(arr2).to_raw().build() == "math::mode([1, 2, '4334', 'Oyelowo', age])"
    assert mode(arr2).fine_tune_params() == "math::mode([$_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, age])"

    # param as arg
    p = Param("size_list")
    assert min(p).to_raw().build() == "math::min($size_list)"
    assert min(p).fine_tune_params() == "math::min($size_list)"

    # product/sum raw numeric array
    arr3 = [1, 2, 3]
    assert product(arr3).to_raw().build() == "math::product([1, 2, 3])"
    assert product(arr3).fine_tune_params() == "math::product($_param_00000001)"
    assert sum(arr3).to_raw().build() == "math::sum([1, 2, 3])"
    assert sum(arr3).fine_tune_params() == "math::sum($_param_00000001)"
