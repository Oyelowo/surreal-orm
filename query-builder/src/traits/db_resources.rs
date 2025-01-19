use std::collections::HashMap;

use crate::{FieldMetadata, Raw, Table};

/// A trait for database resources.
pub trait DbResources {
    /// Returns a list of tables.
    fn tables(&self) -> Vec<Raw> {
        vec![]
    }

    /// Returns a list of fields for each table.
    fn tables_fields_meta(&self) -> HashMap<Table, Vec<FieldMetadata>> {
        HashMap::default()
    }

    /// Returns a list of fields.
    fn analyzers(&self) -> Vec<Raw> {
        vec![]
    }

    /// Returns a list of fields.
    fn functions(&self) -> Vec<Raw> {
        vec![]
    }

    /// Returns a list of fields.
    fn params(&self) -> Vec<Raw> {
        vec![]
    }

    /// Returns a list of fields.
    fn scopes(&self) -> Vec<Raw> {
        vec![]
    }

    /// Returns a list of fields.
    fn tokens(&self) -> Vec<Raw> {
        vec![]
    }

    /// Returns a list of fields.
    fn users(&self) -> Vec<Raw> {
        vec![]
    }
}
