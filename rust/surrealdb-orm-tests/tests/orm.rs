/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};
use static_assertions::*;
use surrealdb::{
    engine::local::{Db, Mem},
    opt::IntoResource,
    sql::Id,
    Result, Surreal,
};
// use surrealdb_derive::{SurrealdbEdge, SurrealdbNode};

use std::fmt::{Debug, Display};
use surrealdb_orm::{
    links::{LinkMany, LinkOne, LinkSelf, Relate},
    RecordId, SurrealdbEdge, SurrealdbNode,
};
use test_case::test_case;
use typed_builder::TypedBuilder;

#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "student")]
pub struct Student {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<RecordId>,
    first_name: String,
    last_name: String,

    #[surrealdb(link_self = "Student", skip_serializing)]
    best_friend: LinkSelf<Student>,

    #[surrealdb(link_one = "Book", skip_serializing)]
    #[serde(rename = "unoBook")]
    fav_book: LinkOne<Book>,

    #[surrealdb(link_one = "Book", skip_serializing)]
    course: LinkOne<Book>,

    #[surrealdb(link_many = "Book", skip_serializing)]
    #[serde(rename = "semCoures")]
    all_semester_courses: LinkMany<Book>,

    #[surrealdb(relate(model = "StudentWritesBook", connection = "->writes->book"))]
    written_books: Relate<Book>,
}

#[derive(SurrealdbEdge, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "writes")]
pub struct Writes<In: SurrealdbNode, Out: SurrealdbNode> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<RecordId>,

    // #[surrealdb(link_one = "Book", skip_serializing)]
    #[serde(rename = "in")]
    _in: In,
    // r#in: In,
    out: Out,
    time_written: String,
}

type StudentWritesBook = Writes<Student, Book>;

#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "book")]
pub struct Book {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<RecordId>,
    title: String,
    content: String,
}

#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "blog")]
pub struct Blog {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<RecordId>,
    title: String,
    content: String,
}
#[cfg(test)]
mod tests {
    /*     use super::*;
    use surrealdb_macros::Field;
    use test_case::test_case;

    #[test]
    fn multiplication_tests1() {
        let x = Student::schema().firstName;
        assert_eq!(x.to_string(), "firstName".to_string())
    }

    #[test]
    fn multiplication_tests2() {
        let x = Student::schema()
            .writes__(Clause::All)
            .book(Clause::Id(RecordId::try_from("book:blaze").unwrap()))
            .title;

        assert_eq!(
            x.to_string(),
            "->writes->book[WHERE id = book:blaze].title".to_string()
        )
    }

    #[test]
    fn multiplication_tests3() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::All)
            .content;

        assert_eq!(
            x.to_string(),
            "->writes[WHERE timeWritten = 12:00]->book.content".to_string()
        )
    }

    #[test]
    fn multiplication_tests4_with_alias() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::Where(
                query().where_(Book::schema().content.contains("Oyelowo in Uranus")),
            ))
            .__as__(Student::schema().writtenBooks);

        assert_eq!(
            x.to_string(),
            "->writes[WHERE timeWritten = 12:00]->book[WHERE content CONTAINS Oyelowo in Uranus] AS writtenBooks"
                .to_string()
        )
    }

    #[test]
    fn multiplication_tests4() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::Where(
                query().where_(Book::schema().content.contains("Oyelowo in Uranus")),
            ))
            .content;

        assert_eq!(
            x.to_string(),
            "->writes[WHERE timeWritten = 12:00]->book[WHERE content CONTAINS Oyelowo in Uranus].content"
                .to_string()
        )
    }

    #[test]
    fn multiplication_tests5() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::Id(RecordId::try_from("book:oyelowo").unwrap()))
            .content;

        assert_eq!(
            x.to_string(),
            "->writes[WHERE timeWritten = 12:00]->book[WHERE id = book:oyelowo].content"
                .to_string()
        )
    }

    #[test]
    fn multiplication_tests6() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::Id("book:oyelowo".try_into().unwrap()))
            .__as__(Student::schema().writtenBooks);

        assert_eq!(
            x.to_string(),
            "->writes[WHERE timeWritten = 12:00]->book[WHERE id = book:oyelowo] AS writtenBooks"
                .to_string()
        )
    }

    #[test]
    fn multiplication_tests7() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::Id("book:oyelowo".try_into().unwrap()))
            .__as__("real_deal");

        assert_eq!(
            x.to_string(),
            "->writes[WHERE timeWritten = 12:00]->book[WHERE id = book:oyelowo] AS real_deal"
                .to_string()
        )
    }

    #[test]
    fn multiplication_tests8() {
        use serde_json;

        let sur_id = RecordId::new("alien", "oyelowo");
        let json = serde_json::to_string(&sur_id).unwrap();
        assert_eq!(json, "\"alien:oyelowo\"");

        let sur_id = RecordId::try_from("alien:oyelowo").unwrap();
        let json = serde_json::to_string(&sur_id).unwrap();
        assert_eq!(json, "\"alien:oyelowo\"");
    } */
}
