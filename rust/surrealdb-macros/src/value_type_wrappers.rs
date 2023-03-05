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

use crate::{db_field::Binding, model_id::SurrealdbOrmError, Parametric};

#[derive(Debug, Serialize, Clone)]
pub struct SurrealId(RecordId);

impl Parametric for SurrealId {
    fn get_bindings(&self) -> crate::BindingsList {
        let val: sql::Thing = self.to_owned().into();
        let val: sql::Value = val.into();
        vec![Binding::new(val)]
    }
}

impl ::std::fmt::Display for SurrealId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_fmt(format_args!("{}", self.to_raw()))
    }
}

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

impl From<SurrealId> for sql::Thing {
    fn from(value: SurrealId) -> Self {
        value.0
    }
}

impl From<sql::Thing> for SurrealId {
    fn from(value: sql::Thing) -> Self {
        Self(value)
    }
}

// surrealdb::opt::RecordId is surrealdb::sql::Thing
// impl From<RecordId> for SurrealId {
//     fn from(value: RecordId) -> Self {
//         Self(value)
//     }
// }

impl Into<sql::Value> for SurrealId {
    fn into(self) -> sql::Value {
        self.0.into()
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
enum CoordinateValue {
    F64((f64, f64)),
    String((String, String)),
}

#[derive(Debug, Clone, Serialize)]
pub struct GeometryCustom(pub Geometry);

impl<'de> Deserialize<'de> for GeometryCustom {
    fn deserialize<D>(deserializer: D) -> std::result::Result<GeometryCustom, D::Error>
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
            GeometryCollection { geometries: Vec<GeometryCustom> },
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
