use std::{
    fmt::{self, Display},
    ops::Deref,
};

use serde::{Deserialize, Serialize};
use surrealdb::sql::{self, thing};

use crate::{
    binding::Binding, binding::BindingsList, errors::SurrealdbOrmError, filter::Conditional,
    statements::SelectStatement, Erroneous, Field, Parametric,
};

pub struct Namespace(sql::Idiom);
pub struct Database(sql::Idiom);
pub struct Login(sql::Idiom);
pub struct Token(sql::Idiom);
pub struct Scope(sql::Idiom);
pub struct Table(sql::Table);
pub struct Event(sql::Idiom);
pub struct TableIndex(sql::Idiom);

impl Table {
    pub fn new(name: impl Into<sql::Table>) -> Self {
        Self(name.into())
    }
}

impl Deref for Table {
    type Target = sql::Table;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
macro_rules! impl_new_for_all {
    ($($types_:ty),*) => {
        $(
        impl $types_ {
            pub fn new(name: impl Into<String>) -> Self {
                Self(name.into().into())
            }
        }
    )*
    };
}

impl_new_for_all!(Namespace, Database, Login, Token, Scope, Event, TableIndex);

macro_rules! impl_display_for_all {
    ($($types_:ty),*) => {
        $(
        impl Display for $types_ {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl From<$types_> for String {
            fn from(value: $types_) -> Self {
                let value: String = value.0.to_string();
                value
            }
        }
        impl From<&str> for $types_ {
            fn from(value: &str) -> Self {
                Self(value.to_string().into())
            }
        }

        impl From<String> for $types_ {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }

        impl From<$types_> for sql::Value {
            fn from(value: $types_) -> Self {
                value.0.into()
            }
        }

    //     impl<T> From<T> for $types_
    //     where
    //         T: Into<String>,
    //     {
    //         fn from(value: T) -> Self {
    //             Self(value.into().into())
    //         }
    // }
    )*
    };
}
impl_display_for_all!(Namespace, Database, Login, Token, Scope, Table, Event, TableIndex);

pub enum TokenType {
    EDDSA,
    ES256,
    ES384,
    ES512,
    HS256,
    HS384,
    HS512,
    PS256,
    PS384,
    PS512,
    RS256,
    RS384,
    RS512,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::EDDSA => write!(f, "EDDSA"),
            TokenType::ES256 => write!(f, "ES256"),
            TokenType::ES384 => write!(f, "ES384"),
            TokenType::ES512 => write!(f, "ES512"),
            TokenType::HS256 => write!(f, "HS256"),
            TokenType::HS384 => write!(f, "HS384"),
            TokenType::HS512 => write!(f, "HS512"),
            TokenType::PS256 => write!(f, "PS256"),
            TokenType::PS384 => write!(f, "PS384"),
            TokenType::PS512 => write!(f, "PS512"),
            TokenType::RS256 => write!(f, "RS256"),
            TokenType::RS384 => write!(f, "RS384"),
            TokenType::RS512 => write!(f, "RS512"),
        }
    }
}

pub enum TokenTarget {
    Namespace,
    Database,
    Scope(String),
}

impl Display for TokenTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let target_str = match self {
            TokenTarget::Namespace => "NAMESPACE".into(),
            TokenTarget::Database => "DATABASE".into(),
            TokenTarget::Scope(scope) => format!("SCOPE {}", scope),
        };
        write!(f, "{}", target_str)
    }
}

pub struct Name(sql::Idiom);

impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl Name {
    pub fn new(name: sql::Idiom) -> Self {
        Self(name)
    }
}

// impl From<sql::Idiom> for Name {
//     fn from(value: sql::Idiom) -> Self {
//         todo!()
//     }
// }

impl From<Name> for sql::Idiom {
    fn from(value: Name) -> Self {
        value.0
    }
}

impl From<Name> for sql::Value {
    fn from(value: Name) -> Self {
        value.0.into()
    }
}

impl<T> From<T> for Name
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into().into())
    }
}

pub struct Duration(sql::Duration);

impl From<self::Duration> for sql::Duration {
    fn from(value: self::Duration) -> Self {
        value.0
    }
}

impl From<Duration> for sql::Value {
    fn from(value: self::Duration) -> Self {
        value.0.into()
    }
}
impl From<sql::Duration> for self::Duration {
    fn from(value: sql::Duration) -> Self {
        Self(value)
    }
}

impl From<&std::time::Duration> for Duration {
    fn from(value: &std::time::Duration) -> Self {
        Self(value.to_owned().into())
    }
}

impl From<std::time::Duration> for Duration {
    fn from(value: std::time::Duration) -> Self {
        Self(value.into())
    }
}

impl Deref for Duration {
    type Target = sql::Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ArrayCustom(sql::Array);

pub struct NONE;

impl From<NONE> for sql::Value {
    fn from(value: NONE) -> Self {
        sql::Value::Idiom(value.to_string().into())
    }
}

impl Display for NONE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NONE")
    }
}

impl<T> From<Vec<T>> for ArrayCustom
where
    T: Into<sql::Value>,
{
    fn from(value: Vec<T>) -> Self {
        Self(
            value
                .into_iter()
                .map(|v| v.into())
                .collect::<Vec<sql::Value>>()
                .into(),
        )
    }
}

impl<T, const N: usize> From<&[T; N]> for ArrayCustom
where
    T: Into<sql::Value> + Clone,
{
    fn from(value: &[T; N]) -> Self {
        Self(
            value
                .into_iter()
                .map(|v| v.clone().into())
                .collect::<Vec<sql::Value>>()
                .into(),
        )
    }
}

impl From<ArrayCustom> for sql::Value {
    fn from(value: ArrayCustom) -> Self {
        Self::Array(value.0.into())
    }
}
#[derive(Clone)]
pub enum Expression {
    SelectStatement(SelectStatement),
    Value(sql::Value),
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expression = match self {
            Expression::SelectStatement(s) => format!("({s})"),
            // Expression::SelectStatement(s) => s.get_bindings().first().unwrap().get_raw(),
            Expression::Value(v) => {
                let bindings = self.get_bindings();
                assert_eq!(bindings.len(), 1);
                format!("{}", self.get_bindings().first().expect("Param must have been generated for value. This is a bug. Please report here: ").get_param())
            }
        };
        write!(f, "{}", expression)
    }
}

impl Parametric for Expression {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Expression::SelectStatement(s) => s
                .get_bindings()
                .into_iter()
                // query must have already been built and bound
                .map(|b| b.with_raw(format!("({s})")))
                .collect::<_>(),
            Expression::Value(sql_value) => {
                // let sql_value = sql::json(&serde_json::to_string(&v).unwrap()).unwrap();
                let sql_value: sql::Value = sql_value.to_owned();
                vec![Binding::new(sql_value.clone()).with_raw(sql_value.to_raw_string())]
            }
        }
    }
}

impl From<SelectStatement> for Expression {
    fn from(value: SelectStatement) -> Self {
        Self::SelectStatement(value)
    }
}

impl<T: Into<sql::Value>> From<T> for Expression {
    fn from(value: T) -> Self {
        Self::Value(value.into())
    }
}

#[derive(Debug)]
pub enum Return {
    None,
    Before,
    After,
    Diff,
    Projections(Vec<Field>),
}

impl Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let return_type = match self {
            Return::None => "NONE ",
            Return::Before => "BEFORE ",
            Return::After => "AFTER ",
            Return::Diff => "DIFF ",
            Return::Projections(projections) => {
                let projections = projections
                    .iter()
                    .map(|p| format!("{}", p))
                    .collect::<Vec<String>>()
                    .join(", ");
                &projections
            }
        };
        write!(f, "RETURN {return_type} ")
    }
}

impl From<Vec<&Field>> for Return {
    fn from(value: Vec<&Field>) -> Self {
        Self::Projections(value.into_iter().map(ToOwned::to_owned).collect::<Vec<_>>())
    }
}

impl From<Vec<Field>> for Return {
    fn from(value: Vec<Field>) -> Self {
        Self::Projections(value)
    }
}

impl<const N: usize> From<&[Field; N]> for Return {
    fn from(value: &[Field; N]) -> Self {
        Self::Projections(value.to_vec())
    }
}

impl<const N: usize> From<&[&Field; N]> for Return {
    fn from(value: &[&Field; N]) -> Self {
        Self::Projections(
            value
                .to_vec()
                .into_iter()
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SurrealId(surrealdb::opt::RecordId);

impl Deref for SurrealId {
    type Target = surrealdb::opt::RecordId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Conditional for SurrealId {
    fn get_condition_query_string(&self) -> String {
        self.to_string()
    }
}

impl Erroneous for SurrealId {
    fn get_errors(&self) -> Vec<String> {
        vec![]
    }
}

impl Parametric for SurrealId {
    fn get_bindings(&self) -> BindingsList {
        let val: sql::Thing = self.to_owned().into();
        let val: sql::Value = val.into();
        vec![Binding::new(val)]
    }
}

impl ::std::fmt::Display for SurrealId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_fmt(format_args!("{}", self.to_string()))
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

#[derive(Deserialize)]
#[serde(untagged)]
enum CoordinateValue {
    F64((f64, f64)),
    String((String, String)),
}

#[derive(Debug, Clone, Serialize)]
pub struct GeometryCustom(pub sql::Geometry);

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

impl From<sql::Geometry> for GeometryCustom {
    fn from(value: sql::Geometry) -> Self {
        Self(value)
    }
}

impl From<GeometryCustom> for sql::Geometry {
    fn from(value: GeometryCustom) -> Self {
        value.0
    }
}

impl Deref for GeometryCustom {
    type Target = sql::Geometry;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
