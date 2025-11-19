from surreal_orm.query_builder import (
    area,
    bearing,
    centroid,
    distance,
    hash,
    field,
    Param,
)


def test_area_with_field_and_polygon_raw():
    f = field("city")
    r = area(f)
    assert r.fine_tune_params() == "geo::area(city)"
    assert r.to_raw().build() == "geo::area(city)"


def test_bearing_raw_points_and_field():
    p1 = (40.02, 116.34)
    p2 = (80.02, 103.19)
    r = bearing(p1, p2)
    assert r.fine_tune_params() == "geo::bearing($_param_00000001, $_param_00000002)"
    assert r.to_raw().build() == "geo::bearing((40.02, 116.34), (80.02, 103.19))"

    hometown = field("hometown")
    r2 = bearing(hometown, p2)
    assert r2.fine_tune_params() == "geo::bearing(hometown, $_param_00000001)"
    assert r2.to_raw().build() == "geo::bearing(hometown, (80.02, 103.19))"


def test_centroid_and_distance_variants():
    # Simulate a raw geometry object exposing to_raw() and fine_tune_params()
    class RawGeom:
        def __init__(self, s):
            self.s = s

        def to_raw(self):
            class R:
                def __init__(self, v):
                    self._v = v

                def build(self):
                    return self._v

            return R(self.s)

        def fine_tune_params(self):
            # Return a single placeholder so geo functions allocate one param for the geometry
            return "$_param_00000001"

    poly = RawGeom("{ type: 'Polygon', coordinates: [...] }")
    c = centroid(poly)
    assert c.fine_tune_params() == "geo::centroid($_param_00000001)"

    p1 = (40.02, 116.34)
    p2 = (80.02, 103.19)
    d = distance(p1, p2)
    assert d.fine_tune_params() == "geo::distance($_param_00000001, $_param_00000002)"
    assert d.to_raw().build() == "geo::distance((40.02, 116.34), (80.02, 103.19))"


def test_hash_decode_encode():
    r = hash.decode("mpuxk4s24f51")
    assert r.fine_tune_params() == "geo::hash::decode($_param_00000001)"
    assert r.to_raw().build() == "geo::hash::decode('mpuxk4s24f51')"

    f = field("city")
    r2 = hash.encode(f)
    assert r2.fine_tune_params() == "geo::hash::encode(city)"
    assert r2.to_raw().build() == "geo::hash::encode(city)"

    r3 = hash.encode(p1 := (40.02, 116.34), 5)
    assert r3.fine_tune_params() == "geo::hash::encode($_param_00000001, $_param_00000002)"
    assert r3.to_raw().build() == "geo::hash::encode((40.02, 116.34), 5)"
