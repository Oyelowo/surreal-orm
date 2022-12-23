#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
// #![feature(inherent_associated_types)]
// #![feature(const_mut_refs)]

// For testing macros made here
use serde::{Deserialize, Serialize};
use surrealdb_macros::{FieldsGetter, SurrealdbModel,HelloMacro, MyTrait};

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
pub struct Account {
    id: Option<String>,
    handle: String,
    password: String,
    email: String,
    // projects: ForeignVec<Project>,
}

use surreal_simple_querybuilder::prelude::*;

fn main() {
    Account::get_schema().email.contains_none(values)
    // Account::get_schema().handle
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
