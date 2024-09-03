/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
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

use crate::{Buildable, Erroneous, Function, GeometryLike, Parametric};

pub(crate) fn create_geo_with_single_arg(
    geometry: impl Into<GeometryLike>,
    fn_suffix: &str,
) -> Function {
    let geometry: GeometryLike = geometry.into();

    Function {
        query_string: format!("geo::{fn_suffix}({})", geometry.build()),
        bindings: geometry.get_bindings(),
        errors: geometry.get_errors(),
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
        errors: point1.get_errors(),
    }
}

/// The geo::area function calculates the area of a geometry.
pub fn area_fn(geometry: impl Into<GeometryLike>) -> Function {
    create_geo_with_single_arg(geometry, "area")
}

/// The geo::area function calculates the area of a geometry.
/// Also aliased as `geo_area!`.
///
/// # Arguments
///
/// * `geometry` - The geometry to calculate the area of. This can be a geometry, field, or parameter.
///
/// # Example
///
/// ```rust
/// # use surreal_query_builder as  surreal_orm;
/// use surreal_orm::{*, functions::geo};
/// use ::geo::{polygon};
/// let polygon = polygon!(
///     exterior: [
///         (x: -111., y: 45.),
///         (x: -111., y: 41.),
///         (x: -104., y: 41.),
///         (x: -104., y: 45.),
///     ],
///     interiors: [
///         [
///             (x: -110., y: 44.),
///             (x: -110., y: 42.),
///             (x: -105., y: 42.),
///             (x: -105., y: 44.),
///         ],
///     ],
/// );
///
/// let result = geo::area!(polygon);
/// assert_eq!(result.to_raw().build(), "geo::area({ type: 'Polygon', coordinates: [[[-111, 45], [-111, 41], [-104, 41], [-104, 45], [-111, 45]], [[[-110, 44], [-110, 42], [-105, 42], [-105, 44], [-110, 44]]]] })");
///
/// # let geometry_field = Field::new("geometry_field");
///  geo::area!(geometry_field);
/// # let geometry_param = Param::new("geometry_param");
/// geo::area!(geometry_param);
/// ```
#[macro_export]
macro_rules! geo_area {
    ( $geometry:expr ) => {
        $crate::functions::geo::area_fn($geometry)
    };
}

pub use geo_area as area;

/// The geo::bearing function calculates the bearing between two geolocation points.
pub fn bearing_fn(point1: impl Into<GeometryLike>, point2: impl Into<GeometryLike>) -> Function {
    create_geo_fn_with_two_args(point1, point2, "bearing")
}

/// The geo::bearing function calculates the bearing between two geolocation points.
/// Also aliased as `geo_bearing!`.
///
/// # Arguments
///
/// * `point1` - The first point to calculate the bearing from. This can be a geometry, field, or parameter.
/// * `point2` - The second point to calculate the bearing to. This can be a geometry, field, or parameter.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as  surreal_orm;
/// use surreal_orm::{*, functions::geo};
/// use ::geo::point;
/// let point1 = point! {
///     x: 40.02f64,
///     y: 116.34,
/// };
/// let point2 = point! {
///     x: 40.02f64,
///     y: 116.34,
/// };
/// let result = geo::bearing!(point1, point2);
/// assert_eq!(result.to_raw().build(), "geo::bearing((40.02, 116.34), (40.02, 116.34))");
/// # let point1_field = Field::new("point1_field");
/// # let point2_field = Field::new("point2_field");
/// geo::bearing!(point1_field, point2_field);
/// # let point1_param = Param::new("point1_param");
/// # let point2_param = Param::new("point2_param");
/// geo::bearing!(point1_param, point2_param);
/// ```
#[macro_export]
macro_rules! geo_bearing {
    ( $point1:expr,  $point2:expr ) => {
        $crate::functions::geo::bearing_fn($point1, $point2)
    };
}

pub use geo_bearing as bearing;

/// The geo::centroid function calculates the centroid between two geolocation points.
pub fn centroid_fn(geometry: impl Into<GeometryLike>) -> Function {
    create_geo_with_single_arg(geometry, "centroid")
}

/// The geo::centroid function calculates the centroid between two geolocation points.
/// Also aliased as `geo_centroid!`.
///
/// # Arguments
///
/// * `geometry` - The geometry to calculate the centroid of. This can be a geometry, field, or parameter.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as  surreal_orm;
/// use surreal_orm::{*, functions::geo};
/// use ::geo::polygon;
/// let polygon = polygon!(
///    exterior: [
///         (x: -111., y: 45.),
///         (x: -111., y: 41.),
///         (x: -104., y: 41.),
///         (x: -104., y: 45.),
///    ],
///    interiors: [[
///         (x: -110., y: 44.),
///         (x: -110., y: 42.),
///         (x: -105., y: 42.),
///         (x: -105., y: 44.),
///    ]],
/// );
/// let result = geo::centroid!(polygon);
/// assert_eq!(result.to_raw().build(), "geo::centroid({ type: 'Polygon', coordinates: [[[-111, 45], [-111, 41], [-104, 41], [-104, 45], [-111, 45]], [[[-110, 44], [-110, 42], [-105, 42], [-105, 44], [-110, 44]]]] })");
/// # let geometry_field = Field::new("geometry_field");
/// geo::centroid!(geometry_field);
/// # let geometry_param = Param::new("geometry_param");
/// geo::centroid!(geometry_param);
/// ```
#[macro_export]
macro_rules! geo_centroid {
    ( $geometry:expr ) => {
        $crate::functions::geo::centroid_fn($geometry)
    };
}

pub use geo_centroid as centroid;

/// The geo::distance function calculates the haversine distance, in metres, between two geolocation
/// points.
pub fn distance_fn(point1: impl Into<GeometryLike>, point2: impl Into<GeometryLike>) -> Function {
    create_geo_fn_with_two_args(point1, point2, "distance")
}

/// The geo::distance function calculates the haversine distance, in metres, between two geolocation
/// points.
/// Also aliased as `geo_distance!`.
///
/// # Arguments
///
/// * `point1` - The first point to calculate the distance from. This can be a geometry, field, or parameter.
/// * `point2` - The second point to calculate the distance to. This can be a geometry, field, or parameter.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as  surreal_orm;
/// use surreal_orm::{*, functions::geo};
/// use ::geo::point;
/// let point1 = point! {
///    x: 40.02f64,
///    y: 116.34,
/// };
/// let point2 = point! {
///   x: 40.02f64,
///   y: 116.34,
/// };
/// let result = geo::distance!(point1, point2);
/// assert_eq!(result.to_raw().build(), "geo::distance((40.02, 116.34), (40.02, 116.34))");
/// # let point1_field = Field::new("point1_field");
/// # let point2_field = Field::new("point2_field");
/// geo::distance!(point1_field, point2_field);
/// # let point1_param = Param::new("point1_param");
/// # let point2_param = Param::new("point2_param");
/// geo::distance!(point1_param, point2_param);
/// ```
#[macro_export]
macro_rules! geo_distance {
    ( $point1:expr,  $point2:expr ) => {
        $crate::functions::geo::distance_fn($point1, $point2)
    };
}
pub use geo_distance as distance;

/// This module contains functions for working with geohashes.
pub mod hash {
    use crate::{Buildable, Erroneous, Function, GeometryLike, NumberLike, Parametric, StrandLike};

    /// represents a geohash
    pub type GeoHash = StrandLike;

    /// The geo::hash::decode function converts a geohash into a geolocation point.
    pub fn decode_fn(geohash: impl Into<GeoHash>) -> Function {
        let string: GeoHash = geohash.into();

        Function {
            query_string: format!("geo::hash::decode({})", string.build()),
            bindings: string.get_bindings(),
            errors: string.get_errors(),
        }
    }

    /// The geo::hash::decode function converts a geohash into a geolocation point.
    /// Also aliased as `geo_hash_decode!`.
    ///
    /// # Arguments
    ///
    /// * `geohash` - The geohash to decode. This can be a string, field, or parameter.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::geo};
    /// let geohash = "u4pruydqqvj";
    /// let result = geo::hash::decode!(geohash);
    /// assert_eq!(result.to_raw().build(), "geo::hash::decode('u4pruydqqvj')");
    /// # let geohash_field = Field::new("geohash_field");
    /// geo::hash::decode!(geohash_field);
    /// # let geohash_param = Param::new("geohash_param");
    /// geo::hash::decode!(geohash_param);
    /// ```
    #[macro_export]
    macro_rules! geo_hash_decode {
        ( $geometry:expr ) => {
            $crate::functions::geo::hash::decode_fn($geometry)
        };
    }
    pub use geo_hash_decode as decode;

    /// The geo::hash::encode function converts a geolocation point into a geohash.
    pub fn encode_fn(
        geometry: impl Into<GeometryLike>,
        accuracy: Option<impl Into<NumberLike>>,
    ) -> Function {
        let geometry: GeometryLike = geometry.into();

        let mut bindings = geometry.get_bindings();
        let mut errors = geometry.get_errors();

        let str = if let Some(accuracy) = accuracy {
            let accuracy: NumberLike = accuracy.into();
            bindings.extend(accuracy.get_bindings());
            errors.extend(accuracy.get_errors());

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
            errors,
        }
    }

    /// The geo::hash::encode function converts a geolocation point into a geohash.
    /// Also aliased as `geo_hash_encode!`.
    ///
    /// # Arguments
    /// * `geometry` - The geometry to encode. This can be a geometry, field, or parameter.
    /// * `accuracy` - The accuracy of the geohash. This can be a number, field, or parameter.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::geo};
    /// use ::geo::point;
    /// let point = point! {
    ///   x: 40.02f64,
    ///   y: 116.34,
    /// };
    /// let result = geo::hash::encode!(point);
    /// assert_eq!(result.to_raw().build(), "geo::hash::encode((40.02, 116.34))");
    /// # let point_field = Field::new("point_field");
    /// geo::hash::encode!(point_field);
    /// # let point_param = Param::new("point_param");
    /// geo::hash::encode!(point_param);
    /// # let accuracy = 5;
    /// geo::hash::encode!(point, accuracy);
    /// # let accuracy_field = Field::new("accuracy_field");
    /// geo::hash::encode!(point, accuracy_field);
    /// # let accuracy_param = Param::new("accuracy_param");
    /// geo::hash::encode!(point, accuracy_param);
    /// ```
    #[macro_export]
    macro_rules! geo_hash_encode {
        ( $geometry:expr ) => {
            $crate::functions::geo::hash::encode_fn($geometry, None as Option<$crate::NumberLike>)
        };
        ( $geometry:expr, $accuracy:expr ) => {
            $crate::functions::geo::hash::encode_fn($geometry, Some($accuracy))
        };
    }
    pub use geo_hash_encode as encode;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use ::geo::{point, polygon};
    use functions::geo;

    #[test]
    fn test_area_with_field() {
        let city = Field::new("city");
        let result = geo::area_fn(city);

        assert_eq!(result.fine_tune_params(), "geo::area(city)");
        assert_eq!(result.to_raw().build(), "geo::area(city)");
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
        let result = geo::area_fn(poly);
        assert_eq!(result.fine_tune_params(), "geo::area($_param_00000001)");
        assert_eq!(
        result.to_raw().build(),
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
        let result = geo::area!(poly);
        assert_eq!(result.fine_tune_params(), "geo::area($_param_00000001)");
        assert_eq!(
        result.to_raw().build(),
        "geo::area({ type: 'Polygon', coordinates: [[[-111, 45], [-111, 41], [-104, 41], [-104, 45], [-111, 45]], [[[-110, 44], [-110, 42], [-105, 42], [-105, 44], [-110, 44]]]] })"
    );
    }

    #[test]
    fn test_area_macro_with_fields() {
        let poly = Field::new("poly");
        let result = geo::area!(poly);
        assert_eq!(result.fine_tune_params(), "geo::area(poly)");
        assert_eq!(result.to_raw().build(), "geo::area(poly)");
    }

    #[test]
    fn test_area_macro_with_params() {
        let poly = Param::new("poly");
        let result = geo::area!(poly);
        assert_eq!(result.fine_tune_params(), "geo::area($poly)");
        assert_eq!(result.to_raw().build(), "geo::area($poly)");
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
        let result = geo::bearing_fn(point1, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::bearing($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
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
        let result = geo::bearing_fn(hometown, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::bearing(hometown, $_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
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
        let result = geo::bearing!(point1, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::bearing($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
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
        let result = geo::bearing!(hometown, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::bearing(hometown, $_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "geo::bearing(hometown, (80.02, 103.19))"
        );
    }

    #[test]
    fn test_bearing_macro_with_raw_params() {
        let hometown = Param::new("hometown");
        let point2 = Param::new("point2");

        let result = geo::bearing!(hometown, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::bearing($hometown, $point2)"
        );
        assert_eq!(result.to_raw().build(), "geo::bearing($hometown, $point2)");
    }

    #[test]
    fn test_centroid_with_field() {
        let city = Field::new("city");
        let result = centroid_fn(city);

        assert_eq!(result.fine_tune_params(), "geo::centroid(city)");
        assert_eq!(result.to_raw().build(), "geo::centroid(city)");
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
        let result = geo::centroid_fn(poly);
        assert_eq!(result.fine_tune_params(), "geo::centroid($_param_00000001)");
        assert_eq!(
        result.to_raw().build(),
        "geo::centroid({ type: 'Polygon', coordinates: [[[-111, 45], [-111, 41], [-104, 41], [-104, 45], [-111, 45]], [[[-110, 44], [-110, 42], [-105, 42], [-105, 44], [-110, 44]]]] })"
    );
    }

    #[test]
    fn test_centroid_macro_with_field() {
        let city = Field::new("city");
        let result = geo::centroid!(city);

        assert_eq!(result.fine_tune_params(), "geo::centroid(city)");
        assert_eq!(result.to_raw().build(), "geo::centroid(city)");
    }

    #[test]
    fn test_centroid_macro_with_param() {
        let city = Param::new("city");
        let result = geo::centroid!(city);

        assert_eq!(result.fine_tune_params(), "geo::centroid($city)");
        assert_eq!(result.to_raw().build(), "geo::centroid($city)");
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
        let result = geo::centroid!(poly);
        assert_eq!(result.fine_tune_params(), "geo::centroid($_param_00000001)");
        assert_eq!(
        result.to_raw().build(),
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
        let result = geo::distance_fn(point1, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::distance($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
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
        let result = geo::distance_fn(hometown, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::distance(hometown, $_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "geo::distance(hometown, (80.02, 103.19))"
        );
    }

    #[test]
    fn test_distance_with_only_fields() {
        let hometown = Field::new("hometown");
        let yukon = Field::new("yukon");

        let result = geo::distance_fn(hometown, yukon);
        assert_eq!(result.fine_tune_params(), "geo::distance(hometown, yukon)");
        assert_eq!(result.to_raw().build(), "geo::distance(hometown, yukon)");
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
        let result = geo::distance!(point1, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::distance($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
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
        let result = geo::distance!(hometown, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::distance(hometown, $_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "geo::distance(hometown, (80.02, 103.19))"
        );
    }

    #[test]
    fn test_distance_macro_with_only_fields() {
        let hometown = Field::new("hometown");
        let yukon = Field::new("yukon");

        let result = geo::distance!(hometown, yukon);
        assert_eq!(result.fine_tune_params(), "geo::distance(hometown, yukon)");
        assert_eq!(result.to_raw().build(), "geo::distance(hometown, yukon)");
    }

    #[test]
    fn test_distance_macro_with_only_params() {
        let hometown = Param::new("hometown");
        let yukon = Param::new("yukon");

        let result = geo::distance!(hometown, yukon);
        assert_eq!(
            result.fine_tune_params(),
            "geo::distance($hometown, $yukon)"
        );
        assert_eq!(result.to_raw().build(), "geo::distance($hometown, $yukon)");
    }

    #[test]
    fn test_hash_decode_with_field() {
        let city = Field::new("city");
        let result = geo::hash::decode_fn(city);

        assert_eq!(result.fine_tune_params(), "geo::hash::decode(city)");
        assert_eq!(result.to_raw().build(), "geo::hash::decode(city)");
    }

    #[test]
    fn test_hash_decode_with_string() {
        let result = hash::decode_fn("mpuxk4s24f51");
        assert_eq!(
            result.fine_tune_params(),
            "geo::hash::decode($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "geo::hash::decode('mpuxk4s24f51')");
    }

    #[test]
    fn test_hash_decode_macro_with_field() {
        let city = Field::new("city");
        let result = geo::hash::decode!(city);

        assert_eq!(result.fine_tune_params(), "geo::hash::decode(city)");
        assert_eq!(result.to_raw().build(), "geo::hash::decode(city)");
    }

    #[test]
    fn test_hash_decode_macro_with_param() {
        let city = Param::new("city");
        let result = geo::hash::decode!(city);

        assert_eq!(result.fine_tune_params(), "geo::hash::decode($city)");
        assert_eq!(result.to_raw().build(), "geo::hash::decode($city)");
    }

    #[test]
    fn test_hash_decode_macro_with_string() {
        let result = geo::hash::decode!("mpuxk4s24f51");
        assert_eq!(
            result.fine_tune_params(),
            "geo::hash::decode($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "geo::hash::decode('mpuxk4s24f51')");
    }

    #[test]
    fn test_hash_encode_with_field_and_empty_accuracy() {
        let city = Field::new("city");
        let result = geo::hash::encode_fn(city, None as Option<NumberLike>);

        assert_eq!(result.fine_tune_params(), "geo::hash::encode(city)");
        assert_eq!(result.to_raw().build(), "geo::hash::encode(city)");
    }

    #[test]
    fn test_hash_encode_with_field_and_field_accuracy() {
        let city = Field::new("city");
        let accuracy = Field::new("accuracy");
        let result = geo::hash::encode_fn(city, Some(accuracy));

        assert_eq!(
            result.fine_tune_params(),
            "geo::hash::encode(city, accuracy)"
        );
        assert_eq!(result.to_raw().build(), "geo::hash::encode(city, accuracy)");
    }

    #[test]
    fn test_hash_encode_with_field_and_number_accuracy() {
        let city = Field::new("city");
        let result = geo::hash::encode_fn(city, Some(5));

        assert_eq!(
            result.fine_tune_params(),
            "geo::hash::encode(city, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "geo::hash::encode(city, 5)");
    }

    #[test]
    fn test_hash_encode_with_point() {
        let point = point! {
            x: 40.02f64,
            y: 116.34,
        };

        let result = geo::hash::encode_fn(point, Some(5));
        assert_eq!(
            result.fine_tune_params(),
            "geo::hash::encode($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "geo::hash::encode((40.02, 116.34), 5)"
        );
    }
    // Macro version
    #[test]
    fn test_hash_encode_macro_with_field_and_empty_accuracy_not_mentioned_at_all() {
        let city = Field::new("city");
        let result = geo::hash::encode!(city);

        assert_eq!(result.fine_tune_params(), "geo::hash::encode(city)");
        assert_eq!(result.to_raw().build(), "geo::hash::encode(city)");
    }

    #[test]
    fn test_hash_encode_macro_with_field_and_empty_accuracy() {
        let city = Field::new("city");
        let result = geo::hash::encode!(city);

        assert_eq!(result.fine_tune_params(), "geo::hash::encode(city)");
        assert_eq!(result.to_raw().build(), "geo::hash::encode(city)");
    }

    #[test]
    fn test_hash_encode_macro_with_field_and_field_accuracy() {
        let city = Field::new("city");
        let accuracy = Field::new("accuracy");
        let result = geo::hash::encode!(city, accuracy);

        assert_eq!(
            result.fine_tune_params(),
            "geo::hash::encode(city, accuracy)"
        );
        assert_eq!(result.to_raw().build(), "geo::hash::encode(city, accuracy)");
    }

    #[test]
    fn test_hash_encode_macro_with_param_and_no_accuracy_listed() {
        let city = Param::new("city");
        let result = geo::hash::encode!(city);

        assert_eq!(result.fine_tune_params(), "geo::hash::encode($city)");
        assert_eq!(result.to_raw().build(), "geo::hash::encode($city)");
    }

    #[test]
    fn test_hash_encode_macros_with_field_and_number_accuracy() {
        let city = Field::new("city");
        let result = geo::hash::encode!(city, 5);

        assert_eq!(
            result.fine_tune_params(),
            "geo::hash::encode(city, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "geo::hash::encode(city, 5)");
    }

    #[test]
    fn test_hash_encode_macro_with_point() {
        let point = point! {
            x: 40.02f64,
            y: 116.34,
        };

        let result = geo::hash::encode!(point, 5);
        assert_eq!(
            result.fine_tune_params(),
            "geo::hash::encode($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "geo::hash::encode((40.02, 116.34), 5)"
        );
    }
}
