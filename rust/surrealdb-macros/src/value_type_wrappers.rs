use std::ops::Deref;

use serde::{Deserialize, Serialize};
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
                coordinates: (f64, f64),
            },
            LineString {
                coordinates: Vec<(f64, f64)>,
            },
            Polygon {
                coordinates: Vec<Vec<(f64, f64)>>,
            },
            MultiPoint {
                coordinates: Vec<(f64, f64)>,
            },
            MultiLineString {
                coordinates: Vec<Vec<(f64, f64)>>,
            },
            MultiPolygon {
                coordinates: Vec<Vec<Vec<(f64, f64)>>>,
            },
            GeometryCollection {
                geometries: Vec<GeometryCustom>,
            },
        }

        let geo_type = GeometryType::deserialize(deserializer)?;
        let surreal_geometry = match geo_type {
            GeometryType::Point { coordinates } => {
                sql::Geometry::Point(geo::Point::from(coordinates)).into()
            }
            GeometryType::LineString { coordinates } => {
                sql::Geometry::Line(geo::LineString::from(coordinates))
            }
            GeometryType::Polygon { coordinates } => sql::Geometry::Polygon(geo::Polygon::new(
                geo::LineString::from(coordinates.clone().into_iter().next().unwrap()),
                coordinates
                    .clone()
                    .into_iter()
                    .skip(1)
                    .map(|ls| ls.into())
                    .collect(),
            ))
            .into(),
            GeometryType::MultiPoint { coordinates } => {
                sql::Geometry::MultiPoint(geo::MultiPoint::from(coordinates)).into()
            }
            GeometryType::MultiLineString { coordinates } => Geometry::MultiLine(
                geo::MultiLineString::new(coordinates.into_iter().map(|ls| ls.into()).collect()),
            )
            .into(),
            GeometryType::MultiPolygon { coordinates } => {
                sql::Geometry::MultiPolygon(geo::MultiPolygon::new(
                    coordinates
                        .into_iter()
                        .map(|p| {
                            geo::Polygon::new(
                                geo::LineString::from(p.clone().into_iter().next().unwrap()),
                                p.into_iter().skip(1).map(|ls| ls.into()).collect(),
                            )
                        })
                        .collect(),
                ))
                .into()
            }
            GeometryType::GeometryCollection { geometries } => {
                let geometries: Vec<Geometry> = geometries.into_iter().map(|g| g.0).collect();
                sql::Geometry::Collection(geometries).into()
            }
        };

        Ok(surreal_geometry.into())
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
