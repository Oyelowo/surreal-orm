/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use pretty_assertions::assert_eq;
use surreal_models::{student, Student};
use surreal_orm::{index, this, value, where_, All, Buildable, Operatable, SchemaGetter, ToRaw, E};

#[test]
fn test_param_with_path() {
    let param_with_path = this()
        .with_path::<Student>(index(2))
        .bestFriend()
        .bestFriend()
        .course()
        .title;

    assert_eq!(
        param_with_path.to_raw().build(),
        "$this[2].bestFriend.bestFriend.course.title"
    );
}

#[test]
fn test_param_with_path_with_index_square_bracket_variation() {
    // You can also use `[2]` instead of `index(2)`.
    let param_with_path = value()
        .with_path::<Student>([2])
        .bestFriend()
        .bestFriend()
        .course()
        .title;

    assert_eq!(
        param_with_path.to_raw().build(),
        "$value[2].bestFriend.bestFriend.course.title"
    );
}

#[test]
fn test_param_with_path_no_clause() {
    let param_with_path = this()
        .with_path::<Student>([2])
        .bestFriend()
        .bestFriend()
        .course()
        .title;

    assert_eq!(
        param_with_path.to_raw().build(),
        "$this[2].bestFriend.bestFriend.course.title"
    );
}

#[test]
fn test_param_with_path_with_clause() {
    let student::Schema { age, .. } = Student::schema();

    let param_with_path = this()
        .with_path::<Student>(where_(age.greater_than(18)))
        .bestFriend()
        .semesterCourses([5])
        .title;

    assert_eq!(
        param_with_path.to_raw().build(),
        "$this[WHERE age > 18].bestFriend.semesterCourses[5].title"
    );
}

#[test]
fn test_param_with_path_with_all_wildcard() {
    let param_with_path = this()
        .with_path::<Student>(All)
        .bestFriend()
        .semesterCourses(index(5))
        .title;

    assert_eq!(
        param_with_path.to_raw().build(),
        "$this[*].bestFriend.semesterCourses[5].title"
    );
}

#[test]
fn test_param_with_path_multiple_indexes() {
    let param_with_path = this()
        .with_path::<Student>([2])
        .bestFriend()
        .semesterCourses([5])
        .title;

    assert_eq!(
        param_with_path.to_raw().build(),
        "$this[2].bestFriend.semesterCourses[5].title"
    );
}

#[test]
fn test_param_with_path_simple() {
    let param_with_path = this().with_path::<Student>([2]).firstName;

    assert_eq!(param_with_path.to_raw().build(), "$this[2].firstName");
}

#[test]
fn test_param_simple_clause() {
    let param_with_path = this().with_path::<Student>(E).lastName;

    assert_eq!(param_with_path.to_raw().build(), "$this.lastName");
}

#[test]
fn basic() {
    let param_with_path = this();

    assert_eq!(param_with_path.to_raw().build(), "$this");
}
