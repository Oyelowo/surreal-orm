use std::{
    fmt::{self, Display},
    iter::Skip,
    ops::Deref,
};

use serde::{
    de::{self, SeqAccess, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize,
};
use surrealdb::{
    opt::RecordId,
    sql::{self, thing, Geometry},
};

use crate::model_id::SurrealdbOrmError;

#[derive(Debug, Serialize, Clone)]
pub struct SurrealId(RecordId);

impl<'de> Deserialize<'de> for SurrealId {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(SurrealId(thing(&s).map_err(serde::de::Error::custom)?))
    }
}

impl TryFrom<&str> for SurrealId {
    type Error = SurrealdbOrmError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // TODO: Improve error handling
        Ok(Self(thing(&value.to_string()).unwrap()))
    }
}

impl From<RecordId> for SurrealId {
    fn from(value: RecordId) -> Self {
        Self(value)
    }
}

impl From<SurrealId> for RecordId {
    fn from(value: SurrealId) -> Self {
        value.0
    }
}

impl Deref for SurrealId {
    type Target = RecordId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum PolygonCoordinates {
    F64(Vec<Vec<(f64, f64)>>),
    String(Vec<Vec<(String, String)>>),
}

#[derive(Debug, Clone, Serialize)]
pub struct GeometryCustom(pub Geometry);

impl<'de> Deserialize<'de> for GeometryCustom {
    fn deserialize<D>(deserializer: D) -> std::result::Result<GeometryCustom, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(tag = "type")]
        enum GeometryType {
            Point {
                coordinates: [f64; 2],
            },
            LineString {
                coordinates: Vec<[f64; 2]>,
            },
            Polygon {
                coordinates: PolygonCoordinates,
            },
            MultiPoint {
                coordinates: Vec<[f64; 2]>,
            },
            MultiLineString {
                coordinates: Vec<Vec<[f64; 2]>>,
            },
            MultiPolygon {
                coordinates: Vec<PolygonCoordinates>,
            },
            GeometryCollection {
                geometries: Vec<GeometryCustom>,
            },
        }

        let geo_type = GeometryType::deserialize(deserializer)?;

        let surreal_geometry = match geo_type {
            GeometryType::Point { coordinates } => {
                sql::Geometry::Point(geo::Point::from(coordinates))
            }
            GeometryType::LineString { coordinates } => sql::Geometry::Line(geo::LineString::from(
                coordinates
                    .into_iter()
                    .map(|c| geo::Coord { x: c[0], y: c[1] })
                    .collect::<Vec<geo::Coord>>(),
            )),
            GeometryType::Polygon { coordinates } => match coordinates {
                PolygonCoordinates::F64(coords) => {
                    sql::Geometry::Polygon(deserialize_polygon_from_coords(coords))
                }
                PolygonCoordinates::String(coords) => {
                    sql::Geometry::Polygon(deserialize_polygon_from_coords(coords))
                }
            },
            GeometryType::MultiPoint { coordinates } => {
                sql::Geometry::MultiPoint(geo::MultiPoint::from(
                    coordinates
                        .into_iter()
                        .map(|c| geo::Point::from(c))
                        .collect::<Vec<geo::Point>>(),
                ))
            }
            GeometryType::MultiLineString { coordinates } => {
                sql::Geometry::MultiLine(geo::MultiLineString::new(
                    coordinates
                        .into_iter()
                        .map(|ls| {
                            geo::LineString::from(
                                ls.into_iter()
                                    .map(|c| geo::Coord { x: c[0], y: c[1] })
                                    .collect::<Vec<geo::Coord>>(),
                            )
                        })
                        .collect(),
                ))
            }
            GeometryType::MultiPolygon { coordinates } => {
                let polygons = coordinates
                    .into_iter()
                    .map(|p| match p {
                        PolygonCoordinates::F64(coords) => deserialize_polygon_from_coords(coords),
                        PolygonCoordinates::String(coords) => {
                            deserialize_polygon_from_coords(coords)
                        }
                    })
                    .collect();
                sql::Geometry::MultiPolygon(geo::MultiPolygon::new(polygons))
            }
            GeometryType::GeometryCollection { geometries } => {
                let geometries: Vec<Geometry> = geometries.into_iter().map(|g| g.0).collect();
                sql::Geometry::Collection(geometries)
            }
        };
        Ok(surreal_geometry.into())
    }
}

fn deserialize_polygon_from_coords<
    T: CoordParser, /* + fmt::Debug, F: Fn(T) -> Option<geo::Coord> */
>(
    coords: Vec<Vec<T>>,
) -> geo::Polygon {
    let exterior = geo::LineString::from(
        coords
            .iter()
            .next()
            .unwrap_or(&vec![])
            .iter()
            .map(|c| {
                // if let Some(coord) = try_parse_coord_f64(c.0.into(), c.1.into()) {
                <T as CoordParser>::parse_value_to_coord(c)
            })
            .collect::<Vec<geo::Coord<f64>>>(),
    );

    let interiors = coords
        .iter()
        .skip(1)
        .map(|ls| {
            ls.iter()
                .map(|c| <T as CoordParser>::parse_value_to_coord(c))
                .collect::<Vec<_>>()
        })
        .map(|coords| geo::LineString::from(coords))
        .collect::<Vec<geo::LineString>>();
    geo::Polygon::new(exterior, interiors)
}

trait CoordParser {
    fn parse_value_to_coord(&self) -> geo::Coord;
}

impl CoordParser for (f64, f64) {
    fn parse_value_to_coord(&self) -> geo::Coord {
        try_parse_coord_f64(self).expect("Invalid coordinate: {self}")
    }
}

impl CoordParser for (String, String) {
    fn parse_value_to_coord(&self) -> geo::Coord {
        try_parse_coord_str(self).expect("Invalid coordinate: {self}")
    }
}

fn try_parse_coord_f64(&(x, y): &(f64, f64)) -> Option<geo::Coord<f64>> {
    Some(geo::Coord { x, y })
}

fn try_parse_coord_str((x, y): &(String, String)) -> Option<geo::Coord<f64>> {
    let x = x.parse::<f64>().ok();
    let y = y.parse::<f64>().ok();
    match (x, y) {
        (Some(x), Some(y)) => Some(geo::Coord { x, y }),
        (_, _) => None,
    }
}

impl From<Geometry> for GeometryCustom {
    fn from(value: Geometry) -> Self {
        Self(value)
    }
}

impl From<GeometryCustom> for Geometry {
    fn from(value: GeometryCustom) -> Self {
        value.0
    }
}

impl Deref for GeometryCustom {
    type Target = Geometry;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
