/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::ops::Deref;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use surreal_orm::{
    arr, cond, create_table_resources,
    functions::crypto,
    statements::{
        define_analyzer, define_event, define_index, define_param, define_scope, define_token,
        define_user, select, AnalyzerFilter, SnowballLanguage, Tokenizer, UserRole,
    },
    SurrealCrudNode, *,
};

use snake_cases::{AnimalSnakeCase, AnimalSnakeCaseEatsCrop};

#[derive(Debug, Clone)]
pub struct Resources;

define_function!(get_animal_by_id(id: int){ return id;} );
define_function!(get_animal_by_id2(id: int){ return id;} );

#[allow(dead_code)]
fn test_funcs() {
    get_animal_by_id!(1);
    get_animal_by_id2!(1);
}

create_param_name_fn!(__some_test_param1);
create_param_name_fn!(__some_test_param2);
create_param_name_fn!(__some_test_param3);

impl DbResources for Resources {
    create_table_resources!(
        Animal,
        AnimalSnakeCase,
        Crop,
        AnimalEatsCrop,
        AnimalSnakeCaseEatsCrop,
        Student,
        Planet
    );

    fn analyzers(&self) -> Vec<Raw> {
        let analyzer1 = define_analyzer("ascii")
            .tokenizers([Tokenizer::Class])
            .filters([
                AnalyzerFilter::Lowercase,
                AnalyzerFilter::Ascii,
                AnalyzerFilter::Edgengram(2, 15),
                AnalyzerFilter::Snowball(SnowballLanguage::English),
            ])
            .to_raw();
        vec![analyzer1]
    }

    fn functions(&self) -> Vec<Raw> {
        vec![
            get_animal_by_id_statement().to_raw(),
            get_animal_by_id2_statement().to_raw(),
        ]
    }

    fn params(&self) -> Vec<Raw> {
        vec![
            define_param(__some_test_param1()).to_raw(),
            define_param(__some_test_param2()).to_raw(),
            define_param(__some_test_param3()).to_raw(),
        ]
    }

    fn scopes(&self) -> Vec<Raw> {
        let user_credentials::Schema {
            email,
            passwordHash,
            ..
        } = &UserCredentials::schema();
        let pass_input = "1234";
        let scope = |scope| {
            define_scope(scope)
                .session(std::time::Duration::from_secs(60 * 60 * 24 * 30))
                .signup(
                    UserCredentials {
                        id: UserCredentials::create_id("oyelowo".to_string()),
                        email: "oyelowo.oss@gmail.com".into(),
                        password_hash: "1234".into(),
                    }
                    .create(),
                )
                .signin(
                    select(All).from(UserCredentials::table_name()).where_(
                        cond(email.equal("oyelowo@codebreather.com"))
                            .and(crypto::argon2::compare!(pass_input, passwordHash.deref())),
                    ),
                )
                .to_raw()
        };
        vec![scope("scope1"), scope("scope2")]
    }

    fn tokens(&self) -> Vec<Raw> {
        let token1 = define_token("oyelowo_token")
            .on_namespace()
            .type_(TokenType::PS512)
            .value("abrakradabra");

        let token2 = define_token("token2")
            .on_database()
            .type_(TokenType::EDDSA)
            .value("abrakradabra");

        let token3 = define_token("oyedayo_token")
            .on_scope("regional")
            .type_(TokenType::HS256)
            .value("abrakradabra");

        vec![token1.to_raw(), token2.to_raw(), token3.to_raw()]
    }

    fn users(&self) -> Vec<Raw> {
        let user1 = define_user("oyelowo")
            .on_database()
            .passhash("$argon2id$v=19$m=19456,t=2,p=1$u1CPdtdC0Ek5GE1gvidj/g$fjFa7PZM+4hp4hlUJN1fz/FaDAf7KY1Qu48F5m5P0V8")
            .role(UserRole::Owner)
            .to_raw();

        let user2 = define_user("oyedayo")
            .on_namespace()
            .passhash("$argon2id$v=19$m=19456,t=2,p=1$u1CPdtdC0Ek5GE1gvidj/g$fjFa7PZM+4hp4hlUJN1fz/FaDAf7KY1Qu48F5m5P0V8")
            .role(UserRole::Editor)
            .to_raw();

        vec![user1, user2]
    }
}

#[derive(Debug, Clone)]
pub struct ResourcesV2;

impl DbResources for ResourcesV2 {
    create_table_resources!(AnimalV2, Crop, AnimalEatsCropV2, PlanetV2, NewStuff);
}

#[derive(Debug, Clone)]
pub struct ResourcesV3;

impl DbResources for ResourcesV3 {
    create_table_resources!(Crop, Student);
}

#[derive(Debug, Clone)]
pub struct ResourcesV4;

impl DbResources for ResourcesV4 {
    create_table_resources!(Crop, Student);
}

#[derive(Debug, Clone)]
pub struct ResourcesV5;

impl DbResources for ResourcesV5 {
    create_table_resources!(Student);
}

#[derive(Debug, Clone)]
pub struct ResourcesV6;

impl DbResources for ResourcesV6 {}

#[derive(Debug, Clone)]
pub struct ResourcesV7;

impl DbResources for ResourcesV7 {
    create_table_resources!(Animal, Crop, AnimalEatsCrop, Planet);
}

#[derive(Debug, Clone)]
pub struct ResourcesV8;

impl DbResources for ResourcesV8 {}

#[derive(Debug, Clone)]
pub struct ResourcesV9;

impl DbResources for ResourcesV9 {}

#[derive(Debug, Clone)]
pub struct ResourcesV10;

impl DbResources for ResourcesV10 {
    create_table_resources!(Animal, Crop, Planet);
}

pub mod invalid_cases {
    use super::*;

    #[derive(Node, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(
        table_name = "user_renaming_from_currently_used_field_name_disallowed",
        schemafull
    )]
    pub struct UserRenamingFromCurrentlyUsedFieldNameDisallowed {
        pub id: SurrealSimpleId<Self>,
        #[surreal_orm(old_name = "firstName")]
        pub username: String,
        pub first_name: String,
        pub created_at: chrono::DateTime<Utc>,
    }

    pub struct ResourcesVRenamingFromCurrentlyUsedFieldNameDisallowed;
    impl DbResources for ResourcesVRenamingFromCurrentlyUsedFieldNameDisallowed {
        create_table_resources!(UserRenamingFromCurrentlyUsedFieldNameDisallowed);
    }

    #[derive(Node, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(
        table_name = "user_renaming_with_same_old_field_name_disallowed",
        schemafull
    )]
    pub struct UserRenamingWithSameOldFieldNameDisallowed {
        pub id: SurrealSimpleId<Self>,
        #[surreal_orm(old_name = "firstName")]
        pub first_name: String,
        pub another_stuff: String,
        pub created_at: chrono::DateTime<Utc>,
    }

    #[derive(Debug, Clone)]
    pub struct ResourcesVRenamingWithSameOldFieldNameDisallowed;

    impl DbResources for ResourcesVRenamingWithSameOldFieldNameDisallowed {
        create_table_resources!(UserRenamingWithSameOldFieldNameDisallowed);
    }
}

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "user_credentials", schemafull)]
pub struct UserCredentials {
    pub id: SurrealId<Self, String>,
    pub email: String,
    pub password_hash: String,
}

#[derive(Node, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "new_stuff", schemafull)]
pub struct NewStuff {
    pub id: SurrealSimpleId<Self>,
    pub first_name: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Node, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "planet", schemafull)]
pub struct Planet {
    pub id: SurrealSimpleId<Self>,
    pub first_name: String,
    pub population: u64,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub labels: Vec<String>,
}

#[derive(Node, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "planet", schemafull, relax_table_name)]
pub struct PlanetV2 {
    pub id: SurrealSimpleId<Self>,
    // #[surreal_orm(old_name = "firstName")]
    pub new_name: String,
    pub population: u64,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub labels: Vec<String>,
}

#[derive(Node, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student", schemafull)]
pub struct Student {
    pub id: SurrealSimpleId<Self>,
    pub university: String,
    pub age: u8,
    pub updated_at: chrono::DateTime<Utc>,
    pub created_at: chrono::DateTime<Utc>,
}

pub mod snake_cases {
    use super::*;

    #[derive(Node, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
    #[surreal_orm(table_name = "animal_snake_case", schemafull)]
    pub struct AnimalSnakeCase {
        pub id: SurrealSimpleId<Self>,
        pub species: String,
        pub attributes: Vec<String>,
        pub created_at: chrono::DateTime<Utc>,
        pub updated_at: chrono::DateTime<Utc>,
        pub velocity: u64,
    }

    // We are relaxing table name, so that this serves as second version of AnimalSnakeCase
    #[derive(Node, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
    #[surreal_orm(table_name = "animal_snake_case", schemafull, relax_table_name)]
    pub struct AnimalSnakeCaseV2 {
        pub id: SurrealSimpleId<Self>,
        pub species: String,
        #[surreal_orm(old_name = "attributes")]
        pub characteristics: Vec<String>,
        pub velocity: u64,
    }

    #[derive(Edge, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
    #[surreal_orm(table_name = "eats_snake_case", schemafull)]
    pub struct EatsSnakeCase<In: Node, Out: Node> {
        pub id: SurrealSimpleId<Self>,
        #[serde(rename = "in")]
        pub in_: In,
        pub out: Out,
        pub place: String,
        pub created_at: chrono::DateTime<Utc>,
    }

    pub type AnimalSnakeCaseEatsCrop = EatsSnakeCase<AnimalSnakeCase, Crop>;
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "animal", schemafull)]
pub struct Animal {
    pub id: SurrealSimpleId<Self>,
    pub species: String,
    pub attributes: Vec<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub velocity: u64,
}

impl TableResources for Animal {
    fn events_definitions() -> Vec<Raw> {
        let animal::Schema {
            species, velocity, ..
        } = Self::schema();

        let event1 = define_event("event1".to_string())
            .on_table("animal".to_string())
            .when(cond(species.eq("Homo Erectus")).and(velocity.gt(545)))
            .then(select(All).from(Crop::table_name()))
            .to_raw();

        let event2 = define_event("event2".to_string())
            .on_table("animal".to_string())
            .when(cond(species.eq("Homo Sapien")).and(velocity.lt(10)))
            .then(select(All).from(AnimalEatsCrop::table_name()))
            .to_raw();
        vec![event1, event2]
    }

    fn indexes_definitions() -> Vec<Raw> {
        let animal::Schema {
            species, velocity, ..
        } = Self::schema();

        let idx1 = define_index("species_speed_idx".to_string())
            .on_table(Self::table_name())
            .fields(arr![species, velocity])
            .unique()
            .to_raw();

        vec![idx1]
    }
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "animal", schemafull, relax_table_name)]
pub struct AnimalV2 {
    pub id: SurrealSimpleId<Self>,
    pub species: String,
    #[surreal_orm(old_name = "attributes")]
    pub characteristics: Vec<String>,
    pub velocity: u64,
}

impl TableResources for AnimalV2 {
    fn events_definitions() -> Vec<Raw> {
        let animal_v_2::Schema {
            species, velocity, ..
        } = Self::schema();

        let event1 = define_event("event1".to_string())
            .on_table("animal".to_string())
            .when(cond(species.eq("Homo Habillis").and(velocity.gt(545))))
            .then(select(All).from(Crop::table_name()))
            .to_raw();

        vec![event1]
    }

    fn indexes_definitions() -> Vec<Raw> {
        let animal_v_2::Schema {
            characteristics,
            velocity,
            ..
        } = Self::schema();

        let idx1 = define_index("species_speed_idx".to_string())
            .on_table(Self::table_name())
            .fields(arr![velocity, characteristics])
            .unique()
            .to_raw();

        vec![idx1]
    }
}

#[derive(Edge, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "eats", schemafull)]
pub struct Eats<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in")]
    pub in_: In,
    pub out: Out,
    pub place: String,
    pub created_at: chrono::DateTime<Utc>,
}

pub type AnimalEatsCrop = Eats<Animal, Crop>;

// #[derive(Edge, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
#[derive(TableResources, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
// #[surreal_orm(table_name = "eats", schemafull, relax_table_name)]
pub struct EatsV2<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in")]
    pub in_: In,
    pub out: Out,
    pub location: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

pub type AnimalEatsCropV2 = EatsV2<AnimalV2, Crop>;

#[derive(Node, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "crop", schemafull)]
pub struct Crop {
    pub id: SurrealSimpleId<Self>,
    pub color: String,
}

use surreal_orm::ToRaw as _;
impl<In: Node, Out: Node> EatsV2<In, Out> {
    pub const fn __get_serializable_field_names() -> [&'static str; 6usize] {
        [
            "surreal_orm :: Field :: new (\"id\")",
            "surreal_orm :: Field :: new (\"in\")",
            "surreal_orm :: Field :: new (\"out\")",
            "surreal_orm :: Field :: new (\"location\")",
            "surreal_orm :: Field :: new (\"createdAt\")",
            "surreal_orm :: Field :: new (\"updatedAt\")",
        ]
    }
}
impl<In: Node, Out: Node> surreal_orm::SchemaGetter for EatsV2<In, Out> {
    type Schema = ________internal_eats_v_2_schema::EatsV2;
    fn schema() -> eats_v_2::Schema {
        eats_v_2::Schema::new()
    }
    fn schema_prefixed(
        prefix: impl ::std::convert::Into<surreal_orm::ValueLike>,
    ) -> eats_v_2::Schema {
        eats_v_2::Schema::new_prefixed(prefix)
    }
}
#[allow(non_snake_case)]
impl<In: Node, Out: Node> surreal_orm::Edge for EatsV2<In, Out> {
    type In = In;
    type Out = Out;
    type TableNameChecker = ________internal_eats_v_2_schema::TableNameStaticChecker;
    #[allow(non_snake_case)]
    fn get_table_name() -> surreal_orm::Table {
        "eats".into()
    }
}
#[allow(non_snake_case)]
#[derive(surreal_orm::serde::Serialize, surreal_orm::serde::Deserialize, Debug, Clone, Default)]
pub struct EatsV2NonNullUpdater {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: ::std::option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub createdAt: ::std::option::Option<chrono::DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updatedAt: ::std::option::Option<chrono::DateTime<Utc>>,
}
#[allow(non_snake_case)]
#[derive(surreal_orm::serde::Serialize, surreal_orm::serde::Deserialize, Debug, Clone)]
pub struct EatsV2RenamedCreator {
    pub id: &'static str,
    pub in_: &'static str,
    pub out: &'static str,
    pub location: &'static str,
    pub createdAt: &'static str,
    pub updatedAt: &'static str,
}
#[allow(non_snake_case)]
impl<In: Node, Out: Node> surreal_orm::Model for EatsV2<In, Out> {
    type Id = SurrealSimpleId<Self>;
    type NonNullUpdater = EatsV2NonNullUpdater;
    type StructRenamedCreator = EatsV2RenamedCreator;
    fn table_name() -> surreal_orm::Table {
        "eats".into()
    }
    fn get_id(self) -> Self::Id {
        self.id
    }
    fn get_id_as_thing(&self) -> surreal_orm::sql::Thing {
        surreal_orm::sql::thing(self.id.to_raw().as_str()).unwrap()
    }
    fn get_serializable_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![
            surreal_orm::Field::new("id"),
            surreal_orm::Field::new("in"),
            surreal_orm::Field::new("out"),
            surreal_orm::Field::new("location"),
            surreal_orm::Field::new("createdAt"),
            surreal_orm::Field::new("updatedAt"),
        ];
    }
    fn get_linked_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![];
    }
    fn get_link_one_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![];
    }
    fn get_link_self_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![];
    }
    fn get_link_one_and_self_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![];
    }
    fn get_link_many_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![];
    }
    fn define_table() -> surreal_orm::Raw {
        surreal_orm::statements::define_table(Self::table_name())
            .schemafull()
            .to_raw()
    }
    fn define_fields() -> ::std::vec::Vec<surreal_orm::Raw> {
        vec![
            surreal_orm::statements::define_field(surreal_orm::Field::new("id"))
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Record(::std::vec![
                    Self::table_name()
                ]))
                .to_raw(),
            surreal_orm::statements::define_field(surreal_orm::Field::new("in"))
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Record(::std::vec![]))
                .to_raw(),
            surreal_orm::statements::define_field(surreal_orm::Field::new("out"))
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Record(::std::vec![]))
                .to_raw(),
            surreal_orm::statements::define_field(surreal_orm::Field::new("location"))
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::String)
                .to_raw(),
            surreal_orm::statements::define_field(surreal_orm::Field::new("createdAt"))
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Datetime)
                .to_raw(),
            surreal_orm::statements::define_field(surreal_orm::Field::new("updatedAt"))
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Datetime)
                .to_raw(),
        ]
    }
    fn get_field_meta() -> ::std::vec::Vec<surreal_orm::FieldMetadata> {
        return vec![
            surreal_orm::FieldMetadata {
                name: "id".into(),
                old_name: ::std::option::Option::None,
                definition: ::std::vec![surreal_orm::statements::define_field(
                    surreal_orm::Field::new("id")
                )
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Record(::std::vec![
                    Self::table_name()
                ]))
                .to_raw()],
            },
            surreal_orm::FieldMetadata {
                name: "in".into(),
                old_name: ::std::option::Option::None,
                definition: ::std::vec![surreal_orm::statements::define_field(
                    surreal_orm::Field::new("in")
                )
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Record(::std::vec![]))
                .to_raw()],
            },
            surreal_orm::FieldMetadata {
                name: "out".into(),
                old_name: ::std::option::Option::None,
                definition: ::std::vec![surreal_orm::statements::define_field(
                    surreal_orm::Field::new("out")
                )
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Record(::std::vec![]))
                .to_raw()],
            },
            surreal_orm::FieldMetadata {
                name: "location".into(),
                old_name: ::std::option::Option::None,
                definition: ::std::vec![surreal_orm::statements::define_field(
                    surreal_orm::Field::new("location")
                )
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::String)
                .to_raw()],
            },
            surreal_orm::FieldMetadata {
                name: "createdAt".into(),
                old_name: ::std::option::Option::None,
                definition: ::std::vec![surreal_orm::statements::define_field(
                    surreal_orm::Field::new("createdAt")
                )
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Datetime)
                .to_raw()],
            },
            surreal_orm::FieldMetadata {
                name: "updatedAt".into(),
                old_name: ::std::option::Option::None,
                definition: ::std::vec![surreal_orm::statements::define_field(
                    surreal_orm::Field::new("updatedAt")
                )
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Datetime)
                .to_raw()],
            },
        ];
    }
}
#[allow(non_snake_case)]
pub mod eats_v_2 {
    pub use super::________internal_eats_v_2_schema::_____schema_def::Schema;
}
#[allow(non_snake_case)]
mod ________internal_eats_v_2_schema {
    use surreal_orm::Buildable as _;
    use surreal_orm::Erroneous as _;
    use surreal_orm::Node;
    use surreal_orm::Parametric as _;
    pub struct TableNameStaticChecker {
        pub eats: ::std::string::String,
    }
    pub(super) mod _____field_names {
        use super::super::*;
        use surreal_orm::Buildable as _;
        use surreal_orm::Parametric as _;
        #[derive(Debug, Clone)]
        pub struct Id_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for Id_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for Id_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&Id_______________> for surreal_orm::ValueLike {
            fn from(value: &Id_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<Id_______________> for surreal_orm::ValueLike {
            fn from(value: Id_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&Id_______________> for surreal_orm::Field {
            fn from(field_name: &Id_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<Id_______________> for surreal_orm::Field {
            fn from(field_name: Id_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for Id_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for Id_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::Id_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::Id_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::Id_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::Id_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<surreal_orm::sql::Thing> for self::Id_______________ {}
        impl surreal_orm::Patchable<surreal_orm::sql::Thing> for self::Id_______________ {}
        #[derive(Debug, Clone)]
        pub struct In_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for In_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for In_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&In_______________> for surreal_orm::ValueLike {
            fn from(value: &In_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<In_______________> for surreal_orm::ValueLike {
            fn from(value: In_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&In_______________> for surreal_orm::Field {
            fn from(field_name: &In_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<In_______________> for surreal_orm::Field {
            fn from(field_name: In_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for In_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for In_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::In_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::In_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::In_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::In_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<In: Node> surreal_orm::SetterAssignable<In> for self::In_______________ {}
        impl<In: Node> surreal_orm::Patchable<In> for self::In_______________ {}
        #[derive(Debug, Clone)]
        pub struct Out_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for Out_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for Out_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&Out_______________> for surreal_orm::ValueLike {
            fn from(value: &Out_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<Out_______________> for surreal_orm::ValueLike {
            fn from(value: Out_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&Out_______________> for surreal_orm::Field {
            fn from(field_name: &Out_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<Out_______________> for surreal_orm::Field {
            fn from(field_name: Out_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for Out_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for Out_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::Out_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::Out_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::Out_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::Out_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<Out: Node> surreal_orm::SetterAssignable<surreal_orm::sql::Thing>
            for self::Out_______________
        {
        }
        impl<Out: Node> surreal_orm::Patchable<surreal_orm::sql::Thing> for self::Out_______________ {}
        #[derive(Debug, Clone)]
        pub struct Location_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for Location_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for Location_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&Location_______________> for surreal_orm::ValueLike {
            fn from(value: &Location_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<Location_______________> for surreal_orm::ValueLike {
            fn from(value: Location_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&Location_______________> for surreal_orm::Field {
            fn from(field_name: &Location_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<Location_______________> for surreal_orm::Field {
            fn from(field_name: Location_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for Location_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for Location_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::Location_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::Location_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::Location_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::Location_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<String> for self::Location_______________ {}
        impl surreal_orm::Patchable<String> for self::Location_______________ {}
        #[derive(Debug, Clone)]
        pub struct CreatedAt_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for CreatedAt_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for CreatedAt_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&CreatedAt_______________> for surreal_orm::ValueLike {
            fn from(value: &CreatedAt_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<CreatedAt_______________> for surreal_orm::ValueLike {
            fn from(value: CreatedAt_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&CreatedAt_______________> for surreal_orm::Field {
            fn from(field_name: &CreatedAt_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<CreatedAt_______________> for surreal_orm::Field {
            fn from(field_name: CreatedAt_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for CreatedAt_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for CreatedAt_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::CreatedAt_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::CreatedAt_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::CreatedAt_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::CreatedAt_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<chrono::DateTime<Utc>> for self::CreatedAt_______________ {}
        impl surreal_orm::Patchable<chrono::DateTime<Utc>> for self::CreatedAt_______________ {}
        #[derive(Debug, Clone)]
        pub struct UpdatedAt_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for UpdatedAt_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for UpdatedAt_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&UpdatedAt_______________> for surreal_orm::ValueLike {
            fn from(value: &UpdatedAt_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<UpdatedAt_______________> for surreal_orm::ValueLike {
            fn from(value: UpdatedAt_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&UpdatedAt_______________> for surreal_orm::Field {
            fn from(field_name: &UpdatedAt_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<UpdatedAt_______________> for surreal_orm::Field {
            fn from(field_name: UpdatedAt_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for UpdatedAt_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for UpdatedAt_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::UpdatedAt_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::UpdatedAt_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::UpdatedAt_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::UpdatedAt_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<chrono::DateTime<Utc>> for self::UpdatedAt_______________ {}
        impl surreal_orm::Patchable<chrono::DateTime<Utc>> for self::UpdatedAt_______________ {}
    }
    pub mod _____schema_def {
        use super::_____field_names;
        #[allow(non_snake_case)]
        #[derive(Debug, Clone)]
        pub struct Schema {
            pub id: _____field_names::Id_______________,
            pub in_: _____field_names::In_______________,
            pub out: _____field_names::Out_______________,
            pub location: _____field_names::Location_______________,
            pub createdAt: _____field_names::CreatedAt_______________,
            pub updatedAt: _____field_names::UpdatedAt_______________,
            pub(super) ___________graph_traversal_string: ::std::string::String,
            pub(super) ___________bindings: surreal_orm::BindingsList,
            pub(super) ___________errors: ::std::vec::Vec<::std::string::String>,
        }
    }
    pub type EatsV2 = _____schema_def::Schema;
    impl surreal_orm::Buildable for EatsV2 {
        fn build(&self) -> ::std::string::String {
            self.___________graph_traversal_string.to_string()
        }
    }
    impl surreal_orm::Parametric for EatsV2 {
        fn get_bindings(&self) -> surreal_orm::BindingsList {
            self.___________bindings.to_vec()
        }
    }
    impl surreal_orm::Erroneous for EatsV2 {
        fn get_errors(&self) -> Vec<::std::string::String> {
            self.___________errors.to_vec()
        }
    }
    impl surreal_orm::Aliasable for &EatsV2 {}
    impl surreal_orm::Parametric for &EatsV2 {
        fn get_bindings(&self) -> surreal_orm::BindingsList {
            self.___________bindings.to_vec()
        }
    }
    impl surreal_orm::Buildable for &EatsV2 {
        fn build(&self) -> ::std::string::String {
            self.___________graph_traversal_string.to_string()
        }
    }
    impl surreal_orm::Erroneous for &EatsV2 {
        fn get_errors(&self) -> ::std::vec::Vec<::std::string::String> {
            self.___________errors.to_vec()
        }
    }
    impl EatsV2 {
        pub fn new() -> Self {
            Self {
                id: "id".into(),
                in_: "in".into(),
                out: "out".into(),
                location: "location".into(),
                createdAt: "createdAt".into(),
                updatedAt: "updatedAt".into(),
                ___________graph_traversal_string: "".into(),
                ___________bindings: vec![],
                ___________errors: vec![],
            }
        }
        pub fn new_prefixed(prefix: impl ::std::convert::Into<surreal_orm::ValueLike>) -> Self {
            let prefix: surreal_orm::ValueLike = prefix.into();
            Self {
                id: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "id"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                in_: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "in"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                out: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "out"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                location: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "location"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                createdAt: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "createdAt"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                updatedAt: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "updatedAt"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                ___________graph_traversal_string: prefix.build(),
                ___________bindings: prefix.get_bindings(),
                ___________errors: vec![],
            }
        }
        pub fn empty() -> Self {
            Self {
                id: "".into(),
                in_: "".into(),
                out: "".into(),
                location: "".into(),
                createdAt: "".into(),
                updatedAt: "".into(),
                ___________graph_traversal_string: "".into(),
                ___________bindings: vec![],
                ___________errors: vec![],
            }
        }
        pub fn __________connect_edge_to_graph_traversal_string(
            connection: impl surreal_orm::Buildable + surreal_orm::Parametric + surreal_orm::Erroneous,
            clause: impl Into<surreal_orm::EdgeClause>,
        ) -> Self {
            let mut schema_instance = Self::empty();
            let clause: surreal_orm::EdgeClause = clause.into();
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
            let schema_edge_str_with_arrow = format!("{}{}", connection.build(), clause.build(),);
            schema_instance
                .___________graph_traversal_string
                .push_str(schema_edge_str_with_arrow.as_str());
            let ___________graph_traversal_string =
                &schema_instance.___________graph_traversal_string;
            schema_instance.id = schema_instance
                .id
                .set_graph_string(format!("{}.{}", ___________graph_traversal_string, "id"))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.in_ = schema_instance
                .in_
                .set_graph_string(format!("{}.{}", ___________graph_traversal_string, "in"))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.out = schema_instance
                .out
                .set_graph_string(format!("{}.{}", ___________graph_traversal_string, "out"))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.location = schema_instance
                .location
                .set_graph_string(format!(
                    "{}.{}",
                    ___________graph_traversal_string, "location"
                ))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.createdAt = schema_instance
                .createdAt
                .set_graph_string(format!(
                    "{}.{}",
                    ___________graph_traversal_string, "createdAt"
                ))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.updatedAt = schema_instance
                .updatedAt
                .set_graph_string(format!(
                    "{}.{}",
                    ___________graph_traversal_string, "updatedAt"
                ))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance
        }
    }
}
#[allow(non_snake_case)]
fn test_eatsv2_edge_name() {
    surreal_orm::validators::assert_impl_one!(String: ::std::convert::Into<surreal_orm::sql::Strand>);
    surreal_orm::validators::assert_impl_one!(chrono::DateTime<Utc> : ::std::convert::Into<surreal_orm::sql::Datetime>);
    surreal_orm::validators::assert_impl_one!(chrono::DateTime<Utc> : ::std::convert::Into<surreal_orm::sql::Datetime>);
}
