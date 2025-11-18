from surreal_orm.query_builder import (
    field,
    length,
    reverse,
    trim,
    slug,
    lowercase,
    uppercase,
    split,
)


def test_length_field_and_literal():
    name = field("name")
    res = length(name)
    assert res.fine_tune_params() == "string::length(name)"
    assert res.to_raw().build() == "string::length(name)"

    res2 = length("toronto")
    assert res2.fine_tune_params() == "string::length($_param_00000001)"
    assert res2.to_raw().build() == "string::length('toronto')"


def test_reverse_and_trim_and_slug():
    name = field("name")
    r = reverse(name)
    assert r.fine_tune_params() == "string::reverse(name)"
    assert r.to_raw().build() == "string::reverse(name)"

    r2 = reverse("oyelowo")
    assert r2.fine_tune_params() == "string::reverse($_param_00000001)"
    assert r2.to_raw().build() == "string::reverse('oyelowo')"

    t = trim(name)
    assert t.fine_tune_params() == "string::trim(name)"
    assert t.to_raw().build() == "string::trim(name)"

    t2 = trim("oyelowo")
    assert t2.fine_tune_params() == "string::trim($_param_00000001)"
    assert t2.to_raw().build() == "string::trim('oyelowo')"

    s = slug(name)
    assert s.fine_tune_params() == "string::slug(name)"
    assert s.to_raw().build() == "string::slug(name)"

    s2 = slug("Codebreather is from #Jupiter")
    assert s2.fine_tune_params() == "string::slug($_param_00000001)"
    assert s2.to_raw().build() == "string::slug('Codebreather is from #Jupiter')"


def test_case_and_split():
    name = field("name")
    low = lowercase(name)
    assert low.fine_tune_params() == "string::lowercase(name)"
    assert low.to_raw().build() == "string::lowercase(name)"

    up = uppercase("hello")
    assert up.fine_tune_params() == "string::uppercase($_param_00000001)"
    assert up.to_raw().build() == "string::uppercase('hello')"

    sp = split("With great power, comes great responsibility", ", ")
    assert sp.fine_tune_params() == "string::split($_param_00000001, $_param_00000002)"
    assert sp.to_raw().build() == "string::split('With great power, comes great responsibility', ', ')"
