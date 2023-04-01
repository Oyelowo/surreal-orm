#[derive(serde::Serialize, Debug, Clone)]
pub enum StrandLike {
    Strand(sql::Strand),
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

impl<T: Into<sql::Strand>> From<T> for StrandLike {
    fn from(value: T) -> Self {
        let value: sql::Geometry = value.into();
        Self::Geometry(value.into())
    }
}

impl From<Field> for StrandLike {
    fn from(val: Field) -> Self {
        StrandLike::Field(val.into())
    }
}

impl From<Param> for StrandLike {
    fn from(val: Param) -> Self {
        StrandLike::Param(val.into())
    }
}

impl From<&Field> for StrandLike {
    fn from(val: &Field) -> Self {
        StrandLike::Field(val.into())
    }
}

impl From<sql::Value> for StrandLike {
    fn from(value: sql::Value) -> StrandLike {
        Self::Geometry(value)
    }
}

impl From<StrandLike> for sql::Value {
    fn from(val: StrandLike) -> sql::Value {
        match val {
            StrandLike::StrandLike(g) => g.into(),
            StrandLike::Field(f) => f.into(),
            StrandLike::Param(p) => p.into(),
        }
    }
}
