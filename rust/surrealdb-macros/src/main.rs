#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
// #![feature(inherent_associated_types)]
// #![feature(const_mut_refs)]

// For testing macros made here
use serde::{Deserialize, Serialize};
use surrealdb_macros::{FieldsGetter, HelloMacro, MyTrait, SurrealdbModel};

#[derive(HelloMacro)]
struct Pancakes;

#[derive(MyTrait)]
#[my_trait(answer = 50, level = "high")]
struct Foo {
    group: String,
}

#[derive(FieldsGetter, Serialize, Deserialize)]
#[field_getter(rename_all(serialize = "snake_case"))]
// #[serde(rename_all = "camelCase")]
pub struct ConsumingType {
    // #[serde(rename = "lowo_cool")]
    #[serde(rename(serialize = "lowo_cool", deserialize = "lowo_cool"))]
    pub name_of_me: String,

    #[serde(rename = "lmsar")]
    pub age: u8,
}

// use serde::{Deserialize, Serialize};
// use surreal_simple_querybuilder::prelude::*;
use surrealdb_derive::SurrealdbModel;

#[derive(SurrealdbModel, Default)]
#[surrealdb(rename_all = "camelCase")]
pub struct Account {
    id: Option<String>,
    handle: String,
    // #[surrealdb(rename = "nawao")]
    first_name: String,
    #[surrealdb(rename = "lastName")]
    another_name: String,
    chess: String,
    nice_poa: String,
    password: String,
    email: String,

    #[surrealdb(relate = "->runs->Project")]
    projects: ForeignVec<Project>,
}

#[derive(SurrealdbModel, Default)]
#[surrealdb(rename_all = "camelCase")]
pub struct Project {
    id: Option<String>,
    title: String,

    // #[surrealdb(relate = "->run_by->Accout")]
    // account: ForeignVec<Project>,
    // projects: ForeignVec<Project>,
}

use surreal_simple_querybuilder::prelude::*;

fn main() {
    // Account::schema.fav_proj()
    // Account::schema.projects().title
    // Account::schema.projects()
    Account::get_schema()
        .projects()
        .title
        .contains_none("values");
    // Account::schema.fav_proj().title.contains_any("values");
    // Account::get_fields_serialized()
    // Account::get_schema().email;
    // Account::get_schema().lastName
    // Account::schema.firstName
    // Account::get_schema().firstName.contains_one("value");
    // Account::get_schema()
    // Account::schema.nicePoa
    // Account::get_schema().firstName
    // Account::get_schema().email.contains_all(values)
    // account::schema::model
    let ConsumingTypeFields {
        lowo_cool, lmsar, ..
    } = ConsumingType::get_fields_serialized();

    println!("rere{lowo_cool}, {lmsar}")
}

#[test]
fn default() {
    assert_eq!(Foo::answer(), 50);
    assert!(Foo::level().contains("High"));
    assert!(!Foo::level().contains("Low"));
}
