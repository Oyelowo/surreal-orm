use geo;
use surrealdb::sql;

use super::{Field, Param};

// pub struct GeometryLike(sql::Value);
//
// impl From<GeometryLike> for sql::Value {
//     fn from(value: GeometryLike) -> Self {
//         value.0
//     }
// }
//
// impl<T: Into<sql::Geometry>> From<T> for GeometryLike {
//     fn from(value: T) -> Self {
//         let value: sql::Geometry = value.into();
//         Self(value.into())
//     }
// }
//
// impl From<Field> for GeometryLike {
//     fn from(value: Field) -> Self {
//         Self(value.into())
//     }
// }
//
// impl From<Param> for GeometryLike {
//     fn from(value: Param) -> Self {
//         Self(value.into())
//     }
// }
#[derive(serde::Serialize, Debug, Clone)]
pub enum GeometryLike {
    Geometry(sql::Geometry),
    Field(sql::Idiom),
    Param(sql::Param),
}

// macro_rules! impl_geometry_like_from {
//     ($($t:ty),*) => {
//         $(impl From<$t> for GeometryLike {
//             fn from(value: $t) -> Self {
//                 Self::Geometry(sql::Geometry::from(value))
//             }
//         })*
//     };
// }
//
// impl_geometry_like_from!(
//     geo::Polygon,
//     geo::Point,
//     geo::LineString,
//     geo::MultiPoint,
//     geo::MultiPolygon,
//     geo::MultiLineString
// );

impl<T: Into<sql::Geometry>> From<T> for GeometryLike {
    fn from(value: T) -> Self {
        let value: sql::Geometry = value.into();
        Self::Geometry(value.into())
    }
}

impl From<Field> for GeometryLike {
    fn from(val: Field) -> Self {
        GeometryLike::Field(val.into())
    }
}

impl From<Param> for GeometryLike {
    fn from(val: Param) -> Self {
        GeometryLike::Param(val.into())
    }
}

impl From<&Field> for GeometryLike {
    fn from(val: &Field) -> Self {
        GeometryLike::Field(val.into())
    }
}

impl From<sql::Value> for GeometryLike {
    fn from(value: sql::Value) -> GeometryLike {
        Self::Geometry(value)
    }
}

impl From<GeometryLike> for sql::Value {
    fn from(val: GeometryLike) -> sql::Value {
        match val {
            GeometryLike::Geometry(g) => g.into(),
            GeometryLike::Field(f) => f.into(),
            GeometryLike::Param(p) => p.into(),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum CoordinateValue {
    F64((f64, f64)),
    String((String, String)),
}

#[derive(Debug, Clone, Serialize)]
pub struct Geometry(pub sql::Geometry);

impl<'de> Deserialize<'de> for Geometry {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Geometry, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        type PointCoords = CoordinateValue;
        type PolygonCoords = Vec<Vec<CoordinateValue>>;
        type LineCoords = Vec<CoordinateValue>;

        #[derive(Deserialize)]
        #[serde(tag = "type")]
        enum GeometryType {
            Point { coordinates: PointCoords },
            LineString { coordinates: LineCoords },
            Polygon { coordinates: PolygonCoords },

            MultiPoint { coordinates: Vec<PointCoords> },
            MultiLineString { coordinates: Vec<LineCoords> },
            MultiPolygon { coordinates: Vec<PolygonCoords> },
            GeometryCollection { geometries: Vec<Geometry> },
        }

        let geo_type = GeometryType::deserialize(deserializer)?;

        let surreal_geometry = match geo_type {
            GeometryType::Point { coordinates } => {
                sql::Geometry::Point(geo::Point::from(coordinates.parse_value_to_coord()))
            }
            GeometryType::LineString { coordinates } => sql::Geometry::Line(geo::LineString::from(
                coordinates
                    .iter()
                    .map(|c| c.parse_value_to_coord())
                    .collect::<Vec<_>>(),
            )),
            GeometryType::Polygon { coordinates } => {
                sql::Geometry::Polygon(deserialize_polygon_from_coords(coordinates))
            }
            GeometryType::MultiPoint { coordinates } => {
                sql::Geometry::MultiPoint(geo::MultiPoint::from(
                    coordinates
                        .iter()
                        .map(|c| c.parse_value_to_coord())
                        .collect::<Vec<_>>(),
                ))
            }
            GeometryType::MultiLineString { coordinates } => {
                sql::Geometry::MultiLine(geo::MultiLineString::new(
                    coordinates
                        .iter()
                        .map(|ls| {
                            geo::LineString::from(
                                ls.into_iter()
                                    .map(|c| c.parse_value_to_coord())
                                    .collect::<Vec<_>>(),
                            )
                        })
                        .collect(),
                ))
            }
            GeometryType::MultiPolygon { coordinates } => {
                let polygons = coordinates
                    .into_iter()
                    .map(deserialize_polygon_from_coords)
                    .collect();
                sql::Geometry::MultiPolygon(geo::MultiPolygon::new(polygons))
            }
            GeometryType::GeometryCollection { geometries } => {
                let geometries = geometries.into_iter().map(|g| g.0).collect();
                sql::Geometry::Collection(geometries)
            }
        };
        Ok(surreal_geometry.into())
    }
}

fn deserialize_polygon_from_coords(coords: Vec<Vec<CoordinateValue>>) -> geo::Polygon {
    let exterior = geo::LineString::from(
        coords
            .iter()
            .next()
            .unwrap_or(&vec![])
            .iter()
            .map(|c| c.parse_value_to_coord())
            .collect::<Vec<geo::Coord<f64>>>(),
    );

    let interiors = coords
        .iter()
        .skip(1)
        .map(|ls| {
            ls.iter()
                .map(|c| c.parse_value_to_coord())
                .collect::<Vec<_>>()
        })
        .map(|coords| geo::LineString::from(coords))
        .collect::<Vec<geo::LineString>>();
    geo::Polygon::new(exterior, interiors)
}

trait CoordParser {
    fn parse_value_to_coord(&self) -> geo::Coord;
}

impl CoordParser for CoordinateValue {
    fn parse_value_to_coord(&self) -> geo::Coord {
        match self {
            CoordinateValue::F64(coord_f64) => coord_f64.parse_value_to_coord(),
            CoordinateValue::String(coord_stringified_f64) => {
                coord_stringified_f64.parse_value_to_coord()
            }
        }
    }
}

impl CoordParser for (f64, f64) {
    fn parse_value_to_coord(&self) -> geo::Coord {
        Some(geo::Coord {
            x: self.0,
            y: self.1,
        })
        .expect("Invalid coordinate: {self}")
    }
}

impl CoordParser for (String, String) {
    fn parse_value_to_coord(&self) -> geo::Coord {
        let x = self.0.parse::<f64>().ok();
        let y = self.1.parse::<f64>().ok();

        match (x, y) {
            (Some(x), Some(y)) => Some(geo::Coord { x, y }),
            (_, _) => None,
        }
        .expect("Invalid coordinate: {self}")
    }
}

impl From<sql::Geometry> for Geometry {
    fn from(value: sql::Geometry) -> Self {
        Self(value)
    }
}

impl From<Geometry> for sql::Geometry {
    fn from(value: Geometry) -> Self {
        value.0
    }
}

impl Deref for Geometry {
    type Target = sql::Geometry;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
