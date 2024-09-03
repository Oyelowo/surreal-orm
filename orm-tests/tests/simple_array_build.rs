/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use pretty_assertions::assert_eq;
use surreal_models::Alien;
use surreal_orm::{
    index, this, where_, All, AllGetter, Buildable, Last, Operatable, SchemaGetter, ToRaw,
};

#[test]
fn test_simple_array_element_access_with_last() {
    let last_tag = Alien::schema().tags(Last);
    assert_eq!(last_tag.fine_tune_params(), "tags[$]");
    assert_eq!(last_tag.to_raw().build(), "tags[$]");
}

#[test]
fn test_simple_array_element_access_with_index() {
    let last_tag = Alien::schema().tags(index(0));
    assert_eq!(last_tag.fine_tune_params(), "tags[$_param_00000001]");
    assert_eq!(last_tag.to_raw().build(), "tags[0]");
}

#[test]
fn test_simple_array_element_access_with_index_2() {
    let last_tag = Alien::schema().tags(index(1));
    assert_eq!(last_tag.fine_tune_params(), "tags[$_param_00000001]");
    assert_eq!(last_tag.to_raw().build(), "tags[1]");
}

#[test]
fn test_simple_array_element_access_with_filter() {
    let last_tag = Alien::schema().tags(index(1)).greater_than(1);
    assert_eq!(
        last_tag.fine_tune_params(),
        "tags[$_param_00000001] > $_param_00000002"
    );
    assert_eq!(last_tag.to_raw().build(), "tags[1] > 1");
}

#[test]
fn test_simple_array_element_access_with_all() {
    let last_tag = Alien::schema().tags(All);
    assert_eq!(last_tag.fine_tune_params(), "tags[*]");
    assert_eq!(last_tag.to_raw().build(), "tags[*]");
}

#[test]
fn test_simple_array_element_access_with_where_clause_no_all_asteriks_object_access() {
    let last_tag = Alien::schema().tags(where_(this().less_than(18)));
    assert_eq!(
        last_tag.fine_tune_params(),
        "tags[WHERE $this < $_param_00000001]"
    );
    assert_eq!(last_tag.to_raw().build(), "tags[WHERE $this < 18]");
}

#[test]
fn test_simple_array_element_access_with_where_clause() {
    let last_tag = Alien::schema().tags(where_(this().less_than(18))).all();
    assert_eq!(
        last_tag.fine_tune_params(),
        "tags[WHERE $this < $_param_00000001].*"
    );
    assert_eq!(last_tag.to_raw().build(), "tags[WHERE $this < 18].*");
}
