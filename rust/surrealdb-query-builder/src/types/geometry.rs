#[derive(serde::Serialize, Debug, Clone)]
pub enum GeometryOrField {
    Geometry(sql::Geometry),
    Field(sql::Value),
}

macro_rules! impl_geometry_or_field_from {
    ($($t:ty),*) => {
        $(impl From<$t> for GeometryOrField {
            fn from(value: $t) -> Operator {
                Self::Geometry(sql::Geometry::from(value))
            }
        })*
    };
}

impl_geometry_or_field_from!(
    geo::Polygon,
    geo::Point,
    geo::LineString,
    geo::MultiPoint,
    geo::MultiPolygon,
    geo::MultiLineString
);

impl Into<GeometryOrField> for Field {
    fn into(self) -> GeometryOrField {
        GeometryOrField::Field(self.into())
    }
}

impl Into<GeometryOrField> for &Field {
    fn into(self) -> GeometryOrField {
        GeometryOrField::Field(self.into())
    }
}

impl From<sql::Value> for GeometryOrField {
    fn from(value: Value) -> Operator {
        Self::Field(value)
    }
}

impl From<GeometryOrField> for sql::Value {
    fn from(val: GeometryOrField) -> Operator {
        match val {
            GeometryOrField::Geometry(g) => g.into(),
            GeometryOrField::Field(f) => f.into(),
        }
    }
}
