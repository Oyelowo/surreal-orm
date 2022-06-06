#![allow(dead_code)]
#![allow(non_snake_case)]

// For testing macros made here
use my_macros::{HelloMacro, KeyNamesGetter, MyTrait};
use serde::{Deserialize, Serialize};
// use my_macros::KeyNamesGetter;

// use serde::{Serialize, Deserialize};

#[derive(HelloMacro)]
struct Pancakes;

#[derive(MyTrait)]
#[my_trait(answer = 50, level = "high")]
struct Foo {
    group: String,
}

#[derive(KeyNamesGetter, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsumingType {
    // #[serde(rename = "lowo_cool")]
    #[serde(rename(serialize = "lowo_cool", deserialize = "lowo_cool"))]
    pub name_of_me: String,

    #[serde(rename = "lmsar")]
    pub age: u8,
}

fn main() {
    let ConsumingTypeKeyNames {
        lowo_cool, lmsar, ..
    } = ConsumingType::get_field_names();

    println!("rere{lowo_cool}, {lmsar}")
}

#[test]
fn default() {
    assert_eq!(Foo::answer(), 50);
    assert!(Foo::level().contains("High"));
    assert!(!Foo::level().contains("Low"));
}

#[test]
fn keys_getter_1() {
    #[derive(KeyNamesGetter, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Consumer {
        #[serde(rename = "lowo_cool")]
        pub name_of_me: String,

        #[serde(rename = "age_count")]
        pub age: u8,
    }

    let ConsumerKeyNames {
        lowo_cool,
        age_count,
    } = Consumer::get_field_names();

    assert_eq!(lowo_cool, "lowo_cool");
    assert_eq!(age_count, "age_count");
}

#[test]
fn keys_getter_4() {
    #[derive(KeyNamesGetter, Serialize, Deserialize)]
    #[key_getter(rename_all = "camelCase")]
    pub struct Consumer {
        pub name_of_me: String,

        pub age: u8,
    }

    let ConsumerKeyNames { nameOfMe, age } = Consumer::get_field_names();

    assert_eq!(nameOfMe, "nameOfMe");
    assert_eq!(age, "age");
}

#[test]
fn keys_getter_5() {
    #[derive(KeyNamesGetter, Serialize, Deserialize)]
    #[key_getter(rename_all = "snake_case")]
    pub struct Consumer {
        #[warn(non_snake_case)]
        pub nameOfMe: String,

        pub age: u8,
    }

    let ConsumerKeyNames { name_of_me, age } = Consumer::get_field_names();

    assert_eq!(name_of_me, "name_of_me");
    assert_eq!(age, "age");
}

#[test]
fn keys_getter_6() {
    #[derive(KeyNamesGetter, Serialize, Deserialize)]
    #[key_getter(rename_all = "snake_case")]
    pub struct Consumer {
        #[warn(non_snake_case)]
        pub name_of_me: String,

        pub ageCount: u8,

        #[key_getter(rename = "username")]
        pub first_name: u8,
    }

    let ConsumerKeyNames {
        name_of_me,
        age_count,
        username,
    } = Consumer::get_field_names();

    assert_eq!(name_of_me, "name_of_me");
    assert_eq!(age_count, "age_count");
    assert_eq!(username, "username");
}

#[test]
fn keys_getter_7() {
    #[derive(KeyNamesGetter, Serialize, Deserialize)]
    #[key_getter(rename_all = "snake_case")]
    pub struct Consumer {
        #[warn(non_snake_case)]
        pub name_of_me: String,

        pub ageCount: u8,

        #[key_getter(case = "camel")]
        pub first_name: u8,
    }

    let ConsumerKeyNames {
        name_of_me,
        age_count,
        firstName,
    } = Consumer::get_field_names();

    assert_eq!(name_of_me, "name_of_me");
    assert_eq!(age_count, "age_count");
    assert_eq!(firstName, "firstName");
}

#[test]
fn keys_getter_8() {
    #[derive(KeyNamesGetter, Serialize, Deserialize)]
    #[key_getter(rename_all = "kebab-case")]
    pub struct Consumer {
        #[warn(non_snake_case)]
        pub name_of_me: String,

        pub ageCount: u8,

        #[key_getter(case = "camel")]
        pub first_name: u8,
    }

    let ConsumerKeyNames {
        nameOfMe,
        ageCount,
        firstName,
    } = Consumer::get_field_names();

    assert_eq!(nameOfMe, "name-of-me");
    assert_eq!(ageCount, "age-count");
    assert_eq!(firstName, "firstName");
}

#[test]
fn handle_nested_values() {
    #[derive(KeyNamesGetter, Serialize, Deserialize)]
    #[key_getter(rename_all = "camelCase")]
    pub struct Consumer {
        #[serde(rename(serialize = "lowo_cool", deserialize = "lowo_cool"))]
        pub name_of_me: String,

        #[serde(rename(serialize = "age"))]
        pub ageCount: u8,

        #[serde(rename = "simple_name")]
        pub first_name: u8,
    }

    let ConsumerKeyNames {
        lowo_cool,
        age,
        simple_name,
    } = Consumer::get_field_names();

    assert_eq!(lowo_cool, "lowo_cool");
    assert_eq!(age, "age");
    assert_eq!(simple_name, "simple_name");
}
