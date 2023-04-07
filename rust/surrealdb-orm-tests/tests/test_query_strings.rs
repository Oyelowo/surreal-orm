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
use surrealdb::sql;
use surrealdb::{
    engine::local::{Db, Mem},
    opt::IntoResource,
    sql::Id,
    Result, Surreal,
};
use surrealdb_models::{book, student, writes_schema, Book, Student, StudentWritesBook};
use surrealdb_orm::*;
use surrealdb_orm::{
    array, cond,
    statements::{order, relate, select},
};
use test_case::test_case;
// use surrealdb_derive::{SurrealdbEdge, SurrealdbNode};

use std::fmt::{Debug, Display};
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

// fn replace_params(input: &str, suffix: u32) -> String {
// fn replace_params(input: &str) -> String {
//     let suffix = 123456;
//     let mut output = String::new();
//     let mut last_end = 0;
//     while let Some(start) = input[last_end..].find("$_param_") {
//         let end = start + 16;
//         output.push_str(&input[last_end..last_end + start]);
//         output.push_str(&format!("$_param_{}", suffix));
//         last_end += end;
//     }
//     output.push_str(&input[last_end..]);
//     output
// }

use regex::Regex;
fn replace_params(query: &str) -> String {
    let mut count = 0;
    let re = regex::Regex::new(r"_param_[[:xdigit:]]+").unwrap();
    re.replace_all(query, |caps: &regex::Captures<'_>| {
        count += 1;
        format!("_param_{:08}", count)
    })
    .to_string()
}

// fn replace_params(query: &str) -> String {
//     let re = regex::Regex::new(r"_param_[[:xdigit:]]{8}").unwrap();
//     let mut count = 0;
//     let new_str = re.replace_all(query, |captures: &regex::Captures<'_>| {
//         count += 1;
//         format!("$_param_{:08x}", count)
//     });

//     new_str.to_string()
// }

#[test]
fn multiplication_tests1() {
    let student::Student {
        id,
        firstName,
        lastName,
        bestFriend,
        unoBook,
        course,
        semCoures,
        age,
        ..
    } = &Student::schema();
    let st = Student::schema();
    let bk = &Book::schema();
    let wrt = &StudentWritesBook::schema();
    let writes_schema::Writes { timeWritten, .. } = StudentWritesBook::schema();
    let book::Book { content, .. } = Book::schema();
    // Student ===
    // ->writes->book as novel;
    // student:1->writes->book:2
    // (Select ... from .. where ..)->writes->(select ....)
    //
    // friend.name
    // friend[where age > 5].name
    // friend:1.name ..... not possible
    let xx = Student::with(Empty).writes__(Empty).book(Empty);
    assert_eq!(xx.to_string(), "student->writes->book".to_string());

    let written_book_selection = st
        .bestFriend(Empty)
        .writes__(wrt.timeWritten.equal("12:00"))
        .book(bk.content.contains("Oyelowo in Uranus"))
        .__as__(Student::aliases().writtenBooks);

    // assert_eq!(written_book_selection, "34".to_string());

    // Chain::new(age.clone())
    //     .chain(firstName)
    //     .greater_than_or_equal(20);

    // let xx = firstName
    //     .less_than(age)
    //     .greater_than(age)
    //     .less_than(firstName)
    //     .add(age)
    //     .multiply(id)
    //     .subtract(bestFriend)
    //     .divide(course)
    //     .fuzzy_equal(unoBook)
    //     .and(firstName.greater_than(age))
    //     .or(age.less_than_or_equal(writtenBooks))
    //     .or(age.greater_than(age));
    //
    // println!("maerfineirNAMAAAA :{xx}");

    let st = Student::schema();
    let written_book_selection = st
        .bestFriend(Empty)
        .writes__(wrt.timeWritten.equal("12:00"))
        .book(bk.content.contains("Oyelowo in Uranus"))
        .__as__(Student::aliases().writtenBooks);

    let rer = "".to_string().is_empty();

    #[derive(Serialize, Deserialize)]
    struct LIKE;

    impl Display for LIKE {
        fn fmt(&self, f: &mut _core::fmt::Formatter<'_>) -> _core::fmt::Result {
            f.write_str("LIKE")
        }
    }
    #[derive(Serialize, Deserialize)]
    struct OR;

    impl Display for OR {
        fn fmt(&self, f: &mut _core::fmt::Formatter<'_>) -> _core::fmt::Result {
            f.write_str("OR")
        }
    }
    #[derive(Serialize, Deserialize)]
    struct AND;

    impl Display for AND {
        fn fmt(&self, f: &mut _core::fmt::Formatter<'_>) -> _core::fmt::Result {
            f.write_str("AND")
        }
    }
    // age.and(firstName)

    let book::Book { content, .. } = Book::schema();

    cond(
        content
            .contains_any(vec!["Dyayo", "fdfd"])
            .contains_any(array!["Dyayo", "fdfd"])
            .contains_all(vec!["Dyayo", "fdfd"])
            .contains_all(array!["Dyayo", "fdfd"])
            .contains_none(vec!["Dyayo", "fdfd"])
            .contains_none(array!["Dyayo", "fdfd"])
            .contains_none(array![1, 3])
            .or("lowo"),
    )
    .and(age.less_than(55))
    .or(age.greater_than(17))
    .or(firstName.equal("Oyelowo"))
    .and(lastName.equal("Oyedayo"));

    let mut query1 = select(All)
        .from(Book::get_table_name())
        .where_(
            cond(content.like("lowo").and(age).greater_than_or_equal(600))
                .or(firstName.equal("Oyelowo"))
                .and(lastName.equal("Oyedayo")),
        )
        .group_by(content)
        .order_by(order(lastName).desc())
        .limit(50)
        .start(20)
        .timeout(Duration::from_secs(9))
        .parallel();

    let is_lowo = true;
    if is_lowo {
        query1 = query1.limit(50).group_by(age);
    }

    // let xx: Vec<Book> = query1.return_many(db.clone()).await.unwrap();

    insta::assert_display_snapshot!(&query1.fine_tune_params());
    insta::assert_debug_snapshot!(replace_params(&format!("{:?}", query1.get_bindings())));

    let ref student_table = Student::get_table_name();
    let ref book_table = Book::get_table_name();
    let ref book_id = SurrealId::try_from("book:1").unwrap();
    let ref student_id = SurrealId::try_from("student:1").unwrap();

    let mut query = select(All)
        .select(age)
        .select(firstName)
        .select(&[firstName, unoBook])
        .select(vec![firstName, unoBook])
        .from(student_table)
        .from(&[student_table, book_table])
        .from(vec![student_table, book_table])
        .from(book_id)
        .from(&[book_id, student_id])
        .from(vec![book_id, student_id])
        .from(vec![SurrealId::try_from("book:1").unwrap()])
        .from(query1)
        .where_(
            cond(
                age.greater_than(age)
                    .greater_than_or_equal(age)
                    .less_than_or_equal(20)
                    .like(firstName)
                    .add(5)
                    .subtract(10)
                    .and(unoBook)
                    .or(age),
            )
            .and(bestFriend.exactly_equal("Oyelowo"))
            .or(firstName.equal(true))
            .and(age.greater_than_or_equal(150)),
        )
        // .where_(
        //     cond!(age q!(>=) "12:00" OR firstName LIKE "oyelowo" AND lastName q!(~) "oyedyao"  AND age q!(>) 150),
        // )
        .order_by(order(firstName).rand().desc())
        .order_by(order(lastName).collate().asc())
        .order_by(order(id).numeric().desc())
        .order_by(vec![order(id).numeric().desc()])
        .order_by(&[order(id).numeric().desc(), order(firstName).desc()])
        .order_by(&[order(id).numeric().desc(), order(firstName).desc()])
        .group_by(course)
        .group_by(firstName)
        .group_by(&[lastName, unoBook, &Field::new("lowo")])
        .group_by(vec![lastName, unoBook, &Field::new("lowo")])
        .start(5)
        .limit(400)
        .fetch(firstName)
        .fetch(lastName)
        .fetch(&[age, unoBook])
        .fetch(vec![age, unoBook])
        .split(lastName)
        .split(firstName)
        .split(&[firstName, semCoures])
        .split(vec![firstName, semCoures])
        .timeout(Duration::from_secs(8))
        .parallel();

    let is_oyelowo = true;
    if is_oyelowo {
        query = query.group_by(&[age, bestFriend, &Field::new("dayo")]);
    }

    // stringify_tokens!("lowo", "knows", 5);

    // stringify_tokens2!("lowo", 5);
    let SELECT = "SELECT";
    let name = "name";
    let WHERE = "WHERE";
    let age = "age";

    // let result = sql!(SELECT name WHERE age > 5);
    // let result = sql!(SELECT name WHERE age > 5);

    insta::assert_display_snapshot!(replace_params(&query.to_string()));
    insta::assert_debug_snapshot!(replace_params(&format!("{:?}", query.get_bindings())));
    // assert_eq!(
    //     query.to_string().remove_extra_whitespace(),
    //     "SELECT *, ->writes[WHERE timeWritten = 12:00]->book[WHERE \
    //     content CONTAINS Oyelowo in Uranus] AS writtenBooks FROM \
    //     WHERE age <= 12:00 GROUP BY course, firstName, lastName, \
    //     lastName, unoBook, lowo, age, bestFriend, lowo;"
    //         .remove_extra_whitespace()
    // )
}

#[tokio::test]
async fn relate_query_building_for_ids() {
    use surrealdb::sql::Datetime;

    let student_id = SurrealId::try_from("student:1").unwrap();
    let book_id = SurrealId::try_from("book:2").unwrap();

    let write = StudentWritesBook {
        time_written: Duration::from_secs(343),
        ..Default::default()
    };

    let relate_simple =
        relate(Student::with(student_id).writes__(Empty).book(book_id)).content(write);

    insta::assert_display_snapshot!(&relate_simple.fine_tune_params());
    insta::assert_display_snapshot!(&relate_simple.clone().to_raw());
    insta::assert_debug_snapshot!(replace_params(&format!(
        "{:?}",
        relate_simple.get_bindings()
    )));
}

#[tokio::test]
async fn relate_query_building_for_subqueries() {
    use surrealdb::sql::Datetime;

    let student_id = SurrealId::try_from("student:1").unwrap();
    let book_id = SurrealId::try_from("book:2").unwrap();

    let write = StudentWritesBook {
        time_written: Duration::from_secs(52),
        ..Default::default()
    };
    let relation = relate(
        Student::with(select(All).from(Student::get_table_name()))
            .writes__(Empty)
            .book(
                select(All)
                    .from(Book::get_table_name())
                    .where_(Book::schema().title.like("Oyelowo")),
            ),
    )
    .content(write);
    insta::assert_debug_snapshot!(replace_params(&relation.build()));
    insta::assert_debug_snapshot!(replace_params(&format!("{:?}", relation.get_bindings())));
}

#[test]
fn multiplication_tests2() {
    let x = Student::schema()
        .writes__(Empty)
        .book(Book::schema().id.equal(RecordId::from(("book", "blaze"))))
        .title;

    assert_eq!(
        replace_params(&x.build()),
        "->writes->book[WHERE id = $_param_00000001].title".to_string() // "->writes->book[WHERE $_param_00000001 = $_param_00000002].title".to_string()
    );

    insta::assert_debug_snapshot!(replace_params(&format!("{:?}", x.get_bindings())));

    let st_schema = Student::schema();
    // Another case
    let x = st_schema
        .bestFriend(st_schema.age.between(18, 150))
        .bestFriend(Empty)
        .writes__(StudentWritesBook::schema().timeWritten.greater_than(3422))
        .book(Book::schema().id.equal(RecordId::from(("book", "blaze"))))
        .content;

    // insta::assert_display_snapshot!(&x.to_string());

    // insta::assert_display_snapshot!(&x.fine_tune_params());
    // dbg!(&x.get_bindings());
    // dbg!(&x.clone().to_string());
    // dbg!("====================");
    // dbg!("====================");
    // dbg!(&x.clone().to_raw());
    // assert!(false);
    // insta::assert_debug_snapshot!(&x.get_bindings());
    // dbg!(&x.to_raw());
    // insta::assert_display_snapshot!(format!(
    //     "bindings:{:?}, raw:{:?}",
    //     &x.get_bindings(),
    //     &x.to_raw()
    // ));

    insta::assert_display_snapshot!(&x.fine_tune_params());
    insta::assert_display_snapshot!(&x.to_raw());
    // insta::assert_display_snapshot!(replace_params(&x.to_string()));
    // insta::assert_debug_snapshot!(replace_params(&format!("{:?}", x.get_bindings())));
    // assert_eq!(
    //     x.to_string(),
    //     // "->writes->book[WHERE id = book:blaze].title".to_string()
    //     "->writes->book[WHERE id = $_param_00000000].title".to_string()
    // );
    //
    // let m = x.get_bindings();
    // assert_eq!(
    //     serde_json::to_string(&m).unwrap(),
    //     "[[\"_param_00000000\",\"book:blaze\"]]".to_string()
    // );
    // assert_eq!(
    //     format!("{m:?}"),
    //     "[[\"_param_00000000\",\"book:blaze\"]]".to_string()
    // );
    // let query = InsertQuery::new("company")
    //     .fields(&["name", "founded", "founders", "tags"])
    //     .values(&[
    //         &[
    //             "SurrealDB",
    //             "2021-09-10",
    //             "[person:tobie, person:jaime]",
    //             "['big data', 'database']",
    //         ],
    //         &["Acme Inc.", "1967-05-03", "null", "null"],
    //         &["Apple Inc.", "1976-04-01", "null", "null"],
    //     ])
    //     .on_duplicate_key_update(&[("tags", "tags += 'new tag'")])
    //     .build();
    // println!("{}", query);
}

#[test]
fn multiplication_tests3() {
    let x = Student::schema()
        .writes__(StudentWritesBook::schema().timeWritten.equal("12:00"))
        .book(Empty)
        .content;

    assert_eq!(
        replace_params(&x.to_string()),
        "->writes[WHERE timeWritten = $_param_00000001]->book.content".to_string()
    )
}
