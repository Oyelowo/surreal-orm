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
    sql::{Binding, Buildable, ToRawStatement},
    Field,
};

use super::array::Function;

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
    geometry: impl Into<GeometryOrField>,
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
    point1: impl Into<GeometryOrField>,
    point2: impl Into<GeometryOrField>,
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

pub fn area(geometry: impl Into<GeometryOrField>) -> Function {
    create_geo_with_single_arg(geometry, "area")
}

pub fn bearing(point1: impl Into<GeometryOrField>, point2: impl Into<GeometryOrField>) -> Function {
    create_geo_fn_with_two_args(point1, point2, "bearing")
}

pub fn centroid(geometry: impl Into<GeometryOrField>) -> Function {
    create_geo_with_single_arg(geometry, "centroid")
}

pub fn distance(
    point1: impl Into<GeometryOrField>,
    point2: impl Into<GeometryOrField>,
) -> Function {
    create_geo_fn_with_two_args(point1, point2, "distance")
}

pub mod hash {
    pub enum Accuracy {
        Empty,
        Number(surrealdb::sql::Number),
        Field(Field),
    }

    impl<T> From<T> for Accuracy
    where
        T: Into<sql::Number>,
    {
        fn from(value: T) -> Self {
            let value: sql::Number = value.into();
            Self::Number(value)
        }
    }

    impl From<Field> for Accuracy {
        fn from(value: Field) -> Self {
            Self::Field(value)
        }
    }

    impl From<Empty> for Accuracy {
        fn from(value: Empty) -> Self {
            Self::Empty
        }
    }

    use surrealdb::sql;

    use super::create_geo_with_single_arg;
    use crate::{
        field::GeometryOrField,
        functions::array::Function,
        sql::{Binding, Empty},
        Field,
    };

    pub fn decode(geometry: impl Into<GeometryOrField>) -> Function {
        create_geo_with_single_arg(geometry, "hash::decode")
    }

    pub fn encode(geometry: impl Into<GeometryOrField>, accuracy: Accuracy) -> Function {
        let binding = Binding::new(geometry.into());
        let geometry_param = binding.get_param_dollarised();

        let mut bindings = vec![binding];

        let str = match accuracy {
            Accuracy::Empty => format!("geo::hash::encode({geometry_param})",),
            Accuracy::Number(num) => {
                let binding = Binding::new(num);
                let accuracy_param = binding.get_param_dollarised();
                bindings.push(binding);

                format!("geo::hash::encode({geometry_param}, {accuracy_param})",)
            }
            Accuracy::Field(field) => {
                let binding = Binding::new(field);
                let accuracy_param = binding.get_param_dollarised();
                bindings.push(binding);

                format!("geo::hash::encode({geometry_param}, {accuracy_param})",)
            }
        };
        Function {
            query_string: str,
            bindings,
        }
    }
}
#[test]
fn test_area_with_field() {
    let city = Field::new("city");
    let result = area(city);

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
    let result = area(poly);
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
fn test_bearing_with_raw_points() {
    let point1 = point! {
        x: 40.02f64,
        y: 116.34,
    };

    let point2 = point! {
        x: 80.02f64,
        y: 103.19,
    };
    let result = bearing(point1, point2);
    assert_eq!(
        result.fine_tune_params(),
        "geo::bearing($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "geo::bearing((40.02, 116.34), (80.02, 103.19))"
    );
}
