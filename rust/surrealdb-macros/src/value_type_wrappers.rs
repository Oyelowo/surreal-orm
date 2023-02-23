use std::{fmt, iter::Skip, ops::Deref};

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

fn deserialize_f64_or_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    struct F64OrStringVisitor;

    impl<'de> Visitor<'de> for F64OrStringVisitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a float or a string")
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            value
                .parse()
                .map_err(|_| E::invalid_value(Unexpected::Str(value), &self))
        }
    }

    deserializer.deserialize_any(F64OrStringVisitor)
}

// use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
// use std::fmt;

fn deserialize_f64_or_string_tuple<'de, D>(deserializer: D) -> Result<(f64, f64), D::Error>
where
    D: Deserializer<'de>,
    D::Error: serde::de::Error,
{
    struct F64OrStringTupleVisitor;

    impl<'de> Visitor<'de> for F64OrStringTupleVisitor {
        type Value = (f64, f64);

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a tuple of two floats or two strings")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let first = seq
                .next_element::<serde_json::Value>()?
                .and_then(|v| {
                    if let serde_json::Value::Number(n) = v {
                        n.as_f64()
                    } else if let serde_json::Value::String(s) = v {
                        s.parse().ok()
                    } else {
                        None
                    }
                })
                .ok_or_else(|| {
                    de::Error::invalid_value(de::Unexpected::Other("unknown"), &"a float or string")
                })?;

            let second = seq
                .next_element::<serde_json::Value>()?
                .and_then(|v| {
                    if let serde_json::Value::Number(n) = v {
                        n.as_f64()
                    } else if let serde_json::Value::String(s) = v {
                        s.parse().ok()
                    } else {
                        None
                    }
                })
                .ok_or_else(|| {
                    de::Error::invalid_value(de::Unexpected::Other("unknown"), &"a float or string")
                })?;

            Ok((first, second))
        }
    }

    deserializer.deserialize_tuple(2, F64OrStringTupleVisitor)
}
// fn deserialize_f64_or_string_tuple<'de, D>(deserializer: D) -> Result<(f64, f64), D::Error>
// where
//     D: Deserializer<'de>,
// {
//     struct F64OrStringTupleVisitor;
//
//     impl<'de> Visitor<'de> for F64OrStringTupleVisitor {
//         type Value = (f64, f64);
//
//         fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//             formatter.write_str("a tuple of two floats or two strings")
//         }
//
//         fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
//         where
//             A: SeqAccess<'de>,
//         {
//             let first = seq
//                 .next_element::<serde_json::Value>()?
//                 .and_then(|v| {
//                     if let serde_json::Value::Number(n) = v {
//                         n.as_f64()
//                     } else if let serde_json::Value::String(s) = v {
//                         s.parse().ok()
//                     } else {
//                         None
//                     }
//                 })
//                 .ok_or_else(|| A::Error::invalid_value(Unexpected::Other("unknown"), &self))?;
//
//             let second = seq
//                 .next_element::<serde_json::Value>()?
//                 .and_then(|v| {
//                     if let serde_json::Value::Number(n) = v {
//                         n.as_f64()
//                     } else if let serde_json::Value::String(s) = v {
//                         s.parse().ok()
//                     } else {
//                         None
//                     }
//                 })
//                 .ok_or_else(|| A::Error::invalid_value(Unexpected::Other("unknown"), &self))?;
//
//             Ok((first, second))
//         }
//     }
//
//     deserializer.deserialize_tuple(2, F64OrStringTupleVisitor)
// }
// use serde::{Deserialize, Deserializer};
use serde_json::{from_str, Value};

// #[derive(Debug, Deserialize)]
// struct MyStruct {
//     #[serde(deserialize_with = "deserialize_coords")]
//     coords: Vec<Vec<(f64, f64)>>,
// }

// fn deserialize_coords<'de, D>(deserializer: D) -> Result<Vec<Vec<(f64, f64)>>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let value: Value = Deserialize::deserialize(deserializer)?;
//     match value {
//         Value::String(s) => {
//             let coords_str: Vec<&str> = s.split(',').collect();
//             let mut coords: Vec<Vec<(f64, f64)>> = vec![];
//             for coord_str in coords_str {
//                 let parts: Vec<&str> = coord_str.trim().split(' ').collect();
//                 let x: f64 = parts[0].parse().map_err(serde::de::Error::custom)?;
//                 let y: f64 = parts[1].parse().map_err(serde::de::Error::custom)?;
//                 coords.push(vec![(x, y)]);
//             }
//             Ok(coords)
//         }
//         Value::Array(arr) => {
//             let mut coords: Vec<Vec<(f64, f64)>> = vec![];
//             for row in arr {
//                 let mut coord_row: Vec<(f64, f64)> = vec![];
//                 match row {
//                     Value::String(s) => {
//                         let parts: Vec<&str> = s.split(',').collect();
//                         for coord_str in parts {
//                             let parts: Vec<&str> = coord_str.trim().split(' ').collect();
//                             let x: f64 = parts[0].parse().map_err(serde::de::Error::custom)?;
//                             let y: f64 = parts[1].parse().map_err(serde::de::Error::custom)?;
//                             coord_row.push((x, y));
//                         }
//                     }
//                     Value::Array(arr) => {
//                         for coord in arr {
//                             let x = coord[0]
//                                 .as_f64()
//                                 .ok_or_else(|| serde::de::Error::custom("invalid coord"))?;
//                             let y = coord[1]
//                                 .as_f64()
//                                 .ok_or_else(|| serde::de::Error::custom("invalid coord"))?;
//                             coord_row.push((x, y));
//                         }
//                     }
//                     _ => return Err(serde::de::Error::custom("invalid value type")),
//                 }
//                 coords.push(coord_row);
//             }
//             Ok(coords)
//         }
//         _ => Err(serde::de::Error::custom("invalid value type")),
//     }
// }

// use serde::{Deserialize, Deserializer};

// #[derive(Debug, Deserialize)]
// struct MyStruct {
//     #[serde(deserialize_with = "deserialize_coords")]
//     coords: Vec<Vec<(f64, f64)>>,
// }

// fn deserialize_coords<'de, D>(deserializer: D) -> Result<Vec<Vec<(f64, f64)>>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let value = serde_json::Value::deserialize(deserializer)?;
//     match value {
//         serde_json::Value::Array(coords) => {
//             let mut result = vec![];
//             for row in coords {
//                 let mut row_coords = vec![];
//                 match row {
//                     serde_json::Value::Array(pair) => {
//                         let mut pair_coords = vec![];
//                         for p in pair {
//                             match p {
//                                 serde_json::Value::Number(num) => {
//                                     pair_coords.push(num.as_f64().unwrap())
//                                 }
//                                 serde_json::Value::String(s) => pair_coords.push(
//                                     s.parse()
//                                         .map_err(|_| serde::de::Error::custom("invalid coord"))?,
//                                 ),
//                                 _ => {
//                                     return Err(serde::de::Error::custom("invalid value type"));
//                                 }
//                             }
//                         }
//                         if pair_coords.len() != 2 {
//                             return Err(serde::de::Error::custom("invalid coord"));
//                         }
//                         row_coords.push((pair_coords[0], pair_coords[1]));
//                     }
//                     _ => {
//                         return Err(serde::de::Error::custom("invalid value type"));
//                     }
//                 }
//                 result.push(row_coords);
//             }
//             Ok(result)
//         }
//         serde_json::Value::String(s) => {
//             let coords: Vec<Vec<&str>> = serde_json::from_str(&s)
//                 .map_err(|_| serde::de::Error::custom("invalid value format"))?;
//             let mut result = vec![];
//             for row in coords {
//                 let mut row_coords = vec![];
//                 for p in row {
//                     let num = p
//                         .parse()
//                         .map_err(|_| serde::de::Error::custom("invalid coord"))?;
//                     row_coords.push(num);
//                 }
//                 if row_coords.len() != 2 {
//                     return Err(serde::de::Error::custom("invalid coord"));
//                 }
//                 result.push((row_coords[0], row_coords[1]));
//             }
//             Ok(vec![result])
//         }
//         _ => Err(serde::de::Error::custom("invalid value type")),
//     }
// }
fn deserialize_coords<'de, D>(deserializer: D) -> Result<Vec<Vec<(f64, f64)>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Array(coords) => {
            let mut result = vec![];
            for row in coords {
                let mut row_coords = vec![];
                match row {
                    serde_json::Value::Array(pair) => {
                        let mut pair_coords = vec![];
                        for p in pair {
                            match p {
                                serde_json::Value::Number(num) => {
                                    pair_coords.push(num.as_f64().unwrap())
                                }
                                serde_json::Value::String(s) => pair_coords.push(
                                    s.parse()
                                        .map_err(|_| serde::de::Error::custom("invalid coord"))?,
                                ),
                                _ => {
                                    return Err(serde::de::Error::custom("invalid value type"));
                                }
                            }
                        }
                        if pair_coords.len() != 2 {
                            return Err(serde::de::Error::custom("invalid coord"));
                        }
                        row_coords.push((pair_coords[0], pair_coords[1]));
                    }
                    _ => {
                        return Err(serde::de::Error::custom("invalid value type"));
                    }
                }
                result.push(row_coords);
            }
            Ok(result)
        }
        serde_json::Value::String(s) => {
            let coords: Vec<Vec<&str>> = serde_json::from_str(&s)
                .map_err(|_| serde::de::Error::custom("invalid value format"))?;
            let mut result = vec![];
            for row in coords {
                let mut row_coords = vec![];
                for p in row {
                    let num = p
                        .parse()
                        .map_err(|_| serde::de::Error::custom("invalid coord"))?;
                    row_coords.push(num);
                }
                if row_coords.len() != 2 {
                    return Err(serde::de::Error::custom("invalid coord"));
                }
                result.push((row_coords[0], row_coords[1]));
            }
            Ok(vec![result])
        }
        _ => Err(serde::de::Error::custom("invalid value type")),
    }
}
#[derive(Deserialize)]
#[serde(untagged)]
enum PolygonCoordinates {
    F64(Vec<Vec<(f64, f64)>>),
    String(Vec<Vec<Vec<String>>>),
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
            GeometryType::Polygon { coordinates } => match coordinates {
                PolygonCoordinates::F64(coords) => {
                    let exterior = geo::LineString::from(
                        coords
                            .iter()
                            .next()
                            .unwrap_or(&vec![])
                            .iter()
                            .map(|c| {
                                if let Some(coord) = try_parse_coord_f64(c) {
                                    coord
                                } else {
                                    panic!("Invalid coordinate: {:?}", c)
                                }
                            })
                            .collect::<Vec<geo::Coord<f64>>>(),
                    );

                    let interiors = coords
                        .iter()
                        .skip(1)
                        .map(|ls| {
                            ls.iter()
                                .map(|c| {
                                    if let Some(coord) = try_parse_coord_f64(c) {
                                        coord
                                    } else {
                                        panic!("Invalid coordinate: {:?}", c)
                                    }
                                })
                                .collect::<Vec<_>>()
                        })
                        .map(|coords| geo::LineString::from(coords))
                        .collect::<Vec<geo::LineString>>();
                    sql::Geometry::Polygon(geo::Polygon::new(exterior, interiors))
                }
                PolygonCoordinates::String(coords) => {
                    let flattened_coords = coords
                        .into_iter()
                        .flatten()
                        .flatten()
                        .collect::<Vec<String>>();
                    let exterior = geo::LineString::from(
                        flattened_coords
                            .iter()
                            .take(2)
                            .map(|c| {
                                if let Some(coord) = try_parse_coord_str(c) {
                                    coord
                                } else {
                                    panic!("Invalid coordinateoooo: {:?}", c)
                                }
                            })
                            .collect::<Vec<geo::Coord<f64>>>(),
                    );

                    let interiors = flattened_coords
                        .chunks(2)
                        // .iter()
                        .skip(1)
                        .map(|ls| {
                            ls.iter()
                                .map(|c| {
                                    if let Some(coord) = try_parse_coord_str(c) {
                                        coord
                                    } else {
                                        panic!("Invalid coordinatexx: {:?}", c)
                                    }
                                })
                                .collect::<Vec<_>>()
                        })
                        .map(|coords| geo::LineString::from(coords))
                        .collect::<Vec<geo::LineString>>();
                    sql::Geometry::Polygon(geo::Polygon::new(exterior, interiors))
                }
            },
            // GeometryType::Polygon { coordinates } => {
            //     println!("ZOPPPPPP");
            //     let exterior = geo::LineString::from(
            //         coordinates[0]
            //             .iter()
            //             .map(|c| {
            //                 let x = 10;
            //                 // let xx = c[0];
            //                 // let xx = c.0;
            //                 // geo::Coord {
            //                 //     x: string_to_f64(&c.0),
            //                 //     y: string_to_f64(&c.1),
            //                 // }
            //                 geo::Coord { x: c.0, y: c.1 }
            //                 // geo::Coord { x: 35.2, y: 97.2 }
            //             })
            //             .collect::<Vec<geo::Coord>>(),
            //     );
            //     let interiors = coordinates
            //         .iter()
            //         .skip(1)
            //         .map(|ls| {
            //             ls.into_iter()
            //                 .map(|c| {
            //                     let xx = 34;
            //                     // geo::Coord { x: c[0], y: c[1] }
            //                     geo::Coord { x: c.0, y: c.1 }
            //                     // geo::Coord {
            //                     //     x: string_to_f64(&c.0),
            //                     //     y: string_to_f64(&c.1),
            //                     // }
            //                 })
            //                 .collect::<Vec<_>>()
            //         })
            //         .collect::<Vec<_>>();
            //     sql::Geometry::Polygon(geo::Polygon::new(exterior, vec![]))
            // }
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
fn try_parse_coord_f64(c: &(f64, f64)) -> Option<geo::Coord<f64>> {
    Some(geo::Coord { x: c.0, y: c.1 })
}

fn try_parse_coord_str(s: &str) -> Option<geo::Coord<f64>> {
    s.parse::<f64>().ok().map(|f| geo::Coord { x: f, y: f })
}
// fn try_parse_coord_str(c: &str) -> Option<geo::Coord<f64>> {
//     let parts: Vec<_> = c.split(',').map(|s| s.trim().parse::<f64>().ok()).collect();
//     match parts.as_slice() {
//         [Some(x), Some(y)] => Some(geo::Coord { x: *x, y: *y }),
//         _ => None,
//     }
// }
fn try_parse_coord(s: &str) -> Option<geo::Coord<f64>> {
    s.parse::<f64>().ok().map(|x| geo::Coord { x, y: 0.0 })
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
