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
    query_builder::query,
    Clause, SurrealdbEdge, SurrealdbNode,
};
use test_case::test_case;
use typed_builder::TypedBuilder;

#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "student")]
pub struct Student {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<SurId>,
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
    id: Option<SurId>,

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
    id: Option<SurId>,
    title: String,
    content: String,
}

#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "blog")]
pub struct Blog {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<SurId>,
    title: String,
    content: String,
}
// macro_rules! here_ {
//     ($($arg:tt)*) => {{
//         let mut exprs = vec![];
//         let mut ops = vec![];
//         $(
//             match stringify!($arg) {
//                 "=" | "!=" | "==" | "?=" | "*=" | "~" | "!~" | "?~" | "*~" | "<" | "<=" | ">" | ">=" | "+" | "-" | "*" | "/" | "&&" | "||" | "AND" | "OR" | "IS" | "IS NOT" | "CONTAINS" | "∋" | "CONTAINSNOT" | "∌" | "CONTAINSALL" | "⊇" | "CONTAINSANY" | "⊃" | "CONTAINSNONE" | "⊅" | "INSIDE" | "∈" | "NOTINSIDE" | "∉" | "ALLINSIDE" | "⊆" | "ANYINSIDE" | "⊂" | "NONEINSIDE" | "⊄" | "OUTSIDE" | "INTERSECTS" => {
//                     ops.push(stringify!($arg));
//                 },
//                 _ => {
//                     exprs.push($arg.to_string());
//                 }
//             }
//         )*
//         // Do something with `exprs` and `ops` here
//     }}
// }
#[cfg(test)]
mod tests {
    use super::*;
    use surrealdb_macros::{op, where_, DbField};
    use test_case::test_case;

    #[test]
    fn multiplication_tests4_with_alias() {
        op!(">=");
        let xx = &[
            StudentWritesBook::schema().timeWritten.to_string().as_str(),
            op!(">="),
            "12:00",
        ];

        let writes_schema::Writes { timeWritten, .. } = StudentWritesBook::schema();

        where_!(
            timeWritten op!(">=") "12:00"
        );

        where_!(
            timeWritten op!("CONTAINS") "12:00"
        );

        // here_!(timeWritten op!(">=") "12:00");

        let x = Student::schema()
            .writes__(Clause::Where(
                // query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
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
}
