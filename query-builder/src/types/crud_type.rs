/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt::{self, Display};

/// CRUD operation type
#[derive(Clone, Copy, Debug)]
pub enum CrudType {
    /// Creates a new record in the database
    Create,
    /// Retrieves existing records from the database
    Select,
    /// Updates existing records in the database
    Update,
    /// Deletes existing records from the database
    Delete,
}

impl Display for CrudType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let crud_type = match self {
            CrudType::Create => "create",
            CrudType::Select => "select",
            CrudType::Update => "update",
            CrudType::Delete => "delete",
        };
        write!(f, "{}", crud_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crud_type_display() {
        assert_eq!(CrudType::Create.to_string(), "create");
        assert_eq!(CrudType::Select.to_string(), "select");
        assert_eq!(CrudType::Update.to_string(), "update");
        assert_eq!(CrudType::Delete.to_string(), "delete");
    }
}
