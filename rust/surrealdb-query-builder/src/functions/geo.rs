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

use geo::polygon;
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
struct Geometry(sql::Value);

pub fn area(geometry: impl Into<GeometryOrField>) -> Function {
    let binding = Binding::new(geometry.into());
    let string = binding.get_param_dollarised();

    Function {
        query_string: format!("geo::area({})", string),
        bindings: vec![binding],
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
