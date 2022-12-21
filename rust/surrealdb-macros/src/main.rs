#![allow(dead_code)]
#![allow(non_snake_case)]

// For testing macros made here
use serde::{Deserialize, Serialize};
use surrealdb_macros::{FieldsGetter, HelloMacro, MyTrait};

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

fn main() {
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
