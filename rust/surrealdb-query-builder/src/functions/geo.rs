/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Geo functions
// These functions can be used when working with and analysing geospatial data.
//
// Function	Description
// geo::area()	Calculates the area of a geometry
// geo::bearing()	Calculates the bearing between two geolocation points
// geo::centroid()	Calculates the centroid of a geometry
// geo::distance()	Calculates the distance between two geolocation points
// geo::hash::decode()	Decodes a geohash into a geometry point
// geo::hash::encode()	Encodes a geometry point into a geohash
//
//

use geo::{point, polygon};
use surrealdb::sql;

pub(crate) fn create_geo_with_single_arg(
    geometry: impl Into<GeometryLike>,
    fn_suffix: &str,
) -> Function {
    let geometry: GeometryLike = geometry.into();

    Function {
        query_string: format!("geo::{fn_suffix}({})", geometry.build()),
        bindings: geometry.get_bindings(),
    }
}

fn create_geo_fn_with_two_args(
    point1: impl Into<GeometryLike>,
    point2: impl Into<GeometryLike>,
    fn_suffix: &str,
) -> Function {
    let point1: GeometryLike = point1.into();
    let point2: GeometryLike = point2.into();
    let mut bindings = point1.get_bindings();
    bindings.extend(point2.get_bindings());

    Function {
        query_string: format!("geo::{fn_suffix}({}, {})", point1.build(), point2.build()),
        bindings,
    }
}

pub fn area_fn(geometry: impl Into<GeometryLike>) -> Function {
    create_geo_with_single_arg(geometry, "area")
}

#[macro_export]
macro_rules! geo_area {
    ( $geometry:expr ) => {
        $crate::functions::geo::area_fn($geometry)
    };
}

pub use geo_area as area;

pub fn bearing_fn(point1: impl Into<GeometryLike>, point2: impl Into<GeometryLike>) -> Function {
    create_geo_fn_with_two_args(point1, point2, "bearing")
}

#[macro_export]
macro_rules! geo_bearing {
    ( $point1:expr,  $point2:expr ) => {
        $crate::functions::geo::bearing_fn($point1, $point2)
    };
}

pub use geo_bearing as bearing;

pub fn centroid_fn(geometry: impl Into<GeometryLike>) -> Function {
    create_geo_with_single_arg(geometry, "centroid")
}

#[macro_export]
macro_rules! geo_centroid {
    ( $geometry:expr ) => {
        $crate::functions::geo::centroid_fn($geometry)
    };
}

pub use geo_centroid as centroid;

pub fn distance_fn(point1: impl Into<GeometryLike>, point2: impl Into<GeometryLike>) -> Function {
    create_geo_fn_with_two_args(point1, point2, "distance")
}

#[macro_export]
macro_rules! geo_distance {
    ( $point1:expr,  $point2:expr ) => {
        $crate::functions::geo::distance_fn($point1, $point2)
    };
}
pub use geo_distance as distance;

use crate::{
    traits::{Binding, Buildable, ToRaw},
    types::{Field, Function, GeometryLike, NumberLike, Param},
    Parametric,
};

pub mod hash {
    use super::create_geo_with_single_arg;
    use crate::{
        traits::Binding,
        types::{Function, GeometryLike, NumberLike, StrandLike},
        Buildable, Parametric,
    };
    use surrealdb::sql;

    pub type GeoHash = StrandLike;

    pub fn decode_fn(geohash: impl Into<GeoHash>) -> Function {
        let string: GeoHash = geohash.into();

        Function {
            query_string: format!("geo::hash::decode({})", string.build()),
            bindings: string.get_bindings(),
        }
    }
    #[macro_export]
    macro_rules! geo_hash_decode {
        ( $geometry:expr ) => {
            $crate::functions::geo::hash::decode_fn($geometry)
        };
    }
    pub use geo_hash_decode as decode;

    pub fn encode_fn(
        geometry: impl Into<GeometryLike>,
        accuracy: Option<impl Into<NumberLike>>,
    ) -> Function {
        let geometry: GeometryLike = geometry.into();

        let mut bindings = geometry.get_bindings();

        let str = if let Some(accuracy) = accuracy {
            let accuracy: NumberLike = accuracy.into();
            bindings.extend(accuracy.get_bindings());

            format!(
                "geo::hash::encode({}, {})",
                geometry.build(),
                accuracy.build()
            )
        } else {
            format!("geo::hash::encode({})", geometry.build())
        };

        Function {
            query_string: str,
            bindings,
        }
    }

    #[macro_export]
    macro_rules! geo_hash_encode {
        ( $geometry:expr ) => {
            $crate::functions::geo::hash::encode_fn(
                $geometry,
                None as Option<$crate::types::NumberLike>,
            )
        };
        ( $geometry:expr, $accuracy:expr ) => {
            $crate::functions::geo::hash::encode_fn($geometry, Some($accuracy))
        };
    }
    pub use geo_hash_encode as encode;
}
#[test]
fn test_area_with_field() {
    let city = Field::new("city");
    let result = area_fn(city);

    assert_eq!(result.fine_tune_params(), "geo::area(city)");
    assert_eq!(result.to_raw().to_string(), "geo::area(city)");
}
//
#[test]
fn test_area_with_raw_polygon() {
    let poly = polygon!(
            exterior: [
                (x: -111., y: 45.),
                (x: -111., y: 41.),
                (x: -104., y: 41.),
                (x: -104., y: 45.),
            ],
            interiors: [
                [
                    (x: -110., y: 44.),
                    (x: -110., y: 42.),
                    (x: -105., y: 42.),
                    (x: -105., y: 44.),
                ],
            ],
        );
    let result = area_fn(poly);
    assert_eq!(
        result.fine_tune_params(),
        "geo::area($_param_00000001)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::area({ type: 'Polygon', coordinates: [[[-111, 45], [-111, 41], [-104, 41], [-104, 45], [-111, 45]], [[[-110, 44], [-110, 42], [-105, 42], [-105, 44], [-110, 44]]]] })"
    );
}

#[test]
fn test_area_macro_with_raw_polygon() {
    let poly = polygon!(
            exterior: [
                (x: -111., y: 45.),
                (x: -111., y: 41.),
                (x: -104., y: 41.),
                (x: -104., y: 45.),
            ],
            interiors: [
                [
                    (x: -110., y: 44.),
                    (x: -110., y: 42.),
                    (x: -105., y: 42.),
                    (x: -105., y: 44.),
                ],
            ],
        );
    let result = area!(poly);
    assert_eq!(result.fine_tune_params(), "geo::area($_param_00000001)");
    assert_eq!(
        result.to_raw().to_string(),
        "geo::area({ type: 'Polygon', coordinates: [[[-111, 45], [-111, 41], [-104, 41], [-104, 45], [-111, 45]], [[[-110, 44], [-110, 42], [-105, 42], [-105, 44], [-110, 44]]]] })"
    );
}

#[test]
fn test_area_macro_with_fields() {
    let poly = Field::new("poly");
    let result = area!(poly);
    assert_eq!(result.fine_tune_params(), "geo::area(poly)");
    assert_eq!(result.to_raw().to_string(), "geo::area(poly)");
}

#[test]
fn test_area_macro_with_params() {
    let poly = Param::new("poly");
    let result = area!(poly);
    assert_eq!(result.fine_tune_params(), "geo::area($poly)");
    assert_eq!(result.to_raw().to_string(), "geo::area($poly)");
}

#[test]
fn test_bearing_with_raw_points() {
    let point1 = point! {
        x: 40.02f64,
        y: 116.34,
    };

    let point2 = point! {
        x: 80.02f64,
        y: 103.19,
    };
    let result = bearing_fn(point1, point2);
    assert_eq!(
        result.fine_tune_params(),
        "geo::bearing($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::bearing((40.02, 116.34), (80.02, 103.19))"
    );
}

#[test]
fn test_bearing_with_raw_point_with_field() {
    let hometown = Field::new("hometown");

    let point2 = point! {
        x: 80.02f64,
        y: 103.19,
    };
    let result = bearing_fn(hometown, point2);
    assert_eq!(
        result.fine_tune_params(),
        "geo::bearing(hometown, $_param_00000001)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::bearing(hometown, (80.02, 103.19))"
    );
}

#[test]
fn test_bearing_macro_with_raw_points() {
    let point1 = point! {
        x: 40.02f64,
        y: 116.34,
    };

    let point2 = point! {
        x: 80.02f64,
        y: 103.19,
    };
    let result = bearing!(point1, point2);
    assert_eq!(
        result.fine_tune_params(),
        "geo::bearing($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::bearing((40.02, 116.34), (80.02, 103.19))"
    );
}

#[test]
fn test_bearing_macro_with_raw_point_with_field() {
    let hometown = Field::new("hometown");

    let point2 = point! {
        x: 80.02f64,
        y: 103.19,
    };
    let result = bearing!(hometown, point2);
    assert_eq!(
        result.fine_tune_params(),
        "geo::bearing(hometown, $_param_00000001)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::bearing(hometown, (80.02, 103.19))"
    );
}

#[test]
fn test_bearing_macro_with_raw_params() {
    let hometown = Param::new("hometown");
    let point2 = Param::new("point2");

    let result = bearing!(hometown, point2);
    assert_eq!(
        result.fine_tune_params(),
        "geo::bearing($hometown, $point2)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::bearing($hometown, $point2)"
    );
}

#[test]
fn test_centroid_with_field() {
    let city = Field::new("city");
    let result = centroid_fn(city);

    assert_eq!(result.fine_tune_params(), "geo::centroid(city)");
    assert_eq!(result.to_raw().to_string(), "geo::centroid(city)");
}

#[test]
fn test_centroid_with_raw_polygon() {
    let poly = polygon!(
            exterior: [
                (x: -111., y: 45.),
                (x: -111., y: 41.),
                (x: -104., y: 41.),
                (x: -104., y: 45.),
            ],
            interiors: [
                [
                    (x: -110., y: 44.),
                    (x: -110., y: 42.),
                    (x: -105., y: 42.),
                    (x: -105., y: 44.),
                ],
            ],
        );
    let result = centroid_fn(poly);
    assert_eq!(
        result.fine_tune_params(),
        "geo::centroid($_param_00000001)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::centroid({ type: 'Polygon', coordinates: [[[-111, 45], [-111, 41], [-104, 41], [-104, 45], [-111, 45]], [[[-110, 44], [-110, 42], [-105, 42], [-105, 44], [-110, 44]]]] })"
    );
}

#[test]
fn test_centroid_macro_with_field() {
    let city = Field::new("city");
    let result = centroid!(city);

    assert_eq!(result.fine_tune_params(), "geo::centroid(city)");
    assert_eq!(result.to_raw().to_string(), "geo::centroid(city)");
}

#[test]
fn test_centroid_macro_with_param() {
    let city = Param::new("city");
    let result = centroid!(city);

    assert_eq!(result.fine_tune_params(), "geo::centroid($city)");
    assert_eq!(result.to_raw().to_string(), "geo::centroid($city)");
}

#[test]
fn test_centroid_macro_with_raw_polygon() {
    let poly = polygon!(
            exterior: [
                (x: -111., y: 45.),
                (x: -111., y: 41.),
                (x: -104., y: 41.),
                (x: -104., y: 45.),
            ],
            interiors: [
                [
                    (x: -110., y: 44.),
                    (x: -110., y: 42.),
                    (x: -105., y: 42.),
                    (x: -105., y: 44.),
                ],
            ],
        );
    let result = centroid!(poly);
    assert_eq!(result.fine_tune_params(), "geo::centroid($_param_00000001)");
    assert_eq!(
        result.to_raw().to_string(),
        "geo::centroid({ type: 'Polygon', coordinates: [[[-111, 45], [-111, 41], [-104, 41], [-104, 45], [-111, 45]], [[[-110, 44], [-110, 42], [-105, 42], [-105, 44], [-110, 44]]]] })"
    );
}

#[test]
fn test_distance_with_raw_points() {
    let point1 = point! {
        x: 40.02f64,
        y: 116.34,
    };

    let point2 = point! {
        x: 80.02f64,
        y: 103.19,
    };
    let result = distance_fn(point1, point2);
    assert_eq!(
        result.fine_tune_params(),
        "geo::distance($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::distance((40.02, 116.34), (80.02, 103.19))"
    );
}

#[test]
fn test_distance_with_raw_point_with_field() {
    let hometown = Field::new("hometown");

    let point2 = point! {
        x: 80.02f64,
        y: 103.19,
    };
    let result = distance_fn(hometown, point2);
    assert_eq!(
        result.fine_tune_params(),
        "geo::distance(hometown, $_param_00000001)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::distance(hometown, (80.02, 103.19))"
    );
}

#[test]
fn test_distance_with_only_fields() {
    let hometown = Field::new("hometown");
    let yukon = Field::new("yukon");

    let result = distance_fn(hometown, yukon);
    assert_eq!(result.fine_tune_params(), "geo::distance(hometown, yukon)");
    assert_eq!(
        result.to_raw().to_string(),
        "geo::distance(hometown, yukon)"
    );
}

#[test]
fn test_distance_macro_with_raw_points() {
    let point1 = point! {
        x: 40.02f64,
        y: 116.34,
    };

    let point2 = point! {
        x: 80.02f64,
        y: 103.19,
    };
    let result = distance!(point1, point2);
    assert_eq!(
        result.fine_tune_params(),
        "geo::distance($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::distance((40.02, 116.34), (80.02, 103.19))"
    );
}

#[test]
fn test_distance_macro_with_raw_point_with_field() {
    let hometown = Field::new("hometown");

    let point2 = point! {
        x: 80.02f64,
        y: 103.19,
    };
    let result = distance!(hometown, point2);
    assert_eq!(
        result.fine_tune_params(),
        "geo::distance(hometown, $_param_00000001)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::distance(hometown, (80.02, 103.19))"
    );
}

#[test]
fn test_distance_macro_with_only_fields() {
    let hometown = Field::new("hometown");
    let yukon = Field::new("yukon");

    let result = distance!(hometown, yukon);
    assert_eq!(result.fine_tune_params(), "geo::distance(hometown, yukon)");
    assert_eq!(
        result.to_raw().to_string(),
        "geo::distance(hometown, yukon)"
    );
}

#[test]
fn test_distance_macro_with_only_params() {
    let hometown = Param::new("hometown");
    let yukon = Param::new("yukon");

    let result = distance!(hometown, yukon);
    assert_eq!(
        result.fine_tune_params(),
        "geo::distance($hometown, $yukon)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::distance($hometown, $yukon)"
    );
}

#[test]
fn test_hash_decode_with_field() {
    let city = Field::new("city");
    let result = hash::decode_fn(city);

    assert_eq!(result.fine_tune_params(), "geo::hash::decode(city)");
    assert_eq!(result.to_raw().to_string(), "geo::hash::decode(city)");
}

#[test]
fn test_hash_decode_with_string() {
    let result = hash::decode_fn("mpuxk4s24f51");
    assert_eq!(
        result.fine_tune_params(),
        "geo::hash::decode($_param_00000001)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::hash::decode('mpuxk4s24f51')"
    );
}

#[test]
fn test_hash_decode_macro_with_field() {
    let city = Field::new("city");
    let result = hash::decode!(city);

    assert_eq!(result.fine_tune_params(), "geo::hash::decode(city)");
    assert_eq!(result.to_raw().to_string(), "geo::hash::decode(city)");
}

#[test]
fn test_hash_decode_macro_with_param() {
    let city = Param::new("city");
    let result = hash::decode!(city);

    assert_eq!(result.fine_tune_params(), "geo::hash::decode($city)");
    assert_eq!(result.to_raw().to_string(), "geo::hash::decode($city)");
}

#[test]
fn test_hash_decode_macro_with_string() {
    let result = hash::decode!("mpuxk4s24f51");
    assert_eq!(
        result.fine_tune_params(),
        "geo::hash::decode($_param_00000001)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::hash::decode('mpuxk4s24f51')"
    );
}

#[test]
fn test_hash_encode_with_field_and_empty_accuracy() {
    let city = Field::new("city");
    let result = hash::encode_fn(city, None as Option<NumberLike>);

    assert_eq!(result.fine_tune_params(), "geo::hash::encode(city)");
    assert_eq!(result.to_raw().to_string(), "geo::hash::encode(city)");
}

#[test]
fn test_hash_encode_with_field_and_field_accuracy() {
    let city = Field::new("city");
    let accuracy = Field::new("accuracy");
    let result = hash::encode_fn(city, Some(accuracy));

    assert_eq!(
        result.fine_tune_params(),
        "geo::hash::encode(city, accuracy)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::hash::encode(city, accuracy)"
    );
}

#[test]
fn test_hash_encode_with_field_and_number_accuracy() {
    let city = Field::new("city");
    let result = hash::encode_fn(city, Some(5));

    assert_eq!(
        result.fine_tune_params(),
        "geo::hash::encode(city, $_param_00000001)"
    );
    assert_eq!(result.to_raw().to_string(), "geo::hash::encode(city, 5)");
}

#[test]
fn test_hash_encode_with_point() {
    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };

    let result = hash::encode_fn(point, Some(5));
    assert_eq!(
        result.fine_tune_params(),
        "geo::hash::encode($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::hash::encode((40.02, 116.34), 5)"
    );
}
// Macro version
#[test]
fn test_hash_encode_macro_with_field_and_empty_accuracy_not_mentioned_at_all() {
    let city = Field::new("city");
    let result = hash::encode!(city);

    assert_eq!(result.fine_tune_params(), "geo::hash::encode(city)");
    assert_eq!(result.to_raw().to_string(), "geo::hash::encode(city)");
}

#[test]
fn test_hash_encode_macro_with_field_and_empty_accuracy() {
    let city = Field::new("city");
    let result = hash::encode!(city);

    assert_eq!(result.fine_tune_params(), "geo::hash::encode(city)");
    assert_eq!(result.to_raw().to_string(), "geo::hash::encode(city)");
}

#[test]
fn test_hash_encode_macro_with_field_and_field_accuracy() {
    let city = Field::new("city");
    let accuracy = Field::new("accuracy");
    let result = hash::encode!(city, accuracy);

    assert_eq!(
        result.fine_tune_params(),
        "geo::hash::encode(city, accuracy)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::hash::encode(city, accuracy)"
    );
}

#[test]
fn test_hash_encode_macro_with_param_and_no_accuracy_listed() {
    let city = Param::new("city");
    let result = hash::encode!(city);

    assert_eq!(result.fine_tune_params(), "geo::hash::encode($city)");
    assert_eq!(result.to_raw().to_string(), "geo::hash::encode($city)");
}

#[test]
fn test_hash_encode_macros_with_field_and_number_accuracy() {
    let city = Field::new("city");
    let result = hash::encode!(city, 5);

    assert_eq!(
        result.fine_tune_params(),
        "geo::hash::encode(city, $_param_00000001)"
    );
    assert_eq!(result.to_raw().to_string(), "geo::hash::encode(city, 5)");
}

#[test]
fn test_hash_encode_macro_with_point() {
    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };

    let result = hash::encode!(point, 5);
    assert_eq!(
        result.fine_tune_params(),
        "geo::hash::encode($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::hash::encode((40.02, 116.34), 5)"
    );
}
