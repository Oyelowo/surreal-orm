/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::{collections::HashMap, fmt::Display, marker::PhantomData, ops::Deref};

use serde::{Deserialize, Serialize};
use surrealdb::sql::{self, thing, Thing};

use crate::{Erroneous, Model, SurrealOrmError};

/// Wrapper around surrealdb::sql::Thing to extend its capabilities
/// and provide a more ergonomic interface. This is used to create a statically
/// typed id for a model. And is a combinatiion of the model's table name and
/// the id which is anyhting that can be converted to a `surrealdb::sql::Id`.
#[derive(Debug, Clone)]
pub struct SurrealId<T: Model, Id: Into<sql::Id>>(sql::Thing, PhantomData<T>, PhantomData<Id>);

impl<T, Id> From<SurrealId<T, Id>> for sql::Value
where
    T: Model,
    Id: Into<sql::Id>,
{
    fn from(value: SurrealId<T, Id>) -> Self {
        value.0.into()
    }
}

impl<T, Id> From<&SurrealId<T, Id>> for sql::Value
where
    T: Model,
    Id: Into<sql::Id>,
{
    fn from(value: &SurrealId<T, Id>) -> Self {
        value.0.clone().into()
    }
}

// impl<T, Id> From<SurrealId<T, Id>> for ValueLike
// where
//     T: Model,
//     Id: Into<sql::Id>,
// {
//     fn from(value: SurrealId<T, Id>) -> Self {
//         value.0.into()
//     }
// }

impl<T, Id> Serialize for SurrealId<T, Id>
where
    T: Model,
    Id: Into<sql::Id>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T, Id> Deserialize<'de> for SurrealId<T, Id>
where
    T: Model,
    Id: Into<sql::Id>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let thing = Thing::deserialize(deserializer)?;
        Ok(SurrealId(thing, PhantomData, PhantomData))
    }
}

impl<T, Id> Display for SurrealId<T, Id>
where
    T: Model,
    Id: Into<sql::Id>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T, Id> SurrealId<T, Id>
where
    T: Model,
    Id: Into<sql::Id>,
{
    /// Create a new SurrealId from a string
    pub fn new(id: Id) -> Self {
        Self(
            Thing::from((T::table().to_string(), id.into())),
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

    /// Converts the surreal id to a raw string
    pub fn to_raw(&self) -> String {
        self.to_thing().to_raw()
    }
}

impl<T, Id> From<SurrealId<T, Id>> for sql::Thing
where
    T: Model,
    Id: Into<sql::Id>,
{
    fn from(value: SurrealId<T, Id>) -> Self {
        value.0
    }
}

impl<T, Id> From<&SurrealId<T, Id>> for sql::Thing
where
    T: Model,
    Id: Into<sql::Id>,
{
    fn from(value: &SurrealId<T, Id>) -> Self {
        value.0.clone()
    }
}

// impl<T: Model, Id: Into<sql::Id>> Default for SurrealId<T, Id> {
//     fn default() -> Self {
//         SurrealSimpleId::new().into()
//     }
// }

impl<T, Id> Deref for SurrealId<T, Id>
where
    T: Model,
    Id: Into<sql::Id>,
{
    type Target = sql::Thing;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, Id> Erroneous for SurrealId<T, Id>
where
    T: Model,
    Id: Into<sql::Id>,
{
    fn get_errors(&self) -> Vec<String> {
        vec![]
    }
}

impl<T, Id> TryFrom<&str> for SurrealId<T, Id>
where
    T: Model,
    Id: Into<sql::Id>,
{
    type Error = SurrealOrmError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        thing(value)
            .map(|v| SurrealId(v, PhantomData, PhantomData))
            .map_err(|e| SurrealOrmError::InvalidId(e.into()))
    }
}

impl<T, Id> TryFrom<sql::Thing> for SurrealId<T, Id>
where
    T: Model,
    Id: Into<sql::Id>,
{
    type Error = SurrealOrmError;

    fn try_from(value: sql::Thing) -> Result<Self, Self::Error> {
        if value.tb != T::table().to_string() {
            return Err(SurrealOrmError::IdBelongsToAnotherTable(
                value.to_string(),
                T::table().to_string(),
            ));
        }
        Ok(Self(value, PhantomData, PhantomData))
    }
}

/// The default surrealdb id generated as a combination of the model/table name and a random nano id.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealSimpleId<T: Model>(SurrealId<T, String>);

impl<T> Default for SurrealSimpleId<T>
where
    T: Model,
{
    fn default() -> Self {
        SurrealSimpleId::new()
    }
}

impl<T> From<SurrealSimpleId<T>> for sql::Thing
where
    T: Model,
{
    fn from(simple_id: SurrealSimpleId<T>) -> Self {
        simple_id.0 .0
    }
}

impl<T> From<&SurrealSimpleId<T>> for sql::Thing
where
    T: Model,
{
    fn from(simple_id: &SurrealSimpleId<T>) -> Self {
        simple_id.0 .0.clone()
    }
}

impl<T> Display for SurrealSimpleId<T>
where
    T: Model,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0 .0.fmt(f)
    }
}

impl<T, Id> From<SurrealSimpleId<T>> for SurrealId<T, Id>
where
    T: Model,
    Id: Into<sql::Id>,
{
    fn from(simple_id: SurrealSimpleId<T>) -> Self {
        SurrealId(simple_id.0 .0, PhantomData, PhantomData)
    }
}

impl<T> SurrealSimpleId<T>
where
    T: Model,
{
    /// Generates a new random nano id
    pub fn new() -> Self {
        Self(SurrealId(
            Thing::from((T::table().to_string(), sql::Id::rand())),
            PhantomData,
            PhantomData,
        ))
    }

    /// Converts the surreal id to a thing
    pub fn to_thing(&self) -> Thing {
        self.0 .0.clone()
    }

    /// Converts the surreal id to a raw string
    pub fn to_raw(&self) -> String {
        self.to_thing().to_raw()
    }
}

impl<T> Deref for SurrealSimpleId<T>
where
    T: Model + Deref,
{
    type Target = SurrealId<T, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A surrealdb id generated as combination of the model/table name and a uuid for the id part.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealUuid<T: Model>(SurrealId<T, sql::Uuid>);

impl<T: Model> Default for SurrealUuid<T> {
    fn default() -> Self {
        SurrealUuid::new()
    }
}

impl<T: Model> From<SurrealUuid<T>> for sql::Thing {
    fn from(uuid: SurrealUuid<T>) -> Self {
        uuid.0 .0
    }
}

impl<T: Model, Id: Into<sql::Id>> From<SurrealUuid<T>> for SurrealId<T, Id> {
    fn from(uuid: SurrealUuid<T>) -> Self {
        SurrealId(uuid.0 .0, PhantomData, PhantomData)
    }
}

impl<T: Model> SurrealUuid<T> {
    /// Generates a new uuid
    pub fn new() -> Self {
        Self(SurrealId(
            Thing::from((T::table().to_string(), sql::Id::uuid())),
            PhantomData,
            PhantomData,
        ))
    }

    /// Converts the surreal id to a thing
    pub fn to_thing(&self) -> Thing {
        self.0 .0.clone()
    }

    /// Converts the surreal id to a raw string
    pub fn to_raw(&self) -> String {
        self.to_thing().to_raw()
    }
}

impl<T: Model> Display for SurrealUuid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0 .0.fmt(f)
    }
}

/// A surrealdb id generated as combination of the model/table name and a ulid for the id part.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealUlid<T: Model>(SurrealId<T, sql::Uuid>);

impl<T: Model> Default for SurrealUlid<T> {
    fn default() -> Self {
        SurrealUlid::new()
    }
}

impl<T: Model, Id: Into<sql::Id>> From<SurrealUlid<T>> for SurrealId<T, Id> {
    fn from(ulid: SurrealUlid<T>) -> Self {
        SurrealId(ulid.0 .0, PhantomData, PhantomData)
    }
}

impl<T: Model> From<SurrealUlid<T>> for sql::Thing {
    fn from(ulid: SurrealUlid<T>) -> Self {
        ulid.0 .0
    }
}

impl<T: Model> SurrealUlid<T> {
    /// Generates a new ulid
    pub fn new() -> Self {
        Self(SurrealId(
            Thing::from((T::table().to_string(), sql::Id::ulid())),
            PhantomData,
            PhantomData,
        ))
    }

    /// Converts the surreal id to a thing
    pub fn to_thing(&self) -> Thing {
        self.0 .0.clone()
    }

    /// Converts the surreal id to a raw string
    pub fn to_raw(&self) -> String {
        self.to_thing().to_raw()
    }
}

impl<T: Model> Display for SurrealUlid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0 .0.fmt(f)
    }
}

impl<T: Model> From<SurrealSimpleId<T>> for sql::Value {
    fn from(simple_id: SurrealSimpleId<T>) -> Self {
        sql::Value::Thing(simple_id.0 .0)
    }
}

impl<T: Model> From<&SurrealSimpleId<T>> for sql::Value {
    fn from(simple_id: &SurrealSimpleId<T>) -> Self {
        sql::Value::Thing(simple_id.0 .0.clone())
    }
}

impl<T: Model> From<SurrealUuid<T>> for sql::Value {
    fn from(uuid: SurrealUuid<T>) -> Self {
        sql::Value::Thing(uuid.0 .0)
    }
}

impl<T: Model> From<&SurrealUuid<T>> for sql::Value {
    fn from(uuid: &SurrealUuid<T>) -> Self {
        sql::Value::Thing(uuid.0 .0.clone())
    }
}

impl<T: Model> From<SurrealUlid<T>> for sql::Value {
    fn from(ulid: SurrealUlid<T>) -> Self {
        sql::Value::Thing(ulid.0 .0)
    }
}

impl<T: Model> From<&SurrealUlid<T>> for sql::Value {
    fn from(ulid: &SurrealUlid<T>) -> Self {
        sql::Value::Thing(ulid.0 .0.clone())
    }
}

/// For internal testing
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
impl Model for TestUser {
    type Id = TestUserSimpleId;
    type StructRenamedCreator = ();
    fn table() -> crate::Table {
        "user".into()
    }

    fn get_id(self) -> Self::Id {
        TestUserSimpleId::new()
    }

    fn get_id_as_thing(&self) -> sql::Thing {
        TestUserSimpleId::new().to_thing()
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

    fn get_field_meta() -> Vec<crate::FieldMetadata> {
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
        assert!(id.to_string().contains("user:"));
        assert_eq!(
            id.to_string(),
            "user:⟨00000000-0000-0000-0000-000000000000⟩"
        );
    }

    #[test]
    fn test_create_uuid_with_model() {
        let id = TestUser::create_uuid();
        assert!(id.to_string().contains("user:"));
        assert_eq!(id.to_string().len(), 47);
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
        assert!(id.to_string().contains("user:"));
        assert_eq!(id.to_string().len(), 47);
    }

    #[test]
    fn test_create_uuid() {
        let id = SurrealUuid::<TestUser>::new();
        assert!(id.to_string().contains("user:"));
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
                Check that the id is in the format 'table:id'"
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
                Check that the id is in the format 'table:id'"
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
                Check that the id is in the format 'table:id'"
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
                Check that the id is in the format 'table:id'"
            ),
            "Invalid id"
        );
    }
}
