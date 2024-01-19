use std::fmt::{Display, Formatter};

use super::parser::DataType;

pub struct FieldNameNormalized(String);

impl Display for FieldNameNormalized {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FieldNameNormalized {
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
