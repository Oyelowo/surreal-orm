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

use crate::{
    field::GeometryOrField,
    sql::{Binding, Buildable, Empty, Param, ToRawStatement},
    Field,
};

use super::array::Function;

pub struct Geometry(sql::Value);

impl From<Geometry> for sql::Value {
    fn from(value: Geometry) -> Self {
        value.0
    }
}

impl<T: Into<sql::Geometry>> From<T> for Geometry {
    fn from(value: T) -> Self {
        let value: sql::Geometry = value.into();
        Self(value.into())
    }
}

impl From<Field> for Geometry {
    fn from(value: Field) -> Self {
        Self(value.into())
    }
}

impl From<Param> for Geometry {
    fn from(value: Param) -> Self {
        Self(value.into())
    }
}
// enum GeometryOrField {
//     Field(Field),
//     Geometry(sql::Geometry),
// }
//
// impl From<GeometryOrField> for sql::Value {
//     fn from(value: GeometryOrField) -> Self {
//         match value {
//             GeometryOrField::Field(f) => f.into(),
//             GeometryOrField::Geometry(geo) => geo.into(),
//         }
//     }
// }
//
// impl From<Field> for GeometryOrField {
//     fn from(value: Field) -> Self {
//         Self::Field(value)
//     }
// }
//
// impl<T> From<T> for GeometryOrField
// where
//     T: Into<sql::Geometry>,
// {
//     fn from(value: T) -> Self {
//         let value: sql::Geometry = value.into();
//         Self::Geometry(value)
//     }
// }
pub(crate) fn create_geo_with_single_arg(
    geometry: impl Into<Geometry>,
    fn_suffix: &str,
) -> Function {
    let binding = Binding::new(geometry.into());
    let string = binding.get_param_dollarised();

    Function {
        query_string: format!("geo::{fn_suffix}({})", string),
        bindings: vec![binding],
    }
}

fn create_geo_fn_with_two_args(
    point1: impl Into<Geometry>,
    point2: impl Into<Geometry>,
    fn_suffix: &str,
) -> Function {
    let binding1 = Binding::new(point1.into());
    let string1 = binding1.get_param_dollarised();

    let binding2 = Binding::new(point2.into());
    let string2 = binding2.get_param_dollarised();

    Function {
        query_string: format!("geo::{fn_suffix}({}, {})", string1, string2),
        bindings: vec![binding1, binding2],
    }
}

pub fn area_fn(geometry: impl Into<Geometry>) -> Function {
    create_geo_with_single_arg(geometry, "area")
}

#[macro_export]
macro_rules! geo_area {
    ( $geometry:expr ) => {
        crate::functions::geo::area_fn($geometry)
    };
}

pub use geo_area as area;

pub fn bearing_fn(point1: impl Into<Geometry>, point2: impl Into<Geometry>) -> Function {
    create_geo_fn_with_two_args(point1, point2, "bearing")
}

#[macro_export]
macro_rules! geo_bearing {
    ( $point1:expr,  $point2:expr ) => {
        crate::functions::geo::bearing_fn($point1, $point2)
    };
}

pub use geo_bearing as bearing;

pub fn centroid_fn(geometry: impl Into<Geometry>) -> Function {
    create_geo_with_single_arg(geometry, "centroid")
}

#[macro_export]
macro_rules! geo_centroid {
    ( $geometry:expr ) => {
        crate::functions::geo::centroid_fn($geometry)
    };
}

pub use geo_centroid as centroid;

pub fn distance_fn(point1: impl Into<Geometry>, point2: impl Into<Geometry>) -> Function {
    create_geo_fn_with_two_args(point1, point2, "distance")
}

#[macro_export]
macro_rules! geo_distance {
    ( $point1:expr,  $point2:expr ) => {
        crate::functions::geo::distance_fn($point1, $point2)
    };
}
pub use geo_distance as distance;

pub enum NumberOrEmpty {
    Empty,
    Number(surrealdb::sql::Value),
    // Field(Field),
}

// impl From<NumberOrEmpty> for sql::Value {
//     fn from(value: NumberOrEmpty) -> Self {
//         match value {
//             NumberOrEmpty::Empty => sql::Idiom::from("".to_string()).into(),
//             NumberOrEmpty::Number(n) => n.into(),
//             // NumberOrEmpty::Field(f) => f.into(),
//         }
//     }
// }
impl<T> From<T> for NumberOrEmpty
where
    T: Into<sql::Number>,
{
    fn from(value: T) -> Self {
        let value: sql::Number = value.into();
        Self::Number(value.into())
    }
}

impl From<Field> for NumberOrEmpty {
    fn from(value: Field) -> Self {
        Self::Number(value.into())
    }
}

impl From<Empty> for NumberOrEmpty {
    fn from(value: Empty) -> Self {
        Self::Empty
    }
}

pub mod hash {

    type Accuracy = super::NumberOrEmpty;

    use surrealdb::sql;

    use super::{create_geo_with_single_arg, Geometry};
    use crate::{
        field::GeometryOrField,
        functions::array::Function,
        sql::{Binding, Empty, Param},
        Field,
    };

    pub struct GeoHash(sql::Value);

    impl From<&str> for GeoHash {
        fn from(value: &str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for GeoHash {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }

    impl From<Field> for GeoHash {
        fn from(value: Field) -> Self {
            Self(value.into())
        }
    }

    impl From<Param> for GeoHash {
        fn from(value: Param) -> Self {
            Self(value.into())
        }
    }

    impl From<GeoHash> for sql::Value {
        fn from(value: GeoHash) -> Self {
            value.0
        }
    }

    // impl GeoHash {
    //     fn new(hash: String) -> Self {
    //         Self(hash)
    //     }
    // }
    //
    // enum GeoHashDecodeArg {
    //     Field(Field),
    //     GeoHash(GeoHash),
    // }

    pub fn decode_fn(geometry: impl Into<GeoHash>) -> Function {
        let binding = Binding::new(geometry.into());
        let string = binding.get_param_dollarised();

        Function {
            query_string: format!("geo::hash::decode({})", string),
            bindings: vec![binding],
        }
    }
    #[macro_export]
    macro_rules! geo_hash_decode {
        ( $geometry:expr ) => {
            crate::functions::geo::hash::decode_fn($geometry)
        };
    }
    pub use geo_hash_decode as decode;

    pub fn encode_fn(geometry: impl Into<Geometry>, accuracy: impl Into<Accuracy>) -> Function {
        let binding = Binding::new(geometry.into());
        let accuracy: Accuracy = accuracy.into();
        let geometry_param = binding.get_param_dollarised();

        let mut bindings = vec![binding];

        let str = match accuracy {
            Accuracy::Empty => format!("geo::hash::encode({geometry_param})",),
            Accuracy::Number(num) => {
                let binding = Binding::new(num);
                let accuracy_param = binding.get_param_dollarised();
                bindings.push(binding);

                format!("geo::hash::encode({geometry_param}, {accuracy_param})",)
            } // Accuracy::Field(field) => {
              //     let binding = Binding::new(field);
              //     let accuracy_param = binding.get_param_dollarised();
              //     bindings.push(binding);
              //
              //     format!("geo::hash::encode({geometry_param}, {accuracy_param})",)
              // }
        };
        Function {
            query_string: str,
            bindings,
        }
    }

    #[macro_export]
    macro_rules! geo_hash_encode {
        ( $geometry:expr, $accuracy:expr ) => {
            crate::functions::geo::encode_fn($geometry, $accuracy)
        };
        ( $geometry:expr ) => {
            crate::functions::geo::encode_fn($geometry)
        };
    }
    pub use geo_hash_encode as encode;
}
#[test]
fn test_area_with_field() {
    let city = Field::new("city");
    let result = area_fn(city);

    assert_eq!(result.fine_tune_params(), "geo::area($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "geo::area(city)");
}

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
    assert_eq!(result.fine_tune_params(), "geo::area($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "geo::area(poly)");
}

#[test]
fn test_area_macro_with_params() {
    let poly = Param::new("poly");
    let result = area!(poly);
    assert_eq!(result.fine_tune_params(), "geo::area($_param_00000001)");
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
        "geo::bearing($_param_00000001, $_param_00000002)"
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
        "geo::bearing($_param_00000001, $_param_00000002)"
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
        "geo::bearing($_param_00000001, $_param_00000002)"
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

    assert_eq!(result.fine_tune_params(), "geo::centroid($_param_00000001)");
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

    assert_eq!(result.fine_tune_params(), "geo::centroid($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "geo::centroid(city)");
}

#[test]
fn test_centroid_macro_with_param() {
    let city = Param::new("city");
    let result = centroid!(city);

    assert_eq!(result.fine_tune_params(), "geo::centroid($_param_00000001)");
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
        "geo::distance($_param_00000001, $_param_00000002)"
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
    assert_eq!(
        result.fine_tune_params(),
        "geo::distance($_param_00000001, $_param_00000002)"
    );
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
        "geo::distance($_param_00000001, $_param_00000002)"
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
    assert_eq!(
        result.fine_tune_params(),
        "geo::distance($_param_00000001, $_param_00000002)"
    );
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
        "geo::distance($_param_00000001, $_param_00000002)"
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

    assert_eq!(
        result.fine_tune_params(),
        "geo::hash::decode($_param_00000001)"
    );
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

    assert_eq!(
        result.fine_tune_params(),
        "geo::hash::decode($_param_00000001)"
    );
    assert_eq!(result.to_raw().to_string(), "geo::hash::decode(city)");
}

#[test]
fn test_hash_decode_macro_with_param() {
    let city = Param::new("city");
    let result = hash::decode!(city);

    assert_eq!(
        result.fine_tune_params(),
        "geo::hash::decode($_param_00000001)"
    );
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
    let result = hash::encode_fn(city, Empty);

    assert_eq!(
        result.fine_tune_params(),
        "geo::hash::encode($_param_00000001)"
    );
    assert_eq!(result.to_raw().to_string(), "geo::hash::encode(city)");
}

#[test]
fn test_hash_encode_with_field_and_field_accuracy() {
    let city = Field::new("city");
    let accuracy = Field::new("accuracy");
    let result = hash::encode_fn(city, accuracy);

    assert_eq!(
        result.fine_tune_params(),
        "geo::hash::encode($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::hash::encode(city, accuracy)"
    );
}

#[test]
fn test_hash_encode_with_field_and_number_accuracy() {
    let city = Field::new("city");
    let result = hash::encode_fn(city, 5);

    assert_eq!(
        result.fine_tune_params(),
        "geo::hash::encode($_param_00000001, $_param_00000002)"
    );
    assert_eq!(result.to_raw().to_string(), "geo::hash::encode(city, 5)");
}

#[test]
fn test_hash_encode_with_point() {
    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };

    let result = hash::encode_fn(point, 5);
    assert_eq!(
        result.fine_tune_params(),
        "geo::hash::encode($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::hash::encode((40.02, 116.34), 5)"
    );
}
