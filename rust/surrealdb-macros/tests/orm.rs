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

/* fn ewer() {
    struct Nama<T> {}

    impl<T> Nama<T> {
        fn new() -> Self {
            Self::<T>new();
            Self {}
        }
    }
} */
// #[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[derive(SurrealdbEdge, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Writes<In: SurrealdbNode, Out: SurrealdbNode> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<String>,

    // #[surrealdb(link_one = "Book", skip_serializing)]
    r#in: In,
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
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(-2, -4 ; "when both operands are negative")]
    #[test_case(2,  4  ; "when both operands are positive")]
    #[test_case(4,  2  ; "when operands are swapped")]
    fn multiplication_tests(x: i8, y: i8) {
        let actual = (x * y).abs();

        assert_eq!(8, actual)
    }
}
