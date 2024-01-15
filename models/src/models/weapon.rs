/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_orm::{Node, Object, SurrealId, SurrealSimpleId};

// Weapon
#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "weapon")]
pub struct Weapon {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    #[surreal_orm(type_ = "float")]
    pub strength: Strength,
    pub created: DateTime<Utc>,
    #[surreal_orm(nest_object = "Rocket")]
    pub rocket: Rocket,
}
type Strength = f64;

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "weapon", relax_table_name)]
pub struct WeaponOld {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    #[surreal_orm(type_ = "float")]
    pub strength: Strength,
    pub nice: bool,
    pub bunch_of_other_fields: i32,
    pub created: DateTime<Utc>,
    // #[surreal_orm(nest_object = "Rocket")]
    // pub rocket: Rocket,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Rocket {
    pub name: String,
    pub strength: u64,
    pub nice: bool,
    pub bunch_of_other_fields: i32,
    pub created: DateTime<Utc>,
}

struct Mana<T>
where
    T: Clone,
{
    pub name: String,
    pub strength: u64,
    pub nice: bool,
    pub bunch_of_other_fields: i32,
    pub created: DateTime<Utc>,
    pub ala: T,
}

// #[derive(Object, Serialize, Deserialize, Debug, Clone, Default)]
// #[serde(rename_all = "camelCase")]
// pub struct Rocket2<T: Clone> {
//     pub name: String,
//     pub strength: u64,
//     pub nice: bool,
//     pub bunch_of_other_fields: i32,
//     pub created: DateTime<Utc>,
//     pub ala: T,
// }

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "weapon_stats")]
pub struct WeaponStats {
    pub id: SurrealSimpleId<Self>,
    pub average_strength: f64,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "account")]
pub struct Account {
    pub id: SurrealId<Self, String>,
    pub balance: f64,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "balance")]
pub struct Balance {
    pub id: SurrealId<Self, String>,
    #[surreal_orm(type_ = "option<string>")]
    pub amount: f64,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "test_stuff")]
pub struct TestStuff {
    pub id: SurrealSimpleId<Self>,
    #[surreal_orm(type_ = "option<int>")]
    pub amt: Option<Strength>,
    #[surreal_orm(type_ = "option<int>")]
    pub amt9: Option<Strength>,
    // Would be autoinferred
    pub amt2: Option<u64>,
    #[surreal_orm(type_ = "array<int>")]
    pub amt3: Vec<u64>,
    // #[surreal_orm(type_ = "int")]
    pub count: u64,
}

// =======================================
// Recursive expansion of the Object macro
// =======================================

use surreal_orm::ToRaw as _;
impl surreal_orm::SchemaGetter for Rocket {
    type Schema = ________internal_rocket_schema::Rocket;
    fn schema() -> rocket::Schema {
        rocket::Schema::new()
    }
    fn schema_prefixed(
        prefix: impl ::std::convert::Into<surreal_orm::ValueLike>,
    ) -> rocket::Schema {
        rocket::Schema::new_prefixed(prefix)
    }
}
impl surreal_orm::Object for Rocket {
    type NonNullUpdater = RocketNonNullUpdater;
}
#[allow(non_snake_case)]
#[derive(surreal_orm::serde::Serialize, surreal_orm::serde::Deserialize, Debug, Clone, Default)]
pub struct RocketNonNullUpdater {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: ::std::option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: ::std::option::Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nice: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bunchOfOtherFields: ::std::option::Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: ::std::option::Option<DateTime<Utc>>,
}
#[allow(non_snake_case)]
pub mod rocket {
    pub use super::________internal_rocket_schema::_____schema_def::Schema;
}
#[allow(non_snake_case)]
mod ________internal_rocket_schema {
    use surreal_orm::Buildable as _;
    use surreal_orm::Erroneous as _;
    use surreal_orm::Parametric as _;
    pub(super) mod _____field_names {
        use super::super::*;
        use surreal_orm::Buildable as _;
        use surreal_orm::Parametric as _;
        #[derive(Debug, Clone)]
        pub struct Name_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for Name_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for Name_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&Name_______________> for surreal_orm::ValueLike {
            fn from(value: &Name_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<Name_______________> for surreal_orm::ValueLike {
            fn from(value: Name_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&Name_______________> for surreal_orm::Field {
            fn from(field_name: &Name_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<Name_______________> for surreal_orm::Field {
            fn from(field_name: Name_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for Name_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for Name_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::Name_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::Name_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::Name_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::Name_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<String> for self::Name_______________ {}
        impl surreal_orm::Patchable<String> for self::Name_______________ {}
        #[derive(Debug, Clone)]
        pub struct Strength_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for Strength_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for Strength_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&Strength_______________> for surreal_orm::ValueLike {
            fn from(value: &Strength_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<Strength_______________> for surreal_orm::ValueLike {
            fn from(value: Strength_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&Strength_______________> for surreal_orm::Field {
            fn from(field_name: &Strength_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<Strength_______________> for surreal_orm::Field {
            fn from(field_name: Strength_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for Strength_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for Strength_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::Strength_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::Strength_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::Strength_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::Strength_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<u64> for self::Strength_______________ {}
        impl surreal_orm::Patchable<u64> for self::Strength_______________ {}
        impl surreal_orm::SetterNumeric<u64> for self::Strength_______________ {}
        impl ::std::convert::From<self::Strength_______________> for surreal_orm::NumberLike {
            fn from(val: self::Strength_______________) -> Self {
                val.0.into()
            }
        }
        impl ::std::convert::From<&self::Strength_______________> for surreal_orm::NumberLike {
            fn from(val: &self::Strength_______________) -> Self {
                val.clone().0.into()
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Add<T>
            for Strength_______________
        {
            type Output = surreal_orm::Operation;
            fn add(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} + {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Sub<T>
            for Strength_______________
        {
            type Output = surreal_orm::Operation;
            fn sub(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} - {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Mul<T>
            for Strength_______________
        {
            type Output = surreal_orm::Operation;
            fn mul(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} * {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Div<T>
            for Strength_______________
        {
            type Output = surreal_orm::Operation;
            fn div(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} / {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Add<T>
            for &Strength_______________
        {
            type Output = surreal_orm::Operation;
            fn add(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} + {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Sub<T>
            for &Strength_______________
        {
            type Output = surreal_orm::Operation;
            fn sub(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} - {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Mul<T>
            for &Strength_______________
        {
            type Output = surreal_orm::Operation;
            fn mul(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} * {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Div<T>
            for &Strength_______________
        {
            type Output = surreal_orm::Operation;
            fn div(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} / {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct Nice_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for Nice_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for Nice_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&Nice_______________> for surreal_orm::ValueLike {
            fn from(value: &Nice_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<Nice_______________> for surreal_orm::ValueLike {
            fn from(value: Nice_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&Nice_______________> for surreal_orm::Field {
            fn from(field_name: &Nice_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<Nice_______________> for surreal_orm::Field {
            fn from(field_name: Nice_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for Nice_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for Nice_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::Nice_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::Nice_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::Nice_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::Nice_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<bool> for self::Nice_______________ {}
        impl surreal_orm::Patchable<bool> for self::Nice_______________ {}
        #[derive(Debug, Clone)]
        pub struct BunchOfOtherFields_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for BunchOfOtherFields_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for BunchOfOtherFields_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&BunchOfOtherFields_______________> for surreal_orm::ValueLike {
            fn from(value: &BunchOfOtherFields_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<BunchOfOtherFields_______________> for surreal_orm::ValueLike {
            fn from(value: BunchOfOtherFields_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&BunchOfOtherFields_______________> for surreal_orm::Field {
            fn from(field_name: &BunchOfOtherFields_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<BunchOfOtherFields_______________> for surreal_orm::Field {
            fn from(field_name: BunchOfOtherFields_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for BunchOfOtherFields_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for BunchOfOtherFields_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize>
            ::std::convert::From<self::BunchOfOtherFields_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::BunchOfOtherFields_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize>
            ::std::convert::From<&self::BunchOfOtherFields_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::BunchOfOtherFields_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<i32> for self::BunchOfOtherFields_______________ {}
        impl surreal_orm::Patchable<i32> for self::BunchOfOtherFields_______________ {}
        impl surreal_orm::SetterNumeric<i32> for self::BunchOfOtherFields_______________ {}
        impl ::std::convert::From<self::BunchOfOtherFields_______________> for surreal_orm::NumberLike {
            fn from(val: self::BunchOfOtherFields_______________) -> Self {
                val.0.into()
            }
        }
        impl ::std::convert::From<&self::BunchOfOtherFields_______________> for surreal_orm::NumberLike {
            fn from(val: &self::BunchOfOtherFields_______________) -> Self {
                val.clone().0.into()
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Add<T>
            for BunchOfOtherFields_______________
        {
            type Output = surreal_orm::Operation;
            fn add(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} + {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Sub<T>
            for BunchOfOtherFields_______________
        {
            type Output = surreal_orm::Operation;
            fn sub(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} - {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Mul<T>
            for BunchOfOtherFields_______________
        {
            type Output = surreal_orm::Operation;
            fn mul(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} * {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Div<T>
            for BunchOfOtherFields_______________
        {
            type Output = surreal_orm::Operation;
            fn div(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} / {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Add<T>
            for &BunchOfOtherFields_______________
        {
            type Output = surreal_orm::Operation;
            fn add(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} + {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Sub<T>
            for &BunchOfOtherFields_______________
        {
            type Output = surreal_orm::Operation;
            fn sub(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} - {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Mul<T>
            for &BunchOfOtherFields_______________
        {
            type Output = surreal_orm::Operation;
            fn mul(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} * {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Div<T>
            for &BunchOfOtherFields_______________
        {
            type Output = surreal_orm::Operation;
            fn div(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} / {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct Created_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for Created_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for Created_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&Created_______________> for surreal_orm::ValueLike {
            fn from(value: &Created_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<Created_______________> for surreal_orm::ValueLike {
            fn from(value: Created_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&Created_______________> for surreal_orm::Field {
            fn from(field_name: &Created_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<Created_______________> for surreal_orm::Field {
            fn from(field_name: Created_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for Created_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for Created_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::Created_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::Created_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::Created_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::Created_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<DateTime<Utc>> for self::Created_______________ {}
        impl surreal_orm::Patchable<DateTime<Utc>> for self::Created_______________ {}
    }
    pub mod _____schema_def {
        use super::_____field_names;
        #[allow(non_snake_case)]
        #[derive(Debug, Clone)]
        pub struct Schema {
            pub name: _____field_names::Name_______________,
            pub strength: _____field_names::Strength_______________,
            pub nice: _____field_names::Nice_______________,
            pub bunchOfOtherFields: _____field_names::BunchOfOtherFields_______________,
            pub created: _____field_names::Created_______________,
            pub(super) ___________graph_traversal_string: ::std::string::String,
            pub(super) ___________bindings: surreal_orm::BindingsList,
            pub(super) ___________errors: ::std::vec::Vec<::std::string::String>,
        }
    }
    pub type Rocket = _____schema_def::Schema;
    impl surreal_orm::Parametric for Rocket {
        fn get_bindings(&self) -> surreal_orm::BindingsList {
            self.___________bindings.to_vec()
        }
    }
    impl surreal_orm::Buildable for Rocket {
        fn build(&self) -> ::std::string::String {
            self.___________graph_traversal_string.to_string()
        }
    }
    impl surreal_orm::Erroneous for Rocket {
        fn get_errors(&self) -> ::std::vec::Vec<::std::string::String> {
            self.___________errors.to_vec()
        }
    }
    impl ::std::fmt::Display for Rocket {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{}", self.___________graph_traversal_string))
        }
    }
    impl surreal_orm::Aliasable for &Rocket {}
    impl surreal_orm::Parametric for &Rocket {
        fn get_bindings(&self) -> surreal_orm::BindingsList {
            self.___________bindings.to_vec()
        }
    }
    impl surreal_orm::Buildable for &Rocket {
        fn build(&self) -> ::std::string::String {
            self.___________graph_traversal_string.to_string()
        }
    }
    impl surreal_orm::Erroneous for &Rocket {
        fn get_errors(&self) -> ::std::vec::Vec<::std::string::String> {
            self.___________errors.to_vec()
        }
    }
    impl Rocket {
        pub fn new() -> Self {
            Self {
                name: "name".into(),
                strength: "strength".into(),
                nice: "nice".into(),
                bunchOfOtherFields: "bunchOfOtherFields".into(),
                created: "created".into(),
                ___________graph_traversal_string: "".into(),
                ___________bindings: vec![],
                ___________errors: vec![],
            }
        }
        pub fn new_prefixed(prefix: impl ::std::convert::Into<surreal_orm::ValueLike>) -> Self {
            let prefix: surreal_orm::ValueLike = prefix.into();
            Self {
                name: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "name"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                strength: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "strength"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                nice: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "nice"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                bunchOfOtherFields: surreal_orm::Field::new(format!(
                    "{}.{}",
                    prefix.build(),
                    "bunchOfOtherFields"
                ))
                .with_bindings(prefix.get_bindings())
                .into(),
                created: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "created"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                ___________graph_traversal_string: prefix.build(),
                ___________bindings: prefix.get_bindings(),
                ___________errors: vec![],
            }
        }
        pub fn empty() -> Self {
            Self {
                name: "".into(),
                strength: "".into(),
                nice: "".into(),
                bunchOfOtherFields: "".into(),
                created: "".into(),
                ___________graph_traversal_string: "".into(),
                ___________bindings: vec![],
                ___________errors: vec![],
            }
        }
        pub fn __________connect_object_to_graph_traversal_string(
            connection: impl surreal_orm::Buildable + surreal_orm::Parametric + surreal_orm::Erroneous,
            clause: impl ::std::convert::Into<surreal_orm::ObjectClause>,
        ) -> Self {
            let mut schema_instance = Self::empty();
            let clause: surreal_orm::ObjectClause = clause.into();
            let bindings = [
                connection.get_bindings().as_slice(),
                clause.get_bindings().as_slice(),
            ]
            .concat();
            let bindings = bindings.as_slice();
            schema_instance.___________bindings = bindings.into();
            let errors = [
                connection.get_errors().as_slice(),
                clause.get_errors().as_slice(),
            ]
            .concat();
            let errors = errors.as_slice();
            schema_instance.___________errors = errors.into();
            let connection_str = format!("{}{}", connection.build(), clause.build());
            schema_instance
                .___________graph_traversal_string
                .push_str(connection_str.as_str());
            let ___________graph_traversal_string =
                &schema_instance.___________graph_traversal_string;
            schema_instance.name = schema_instance
                .name
                .set_graph_string(format!("{}.{}", ___________graph_traversal_string, "name"))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.strength = schema_instance
                .strength
                .set_graph_string(format!(
                    "{}.{}",
                    ___________graph_traversal_string, "strength"
                ))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.nice = schema_instance
                .nice
                .set_graph_string(format!("{}.{}", ___________graph_traversal_string, "nice"))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.bunchOfOtherFields = schema_instance
                .bunchOfOtherFields
                .set_graph_string(format!(
                    "{}.{}",
                    ___________graph_traversal_string, "bunchOfOtherFields"
                ))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.created = schema_instance
                .created
                .set_graph_string(format!(
                    "{}.{}",
                    ___________graph_traversal_string, "created"
                ))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance
        }
    }
}
#[allow(non_snake_case)]
fn test_________internal_rocket_schema_edge_name() {
    surreal_orm::validators::assert_impl_one!(String: ::std::convert::Into<surreal_orm::sql::Strand>);
    surreal_orm::validators::assert_impl_one!(u64: ::std::convert::Into<surreal_orm::sql::Number>);
    surreal_orm::validators::assert_impl_one!(bool: ::std::convert::Into< ::std::primitive::bool>);
    surreal_orm::validators::assert_impl_one!(i32: ::std::convert::Into<surreal_orm::sql::Number>);
    surreal_orm::validators::assert_impl_one!(DateTime<Utc> : ::std::convert::Into<surreal_orm::sql::Datetime>);
}
