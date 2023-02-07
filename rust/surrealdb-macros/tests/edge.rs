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
    query_builder::{query, NodeBuilder, ToNodeBuilder},
    Clause, SurrealdbEdge, /* SurrealdbEdge, */ SurrealdbNode,
};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    // #[serde(skip_serializing_if = "Option::is_none")]
    // #[builder(default, setter(strip_option))]
    id: Option<String>,
    first_name: String,
    last_name: String,
    // #[serde(rename = "lowo_na")]
    // fav_book: LinkOne<Book>,
    // // #[surrealdb(link_one = "Book", skip_serializing)]
    // course: LinkOne<Book>,
    // #[surrealdb(link_many = "Book", skip_serializing)]
    // #[serde(rename = "lowo")]
    // all_semester_courses: LinkMany<Book>,
    // written_blogs: Relate<Book>,
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
#[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone)]
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

#[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<String>,
    title: String,
}
// ==============================================
// Recursive expansion of the SurrealdbNode macro
// ==============================================

impl surrealdb_macros::SurrealdbNode for Student {
    type Schema = student::Student;
    fn schema() -> Self::Schema {
        student::Student::new()
    }
    fn get_key(&self) -> ::std::option::Option<&String> {
        self.id.as_ref()
    }
}
pub mod student {
    use ::serde::Serialize;
    type Book = <super::Book as surrealdb_macros::SurrealdbNode>::Schema;
    #[derive(Debug, Serialize, Default)]
    pub struct Student {
        pub id: surrealdb_macros::DbField,
        pub firstName: surrealdb_macros::DbField,
        pub lastName: surrealdb_macros::DbField,
        pub lowo_na: surrealdb_macros::DbField,
        pub writtenBlogs: surrealdb_macros::DbField,
        pub(crate) ___________graph_traversal_string: ::std::string::String,
    }
    impl ::std::fmt::Display for Student {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{}", self.___________graph_traversal_string))
        }
    }
    impl Student {
        pub fn new() -> Self {
            Self {
                id: "id".into(),
                firstName: "firstName".into(),
                lastName: "lastName".into(),
                lowo_na: "lowo_na".into(),
                writtenBlogs: "writtenBlogs".into(),
                ___________graph_traversal_string: "".to_string(),
            }
        }
        pub fn __with_id__(mut self, id: impl ::std::fmt::Display) -> Self {
            self.___________graph_traversal_string
                .push_str(id.to_string().as_str());
            self
        }
        pub fn __with__(db_name: impl ::std::fmt::Display) -> Self {
            let mut schema_instance = Self::new();
            schema_instance
                .___________graph_traversal_string
                .push_str(db_name.to_string().as_str());
            schema_instance
        }
        pub fn __________connect_to_graph_traversal_string(
            store: &::std::string::String,
            clause: surrealdb_macros::Clause,
        ) -> Self {
            let mut schema_instance = Self::default();
            let connection = format!(
                "{}{}{}",
                store,
                "Student",
                surrealdb_macros::format_clause(clause, "Student")
            );
            schema_instance
                .___________graph_traversal_string
                .push_str(connection.as_str());
            let ___________graph_traversal_string =
                &schema_instance.___________graph_traversal_string;
            schema_instance
                .id
                .push_str(format!("{}.{}", ___________graph_traversal_string, "id").as_str());
            schema_instance.firstName.push_str(
                format!("{}.{}", ___________graph_traversal_string, "firstName").as_str(),
            );
            schema_instance
                .lastName
                .push_str(format!("{}.{}", ___________graph_traversal_string, "lastName").as_str());
            schema_instance
                .lowo_na
                .push_str(format!("{}.{}", ___________graph_traversal_string, "lowo_na").as_str());
            schema_instance.writtenBlogs.push_str(
                format!("{}.{}", ___________graph_traversal_string, "writtenBlogs").as_str(),
            );
            schema_instance
        }
        pub fn Writes__(&self, clause: surrealdb_macros::Clause) -> Writes {
            Writes::__________connect_to_graph_traversal_string(
                &self.___________graph_traversal_string,
                clause,
                "->",
            )
        }
        pub fn lowo_na(&self, clause: surrealdb_macros::Clause) -> Book {
            Book::__________connect_to_graph_traversal_string(
                &self.___________graph_traversal_string,
                clause,
            )
        }
        pub fn __as__(&self, alias: impl ::std::fmt::Display) -> ::std::string::String {
            format!("{} AS {}", self, alias)
        }
        pub fn __as_writtenBlogs__(&self) -> ::std::string::String {
            format!("{} AS {}", self, "writtenBlogs")
        }
    }
    type Writes = super::WritesSchema<Student>;
    impl Writes {
        pub fn Book(&self, clause: surrealdb_macros::Clause) -> Book {
            Book::__________connect_to_graph_traversal_string(
                &self.___________graph_traversal_string,
                clause,
            )
        }
    }
}
fn test_student_edge_name() {
    ::static_assertions::assert_impl_one!(Book: surrealdb_macros::SurrealdbNode);
    type StudentWritesBookTableName =
        <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::TableNameChecker;
    ::static_assertions::assert_fields!(StudentWritesBookTableName: Writes);
    type StudentWritesBookInNode = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::In;
    ::static_assertions::assert_type_eq_all!(StudentWritesBookInNode, Student);
    type StudentWritesBookOutNode = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Out;
    ::static_assertions::assert_type_eq_all!(StudentWritesBookOutNode, Book);
    ::static_assertions::assert_impl_one!(StudentWritesBook: surrealdb_macros::SurrealdbEdge);
    ::static_assertions::assert_impl_one!(Student: surrealdb_macros::SurrealdbNode);
    ::static_assertions::assert_impl_one!(Book: surrealdb_macros::SurrealdbNode);
    ::static_assertions::assert_type_eq_all!(Relate<Book>, surrealdb_macros::links::Relate<Book>);
}

// ==============================================
// Recursive expansion of the SurrealdbEdge macro
// ==============================================

impl<In: surrealdb_macros::SurrealdbNode, Out: surrealdb_macros::SurrealdbNode>
    surrealdb_macros::SurrealdbEdge for Writes<In, Out>
{
    type In = In;
    type Out = Out;
    type TableNameChecker = writes_schema::TableNameStaticChecker;
    type Schema = writes_schema::Writes<String>;
    fn schema() -> Self::Schema {
        writes_schema::Writes::new()
    }
    fn get_key(&self) -> ::std::option::Option<&String> {
        self.id.as_ref()
    }
}
use writes_schema::Writes as WritesSchema;
pub mod writes_schema {
    pub struct TableNameStaticChecker {
        pub Writes: String,
    }
    #[derive(Debug, ::serde::Serialize, Default)]
    pub struct Writes<Model: ::serde::Serialize + Default> {
        pub id: surrealdb_macros::DbField,
        pub in_: surrealdb_macros::DbField,
        pub out: surrealdb_macros::DbField,
        pub timeWritten: surrealdb_macros::DbField,
        pub ___________graph_traversal_string: ::std::string::String,
        ___________model: ::std::marker::PhantomData<Model>,
    }
    impl<Model: ::serde::Serialize + Default> Writes<Model> {
        pub fn new() -> Self {
            Self {
                id: "id".into(),
                in_: "in".into(),
                out: "out".into(),
                timeWritten: "timeWritten".into(),
                ___________graph_traversal_string: "".into(),
                ___________model: ::std::marker::PhantomData,
            }
        }
        pub fn __________connect_to_graph_traversal_string(
            store: &::std::string::String,
            clause: surrealdb_macros::Clause,
            arrow_direction: &str,
        ) -> Self {
            let mut schema_instance = Self::default();
            let schema_edge_str_with_arrow = format!(
                "{}{}{}{}{}",
                store.as_str(),
                arrow_direction,
                "Writes",
                surrealdb_macros::format_clause(clause, "Writes"),
                arrow_direction,
            );
            schema_instance
                .___________graph_traversal_string
                .push_str(schema_edge_str_with_arrow.as_str());
            let ___________graph_traversal_string = &schema_instance
                .___________graph_traversal_string
                .replace(arrow_direction, "");
            schema_instance
                .id
                .push_str(format!("{}.{}", ___________graph_traversal_string, "id").as_str());
            schema_instance
                .in_
                .push_str(format!("{}.{}", ___________graph_traversal_string, "in").as_str());
            schema_instance
                .out
                .push_str(format!("{}.{}", ___________graph_traversal_string, "out").as_str());
            schema_instance.timeWritten.push_str(
                format!("{}.{}", ___________graph_traversal_string, "timeWritten").as_str(),
            );
            schema_instance
        }
    }
}
fn test_writes_edge_name() {}

// ==============================================
// Recursive expansion of the SurrealdbNode macro
// ==============================================

impl surrealdb_macros::SurrealdbNode for Book {
    type Schema = book::Book;
    fn schema() -> Self::Schema {
        book::Book::new()
    }
    fn get_key(&self) -> ::std::option::Option<&String> {
        self.id.as_ref()
    }
}
pub mod book {
    use ::serde::Serialize;
    #[derive(Debug, Serialize, Default)]
    pub struct Book {
        pub id: surrealdb_macros::DbField,
        pub title: surrealdb_macros::DbField,
        pub(crate) ___________graph_traversal_string: ::std::string::String,
    }
    impl ::std::fmt::Display for Book {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{}", self.___________graph_traversal_string))
        }
    }
    impl Book {
        pub fn new() -> Self {
            Self {
                id: "id".into(),
                title: "title".into(),
                ___________graph_traversal_string: "".to_string(),
            }
        }
        pub fn __with_id__(mut self, id: impl ::std::fmt::Display) -> Self {
            self.___________graph_traversal_string
                .push_str(id.to_string().as_str());
            self
        }
        pub fn __with__(db_name: impl ::std::fmt::Display) -> Self {
            let mut schema_instance = Self::new();
            schema_instance
                .___________graph_traversal_string
                .push_str(db_name.to_string().as_str());
            schema_instance
        }
        pub fn __________connect_to_graph_traversal_string(
            store: &::std::string::String,
            clause: surrealdb_macros::Clause,
        ) -> Self {
            let mut schema_instance = Self::default();
            let connection = format!(
                "{}{}{}",
                store,
                "Book",
                surrealdb_macros::format_clause(clause, "Book")
            );
            schema_instance
                .___________graph_traversal_string
                .push_str(connection.as_str());
            let ___________graph_traversal_string =
                &schema_instance.___________graph_traversal_string;
            schema_instance
                .id
                .push_str(format!("{}.{}", ___________graph_traversal_string, "id").as_str());
            schema_instance
                .title
                .push_str(format!("{}.{}", ___________graph_traversal_string, "title").as_str());
            schema_instance
        }
        pub fn __as__(&self, alias: impl ::std::fmt::Display) -> ::std::string::String {
            format!("{} AS {}", self, alias)
        }
    }
}
fn test_book_edge_name() {}

pub mod bookxx {
    pub struct TableNameStaticChecker {
        pub book: String,
    }
}

trait Mana {
    type TableMameChecker;
}

impl Mana for Book {
    type TableMameChecker = bookxx::TableNameStaticChecker;
}

type StudentWritesBook__In = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::In;
type StudentWritesBook__Out = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Out;

type OutBookTableNameChecker = <StudentWritesBook__Out as Mana>::TableMameChecker;

fn eerer() {
    StudentWritesBook__In::schema()
        .Writes__(Clause::All)
        .Book(Clause::All)
        .title;

    StudentWritesBook__Out::schema().title;
    Writes__::default().olbook(Clause::All).title;
}
::static_assertions::assert_fields!(OutBookTableNameChecker: book);

// type Book = <Book as surrealdb_macros::SurrealdbNode>::Schema;
type Writes__ = <StudentWritesBook as SurrealdbEdge>::Schema;
// <Student>
// type BookMa = <Book as surrealdb_macros::SurrealdbNode>::Schema;
// type BookMa = <StudentWritesBook__Out as surrealdb_macros::SurrealdbNode>::Schema;
type BookMa = <<StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Out as surrealdb_macros::SurrealdbNode>::Schema;

trait WriteOutTrait {
    fn olbook(&self, clause: Clause) -> BookMa;
}

impl WriteOutTrait for Writes__ {
    fn olbook(&self, clause: Clause) -> BookMa {
        // BookMa::__________connect_to_graph_traversal_string
        BookMa::__________connect_to_graph_traversal_string(
            &self.___________graph_traversal_string,
            clause,
        )
    }
}
// ->writes->book
// [
// { type: "StudentWritesBook", action: "writes", direction: "right", foreign: ["book"] },
// { type: "StudentWritesBlog", action: "writes", direction: "right", foreign: ["blog"] },
// { type: "StudentWritesTeacher", action: "writes", direction: "left", foreign: ["teacher"] },
// { type: "StudentBuysCar", action: "buys", direction: "right", foreign: ["car"] },
// ]
//
//
// [
// { type: "StudentWritesBook", action: "writes__", direction: "right", foreign: ["book", "blog"] },
// { type: "StudentWritesTeacher", action: "__writes", direction: "left", foreign: ["teacher"] },
// { type: "StudentBuysCar", action: "buys__", direction: "right", foreign: ["car"] },
// ]
//
// // outgoing connection for Writes, hence Writes__
// type WritesSchema = StudentWritesBook ;
// type Writes__ = <StudentWritesBook as SurrealdbEdge>::Schema;
//
// // Connections to Incoming Writes
// type BookSchema = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Out;
// type Book = <BookSchema as surrealdb_macros::SurrealdbNode>::Schema;
//
// type Blogschema = <StudentWritesBlog as surrealdb_macros::SurrealdbEdge>::Out;
// type Blog = <Blogschema as surrealdb_macros::SurrealdbNode>::Schema;
//
// // Incoming connection for Writes, hence __Writes
// type __Writes = <StudentWritesTeacher as SurrealdbEdge>::Schema;
//
// // Connections to Incoming Writes
// // Note that this uses ::In, instead of ::Out
// type TeacherSchema = <StudentWritesTeacher as surrealdb_macros::SurrealdbEdge>::In;
// type Teacher = <TeacherSchema as surrealdb_macros::SurrealdbNode>::Schema;
//
// // Outgoing connection for Writes, hence Buys__
// type Buys__ = <StudentBuysCar as SurrealdbEdge>::Schema;
//
// // Connections to Outgoing Buys
// type CarSchema = <StudentBuysCar as surrealdb_macros::SurrealdbEdge>::Out;
// type Car = <CarSchema as surrealdb_macros::SurrealdbNode>::Schema;
//
// trait WriteArrowRightTrait {
//     fn book(&self, clause: Clause) -> Book;
//     fn blog(&self, clause: Clause) -> Blog;
// }
//
// impl WriteArrowRightTrait for Writes__ {
//     fn book(&self, clause: Clause) -> Book {
//         Book::__________connect_to_graph_traversal_string(
//             &self.___________graph_traversal_string,
//             clause,
//         )
//     }
//
//     fn blog(&self, clause: Clause) -> Blog {
//         Blog::__________connect_to_graph_traversal_string(
//             &self.___________graph_traversal_string,
//             clause,
//         )
//     }
// }
//
//
// trait WriteArrowLeftTrait {
//     fn teacher(&self, clause: Clause) -> Teacher;
// }
//
// impl WriteArrowLeftTrait for __Writes {
//     fn teacher(&self, clause: Clause) -> Teacher {
//         Teacher::__________connect_to_graph_traversal_string(
//             &self.___________graph_traversal_string,
//             clause,
//         )
//     }
// }
//
// trait BuysArrowRightTrait {
//     fn car(&self, clause: Clause) -> Car;
// }
//
// impl BuysArrowRightTrait for Buys__ {
//     fn book(&self, clause: Clause) -> Book {
//         Book::__________connect_to_graph_traversal_string(
//             &self.___________graph_traversal_string,
//             clause,
//         )
//     }
//
//     fn car(&self, clause: Clause) -> Car {
//         Car::__________connect_to_graph_traversal_string(
//             &self.___________graph_traversal_string,
//             clause,
//         )
//     }
// }

type StudentWritesBookTableNameChecker = <StudentWritesBook as SurrealdbEdge>::TableNameChecker;

::static_assertions::assert_fields!(StudentWritesBookTableNameChecker: Writes);
::static_assertions::assert_type_eq_all!(StudentWritesBook__In, Student);
::static_assertions::assert_type_eq_all!(StudentWritesBook__Out, Book);
::static_assertions::assert_type_eq_all!(Relate<StudentWritesBook__Out>, Relate<Book>);

type BookTableNameChecker = <Book as Mana>::TableMameChecker;
::static_assertions::assert_fields!(BookTableNameChecker: book);
fn fdfd() {}
