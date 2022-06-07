#![allow(dead_code)]
#![allow(non_snake_case)]

use my_macros::FieldsGetter;
use serde::{Deserialize, Serialize};

#[test]
fn keys_getter_1() {
    #[derive(FieldsGetter, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Consumer {
        #[serde(rename = "lowo_cool")]
        pub name_of_me: String,

        #[serde(rename = "age_count")]
        pub age: u8,
    }

    let ConsumerFields {
        lowo_cool,
        age_count,
    } = Consumer::get_fields_serialized();

    assert_eq!(lowo_cool, "lowo_cool");
    assert_eq!(age_count, "age_count");
}

#[test]
fn keys_getter_4() {
    #[derive(FieldsGetter, Serialize, Deserialize)]
    #[field_getter(rename_all = "camelCase")]
    pub struct Consumer {
        pub name_of_me: String,

        pub age: u8,
    }

    let ConsumerFields { nameOfMe, age } = Consumer::get_fields_serialized();

    assert_eq!(nameOfMe, "nameOfMe");
    assert_eq!(age, "age");
}

#[test]
fn keys_getter_5() {
    #[derive(FieldsGetter, Serialize, Deserialize)]
    #[field_getter(rename_all = "snake_case")]
    pub struct Consumer {
        #[warn(non_snake_case)]
        pub nameOfMe: String,

        pub age: u8,
    }

    let ConsumerFields { name_of_me, age } = Consumer::get_fields_serialized();

    assert_eq!(name_of_me, "name_of_me");
    assert_eq!(age, "age");
}

#[test]
fn keys_getter_6() {
    #[derive(FieldsGetter, Serialize, Deserialize)]
    #[field_getter(rename_all = "snake_case")]
    pub struct Consumer {
        #[warn(non_snake_case)]
        pub name_of_me: String,

        pub ageCount: u8,

        #[field_getter(rename = "username")]
        pub first_name: u8,
    }

    let ConsumerFields {
        name_of_me,
        age_count,
        username,
    } = Consumer::get_fields_serialized();

    assert_eq!(name_of_me, "name_of_me");
    assert_eq!(age_count, "age_count");
    assert_eq!(username, "username");
}

#[test]
fn keys_getter_7() {
    #[derive(FieldsGetter, Serialize, Deserialize)]
    #[field_getter(rename_all = "snake_case")]
    pub struct Consumer {
        #[warn(non_snake_case)]
        pub name_of_me: String,

        #[serde(rename = "age")]
        pub ageCount: u8,

        #[field_getter(rename = "username")]
        pub first_name: u8,
    }

    let ConsumerFields {
        name_of_me,
        age,
        username,
    } = Consumer::get_fields_serialized();

    assert_eq!(name_of_me, "name_of_me");
    assert_eq!(age, "age");
    assert_eq!(username, "username");
}

#[test]
fn keys_getter_8() {
    #[derive(FieldsGetter, Serialize, Deserialize)]
    #[field_getter(rename_all = "kebab-case")]
    pub struct Consumer {
        #[warn(non_snake_case)]
        pub name_of_me: String,

        pub ageCount: u8,

        #[field_getter(rename = "anotherName")]
        pub first_name: u8,
    }

    let ConsumerFields {
        name_of_me,
        ageCount,
        anotherName,
    } = Consumer::get_fields_serialized();

    assert_eq!(name_of_me, "name-of-me");
    assert_eq!(ageCount, "age-count");
    assert_eq!(anotherName, "anotherName");
}

#[test]
fn handle_nested_values() {
    #[derive(FieldsGetter, Serialize, Deserialize)]
    #[field_getter(rename_all = "camelCase")]
    pub struct Consumer {
        #[serde(rename(serialize = "lowo_cool", deserialize = "lowo_cool"))]
        pub name_of_me: String,

        #[serde(rename(serialize = "age"))]
        pub ageCount: u8,

        #[serde(rename = "simple_name")]
        pub first_name: u8,
    }

    let ConsumerFields {
        lowo_cool,
        age,
        simple_name,
    } = Consumer::get_fields_serialized();

    assert_eq!(lowo_cool, "lowo_cool");
    assert_eq!(age, "age");
    assert_eq!(simple_name, "simple_name");
}

#[test]
fn handle_nested_values_at_struct_lvel() {
    #[derive(FieldsGetter, Serialize, Deserialize)]
    #[serde(rename_all(serialize = "camelCase"))]
    pub struct Consumer {
        #[serde(rename(serialize = "lowo_cool", deserialize = "lowo_cool"))]
        pub name_of_me: String,

        #[serde(rename(serialize = "age"))]
        pub ageCount: u8,

        #[serde(rename = "simple_name")]
        pub first_name: u8,
    }

    let ConsumerFields {
        lowo_cool,
        age,
        simple_name,
    } = Consumer::get_fields_serialized();

    assert_eq!(lowo_cool, "lowo_cool");
    assert_eq!(age, "age");
    assert_eq!(simple_name, "simple_name");
}

#[test]
fn handle_nested_values_at_struct_level_but_ignore_deserialize() {
    #[derive(FieldsGetter, Serialize, Deserialize)]
    #[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
    pub struct Consumer {
        #[serde(rename(serialize = "lowo_cool", deserialize = "lowo_cool"))]
        pub name_of_me: String,

        #[serde(rename(serialize = "age"))]
        pub ageCount: u8,

        #[serde(rename = "simple_name")]
        pub first_name: u8,
    }

    let ConsumerFields {
        lowo_cool,
        age,
        simple_name,
    } = Consumer::get_fields_serialized();

    assert_eq!(lowo_cool, "lowo_cool");
    assert_eq!(age, "age");
    assert_eq!(simple_name, "simple_name");
}
