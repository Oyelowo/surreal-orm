from surreal_orm.query_builder import (
    now,
    day,
    hour,
    minute,
    month,
    nano,
    second,
    unix,
    wday,
    week,
    yday,
    year,
    floor,
    round,
    group,
    timezone,
    max_,
    min_,
    from_,
    field,
    Param,
)


def test_now_and_simple_time_funcs():
    n = now()
    assert n.fine_tune_params() == "now()"
    assert n.to_raw().build() == "now()"

    f = field("rebirth_date")
    assert day(f).to_raw().build() == "time::day(rebirth_date)"

    # string datetime
    assert hour("1970-01-01T00:01:01Z").to_raw().build() == "time::hour('1970-01-01T00:01:01Z')"


def test_floor_round_group_and_from():
    # duration as integer seconds -> formatted
    r = floor("1970-01-01T00:01:01Z", 604800)
    assert r.to_raw().build() == "time::floor('1970-01-01T00:01:01Z', 1w)"
    assert r.fine_tune_params().startswith("time::floor($_param_")

    rt = round("1970-01-01T00:01:01Z", 10)
    assert rt.to_raw().build() == "time::round('1970-01-01T00:01:01Z', 10s)"

    g = group("1970-01-01T00:01:01Z", "year")
    assert g.to_raw().build() == "time::group('1970-01-01T00:01:01Z', 'year')"

    # from:: functions
    assert from_.micros(1).to_raw().build() == "time::from::micros(1)"
    assert from_.unix(1).to_raw().build() == "time::from::unix(1)"


def test_max_min_and_timezone():
    f = field("rebirth_date")
    m = max_(["1970-01-01T00:01:01Z", f])
    assert m.to_raw().build().startswith("time::max(['1970-01-01T00:01:01Z'")

    mi = min_(f)
    assert mi.to_raw().build() == "time::min(rebirth_date)"

    tz = timezone()
    assert tz.to_raw().build() == "time::timezone()"
