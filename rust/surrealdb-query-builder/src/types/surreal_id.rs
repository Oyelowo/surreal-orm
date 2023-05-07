use std::{collections::HashMap, fmt::Display, marker::PhantomData, ops::Deref};

use serde::{Deserialize, Serialize};
use surrealdb::sql::{self, thing, Id, Thing, Uuid};

use crate::{Erroneous, SurrealdbModel, SurrealdbOrmError};

/// Wrapper around surrealdb::sql::Thing to extend its capabilities
/// and provide a more ergonomic interface. This is used to create a statically
/// typed id for a model. And is a combinatiion of the model's table name and
/// the id which is anyhting that can be converted to a `surrealdb::sql::Id`.
#[derive(Debug, Clone)]
pub struct SurrealId<T: SurrealdbModel, Id: Into<sql::Id>>(
    sql::Thing,
    PhantomData<T>,
    PhantomData<Id>,
);

impl<T: SurrealdbModel, Id: Into<sql::Id>> Serialize for SurrealId<T, Id> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T: SurrealdbModel, Id: Into<sql::Id>> Deserialize<'de> for SurrealId<T, Id> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let thing = Thing::deserialize(deserializer)?;
        Ok(SurrealId(thing, PhantomData, PhantomData))
    }
}

impl<T: SurrealdbModel, Id: Into<sql::Id>> Display for SurrealId<T, Id> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl<T, Id> SurrealId<T, Id>
where
    T: SurrealdbModel,
    Id: Into<sql::Id>,
{
    /// Create a new SurrealId from a string
    pub fn new(id: Id) -> Self {
        Self(
            Thing::from((T::table_name().to_string(), id.into())),
            PhantomData,
            PhantomData,
        )
    }

    /// Generates a new random nano id
    pub fn rand() -> SurrealSimpleId<T> {
        SurrealSimpleId::new()
    }

    /// Generates a new ulid
    pub fn ulid() -> SurrealUlid<T> {
        SurrealUlid::new()
    }

    /// Generates a new uuid
    pub fn uuid() -> SurrealUuid<T> {
        SurrealUuid::new()
    }

    /// Generates default id as nano random id used by surrealdb
    pub fn nano_id() -> SurrealSimpleId<T> {
        SurrealSimpleId::new()
    }

    /// Returns the inner `sql::Thing`
    pub fn to_thing(&self) -> Thing {
        self.0.clone()
    }
}

impl<T: SurrealdbModel, Id: Into<sql::Id>> From<SurrealId<T, Id>> for sql::Thing {
    fn from(value: SurrealId<T, Id>) -> Self {
        value.0
    }
}

impl<T: SurrealdbModel, Id: Into<sql::Id>> From<&SurrealId<T, Id>> for sql::Thing {
    fn from(value: &SurrealId<T, Id>) -> Self {
        value.0.clone()
    }
}

impl<T: SurrealdbModel, Id: Into<sql::Id>> Default for SurrealId<T, Id> {
    fn default() -> Self {
        SurrealSimpleId::new().into()
    }
}

impl<T: SurrealdbModel, Id: Into<sql::Id>> Deref for SurrealId<T, Id> {
    type Target = sql::Thing;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: SurrealdbModel, Id: Into<sql::Id>> Erroneous for SurrealId<T, Id> {
    fn get_errors(&self) -> Vec<String> {
        vec![]
    }
}

impl<T: SurrealdbModel, Id: Into<sql::Id>> TryFrom<&str> for SurrealId<T, Id> {
    type Error = SurrealdbOrmError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        thing(&value.to_string())
            .map(|v| SurrealId(v, PhantomData, PhantomData))
            .map_err(|e| SurrealdbOrmError::InvalidId(e.into()))
    }
}

impl<T: SurrealdbModel, Id: Into<sql::Id>> From<sql::Thing> for SurrealId<T, Id> {
    fn from(value: sql::Thing) -> Self {
        Self(value, PhantomData, PhantomData)
    }
}

impl<T: SurrealdbModel, Id: Into<sql::Id>> Into<sql::Value> for SurrealId<T, Id> {
    fn into(self) -> sql::Value {
        self.0.into()
    }
}

// from surrealid to surrealsimle id
// impl<T: SurrealdbModel, Id: Into<sql::Id>> From<SurrealId<T, Id>> for SurrealSimpleId<T> {
//     fn from(surreal_id: SurrealId<T, Id>) -> Self {
//         SurrealSimpleId(surreal_id)
//     }
// }

/// The default surrealdb id generated as a combination of the model/table name and a random nano id.
#[derive(Debug, Clone)]
pub struct SurrealSimpleId<T: SurrealdbModel>(SurrealId<T, String>);

impl<T: SurrealdbModel> Display for SurrealSimpleId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0 .0.fmt(f)
    }
}

impl<T: SurrealdbModel, Id: Into<sql::Id>> From<SurrealSimpleId<T>> for SurrealId<T, Id> {
    fn from(simple_id: SurrealSimpleId<T>) -> Self {
        SurrealId(simple_id.0 .0, PhantomData, PhantomData)
    }
}

impl<T: SurrealdbModel> SurrealSimpleId<T> {
    /// Generates a new random nano id
    pub fn new() -> Self {
        Self(SurrealId(
            Thing::from((T::table_name().to_string(), sql::Id::rand())),
            PhantomData,
            PhantomData,
        ))
    }
}

impl<T: SurrealdbModel + Deref> Deref for SurrealSimpleId<T> {
    type Target = SurrealId<T, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A surrealdb id generated as combination of the model/table name and a uuid for the id part.
#[derive(Debug, Clone)]
pub struct SurrealUuid<T: SurrealdbModel>(SurrealId<T, sql::Uuid>);

impl<T: SurrealdbModel, Id: Into<sql::Id>> From<SurrealUuid<T>> for SurrealId<T, Id> {
    fn from(uuid: SurrealUuid<T>) -> Self {
        SurrealId(uuid.0 .0, PhantomData, PhantomData)
    }
}

impl<T: SurrealdbModel> SurrealUuid<T> {
    /// Generates a new uuid
    pub fn new() -> Self {
        Self(SurrealId(
            Thing::from((T::table_name().to_string(), sql::Id::uuid())),
            PhantomData,
            PhantomData,
        ))
    }
}

impl<T: SurrealdbModel> Display for SurrealUuid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0 .0.fmt(f)
    }
}

/// A surrealdb id generated as combination of the model/table name and a ulid for the id part.
#[derive(Debug, Clone)]
pub struct SurrealUlid<T: SurrealdbModel>(SurrealId<T, sql::Uuid>);

impl<T: SurrealdbModel, Id: Into<sql::Id>> From<SurrealUlid<T>> for SurrealId<T, Id> {
    fn from(ulid: SurrealUlid<T>) -> Self {
        SurrealId(ulid.0 .0, PhantomData, PhantomData)
    }
}

impl<T: SurrealdbModel> SurrealUlid<T> {
    /// Generates a new ulid
    pub fn new() -> Self {
        Self(SurrealId(
            Thing::from((T::table_name().to_string(), sql::Id::ulid())),
            PhantomData,
            PhantomData,
        ))
    }
}

impl<T: SurrealdbModel> Display for SurrealUlid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0 .0.fmt(f)
    }
}

/// For internal testing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestUser;
#[allow(dead_code)]
/// For internal testing
pub type TestUserSimpleId = SurrealSimpleId<TestUser>;
/// For internal testing
pub type TestUserUuid = SurrealUuid<TestUser>;
/// For internal testing
pub type TestUserUlid = SurrealUlid<TestUser>;
/// For internal testing
pub type TestUserStringId = SurrealId<TestUser, String>;
/// For internal testing
pub type TestUserNumberId = SurrealId<TestUser, u64>;
/// For internal testing
pub type TestUserObjectId = SurrealId<TestUser, HashMap<String, String>>;
impl SurrealdbModel for TestUser {
    fn table_name() -> crate::Table {
        "user".into()
    }

    fn get_serializable_fields() -> Vec<crate::Field> {
        unimplemented!()
    }

    fn get_linked_fields() -> Vec<crate::Field> {
        unimplemented!()
    }

    fn get_link_one_fields() -> Vec<crate::Field> {
        unimplemented!()
    }

    fn get_link_self_fields() -> Vec<crate::Field> {
        unimplemented!()
    }

    fn get_link_one_and_self_fields() -> Vec<crate::Field> {
        unimplemented!()
    }

    fn get_link_many_fields() -> Vec<crate::Field> {
        unimplemented!()
    }

    fn define_table() -> crate::Raw {
        unimplemented!()
    }

    fn define_fields() -> Vec<crate::Raw> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use surrealdb::sql::Uuid;

    #[test]
    fn test_create_id_with_model() {
        let id = TestUser::create_id(1);
        assert_eq!(id.to_string(), "user:1");
    }

    #[test]
    fn test_create_id_string_with_model() {
        let id = TestUser::create_id("oyelowo");
        assert_eq!(id.to_string(), "user:oyelowo");
    }

    #[test]
    fn test_create_id_uuid_with_model() {
        let id = TestUser::create_id(Uuid::default());
        assert_eq!(id.to_string().contains("user:"), true);
        assert_eq!(
            id.to_string(),
            "user:⟨00000000-0000-0000-0000-000000000000⟩"
        );
    }

    #[test]
    fn test_create_uuid_with_model() {
        let id = TestUser::create_uuid();
        assert_eq!(id.to_string().contains("user:"), true);
        assert_eq!(id.to_string().len(), 49);
    }

    #[test]
    fn test_create_surreal_id() {
        let id = TestUserNumberId::new(1);
        assert_eq!(id.to_string(), "user:1");
    }

    #[test]
    fn test_create_surreal_id_with_string() {
        let id = TestUserStringId::new("oyelowo".into());
        assert_eq!(id.to_string(), "user:oyelowo");
    }

    #[test]
    fn test_create_surreal_id_with_uuid() {
        let id = TestUserUuid::new();
        assert_eq!(id.to_string().contains("user:"), true);
        assert_eq!(id.to_string().len(), 55);
    }

    #[test]
    fn test_create_uuid() {
        let id = SurrealUuid::<TestUser>::new();
        assert_eq!(id.to_string().contains("user:"), true);
        assert_eq!(id.to_string().len(), 47);
    }

    #[test]
    fn test_surreal_id() {
        let id = TestUserNumberId::try_from("table:1").unwrap();
        assert_eq!(id.to_string(), "table:1");
    }

    #[test]
    fn test_surreal_id_from_string() {
        let id = TestUserStringId::try_from("table:oyelowo").unwrap();
        assert_eq!(id.to_string(), "table:oyelowo");
    }

    #[test]
    fn test_surreal_id_from_number() {
        let id = TestUserNumberId::try_from("table:1").unwrap();
        assert_eq!(id.to_string(), "table:1");
    }

    #[test]
    fn test_surreal_id_from_str_err() {
        let id = TestUserNumberId::try_from("table1").unwrap_err();
        assert!(
            id.to_string().contains(
                "Invalid id. Problem deserializing string to surrealdb::sql::Thing. \
                Check that the id is in the format 'table_name:id'"
            ),
            "Invalid id"
        );
    }

    #[test]
    fn test_surreal_id_from_str_err2() {
        let id = TestUserStringId::try_from("table:").unwrap_err();
        assert!(
            id.to_string().contains(
                "Invalid id. Problem deserializing string to surrealdb::sql::Thing. \
                Check that the id is in the format 'table_name:id'"
            ),
            "Invalid id"
        );
    }

    #[test]
    fn test_surreal_id_from_str_err3() {
        let id = TestUserNumberId::try_from("table:1:2:3").unwrap_err();
        assert!(
            id.to_string().contains(
                "Invalid id. Problem deserializing string to surrealdb::sql::Thing. \
                Check that the id is in the format 'table_name:id'"
            ),
            "Invalid id"
        );
    }

    #[test]
    fn test_surreal_id_from_str_err4() {
        let id = TestUserNumberId::try_from("table:1:2:3").unwrap_err();
        assert!(
            id.to_string().contains(
                "Invalid id. Problem deserializing string to surrealdb::sql::Thing. \
                Check that the id is in the format 'table_name:id'"
            ),
            "Invalid id"
        );
    }
}
