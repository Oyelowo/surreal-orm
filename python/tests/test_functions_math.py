from surreal_orm.query_builder import field, fixed, Param


def test_fixed_fn_with_field_data():
    land_size = field("land_size")
    decimal_place = field("decimal_place")
    result = fixed(land_size, decimal_place)

    assert result.fine_tune_params() == "math::fixed(land_size, decimal_place)"
    assert result.to_raw().build() == "math::fixed(land_size, decimal_place)"


def test_fixed_fn_with_raw_numbers():
    result = fixed(13.45423, 4)
    assert result.fine_tune_params() == "math::fixed($_param_00000001, $_param_00000002)"
    assert result.to_raw().build() == "math::fixed(13.45423f, 4)"


def test_fixed_fn_with_raw_number_with_field():
    land_mass = field("country.land_mass")
    result = fixed(land_mass, 4)
    assert result.fine_tune_params() == "math::fixed(country.land_mass, $_param_00000001)"
    assert result.to_raw().build() == "math::fixed(country.land_mass, 4)"


# Macro versions (simulated same API)

def test_fixed_macro_with_field_data():
    land_size = field("land_size")
    decimal_place = field("decimal_place")
    result = fixed(land_size, decimal_place)

    assert result.fine_tune_params() == "math::fixed(land_size, decimal_place)"
    assert result.to_raw().build() == "math::fixed(land_size, decimal_place)"


def test_fixed_macro_with_params():
    land_size = Param("land_size")
    decimal_place = Param("decimal_place")
    result = fixed(land_size, decimal_place)

    assert result.fine_tune_params() == "math::fixed($land_size, $decimal_place)"
    assert result.to_raw().build() == "math::fixed($land_size, $decimal_place)"


def test_fixed_macro_with_raw_numbers():
    result = fixed(13.45423, 4)
    assert result.fine_tune_params() == "math::fixed($_param_00000001, $_param_00000002)"
    assert result.to_raw().build() == "math::fixed(13.45423f, 4)"


def test_fixed_macro_with_raw_number_with_field():
    land_mass = field("country.land_mass")
    result = fixed(land_mass, 4)
    assert result.fine_tune_params() == "math::fixed(country.land_mass, $_param_00000001)"
    assert result.to_raw().build() == "math::fixed(country.land_mass, 4)"
