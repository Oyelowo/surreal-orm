use std::ops::Deref;

use serde::{Deserialize, Serialize};
use surrealdb::sql::{self, thing, Id, Thing};

use crate::{Erroneous, SurrealdbModel, SurrealdbOrmError};

/// Wrapper around surrealdb::sql::Thing to extend its capabilities
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SurrealId(sql::Thing);

impl SurrealId {
    /// Create a new SurrealId from a string
    pub fn new<T: SurrealdbModel>(id: impl Into<Id>) -> Self {
        Self(Thing::from((T::table_name().to_string(), id.into())))
    }
}

impl Deref for SurrealId {
    type Target = sql::Thing;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Erroneous for SurrealId {
    fn get_errors(&self) -> Vec<String> {
        vec![]
    }
}

impl ::std::fmt::Display for SurrealId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for SurrealId {
    type Error = SurrealdbOrmError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        thing(&value.to_string())
            .map(SurrealId)
            .map_err(|e| SurrealdbOrmError::InvalidId(e.into()))
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

impl Into<sql::Value> for SurrealId {
    fn into(self) -> sql::Value {
        self.0.into()
    }
}

#[cfg(test)]
mod tests {
    use surrealdb::sql::Uuid;

    use crate::Table;

    use super::*;

    struct TestUser;
    impl SurrealdbModel for TestUser {
        fn table_name() -> Table {
            "user".into()
        }

        fn get_serializable_field_names() -> Vec<&'static str> {
            unimplemented!()
        }

        fn define_table() -> crate::Raw {
            unimplemented!()
        }

        fn define_fields() -> Vec<crate::Raw> {
            unimplemented!()
        }
    }

    #[test]
    fn test_create_surreal_id() {
        let id = SurrealId::new::<TestUser>(1);
        assert_eq!(id.to_string(), "user:1");
    }

    #[test]
    fn test_create_surreal_id_with_string() {
        let id = SurrealId::new::<TestUser>("oyelowo");
        assert_eq!(id.to_string(), "user:oyelowo");
    }

    #[test]
    fn test_create_surreal_id_with_uuid() {
        let id = SurrealId::new::<TestUser>(Uuid::default());
        assert_eq!(id.to_string().contains("user:"), true);
        assert_eq!(
            id.to_string(),
            "user:⟨00000000-0000-0000-0000-000000000000⟩"
        );
    }

    #[test]
    fn test_surreal_id() {
        let id = SurrealId::try_from("table:1").unwrap();
        assert_eq!(id.to_string(), "table:1");
    }

    #[test]
    fn test_surreal_id_from_string() {
        let id = SurrealId::try_from("table:oyelowo").unwrap();
        assert_eq!(id.to_string(), "table:oyelowo");
    }

    #[test]
    fn test_surreal_id_from_number() {
        let id = SurrealId::try_from("table:1").unwrap();
        assert_eq!(id.to_string(), "table:1");
    }

    #[test]
    fn test_surreal_id_from_str_err() {
        let id = SurrealId::try_from("table1").unwrap_err();
        assert_eq!(
            id.to_string(),
            "Invalid id. Problem deserializing string to surrealdb::sql::Thing. \
                Check that the id is in the format 'table_name:id'"
        );
    }

    #[test]
    fn test_surreal_id_from_str_err2() {
        let id = SurrealId::try_from("table:").unwrap_err();
        assert_eq!(
            id.to_string(),
            "Invalid id. Problem deserializing string to surrealdb::sql::Thing. \
                Check that the id is in the format 'table_name:id'"
        );
    }

    #[test]
    fn test_surreal_id_from_str_err3() {
        let id = SurrealId::try_from("table:1:2").unwrap_err();
        assert_eq!(
            id.to_string(),
            "Invalid id. Problem deserializing string to surrealdb::sql::Thing. \
                Check that the id is in the format 'table_name:id'"
        );
    }

    #[test]
    fn test_surreal_id_from_str_err4() {
        let id = SurrealId::try_from("table:1:2:3").unwrap_err();
        assert_eq!(
            id.to_string(),
            "Invalid id. Problem deserializing string to surrealdb::sql::Thing. \
                Check that the id is in the format 'table_name:id'"
        );
    }
}
