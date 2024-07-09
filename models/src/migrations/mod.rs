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
            define_param(__some_test_param1())
                .value("test param one")
                .to_raw(),
            define_param(__some_test_param2())
                .value("test param two")
                .to_raw(),
            define_param(__some_test_param3())
                .value("test param three")
                .to_raw(),
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
                    select(All).from(UserCredentials::table()).where_(
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
        table = user_renaming_from_currently_used_field_name_disallowed,
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
        table = user_renaming_with_same_old_field_name_disallowed,
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
#[surreal_orm(table = user_credentials, schemafull)]
pub struct UserCredentials {
    pub id: SurrealId<Self, String>,
    pub email: String,
    pub password_hash: String,
}

#[derive(Node, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = new_stuff, schemafull)]
pub struct NewStuff {
    pub id: SurrealSimpleId<Self>,
    pub first_name: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Node, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = planet, schemafull)]
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
#[surreal_orm(table = planet, schemafull, relax_table)]
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
#[surreal_orm(table = student, schemafull)]
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
    #[surreal_orm(table = animal_snake_case, schemafull)]
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
    #[surreal_orm(table = animal_snake_case, schemafull, relax_table)]
    pub struct AnimalSnakeCaseV2 {
        pub id: SurrealSimpleId<Self>,
        pub species: String,
        #[surreal_orm(old_name = attributes)]
        pub characteristics: Vec<String>,
        pub velocity: u64,
    }

    #[derive(Edge, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
    #[surreal_orm(table = eats_snake_case, schemafull)]
    pub struct EatsSnakeCase<In: Node, Out: Node> {
        pub id: SurrealSimpleId<Self>,
        #[surreal_orm(link_many = "In")]
        pub r#in: LinkOne<In>,
        #[surreal_orm(link_many = "Out")]
        pub out: LinkOne<Out>,
        pub place: String,
        pub created_at: chrono::DateTime<Utc>,
    }

    pub type AnimalSnakeCaseEatsCrop = EatsSnakeCase<AnimalSnakeCase, Crop>;
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = animal, schemafull)]
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
            .then(select(All).from(Crop::table()))
            .to_raw();

        let event2 = define_event("event2".to_string())
            .on_table("animal".to_string())
            .when(cond(species.eq("Homo Sapien")).and(velocity.lt(10)))
            .then(select(All).from(AnimalEatsCrop::table()))
            .to_raw();
        vec![event1, event2]
    }

    fn indexes_definitions() -> Vec<Raw> {
        let animal::Schema {
            species, velocity, ..
        } = Self::schema();

        let idx1 = define_index("species_speed_idx".to_string())
            .on_table(Self::table())
            .fields(arr![species, velocity])
            .unique()
            .to_raw();

        vec![idx1]
    }
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = animal, schemafull, relax_table)]
pub struct AnimalV2 {
    pub id: SurrealSimpleId<Self>,
    pub species: String,
    #[surreal_orm(old_name = attributes)]
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
            .then(select(All).from(Crop::table()))
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
            .on_table(Self::table())
            .fields(arr![velocity, characteristics])
            .unique()
            .to_raw();

        vec![idx1]
    }
}

#[derive(Edge, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = eats, schemafull)]
pub struct Eats<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Self>,
    #[surreal_orm(link_one = In)]
    pub r#in: LinkOne<In>,
    #[surreal_orm(link_one = Out)]
    pub out: LinkOne<Out>,
    pub place: String,
    pub created_at: chrono::DateTime<Utc>,
}

pub type AnimalEatsCrop = Eats<Animal, Crop>;

#[derive(Edge, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
#[surreal_orm(table = eats_v2, schemafull, relax_table)]
pub struct EatsV2<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in")]
    #[surreal_orm(link_one = In, ty = "record<any>")]
    pub in_: LinkOne<In>,
    #[surreal_orm(link_one = Out, ty = "record<any>")]
    pub out: LinkOne<Out>,
    pub location: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

pub type AnimalEatsCropV2 = EatsV2<AnimalV2, Crop>;

#[derive(Node, TableResources, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = "crop", schemafull)]
pub struct Crop {
    pub id: SurrealSimpleId<Self>,
    pub color: String,
}
