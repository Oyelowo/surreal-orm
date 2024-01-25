use std::fmt::Display;

use syn::Ident;

use crate::models::DataType;

#[derive(Debug, PartialEq, Eq)]
pub struct FieldNameSerialized(String);

impl Display for FieldNameSerialized {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FieldNameSerialized {
    pub fn is_id(&self) -> bool {
        self.0 == "id"
    }

    pub fn is_in_edge_node(&self, model_type: DataType) -> bool {
        model_type.is_edge() && self.0 == "in"
    }

    pub fn is_out_edge_node(&self, model_type: DataType) -> bool {
        model_type.is_edge() && self.0 == "out"
    }

    pub fn is_orig_or_dest_edge_node(&self, model_type: &DataType) -> bool {
        model_type.is_edge() && (self.0 == "in" || self.0 == "out")
    }
}

impl From<Ident> for FieldNameSerialized {
    fn from(s: Ident) -> Self {
        Self(s.to_string())
    }
}
