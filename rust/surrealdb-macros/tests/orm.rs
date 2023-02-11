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
use surrealdb_derive::{SurrealdbEdge, SurrealdbNode};

use std::fmt::{Debug, Display};
use surrealdb_macros::{
    links::{LinkMany, LinkOne, LinkSelf, Relate},
    model_id::SurId,
    node_builder::{NodeBuilder as NodeBuilder2, ToNodeBuilder as ToNodeBuilder2},
    query_builder::{query, ToNodeBuilder},
    Clause, SurrealdbEdge, /* SurrealdbEdge, */ SurrealdbNode,
};
use test_case::test_case;
use typed_builder::TypedBuilder;

#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "student")]
pub struct Student {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<String>,
    first_name: String,
    last_name: String,

    #[surrealdb(link_one = "Book", skip_serializing)]
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

// #[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[derive(SurrealdbEdge, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "writes")]
pub struct Writes<In: SurrealdbNode, Out: SurrealdbNode> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<String>,

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
    id: Option<String>,
    title: String,
    content: String,
}

#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "blog")]
pub struct Blog {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<String>,
    title: String,
    content: String,
}
#[cfg(test)]
mod tests {
    use super::*;
    use surrealdb_macros::DbField;
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
            .book(Clause::Id(SurId::try_from("Book:blaze").unwrap()))
            .title;

        assert_eq!(
            x.to_string(),
            "->Writes->Book[WHERE id = Book:blaze].title".to_string()
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
            "->Writes[WHERE timeWritten = 12:00]->Book.content".to_string()
        )
    }

    #[test]
    fn multiplication_tests4_with_alias() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::Where(query().where_(
                Book::schema().content.contains_one("Oyelowo in Uranus"),
            )))
            .__as__(Student::schema().writtenBooks);

        assert_eq!(
            x.to_string(),
            "->Writes[WHERE timeWritten = 12:00]->Book[WHERE content CONTAINS Oyelowo in Uranus] AS writtenBooks"
                .to_string()
        )
    }

    #[test]
    fn multiplication_tests4() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::Where(query().where_(
                Book::schema().content.contains_one("Oyelowo in Uranus"),
            )))
            .content;

        assert_eq!(
            x.to_string(),
            "->Writes[WHERE timeWritten = 12:00]->Book[WHERE content CONTAINS Oyelowo in Uranus].content"
                .to_string()
        )
    }

    #[test]
    fn multiplication_tests5() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::Id(SurId::try_from("Book:oyelowo").unwrap()))
            .content;

        assert_eq!(
            x.to_string(),
            "->Writes[WHERE timeWritten = 12:00]->Book[WHERE id = Book:oyelowo].content"
                .to_string()
        )
    }

    #[test]
    fn multiplication_tests6() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::Id("Book:oyelowo".try_into().unwrap()))
            .__as__(Student::schema().writtenBooks);

        assert_eq!(
            x.to_string(),
            "->Writes[WHERE timeWritten = 12:00]->Book[WHERE id = Book:oyelowo] AS writtenBooks"
                .to_string()
        )
    }

    #[test]
    fn multiplication_tests7() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::Id("Book:oyelowo".try_into().unwrap()))
            .__as__("real_deal");

        assert_eq!(
            x.to_string(),
            "->Writes[WHERE timeWritten = 12:00]->Book[WHERE id = Book:oyelowo] AS real_deal"
                .to_string()
        )
    }
}
