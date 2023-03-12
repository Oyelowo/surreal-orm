/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
// #![allow(dead_code)]
// #![allow(non_upper_case_globals)]
// #![allow(non_snake_case)]
// #![allow(non_camel_case_types)]
// #![allow(unused_imports)]
//
// use serde::{Deserialize, Serialize};
// use static_assertions::*;
// use surrealdb::{
//     engine::local::{Db, Mem},
//     opt::IntoResource,
//     sql::Id,
//     Result, Surreal,
// };
// use surrealdb_derive::{SurrealdbEdge, SurrealdbNode};
//
// use std::fmt::{Debug, Display};
// use surrealdb_macros::{
//     links::{LinkMany, LinkOne, LinkSelf, Relate},
//     model_id::SurId,
//     query_builder_old::query,
//     SurrealdbEdge, /* SurrealdbEdge, */ SurrealdbNode,
// };
// use typed_builder::TypedBuilder;
//
// #[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Student {
//     // #[serde(skip_serializing_if = "Option::is_none")]
//     // #[builder(default, setter(strip_option))]
//     id: Option<SurId>,
//     first_name: String,
//     last_name: String,
//     // #[serde(rename = "lowo_na")]
//     // fav_book: LinkOne<Book>,
//     // // #[surrealdb(link_one = "Book", skip_serializing)]
//     // course: LinkOne<Book>,
//     // #[surrealdb(link_many = "Book", skip_serializing)]
//     // #[serde(rename = "lowo")]
//     // all_semester_courses: LinkMany<Book>,
//     // written_blogs: Relate<Book>,
// }
//
// // #[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone)]
// #[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Writes<In: SurrealdbNode, Out: SurrealdbNode> {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[builder(default, setter(strip_option))]
//     id: Option<SurId>,
//
//     // #[surrealdb(link_one = "Book", skip_serializing)]
//     r#in: In,
//     out: Out,
//     time_written: String,
// }
//
// type StudentWritesBook = Writes<Student, Book>;
//
// #[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Book {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[builder(default, setter(strip_option))]
//     id: Option<SurId>,
//     title: String,
// }
//
// type StudentWritesBlog = Writes<Student, Blog>;
// #[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Blog {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[builder(default, setter(strip_option))]
//     id: Option<SurId>,
//     content: String,
// }
//
// // pub fn format_clause(clause: Clause, table_name: &'static str) -> String {
// //     match clause {
// //         Clause::All => "".into(),
// //         Clause::Where(filter) => {
// //             let filter = filter.to_string();
// //             format!("[WHERE {filter}]")
// //         }
// //         Clause::Id(id) => {
// //             if !id
// //                 .to_string()
// //                 .starts_with(format!("{table_name}:").as_str())
// //             {
// //                 panic!("invalid id {id}. Id does not belong to table {table_name}")
// //             }
// //             format!("[WHERE id = {id}]")
// //         }
// //     }
// // }
// // ==============================================
// // Recursive expansion of the SurrealdbNode macro
// // ==============================================
//
// impl surrealdb_macros::SurrealdbNode for Student {
//     type Schema = student::Student;
//     fn schema() -> Self::Schema {
//         student::Student::new()
//     }
//     fn get_key(&self) -> ::std::option::Option<&SurId> {
//         self.id.as_ref()
//     }
//
//     fn get_table_name() -> &'static str {
//         "student"
//     }
//
//     type TableNameChecker = student::TableNameStaticChecker;
// }
// pub mod student {
//     pub struct TableNameStaticChecker {
//         pub student: String,
//     }
//
//     use ::serde::Serialize;
//     use surrealdb_macros::SurrealdbEdge;
//     type Book = <super::Book as surrealdb_macros::SurrealdbNode>::Schema;
//     #[derive(Debug, Serialize, Default)]
//     pub struct Student {
//         pub id: surrealdb_macros::DbField,
//         pub firstName: surrealdb_macros::DbField,
//         pub lastName: surrealdb_macros::DbField,
//         pub lowo_na: surrealdb_macros::DbField,
//         pub writtenBlogs: surrealdb_macros::DbField,
//         pub(crate) ___________graph_traversal_string: ::std::string::String,
//     }
//     impl ::std::fmt::Display for Student {
//         fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
//             f.write_fmt(format_args!("{}", self.___________graph_traversal_string))
//         }
//     }
//     impl Student {
//         pub fn new() -> Self {
//             Self {
//                 id: "id".into(),
//                 firstName: "firstName".into(),
//                 lastName: "lastName".into(),
//                 lowo_na: "lowo_na".into(),
//                 writtenBlogs: "writtenBlogs".into(),
//                 ___________graph_traversal_string: "".to_string(),
//             }
//         }
//         pub fn __with_id__(mut self, id: impl ::std::fmt::Display) -> Self {
//             self.___________graph_traversal_string
//                 .push_str(id.to_string().as_str());
//             self
//         }
//         pub fn __with__(db_name: impl ::std::fmt::Display) -> Self {
//             let mut schema_instance = Self::new();
//             schema_instance
//                 .___________graph_traversal_string
//                 .push_str(db_name.to_string().as_str());
//             schema_instance
//         }
//         pub fn __________connect_to_graph_traversal_string(
//             store: &::std::string::String,
//             clause: surrealdb_macros::Clause,
//         ) -> Self {
//             let mut schema_instance = Self::default();
//             let connection = format!(
//                 "{}{}{}",
//                 store,
//                 "Student",
//                 surrealdb_macros::format_clause(clause, "Student")
//             );
//             schema_instance
//                 .___________graph_traversal_string
//                 .push_str(connection.as_str());
//             let ___________graph_traversal_string =
//                 &schema_instance.___________graph_traversal_string;
//             schema_instance
//                 .id
//                 .push_str(format!("{}.{}", ___________graph_traversal_string, "id").as_str());
//             schema_instance.firstName.push_str(
//                 format!("{}.{}", ___________graph_traversal_string, "firstName").as_str(),
//             );
//             schema_instance
//                 .lastName
//                 .push_str(format!("{}.{}", ___________graph_traversal_string, "lastName").as_str());
//             schema_instance
//                 .lowo_na
//                 .push_str(format!("{}.{}", ___________graph_traversal_string, "lowo_na").as_str());
//             schema_instance.writtenBlogs.push_str(
//                 format!("{}.{}", ___________graph_traversal_string, "writtenBlogs").as_str(),
//             );
//             schema_instance
//         }
//
//         pub fn lowo_na(&self, clause: surrealdb_macros::Clause) -> Book {
//             Book::__________connect_to_graph_traversal_string(
//                 &self.___________graph_traversal_string,
//                 clause,
//             )
//         }
//         pub fn __as__(&self, alias: impl ::std::fmt::Display) -> ::std::string::String {
//             format!("{} AS {}", self, alias)
//         }
//         pub fn __as_writtenBlogs__(&self) -> ::std::string::String {
//             format!("{} AS {}", self, "writtenBlogs")
//         }
//     }
//
//     impl Student {
//         pub fn writes__(
//             &self,
//             clause: surrealdb_macros::Clause,
//         ) -> _____________writes_outgoing_schema::Writes__ {
//             _____________writes_outgoing_schema::Writes::__________connect_to_graph_traversal_string(
//                 &self.___________graph_traversal_string,
//                 clause,
//                 "->",
//             ).into()
//         }
//     }
//
//     impl Student {
//         pub fn writes_test__(
//             &self,
//             clause: surrealdb_macros::Clause,
//         ) -> _____________writes_outgoing_schema::Writes {
//             _____________writes_outgoing_schema::Writes::__________connect_to_graph_traversal_string(
//                 &self.___________graph_traversal_string,
//                 clause,
//                 "->",
//             )
//         }
//     }
//
//     mod _____________writes_outgoing_schema {
//         use std::ops::Deref;
//
//         type Student = <super::super::StudentWritesBook as surrealdb_macros::SurrealdbEdge>::In;
//         type BookModel = <super::super::StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Out;
//         type BookTableNameChecker =
//             <BookModel as surrealdb_macros::SurrealdbNode>::TableNameChecker;
//
//         // assert destination node name is table name of `book`
//         ::static_assertions::assert_fields!(BookTableNameChecker: book);
//         ::static_assertions::assert_impl_one!(BookModel: surrealdb_macros::SurrealdbNode);
//
//         type Book = <BookModel as surrealdb_macros::SurrealdbNode>::Schema;
//         // type Writes = super::WritesSchema<super::Student, super::Book>;
//         pub type Writes =
//             <super::super::StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Schema;
//         // impl Writes {
//         //     pub fn Book(&self, clause: surrealdb_macros::Clause) -> Book {
//         //         Book::__________connect_to_graph_traversal_string(
//         //             &self.___________graph_traversal_string,
//         //             clause,
//         //         )
//         //     }
//         // }
//         // trait Write__Trait {
//         //     // fn olbook(&self, clause: Clause) -> BookMa;
//         //     fn Book(&self, clause: surrealdb_macros::Clause) -> Book;
//         // }
//
//         pub struct Writes__(Writes);
//
//         impl From<Writes> for Writes__ {
//             fn from(value: Writes) -> Self {
//                 Self(value)
//             }
//         }
//
//         impl ::std::ops::Deref for Writes__ {
//             type Target = Writes;
//
//             fn deref(&self) -> &Self::Target {
//                 &self.0
//             }
//         }
//
//         // impl Write__Trait for Writes__ {
//         impl Writes__ {
//             pub fn banana(&self, clause: surrealdb_macros::Clause) -> Book {
//                 Book::__________connect_to_graph_traversal_string(
//                     &self.___________graph_traversal_string,
//                     clause,
//                 )
//             }
//
//             pub fn olbook(&self, clause: surrealdb_macros::Clause) -> Book {
//                 Book::__________connect_to_graph_traversal_string(
//                     &self.___________graph_traversal_string,
//                     clause,
//                 )
//             }
//         }
//     }
// }
// fn eerer() {
//     // StudentWritesBook__In::schema()
//     //     .writes__(Clause::All)
//     //     .olbook(Clause::All)
//     //     .title;
//     //
//     // StudentWritesBook__Out::schema().title;
//     // Writes__::new().olbook(Clause::All).title;
//
//     Student::schema()
//         .writes__(Clause::All)
//         .banana(Clause::All)
//         .title;
//
//     Student::schema()
//         .writes__(Clause::All)
//         .banana(Clause::All)
//         .id;
//     // Student::schema()
//     //     .writes_test__(Clause::All)
//     //     .olbook(Clause::All)
//     //     .title;
// }
// fn test_student_edge_name() {
//     ::static_assertions::assert_impl_one!(Book: surrealdb_macros::SurrealdbNode);
//     type StudentWritesBookTableName =
//         <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::TableNameChecker;
//     ::static_assertions::assert_fields!(StudentWritesBookTableName: Writes);
//     type StudentWritesBookInNode = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::In;
//     ::static_assertions::assert_type_eq_all!(StudentWritesBookInNode, Student);
//     type StudentWritesBookOutNode = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Out;
//     ::static_assertions::assert_type_eq_all!(StudentWritesBookOutNode, Book);
//     ::static_assertions::assert_impl_one!(StudentWritesBook: surrealdb_macros::SurrealdbEdge);
//     ::static_assertions::assert_impl_one!(Student: surrealdb_macros::SurrealdbNode);
//     ::static_assertions::assert_impl_one!(Book: surrealdb_macros::SurrealdbNode);
//     ::static_assertions::assert_type_eq_all!(Relate<Book>, surrealdb_macros::links::Relate<Book>);
// }
//
// // ==============================================
// // Recursive expansion of the SurrealdbEdge macro
// // ==============================================
//
// impl<In: surrealdb_macros::SurrealdbNode, Out: surrealdb_macros::SurrealdbNode>
//     surrealdb_macros::SurrealdbEdge for Writes<In, Out>
// {
//     type In = In;
//     type Out = Out;
//     type TableNameChecker = writes_schema::TableNameStaticChecker;
//     type Schema = writes_schema::Writes;
//     fn schema() -> Self::Schema {
//         writes_schema::Writes::new()
//     }
//     fn get_key(&self) -> ::std::option::Option<&SurId> {
//         self.id.as_ref()
//     }
//
//     fn get_table_name() -> &'static str {
//         "writes"
//     }
// }
// pub mod writes_schema {
//     use surrealdb_macros::SurrealdbNode;
//
//     pub struct TableNameStaticChecker {
//         pub Writes: String,
//     }
//     #[derive(Debug, ::serde::Serialize, Default)]
//     pub struct Writes {
//         pub id: surrealdb_macros::DbField,
//         pub in_: surrealdb_macros::DbField,
//         pub out: surrealdb_macros::DbField,
//         pub timeWritten: surrealdb_macros::DbField,
//         pub ___________graph_traversal_string: ::std::string::String,
//     }
//     impl Writes {
//         pub fn empty() -> Self {
//             Self {
//                 id: "".into(),
//                 in_: "".into(),
//                 out: "".into(),
//                 timeWritten: "".into(),
//                 ___________graph_traversal_string: "".into(),
//             }
//         }
//         pub fn new() -> Self {
//             Self {
//                 id: "id".into(),
//                 in_: "in".into(),
//                 out: "out".into(),
//                 timeWritten: "timeWritten".into(),
//                 ___________graph_traversal_string: "".into(),
//             }
//         }
//         pub fn __________connect_to_graph_traversal_string(
//             store: &::std::string::String,
//             clause: surrealdb_macros::Clause,
//             arrow_direction: &str,
//         ) -> Self {
//             let mut schema_instance = Self::empty();
//             let schema_edge_str_with_arrow = format!(
//                 "{}{}{}{}{}",
//                 store.as_str(),
//                 arrow_direction,
//                 "Writes",
//                 surrealdb_macros::format_clause(clause, "Writes"),
//                 arrow_direction,
//             );
//
//             schema_instance
//                 .___________graph_traversal_string
//                 .push_str(schema_edge_str_with_arrow.as_str());
//             let ___________graph_traversal_string = &schema_instance
//                 .___________graph_traversal_string
//                 .replace(arrow_direction, "");
//             // schema_instance.id = "".into();
//             // schema_instance.in_ = "".into();
//             // schema_instance.timeWritten = "".into();
//             schema_instance
//                 .id
//                 .push_str(format!("{}.{}", ___________graph_traversal_string, "id").as_str());
//             schema_instance
//                 .in_
//                 .push_str(format!("{}.{}", ___________graph_traversal_string, "in").as_str());
//             schema_instance
//                 .out
//                 .push_str(format!("{}.{}", ___________graph_traversal_string, "out").as_str());
//             schema_instance.timeWritten.push_str(
//                 format!("{}.{}", ___________graph_traversal_string, "timeWritten").as_str(),
//             );
//             schema_instance
//         }
//     }
// }
// fn test_writes_edge_name() {}
//
// // ==============================================
// // Recursive expansion of the SurrealdbNode macro
// // ==============================================
//
// impl surrealdb_macros::SurrealdbNode for Book {
//     type Schema = book::Book;
//     fn schema() -> Self::Schema {
//         book::Book::new()
//     }
//     fn get_key(&self) -> ::std::option::Option<&SurId> {
//         self.id.as_ref()
//     }
//     fn get_table_name() -> &'static str {
//         "book"
//     }
//
//     type TableNameChecker = book::TableNameStaticChecker;
// }
// pub mod book {
//     pub struct TableNameStaticChecker {
//         pub book: String,
//     }
//     use ::serde::Serialize;
//     #[derive(Debug, Serialize, Default)]
//     pub struct Book {
//         pub id: surrealdb_macros::DbField,
//         pub title: surrealdb_macros::DbField,
//         pub(crate) ___________graph_traversal_string: ::std::string::String,
//     }
//     impl ::std::fmt::Display for Book {
//         fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
//             f.write_fmt(format_args!("{}", self.___________graph_traversal_string))
//         }
//     }
//     impl Book {
//         pub fn new() -> Self {
//             Self {
//                 id: "id".into(),
//                 title: "title".into(),
//                 ___________graph_traversal_string: "".to_string(),
//             }
//         }
//         pub fn __with_id__(mut self, id: impl ::std::fmt::Display) -> Self {
//             self.___________graph_traversal_string
//                 .push_str(id.to_string().as_str());
//             self
//         }
//         pub fn __with__(db_name: impl ::std::fmt::Display) -> Self {
//             let mut schema_instance = Self::new();
//             schema_instance
//                 .___________graph_traversal_string
//                 .push_str(db_name.to_string().as_str());
//             schema_instance
//         }
//         pub fn __________connect_to_graph_traversal_string(
//             store: &::std::string::String,
//             clause: surrealdb_macros::Clause,
//         ) -> Self {
//             let mut schema_instance = Self::default();
//             let connection = format!(
//                 "{}{}{}",
//                 store,
//                 "Book",
//                 surrealdb_macros::format_clause(clause, "Book")
//             );
//             schema_instance
//                 .___________graph_traversal_string
//                 .push_str(connection.as_str());
//             let ___________graph_traversal_string =
//                 &schema_instance.___________graph_traversal_string;
//             schema_instance
//                 .id
//                 .push_str(format!("{}.{}", ___________graph_traversal_string, "id").as_str());
//             schema_instance
//                 .title
//                 .push_str(format!("{}.{}", ___________graph_traversal_string, "title").as_str());
//             schema_instance
//         }
//         pub fn __as__(&self, alias: impl ::std::fmt::Display) -> ::std::string::String {
//             format!("{} AS {}", self, alias)
//         }
//     }
// }
// fn test_book_edge_name() {}
//
// impl surrealdb_macros::SurrealdbNode for Blog {
//     type Schema = blog::Blog;
//     fn schema() -> Self::Schema {
//         blog::Blog::new()
//     }
//     fn get_key(&self) -> ::std::option::Option<&SurId> {
//         self.id.as_ref()
//     }
//     fn get_table_name() -> &'static str {
//         "blog"
//     }
//
//     type TableNameChecker = blog::TableNameStaticChecker;
// }
// pub mod blog {
//     pub struct TableNameStaticChecker {
//         pub blog: String,
//     }
//     use ::serde::Serialize;
//     #[derive(Debug, Serialize, Default)]
//     pub struct Blog {
//         pub id: surrealdb_macros::DbField,
//         pub content: surrealdb_macros::DbField,
//         pub(crate) ___________graph_traversal_string: ::std::string::String,
//     }
//     impl ::std::fmt::Display for Blog {
//         fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
//             f.write_fmt(format_args!("{}", self.___________graph_traversal_string))
//         }
//     }
//     impl Blog {
//         pub fn new() -> Self {
//             Self {
//                 id: "id".into(),
//                 content: "content".into(),
//                 ___________graph_traversal_string: "".to_string(),
//             }
//         }
//         pub fn __with_id__(mut self, id: impl ::std::fmt::Display) -> Self {
//             self.___________graph_traversal_string
//                 .push_str(id.to_string().as_str());
//             self
//         }
//         pub fn __with__(db_name: impl ::std::fmt::Display) -> Self {
//             let mut schema_instance = Self::new();
//             schema_instance
//                 .___________graph_traversal_string
//                 .push_str(db_name.to_string().as_str());
//             schema_instance
//         }
//         pub fn __________connect_to_graph_traversal_string(
//             store: &::std::string::String,
//             clause: surrealdb_macros::Clause,
//         ) -> Self {
//             let mut schema_instance = Self::default();
//             let connection = format!(
//                 "{}{}{}",
//                 store,
//                 "Blog",
//                 surrealdb_macros::format_clause(clause, "Blog")
//             );
//             schema_instance
//                 .___________graph_traversal_string
//                 .push_str(connection.as_str());
//             let ___________graph_traversal_string =
//                 &schema_instance.___________graph_traversal_string;
//             schema_instance
//                 .id
//                 .push_str(format!("{}.{}", ___________graph_traversal_string, "id").as_str());
//             schema_instance
//                 .content
//                 .push_str(format!("{}.{}", ___________graph_traversal_string, "content").as_str());
//             schema_instance
//         }
//         pub fn __as__(&self, alias: impl ::std::fmt::Display) -> ::std::string::String {
//             format!("{} AS {}", self, alias)
//         }
//     }
// }
// pub mod bookxx {
//     pub struct TableNameStaticChecker {
//         pub blog: String,
//     }
// }
//
// trait Mana {
//     type TableMameChecker;
// }
//
// impl Mana for Blog {
//     type TableMameChecker = bookxx::TableNameStaticChecker;
// }
//
// type StudentWritesBook__In = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::In;
// type StudentWritesBook__Out = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Out;
//
// type OutBookTableNameChecker = <StudentWritesBook__Out as Mana>::TableMameChecker;
//
// mod xxx {
//
//     // super::Student
//     // super::Student::schema()
//     //     .writes__(Clause::All)
//     //     .olbook(Clause::All)
//     //     .title;
// }
//
// fn eere() {
//     StudentWritesBook__In::schema()
//         .writes__(Clause::All)
//         .olbook(Clause::All)
//         .title;
// }
// // ::static_assertions::assert_fields!(OutBookTableNameChecker: book);
//
// // type Blog = <Book as surrealdb_macros::SurrealdbNode>::Schema;
// pub type Writes__ = <StudentWritesBook as SurrealdbEdge>::Schema;
// // <Student>
// // type BookMa = <Book as surrealdb_macros::SurrealdbNode>::Schema;
// // type BookMa = <StudentWritesBook__Out as surrealdb_macros::SurrealdbNode>::Schema;
// type BookMa = <<StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Out as surrealdb_macros::SurrealdbNode>::Schema;
// mod xana {
//     use super::Student;
//     use super::SurrealdbNode;
//
//     // Student;
// }
// // use nama::*;
// // pub mod nama {
// //     use super::*;
// //
// //     pub trait WriteOutTrait {
// //         fn olbook(&self, clause: Clause) -> BookMa;
// //     }
// //
// //     struct Komo(Writes__);
// //     // impl WriteOutTrait for Writes__ {
// //     impl Komo {
// //         fn olbook(&self, clause: Clause) -> BookMa {
// //             // BookMa::__________connect_to_graph_traversal_string
// //             BookMa::__________connect_to_graph_traversal_string(
// //                 &self.0.___________graph_traversal_string,
// //                 clause,
// //             )
// //         }
// //     }
// // }
// // mod xx {
// //     use super::*;
// pub type Writes2__ = <StudentWritesBlog as SurrealdbEdge>::Schema;
// trait WriteOutTrait2 {
//     fn olbook(&self, clause: Clause) -> BookMa;
// }
//
// impl WriteOutTrait2 for Writes2__ {
//     fn olbook(&self, clause: Clause) -> BookMa {
//         // BookMa::__________connect_to_graph_traversal_string
//         BookMa::__________connect_to_graph_traversal_string(
//             &self.___________graph_traversal_string,
//             clause,
//         )
//     }
// }
// // }
// // ->writes->book
// // [
// // { type: "StudentWritesBook", action: "writes", direction: "right", foreign: ["book"] },
// // { type: "StudentWritesBlog", action: "writes", direction: "right", foreign: ["blog"] },
// // { type: "StudentWritesTeacher", action: "writes", direction: "left", foreign: ["teacher"] },
// // { type: "StudentBuysCar", action: "buys", direction: "right", foreign: ["car"] },
// // ]
// //
// //
// // [
// // { type: "StudentWritesBook", action: "writes__", direction: "right", foreign: ["book", "blog"] },
// // { type: "StudentWritesTeacher", action: "__writes", direction: "left", foreign: ["teacher"] },
// // { type: "StudentBuysCar", action: "buys__", direction: "right", foreign: ["car"] },
// // ]
// //
// // {
// // "Writes__": {
// //   type: "StudentWritesBook",
// //   action: "writes",
// //   direction: "right",
// //   action_type_alias: quote!( type Writes__ = <StudentWritesBook as #crate_name::SurrealdbEdge>::Schema; ),
// //   foreign_node_schema: vec![
// //       quote!(
// //         type BookModel = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Out;
// //         type Book = <BookModel as surrealdb_macros::SurrealdbNode>::Schema;
// //       ),
// //       quote!(
// //         type BlogModel = <StudentWritesBlog as surrealdb_macros::SurrealdbEdge>::Out;
// //         type Blog = <BlogModel as surrealdb_macros::SurrealdbNode>::Schema;
// //       ),
// //   ],
// //   edge_to_nodes_trait_methods: vec![
// //       quote!(
// //          fn book(&self, clause: Clause) -> Book;
// //       ),
// //       quote!(
// //          fn blog(&self, clause: Clause) -> Blog;
// //       ),
// //   ],
// //   edge_to_nodes_trait_methods_impl: vec![
// //       quote!(
// //          fn book(&self, clause: Clause) -> Book {
// //              Book::__________connect_to_graph_traversal_string(
// //                  &self.___________graph_traversal_string,
// //                  clause,
// //              )
// //          }
// //       ),
// //       quote!(
// //          fn blog(&self, clause: Clause) -> Blog {
// //              Blog::__________connect_to_graph_traversal_string(
// //                  &self.___________graph_traversal_string,
// //                  clause,
// //              )
// //          }
// //       ),
// //
// //   ],
// // },
// // "__Writes": {
// //   type: "StudentWritesTeacher",
// //   action: "writes",
// //   direction: "left",
// //   action_type_alias: quote!( type __Writes = <StudentWritesTeacher as #crate_name::SurrealdbEdge>::Schema; ),
// //   foreign_node_schema: vec![
// //       quote!(
// //         type TeacherModel = <StudentWritesTeacher as surrealdb_macros::SurrealdbEdge>::In;
// //         type Teacher = <TeacherSchema as surrealdb_macros::SurrealdbNode>::Schema;
// //       ),
// //   ],
// //   edge_to_nodes_trait_methods: vec![
// //       quote!(
// //          fn teacher(&self, clause: Clause) -> Teacher
// //       ),
// //   ],
// //   edge_to_nodes_trait_methods_impl: vec![
// //       quote!(
// //          fn teacher(&self, clause: Clause) -> Teacher {
// //              Teacher::__________connect_to_graph_traversal_string(
// //                  &self.___________graph_traversal_string,
// //                  clause,
// //              )
// //          }
// //       ),
// //   ],
// // },
// // "Buys__": {
// //   type: "StudentBuysCar ",
// //   action: "buys",
// //   direction: "right",
// //   action_type_alias: quote!( type Buys__ = <StudentBuysCar as SurrealdbEdge>::Schema; ),
// //   foreign_node_schema: vec![
// //       quote!(
// //         type CarModel = <StudentBuysCar as surrealdb_macros::SurrealdbEdge>::Out;
// //         type Car = <CarSchema as surrealdb_macros::SurrealdbNode>::Schema;
// //       ),
// //   ],
// //   edge_to_nodes_trait_methods: vec![
// //       quote!(
// //          fn car(&self, clause: Clause) -> Car;
// //       ),
// //   ],
// //   edge_to_nodes_trait_methods_impl: vec![
// //       quote!(
// //          fn car(&self, clause: Clause) -> Car {
// //              Car::__________connect_to_graph_traversal_string(
// //                  &self.___________graph_traversal_string,
// //                  clause,
// //              )
// //          }
// //       ),
// //   ],
// //   direction: "right", foreign: ["book", "blog"] },
// // { type: "StudentWritesTeacher", action: "__writes", direction: "left", foreign: ["teacher"] },
// // { type: "StudentBuysCar", action: "buys__", direction: "right", foreign: ["car"] },
// // }
// //
// // // outgoing connection for Writes, hence Writes__
// // type WritesSchema = StudentWritesBook ;
// // type Writes__ = <StudentWritesBook as SurrealdbEdge>::Schema;
// //
// // // Connections to Incoming Writes
// // type BookModel = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Out;
// // type Book = <BookModel as surrealdb_macros::SurrealdbNode>::Schema;
// //
// // type BlogModel = <StudentWritesBlog as surrealdb_macros::SurrealdbEdge>::Out;
// // type Blog = <BlogModel as surrealdb_macros::SurrealdbNode>::Schema;
// //
// // // Incoming connection for Writes, hence __Writes
// // type __Writes = <StudentWritesTeacher as SurrealdbEdge>::Schema;
// //
// // // Connections to Incoming Writes
// // // Note that this uses ::In, instead of ::Out
// // type TeacherModel = <StudentWritesTeacher as surrealdb_macros::SurrealdbEdge>::In;
// // type Teacher = <TeacherSchema as surrealdb_macros::SurrealdbNode>::Schema;
// //
// // // Outgoing connection for Writes, hence Buys__
// // type Buys__ = <StudentBuysCar as SurrealdbEdge>::Schema;
// //
// // // Connections to Outgoing Buys
// // type CarModel = <StudentBuysCar as surrealdb_macros::SurrealdbEdge>::Out;
// // type Car = <CarSchema as surrealdb_macros::SurrealdbNode>::Schema;
// //
// // trait WriteArrowRightTrait {
// //     fn book(&self, clause: Clause) -> Book;
// //     fn blog(&self, clause: Clause) -> Blog;
// // }
// //
// // impl WriteArrowRightTrait for Writes__ {
// //     fn book(&self, clause: Clause) -> Book {
// //         Book::__________connect_to_graph_traversal_string(
// //             &self.___________graph_traversal_string,
// //             clause,
// //         )
// //     }
// //
// //     fn blog(&self, clause: Clause) -> Blog {
// //         Blog::__________connect_to_graph_traversal_string(
// //             &self.___________graph_traversal_string,
// //             clause,
// //         )
// //     }
// // }
// //
// //
// // trait WriteArrowLeftTrait {
// //     fn teacher(&self, clause: Clause) -> Teacher;
// // }
// //
// // impl WriteArrowLeftTrait for __Writes {
// //     fn teacher(&self, clause: Clause) -> Teacher {
// //         Teacher::__________connect_to_graph_traversal_string(
// //             &self.___________graph_traversal_string,
// //             clause,
// //         )
// //     }
// // }
// //
// // trait BuysArrowRightTrait {
// //     fn car(&self, clause: Clause) -> Car;
// // }
// //
// // impl BuysArrowRightTrait for Buys__ {
// //     fn car(&self, clause: Clause) -> Car {
// //         Car::__________connect_to_graph_traversal_string(
// //             &self.___________graph_traversal_string,
// //             clause,
// //         )
// //     }
// // }
//
// type StudentWritesBookTableNameChecker = <StudentWritesBook as SurrealdbEdge>::TableNameChecker;
//
// ::static_assertions::assert_fields!(StudentWritesBookTableNameChecker: Writes);
// ::static_assertions::assert_type_eq_all!(StudentWritesBook__In, Student);
// ::static_assertions::assert_type_eq_all!(StudentWritesBook__Out, Book);
// ::static_assertions::assert_type_eq_all!(Relate<StudentWritesBook__Out>, Relate<Book>);
//
// type BookTableNameChecker = <Book as SurrealdbNode>::TableNameChecker;
// ::static_assertions::assert_fields!(BookTableNameChecker: book);
// fn fdfd() {}
