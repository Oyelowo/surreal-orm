/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

#![recursion_limit = "2048"]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use _core::time::Duration;
use insta;
use regex;
use serde::{Deserialize, Serialize};
use static_assertions::*;
use std::fmt::{Debug, Display};
use surrealdb::sql;
use surrealdb::{
    engine::local::{Db, Mem},
    opt::IntoResource,
    sql::Id,
    Result, Surreal,
};
use surrealdb_models::{
    book_schema, student_schema, writes_schema, Book, Student, StudentWritesBook,
};
use surrealdb_orm::statements::{order, relate, select};
use surrealdb_orm::*;
use test_case::test_case;
use typed_builder::TypedBuilder;

trait WhiteSpaceRemoval {
    fn remove_extra_whitespace(&self) -> String
    where
        Self: std::borrow::Borrow<str>,
    {
        let mut result = String::with_capacity(self.borrow().len());
        let mut last_char_was_whitespace = true;

        for c in self.borrow().chars() {
            if c.is_whitespace() {
                if !last_char_was_whitespace {
                    result.push(' ');
                    last_char_was_whitespace = true;
                }
            } else {
                result.push(c);
                last_char_was_whitespace = false;
            }
        }

        result
    }
}
impl WhiteSpaceRemoval for &str {}
impl WhiteSpaceRemoval for String {}

// macro_rules! sql {
//     ($($item:tt)*) => {{
//         let valid_tokens = ["SELECT", "WHERE"];
//         let mut exprs = vec![];
//         $(
//             match stringify!($item) {
//                 $(
//                     x if x == valid_tokens[0] => {
//                         exprs.push(syn::parse_str(x).unwrap());
//                     }
//                 )*
//                 $(
//                     x if x == valid_tokens[1] => {
//                         exprs.push(syn::parse_str(x).unwrap());
//                     }
//                 )*
//                 _ => {
//                     exprs.push(syn::parse_str(stringify!($item)).unwrap_or_else(|_| {
//                         compile_error!(concat!("Invalid expression or token: ", stringify!($item)))
//                     }));
//                 }
//             }
//         )*
//         exprs
//     }};
// }

use serde_json::{Map, Value};

// fn remove_field_from_json_string(json_string: &str, field_name: &str) -> String {
//     let value: Value = serde_json::from_str(json_string).expect("Invalid JSON string");
//
//     let mut map = match value {
//         Value::Object(map) => map,
//         _ => panic!("Expected a JSON object"),
//     };
//
//     map.remove(field_name);
//
//     serde_json::to_string(&Value::Object(map)).expect("Failed to serialize JSON value")
// }

fn remove_field_from_json_string(json_string: &str, field_name: &str) -> String {
    let value: Value = serde_json::from_str(json_string).expect("Invalid JSON string");

    let updated_value = match value {
        Value::Object(mut map) => {
            map.remove(field_name);
            Value::Object(map)
        }
        Value::Array(mut vec) => {
            for element in vec.iter_mut() {
                if let Value::Object(ref mut map) = *element {
                    map.remove(field_name);
                }
            }
            Value::Array(vec)
        }
        _ => value,
    };

    serde_json::to_string(&updated_value).expect("Failed to serialize JSON value")
}

#[test]
fn can_get_structs_relations() {
    let relations_aliases = Student::get_fields_relations_aliased();
    assert_eq!(
        relations_aliases
            .into_iter()
            .map(|r| r.to_raw().build())
            .collect::<Vec<_>>()
            .join(", "),
        "->writes->book AS writtenBooks, ->writes->blog AS blogsssss"
    );
}

#[test]
fn should_not_contain_error_when_invalid_id_use_in_connection() {
    let student_id = SurrealId::try_from("student:1").unwrap();
    let book_id = SurrealId::try_from("book:2").unwrap();

    let write = StudentWritesBook {
        time_written: Duration::from_secs(343),
        ..Default::default()
    };

    let x = relate(Student::with(&student_id).writes__(Empty).book(&book_id))
        .content(write.clone())
        .return_type(ReturnType::Before)
        .parallel();

    assert_eq!(x.get_errors().len(), 0);
    let errors: Vec<String> = vec![];
    assert_eq!(x.get_errors(), errors);
}

#[test]
fn test_relation_graph_with_alias() {
    let student_id = SurrealId::try_from("student:1").unwrap();
    let book_id = SurrealId::try_from("book:2").unwrap();

    let aliased_connection = Student::with(student_id)
        .writes__(Empty)
        .book(book_id)
        .__as__(Student::aliases().writtenBooks);

    assert_eq!(
        aliased_connection.fine_tune_params(),
        "$_param_00000001->writes->$_param_00000002 AS writtenBooks"
    );

    assert_eq!(
        aliased_connection.clone().to_raw().build(),
        "student:1->writes->book:2 AS writtenBooks"
    );

    assert_eq!(aliased_connection.get_errors().len(), 0);
    let errors: Vec<String> = vec![];
    assert_eq!(aliased_connection.get_errors(), errors);
}

#[test]
fn test_recursive_edge_to_edge_connection_as_supported_in_surrealql() {
    // This allows for pattern like this:
    // -- Select all 1st, 2nd, and 3rd level people who this specific person record knows, or likes, as separate outputs
    // SELECT ->knows->(? AS f1)->knows->(? AS f2)->(knows, likes AS e3 WHERE influencer = true)->(? AS f3) FROM person:tobie;

    let student_id = SurrealId::try_from("student:1").unwrap();
    let book_id = SurrealId::try_from("book:2").unwrap();
    let likes = Table::new("likes");
    let writes = StudentWritesBook::table_name();
    let timeWritten = Field::new("timeWritten");

    // let knows = Field::writes("influencer");

    let aliased_connection = Student::with(student_id)
        .writes__(Empty)
        .writes__(Empty)
        .writes__(any_other_edges(&[writes, likes]).where_(timeWritten.less_than_or_equal(50)))
        .book(book_id)
        .__as__(Student::aliases().writtenBooks);

    assert_eq!(
        aliased_connection.fine_tune_params(),
        "$_param_00000001->writes->writes->(writes, writes, likes  WHERE timeWritten <= $_param_00000002)->$_param_00000003 AS writtenBooks"
    );

    assert_eq!(
       dbg!( aliased_connection.clone()).to_raw().build(),
        "student:1->writes->writes->(writes, writes, likes  WHERE timeWritten <= 50)->book:2 AS writtenBooks"
    );

    assert_eq!(aliased_connection.get_errors().len(), 0);
    let errors: Vec<String> = vec![];
    assert_eq!(aliased_connection.get_errors(), errors);
}

#[test]
fn test_any_edge_filter() {
    let student_id = SurrealId::try_from("student:1").unwrap();
    let book_id = SurrealId::try_from("book:2").unwrap();
    let likes = Table::new("likes");
    let wants = Table::new("wants");
    let timeWritten = Field::new("timeWritten");

    let aliased_connection = Student::with(student_id)
        .writes__(any_other_edges(&[wants, likes]).where_(timeWritten.less_than_or_equal(50)))
        .book(book_id)
        .__as__(Student::aliases().writtenBooks);

    assert_eq!(
        aliased_connection.fine_tune_params(),
        "$_param_00000001->(writes, wants, likes  WHERE timeWritten <= $_param_00000002)->$_param_00000003 AS writtenBooks"
    );

    assert_eq!(
        dbg!(aliased_connection.clone()).to_raw().build(),
        "student:1->(writes, wants, likes  WHERE timeWritten <= 50)->book:2 AS writtenBooks"
    );

    assert_eq!(aliased_connection.get_errors().len(), 0);
    let errors: Vec<String> = vec![];
    assert_eq!(aliased_connection.get_errors(), errors);
}

#[test]
fn should_contain_error_when_invalid_id_use_in_connection() {
    let student_id = SurrealId::try_from("student:1").unwrap();
    let book_id = SurrealId::try_from("book:2").unwrap();

    let write = StudentWritesBook {
        time_written: Duration::from_secs(343),
        ..Default::default()
    };

    // Book id used with student schema, while student_id used for book. This should generate
    // two errors
    let x = relate(Student::with(&book_id).writes__(Empty).book(&student_id))
        .content(write.clone())
        .return_type(ReturnType::Before)
        .parallel();

    assert_eq!(x.get_errors().len(), 2);
    assert_eq!(
        x.get_errors(),
        vec![
            "invalid id book:2. Id does not belong to table student",
            "invalid id student:1. Id does not belong to table book"
        ]
    );
}

#[tokio::test]
async fn relate_query() -> surrealdb_orm::SurrealdbOrmResult<()> {
    use surrealdb::sql::Datetime;

    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test")
        .use_db("test")
        .await
        .expect("failed to use db");
    let student_id = SurrealId::try_from("student:1").unwrap();
    let book_id = SurrealId::try_from("book:2").unwrap();

    let write = StudentWritesBook {
        time_written: Duration::from_secs(343),
        ..Default::default()
    };

    let relate_simple = relate(Student::with(student_id).writes__(E).book(book_id)).content(write);
    assert_eq!(
        relate_simple.to_raw().build(),
        "RELATE student:1->writes->book:2 CONTENT { count: 0, timeWritten: { nanos: 0, secs: 343 } } ;"
    );

    // // You can use return one method and it just returns the single object
    // let relate_simple_object = relate_simple.return_one(db.clone()).await?;
    // // Remove id bcos it is non-deterministic i.e changes on every run
    // let relate_simple_object = remove_field_from_json_string(
    //     serde_json::to_string(&relate_simple_object)
    //         .unwrap()
    //         .as_str(),
    //     "id",
    // );
    // insta::assert_display_snapshot!(relate_simple_object);

    // // You can also use return many and it just returns the single object as an array
    // let relate_simple_array = relate_simple.return_many(db.clone()).await?;
    // let relate_simple_object = remove_field_from_json_string(
    //     serde_json::to_string(&relate_simple_object)
    //         .unwrap()
    //         .as_str(),
    //     "id",
    // );
    // insta::assert_display_snapshot!(relate_simple_object);

    Ok(())
}

#[tokio::test]
async fn relate_query_with_sub_query() -> surrealdb_orm::SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test")
        .use_db("test")
        .await
        .expect("failed to use db");
    let student_id = SurrealId::try_from("student:1").unwrap();
    let book_id = SurrealId::try_from("book:2").unwrap();

    let write = StudentWritesBook {
        time_written: Duration::from_secs(52),
        ..Default::default()
    };
    let relate_more = relate(
        Student::with(select(All).from(Student::get_table_name()))
            .writes__(Empty)
            .book(
                select(All).from(Book::get_table_name()), // .where_(Book::schema().title.like("Oyelowo")),
            ),
    )
    .content(write)
    .return_many(db.clone())
    .await?;
    let relate_more =
        remove_field_from_json_string(serde_json::to_string(&relate_more).unwrap().as_str(), "id");

    // TODO: This returns empty array. Figure out if this is the expected behaviour
    insta::assert_display_snapshot!(relate_more);
    Ok(())
}

#[test]
fn multiplication_tests8() {
    use serde_json;

    let sur_id = SurrealId::try_from("alien:oyelowo").unwrap();
    let json = serde_json::to_string(&sur_id).unwrap();
    assert_eq!(json, "\"alien:oyelowo\"");

    let sur_id = RecordId::from(("alien", "oyelowo"));
    let json = serde_json::to_string(&sur_id).unwrap();
    assert_eq!(json, "\"alien:oyelowo\"");
}

// #[test]
// #[cfg(feature = "raw")]
// fn should_display_actual_values_in_raw_format() {
//     let student_id = SurrealId::try_from("student:1").unwrap();
//     let book_id = SurrealId::try_from("book:2").unwrap();

//     let write = StudentWritesBook {
//         time_written: Duration::from_secs(343),
//         ..Default::default()
//     };

//     let raw = relate(Student::with(&student_id).writes__(Empty).book(&book_id))
//         .content(write.clone())
//         .return_(Return::Before)
//         .parallel();

//     insta::assert_display_snapshot!(raw);
//     insta::assert_debug_snapshot!(raw.get_bindings());
// }
