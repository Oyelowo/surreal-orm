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

#[derive(Debug)]
pub struct PolygonBuilder {
    exterior: geo::LineString<f64>,
    interiors: Vec<geo::LineString<f64>>,
}

impl PolygonBuilder {
    pub fn new(exterior: geo::LineString<f64>) -> Self {
        Self {
            exterior,
            interiors: Vec::new(),
        }
    }

    pub fn add_interior(&mut self, interior: geo::LineString<f64>) -> &mut Self {
        self.interiors.push(interior);
        self
    }

    pub fn build(self) -> geo::Polygon<f64> {
        geo::Polygon::new(self.exterior, self.interiors)
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
                coordinates: [f64; 2],
            },
            LineString {
                coordinates: Vec<[f64; 2]>,
            },
            Polygon {
                coordinates: Vec<Vec<[f64; 2]>>,
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
        let geo_type = GeometryType::deserialize(deserializer)?;
        let surreal_geometry = match geo_type {
            GeometryType::Point { coordinates } => {
                sql::Geometry::Point(geo::Point::from(coordinates)).into()
            }
            GeometryType::LineString { coordinates } => {
                sql::Geometry::Line(geo::LineString::from(
                    coordinates
                        .into_iter()
                        .map(|c| geo::Coord { x: c[0], y: c[1] })
                        .collect::<Vec<geo::Coord>>(),
                ))
            }
            .into(),

            GeometryType::Polygon { mut coordinates } => {
                let interior = coordinates.split_off(1);
                let exterior = geo::LineString::from(
                    coordinates[0]
                        .clone()
                        .into_iter()
                        .map(|c| geo::Coord { x: c[0], y: c[1] })
                        .collect::<Vec<geo::Coord>>(),
                );
                let interiors = interior
                    .iter()
                    .map(|ls| {
                        ls.into_iter()
                            .map(|c| geo::Coord { x: c[0], y: c[1] })
                            .collect()
                    })
                    .collect();
                sql::Geometry::Polygon(geo::Polygon::new(exterior, interiors)).into()
            }
            GeometryType::MultiPoint { coordinates } => {
                sql::Geometry::MultiPoint(geo::MultiPoint::from(
                    coordinates
                        .into_iter()
                        .map(|c| geo::Point::from(c))
                        .collect::<Vec<geo::Point>>(),
                ))
                .into()
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
                .into()
            }
            GeometryType::MultiPolygon { coordinates } => {
                let polygons = coordinates
                    .into_iter()
                    .map(|mut p| {
                        let interiors_poly = p.split_off(1);
                        let exterior = geo::LineString::from(
                            p[0].iter()
                                .map(|c| geo::Coord { x: c[0], y: c[1] })
                                .collect::<Vec<geo::Coord>>(),
                        );
                        let interiors = interiors_poly
                            .iter()
                            .map(|ls| {
                                ls.into_iter()
                                    .map(|c| geo::Coord { x: c[0], y: c[1] })
                                    .collect()
                            })
                            .collect();
                        geo::Polygon::new(exterior, interiors)
                    })
                    .collect();
                sql::Geometry::MultiPolygon(geo::MultiPolygon::new(polygons)).into()
            }
            GeometryType::GeometryCollection { geometries } => {
                let geometries: Vec<Geometry> = geometries.into_iter().map(|g| g.0).collect();
                sql::Geometry::Collection(geometries).into()
            }
        };
        Ok(GeometryCustom(surreal_geometry))
    }
}
// impl<'de> Deserialize<'de> for GeometryCustom {
//     fn deserialize<D>(deserializer: D) -> std::result::Result<GeometryCustom, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         #[derive(Deserialize)]
//         #[serde(tag = "type")]
//         enum GeometryType {
//             Point {
//                 coordinates: (f64, f64),
//             },
//             LineString {
//                 coordinates: Vec<(f64, f64)>,
//             },
//             Polygon {
//                 coordinates: Vec<Vec<(f64, f64)>>,
//             },
//             MultiPoint {
//                 coordinates: Vec<(f64, f64)>,
//             },
//             MultiLineString {
//                 coordinates: Vec<Vec<(f64, f64)>>,
//             },
//             MultiPolygon {
//                 coordinates: Vec<Vec<Vec<(f64, f64)>>>,
//             },
//             GeometryCollection {
//                 geometries: Vec<GeometryCustom>,
//             },
//         }
//
//         // let ve = vec![];
//         // ve.splitof
//         let geo_type = GeometryType::deserialize(deserializer)?;
//         let surreal_geometry = match geo_type {
//             GeometryType::Point { coordinates } => {
//                 sql::Geometry::Point(geo::Point::from(coordinates)).into()
//             }
//             GeometryType::LineString { coordinates } => {
//                 sql::Geometry::Line(geo::LineString::from(coordinates))
//             }
//             // GeometryType::Polygon { coordinates } => {
//             //     let (outer, inner) = coordinates.into_iter().map(geo::LineString::from).fold(
//             //         (None, Vec::new()),
//             //         |(outer, mut inner), ls| match outer {
//             //             Some(_) => (outer, inner),
//             //             None => {
//             //                 inner.push(ls.into());
//             //                 (Some(ls), inner)
//             //             }
//             //         },
//             //     );
//             //     Geometry::Polygon(geo::Polygon::new(outer.unwrap(), inner)).into()
//             // }
//             GeometryType::Polygon { mut coordinates } => {
//                 let interior = coordinates.split_off(1);
//                 let mut builder = PolygonBuilder::new(geo::LineString::from(
//                     coordinates[0]
//                         .iter()
//                         .map(|&c| geo::Coord { x: c.0, y: c.1 })
//                         .collect::<Vec<_>>(),
//                 ));
//
//                 interior.iter().skip(1).for_each(|interior_coords| {
//                     builder.add_interior(geo::LineString::from(
//                         interior_coords
//                             .iter()
//                             .map(|&c| geo::Coord { x: c.0, y: c.1 })
//                             .collect::<Vec<_>>(),
//                     ));
//                 });
//
//                 sql::Geometry::Polygon(builder.build()).into()
//             }
//             GeometryType::MultiPoint { coordinates } => {
//                 sql::Geometry::MultiPoint(geo::MultiPoint::from(coordinates)).into()
//             }
//             GeometryType::MultiLineString { coordinates } => Geometry::MultiLine(
//                 geo::MultiLineString::new(coordinates.into_iter().map(|ls| ls.into()).collect()),
//             )
//             .into(),
//
//             GeometryType::MultiPolygon { coordinates } => {
//                 sql::Geometry::MultiPolygon(geo::MultiPolygon::new(
//                     coordinates
//                         .into_iter()
//                         .map(|p| {
//                             let mut builder = PolygonBuilder::new(geo::LineString::from(
//                                 p[0].iter()
//                                     .map(|&c| geo::Coord { x: c.0, y: c.1 })
//                                     .collect::<Vec<_>>(),
//                             ));
//
//                             for interior_coords in p.iter().skip(1) {
//                                 builder.add_interior(geo::LineString::from(
//                                     interior_coords
//                                         .iter()
//                                         .map(|&c| geo::Coord { x: c.0, y: c.1 })
//                                         .collect::<Vec<_>>(),
//                                 ));
//                             }
//                             builder.build()
//                         })
//                         .collect(),
//                 ))
//                 .into()
//             }
//             GeometryType::GeometryCollection { geometries } => {
//                 let geometries: Vec<Geometry> = geometries.into_iter().map(|g| g.0).collect();
//                 sql::Geometry::Collection(geometries).into()
//             }
//         };
//
//         Ok(surreal_geometry.into())
//     }
// }

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
