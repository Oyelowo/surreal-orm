use std::collections::HashMap;

use crate::{FieldMetadata, Raw, Table};

///
pub trait DbResources {
    ///
    fn tables(&self) -> Vec<Raw> {
        vec![]
    }

    ///
    fn tables_fields_meta(&self) -> HashMap<Table, Vec<FieldMetadata>> {
        HashMap::default()
    }

    ///
    fn analyzers(&self) -> Vec<Raw> {
        vec![]
    }

    ///
    fn functions(&self) -> Vec<Raw> {
        vec![]
    }

    ///
    fn params(&self) -> Vec<Raw> {
        vec![]
    }

    ///
    fn scopes(&self) -> Vec<Raw> {
        vec![]
    }

    ///
    fn tokens(&self) -> Vec<Raw> {
        vec![]
    }

    ///
    fn users(&self) -> Vec<Raw> {
        vec![]
    }
}
