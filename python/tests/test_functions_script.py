from surreal_orm.query_builder import function_fn, Param


def test_function_without_args():
    f2 = function_fn([], "{ return [1, 2, 3].map(v => v * 10); }")
    assert f2.build() == "function() { return [1, 2, 3].map(v => v * 10); }"
    assert f2.to_raw().build() == "function() { return [1, 2, 3].map(v => v * 10); }"


def test_function_with_params_with_variable_interpolation_in_the_function():
    value = Param("value")
    words = Param("words")

    f2 = function_fn([value, words], "{ return `${arguments[0]} is ${arguments[1]}`; }")

    assert f2.build() == "function($value, $words) { return `${arguments[0]} is ${arguments[1]}`; }"
    assert f2.to_raw().build() == "function($value, $words) { return `${arguments[0]} is ${arguments[1]}`; }"


def test_function_with_args():
    name = Param("name")
    idp = Param("id")

    f2 = function_fn([name, idp], "{ return [1, 2, 3].map(v => v * 10); }")
    assert f2.build() == "function($name, $id) { return [1, 2, 3].map(v => v * 10); }"
    assert f2.to_raw().build() == "function($name, $id) { return [1, 2, 3].map(v => v * 10); }"


def test_function_with_args_code_str():
    name = Param("name")
    idp = Param("id")

    f2 = function_fn([name, idp], "{ return [1, 2, 3].map(v => v * 10 * $name * $id) ; }")
    assert f2.build() == "function($name, $id) { return [1, 2, 3].map(v => v * 10 * $name * $id) ; }"
    assert f2.to_raw().build() == "function($name, $id) { return [1, 2, 3].map(v => v * 10 * $name * $id) ; }"
