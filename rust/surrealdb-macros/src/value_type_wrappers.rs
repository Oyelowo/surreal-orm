use std::{iter::Skip, ops::Deref};

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
        enum GeometryType
</* T: TryInto<f64> */>
        {
            Point {
                coordinates: [f64; 2],
            },
            LineString {
                coordinates: Vec<[f64; 2]>,
            },
            Polygon {
                coordinates: Vec<Vec<(String, String)>>,
            },
            MultiPoint {
                coordinates: Vec<[f64; 2]>,
            },
            MultiLineString {
                coordinates: Vec<Vec<[f64; 2]>>,
            },
            MultiPolygon {
                coordinates: Vec<Vec<Vec<[f64; 2]>>>,
            },
            GeometryCollection {
                geometries: Vec<GeometryCustom>,
            },
        }

        println!("ZOPPPPPP BEFORE");
        let geo_type = GeometryType::deserialize(deserializer)?;
        println!("ZOPPPPPP AFTER");

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

            GeometryType::Polygon { coordinates } => {
                println!("ZOPPPPPP");
                let exterior = geo::LineString::from(
                    coordinates[0]
                        .iter()
                        .map(|c: &(String, String)| {
                            let x = 10;
                            // let xx = c[0];
                            // let xx = c.0;
                            geo::Coord {
                                x: string_to_f64(&c.0),
                                y: string_to_f64(&c.1),
                            }
                            // geo::Coord { x: 35.2, y: 97.2 }
                        })
                        .collect::<Vec<geo::Coord>>(),
                );
                let interiors = coordinates
                    .iter()
                    .skip(1)
                    .map(|ls| {
                        ls.into_iter()
                            .map(|c| {
                                let xx = 34;
                                // geo::Coord { x: c[0], y: c[1] }
                                geo::Coord {
                                    x: string_to_f64(&c.0),
                                    y: string_to_f64(&c.1),
                                }
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                sql::Geometry::Polygon(geo::Polygon::new(exterior, vec![]))
            }
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
                    .map(|p| {
                        let exterior = geo::LineString::from(
                            p[0].iter()
                                .map(|c| geo::Coord { x: c[0], y: c[1] })
                                .collect::<Vec<geo::Coord>>(),
                        );
                        let interiors = p
                            .iter()
                            .skip(1)
                            .map(|ls| {
                                ls.into_iter()
                                    .map(|c| geo::Coord { x: c[0], y: c[1] })
                                    .collect()
                            })
                            .collect();
                        geo::Polygon::new(exterior, interiors)
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

fn string_to_f64(string: &String) -> f64 {
    // serde_json::to_string(&serde_json::to_string(string).unwrap())
    string.parse().expect("numnopasse")
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
