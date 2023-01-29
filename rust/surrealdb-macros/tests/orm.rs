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
    model_id::SurIdComplex,
    node_builder::{NodeBuilder as NodeBuilder2, ToNodeBuilder as ToNodeBuilder2},
    query_builder::{query, ToNodeBuilder},
    Clause, SurrealdbEdge, /* SurrealdbEdge, */ SurrealdbNode,
};
use test_case::test_case;
use typed_builder::TypedBuilder;

#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    // #[serde(skip_serializing_if = "Option::is_none")]
    // #[builder(default, setter(strip_option))]
    id: Option<String>,
    first_name: String,
    last_name: String,

    #[surrealdb(link_one = "Book", skip_serializing)]
    #[serde(rename = "lowo_na")]
    fav_book: LinkOne<Book>,
    // // #[surrealdb(link_one = "Book", skip_serializing)]
    // course: LinkOne<Book>,
    // #[surrealdb(link_many = "Book", skip_serializing)]
    // #[serde(rename = "lowo")]
    // all_semester_courses: LinkMany<Book>,
    #[surrealdb(relate(edge = "StudentWritesBook", link = "->Writes->Book"))]
    written_blogs: Relate<Book>,
}

// #[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[derive(SurrealdbEdge, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Writes<In: SurrealdbNode, Out: SurrealdbNode> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<String>,

    // #[surrealdb(link_one = "Book", skip_serializing)]
    #[serde(rename = "in")]
    _in: In,
    out: Out,
    time_written: String,
}

type StudentWritesBook = Writes<Student, Book>;

#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Book {
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
        let x = Student::get_schema().firstName;
        assert_eq!(x.to_string(), "firstName".to_string())
    }

    #[test]
    fn multiplication_tests2() {
        let x = Student::get_schema()
            .Writes__(Clause::All)
            .Book(Clause::All)
            .title;

        assert_eq!(x.to_string(), "->Writes->Book.title".to_string())
    }

    #[test]
    fn multiplication_tests3() {
        let x = Student::get_schema()
            .Writes__(Clause::Where(
                query()
                    .and_where(Student::get_schema().firstName.contains_one("lowo"))
                    .build(),
            ))
            .Book(Clause::All)
            .content;

        assert_eq!(
            x.to_string(),
            "->Writes[WHERE firstName CONTAINS lowo]->Book.content".to_string()
        )
    }

    #[test]
    fn multiplication_tests4() {
        let x = Student::get_schema()
            .Writes__(Clause::Where(
                query()
                    .and_where(Student::get_schema().firstName.contains_one("lowo"))
                    .build(),
            ))
            .Book(Clause::Where(
                query()
                    .and_where(Book::get_schema().content.equals("Oyelowo in Uranus"))
                    .build(),
            ))
            .content;

        assert_eq!(
            x.to_string(),
            "->Writes[WHERE firstName CONTAINS lowo]->Book[WHERE content = Oyelowo in Uranus].content".to_string()
        )
    }

    #[test]
    fn multiplication_tests5() {
        let x = Student::get_schema()
            .Writes__(Clause::Where(
                query()
                    .and_where(Student::get_schema().firstName.contains_one("lowo"))
                    .build(),
            ))
            .Book(Clause::Id("Book:oyelowo".into()))
            .content;

        assert_eq!(
            x.to_string(),
            "->Writes[WHERE firstName CONTAINS lowo]->Book[WHERE id = Book:oyelowo].content"
                .to_string()
        )
    }
}
