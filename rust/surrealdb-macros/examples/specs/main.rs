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
    db_field::{cond, empty, Parametric},
    links::{LinkMany, LinkOne, LinkSelf, Reference, Relate},
    value_type_wrappers::SurrealId,
    RecordId, SurrealdbEdge, SurrealdbNode,
};
use typed_builder::TypedBuilder;

// ::static_assertions::assert_impl_one!()
// #[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone /* , Default */)]
#[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone /* , Default */)]
#[serde(rename_all = "camelCase")]
// #[surrealdb(table_name = "student")]
pub struct Student {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<RecordId>,
    first_name: String,
    last_name: String,

    // #[surrealdb(link_self = "Student")]
    best_class_mate: LinkSelf<Student>,

    // #[surrealdb(link_one = "Book")]
    #[serde(rename = "lowo_na")]
    fav_book: LinkOne<Book>,

    // #[surrealdb(link_one = "Blog")]
    course: LinkOne<Blog>,

    // #[surrealdb(link_many = "Book")]
    #[serde(rename = "lowo")]
    all_semester_courses: LinkMany<Book>,

    // #[surrealdb(relate(model = "StudentWritesBook", connection = "->rites->book"))]
    written_blogs: Relate<Book>,
}

// #[derive(SurrealdbEdge, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
// #[surrealdb(table_name = "rites", relax_table_name)]
pub struct Writes<In: SurrealdbNode, Out: SurrealdbNode> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<RecordId>,

    // #[surrealdb(link_one = "Book", skip_serializing)]
    r#in: In,
    out: Out,
    time_written: String,
}

type StudentWritesBook = Writes<Student, Book>;

// #[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
#[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
// #[surrealdb(table_name = "book")]
pub struct Book {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<RecordId>,
    title: String,
}

// #[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
#[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
// #[surrealdb(table_name = "blog")]
pub struct Blog {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<RecordId>,
    content: String,
}

// ==============================================
// Recursive expansion of the SurrealdbNode macro
// ==============================================

impl surrealdb_macros::SurrealdbModel for Student {
    fn table_name() -> ::surrealdb::sql::Table {
        "student".into()
    }

    fn get_serializable_field_names() -> Vec<&'static str> {
        todo!()
    }
}
impl surrealdb_macros::SurrealdbNode for Student {
    type TableNameChecker = student::TableNameStaticChecker;
    type Schema = student::Student;
    fn schema() -> Self::Schema {
        student::Student::new()
    }

    fn get_table_name() -> ::surrealdb::sql::Table {
        "student".into()
    }

    fn get_key<T: From<surrealdb_macros::RecordId>>(self) -> ::std::option::Option<T> {
        let record_id = self
            .id
            .map(|id| surrealdb_macros::RecordId::from(id).into());
        record_id
    }
}
pub mod student {
    use ::serde::Serialize;
    use surrealdb_macros::Parametric as _;
    struct Parametric {}
    pub struct TableNameStaticChecker {
        pub student: String,
    }
    type Book = <super::Book as surrealdb_macros::SurrealdbNode>::Schema;
    #[derive(Debug)]
    pub struct Student {
        pub id: surrealdb_macros::DbField,
        pub firstName: surrealdb_macros::DbField,
        pub lastName: surrealdb_macros::DbField,
        pub age: surrealdb_macros::DbField,
        pub bestFriend: surrealdb_macros::DbField,
        pub unoBook: surrealdb_macros::DbField,
        pub course: surrealdb_macros::DbField,
        pub semCoures: surrealdb_macros::DbField,
        pub writtenBooks: surrealdb_macros::DbField,
        pub(crate) ___________graph_traversal_string: ::std::string::String,
        ___________bindings: surrealdb_macros::BindingsList,
    }

    impl surrealdb_macros::Parametric for Student {
        fn get_bindings(&self) -> surrealdb_macros::db_field::BindingsList {
            self.___________bindings.to_vec()
        }
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
                age: "age".into(),
                bestFriend: "bestFriend".into(),
                unoBook: "unoBook".into(),
                course: "course".into(),
                semCoures: "semCoures".into(),
                writtenBooks: "writtenBooks".into(),
                ___________graph_traversal_string: "".into(),
                ___________bindings: vec![],
            }
        }
        pub fn empty() -> Self {
            Self {
                id: "".into(),
                firstName: "".into(),
                lastName: "".into(),
                age: "".into(),
                bestFriend: "".into(),
                unoBook: "".into(),
                course: "".into(),
                semCoures: "".into(),
                writtenBooks: "".into(),
                ___________graph_traversal_string: "".into(),
                ___________bindings: vec![],
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
            filter: impl Into<surrealdb_macros::DbFilter>,
            existing_bindings: surrealdb_macros::BindingsList,
        ) -> Self {
            use surrealdb_macros::Parametric as _;
            let mut schema_instance = Self::empty();
            let filter: surrealdb_macros::DbFilter = filter.into();
            let bindings = [&existing_bindings[..], &filter.get_bindings()[..]].concat();
            let bindings = bindings.as_slice();

            schema_instance.___________bindings = bindings.into();

            let connection = format!(
                "{}{}{}",
                store,
                "student",
                surrealdb_macros::format_filter(filter)
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
                .age
                .push_str(format!("{}.{}", ___________graph_traversal_string, "age").as_str());
            schema_instance.bestFriend.push_str(
                format!("{}.{}", ___________graph_traversal_string, "bestFriend").as_str(),
            );
            schema_instance
                .unoBook
                .push_str(format!("{}.{}", ___________graph_traversal_string, "unoBook").as_str());
            schema_instance
                .course
                .push_str(format!("{}.{}", ___________graph_traversal_string, "course").as_str());
            schema_instance.semCoures.push_str(
                format!("{}.{}", ___________graph_traversal_string, "semCoures").as_str(),
            );
            schema_instance.writtenBooks.push_str(
                format!("{}.{}", ___________graph_traversal_string, "writtenBooks").as_str(),
            );
            schema_instance
        }
        pub fn bestFriend(&self, filter: impl Into<surrealdb_macros::DbFilter>) -> Student {
            Student::__________connect_to_graph_traversal_string(
                &self.___________graph_traversal_string,
                filter,
                self.get_bindings(),
            )
        }
        pub fn unoBook(&self, filter: impl Into<surrealdb_macros::DbFilter>) -> Book {
            Book::__________connect_to_graph_traversal_string(
                &self.___________graph_traversal_string,
                filter,
                self.get_bindings(),
            )
        }
        pub fn course(&self, filter: impl Into<surrealdb_macros::DbFilter>) -> Book {
            Book::__________connect_to_graph_traversal_string(
                &self.___________graph_traversal_string,
                filter,
                self.get_bindings(),
            )
        }
        pub fn semCoures(&self, filter: impl Into<surrealdb_macros::DbFilter>) -> Book {
            Book::__________connect_to_graph_traversal_string(
                &self.___________graph_traversal_string,
                filter,
                self.get_bindings(),
            )
        }
        pub fn __as__<'a, T>(&self, alias: T) -> ::std::string::String
        where
            T: Into<::std::borrow::Cow<'a, surrealdb_macros::DbField>>,
        {
            let alias: &surrealdb_macros::DbField = &alias.into();
            format!("{} AS {}", self, alias.to_string())
        }
    }
    use super::StudentWritesBook;
    impl Student {
        pub fn writes__(
            &self,
            filterable: impl Into<surrealdb_macros::DbFilter>,
        ) -> writes___schema________________::Writes__ {
            let filter: surrealdb_macros::DbFilter = filterable.into();
            writes___schema________________::Writes::__________connect_to_graph_traversal_string(
                &self.___________graph_traversal_string,
                filter,
                "->",
                self.get_bindings(),
            )
            .into()
        }
    }
    mod writes___schema________________ {
        use surrealdb_macros::db_field::Parametric as _;
        use surrealdb_macros::db_field::Parametric as _;

        use super::StudentWritesBook;
        type ______________BookModel =
            <super::super::StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Out;
        type Book = <______________BookModel as surrealdb_macros::SurrealdbNode>::Schema;
        pub type Writes =
            <super::super::StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Schema;
        pub struct Writes__(Writes);
        impl From<Writes> for Writes__ {
            fn from(value: Writes) -> Self {
                Self(value)
            }
        }
        impl ::std::ops::Deref for Writes__ {
            type Target = Writes;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl Writes__ {
            pub fn book(&self, filterable: impl Into<surrealdb_macros::DbFilter>) -> Book {
                let filter: surrealdb_macros::DbFilter = filterable.into();
                Book::__________connect_to_graph_traversal_string(
                    &self.___________graph_traversal_string,
                    filter,
                    self.get_bindings(),
                )
            }
        }
    }
}
fn test_student_edge_name() {
    ::static_assertions::assert_type_eq_all!(
        LinkSelf<Student>,
        surrealdb_macros::links::LinkSelf<Student>
    );
    ::static_assertions::assert_impl_one!(Student: surrealdb_macros::SurrealdbNode);
    ::static_assertions::assert_type_eq_all!(LinkOne<Book>, surrealdb_macros::links::LinkOne<Book>);
    ::static_assertions::assert_impl_one!(Book: surrealdb_macros::SurrealdbNode);
    ::static_assertions::assert_type_eq_all!(LinkOne<Book>, surrealdb_macros::links::LinkOne<Book>);
    ::static_assertions::assert_impl_one!(Book: surrealdb_macros::SurrealdbNode);
    ::static_assertions::assert_type_eq_all!(
        LinkMany<Book>,
        surrealdb_macros::links::LinkMany<Book>
    );
    ::static_assertions::assert_impl_one!(Book: surrealdb_macros::SurrealdbNode);
    type StudentWritesBookHomeNode = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::In;
    type StudentWritesBookHomeNodeTableNameChecker =
        <StudentWritesBookHomeNode as surrealdb_macros::SurrealdbNode>::TableNameChecker;
    ::static_assertions::assert_type_eq_all!(StudentWritesBookHomeNode, Student);
    ::static_assertions::assert_impl_one!(
        StudentWritesBookHomeNode: surrealdb_macros::SurrealdbNode
    );
    type StudentWritesBookForeignNode = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Out;
    type StudentWritesBookForeignNodeTableNameChecker =
        <StudentWritesBookForeignNode as surrealdb_macros::SurrealdbNode>::TableNameChecker;
    ::static_assertions::assert_fields!(StudentWritesBookForeignNodeTableNameChecker: book);
    ::static_assertions::assert_impl_one!(
        StudentWritesBookForeignNode: surrealdb_macros::SurrealdbNode
    );
    type StudentWritesBookEdgeTableNameChecker =
        <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::TableNameChecker;
    ::static_assertions::assert_fields!(StudentWritesBookEdgeTableNameChecker: writes);
    ::static_assertions::assert_impl_one!(StudentWritesBook: surrealdb_macros::SurrealdbEdge);
    ::static_assertions::assert_type_eq_all!(
        Relate<Book>,
        surrealdb_macros::links::Relate<StudentWritesBookForeignNode>
    );
}

// ==============================================
// Recursive expansion of the SurrealdbEdge macro
// ==============================================

impl<In: surrealdb_macros::SurrealdbNode, Out: surrealdb_macros::SurrealdbNode>
    surrealdb_macros::SurrealdbModel for Writes<In, Out>
{
    fn table_name() -> ::surrealdb::sql::Table {
        "rites".into()
    }

    fn get_serializable_field_names() -> Vec<&'static str> {
        todo!()
    }
}

impl<In: surrealdb_macros::SurrealdbNode, Out: surrealdb_macros::SurrealdbNode>
    surrealdb_macros::SurrealdbEdge for Writes<In, Out>
{
    type In = In;
    type Out = Out;
    type TableNameChecker = writes_schema::TableNameStaticChecker;
    type Schema = writes_schema::Writes;
    fn schema() -> Self::Schema {
        writes_schema::Writes::new()
    }
    fn get_key<T: From<surrealdb_macros::RecordId>>(self) -> ::std::option::Option<T> {
        let record_id = self
            .id
            .map(|id| surrealdb_macros::RecordId::from(id).into());
        record_id
    }

    fn get_table_name() -> ::surrealdb::sql::Table {
        "rites".into()
    }
}
pub mod writes_schema {
    use surrealdb_macros::{
        db_field::{BindingsList, Parametric},
        DbField, SurrealdbNode,
    };
    pub struct TableNameStaticChecker {
        pub writes: String,
    }
    #[derive(Debug)]
    pub struct Writes {
        pub id: surrealdb_macros::DbField,
        pub in_: surrealdb_macros::DbField,
        pub out: surrealdb_macros::DbField,
        pub timeWritten: surrealdb_macros::DbField,
        pub ___________graph_traversal_string: ::std::string::String,
        ___________bindings: BindingsList,
    }
    impl Parametric for Writes {
        fn get_bindings(&self) -> surrealdb_macros::db_field::BindingsList {
            self.___________bindings.to_vec()
        }
    }
    impl Writes {
        pub fn new() -> Self {
            Self {
                id: "id".into(),
                in_: "in".into(),
                out: "out".into(),
                timeWritten: "timeWritten".into(),
                ___________graph_traversal_string: "".into(),
                ___________bindings: vec![],
            }
        }
        pub fn empty() -> Self {
            Self {
                id: "".into(),
                in_: "".into(),
                out: "".into(),
                timeWritten: "".into(),
                ___________graph_traversal_string: "".into(),
                ___________bindings: vec![],
            }
        }
        pub fn __________connect_to_graph_traversal_string(
            store: &::std::string::String,
            filterable: impl Into<surrealdb_macros::DbFilter>,
            arrow_direction: &str,
            existing_bindings: BindingsList,
        ) -> Self {
            let mut schema_instance = Self::empty();
            let filter: surrealdb_macros::DbFilter = filterable.into();
            let bindings = [&existing_bindings[..], &filter.get_bindings()[..]].concat();
            let bindings = bindings.as_slice();

            schema_instance.___________bindings = bindings.into();
            // schema_instance
            //     .___________bindings
            //     .extend(bindings.to_vec());
            let schema_edge_str_with_arrow = format!(
                "{}{}{}{}{}",
                store.as_str(),
                arrow_direction,
                "rites",
                surrealdb_macros::format_filter(filter),
                arrow_direction,
            );
            schema_instance
                .___________graph_traversal_string
                .push_str(schema_edge_str_with_arrow.as_str());
            let ___________graph_traversal_string = &schema_instance
                .___________graph_traversal_string
                .replace(arrow_direction, "");
            // schema_instance
            //     .id
            //     .push_str(format!("{}.{}", ___________graph_traversal_string, "id").as_str());
            // schema_instance
            //     .in_
            //     .push_str(format!("{}.{}", ___________graph_traversal_string, "in").as_str());
            // schema_instance
            //     .out
            //     .push_str(format!("{}.{}", ___________graph_traversal_string, "out").as_str());
            // schema_instance.timeWritten.push_str(
            //     format!("{}.{}", ___________graph_traversal_string, "timeWritten").as_str(),
            // );
            schema_instance.timeWritten =
                DbField::new(format!("{}.{}", ___________graph_traversal_string, "id"))
                    .____________update_many_bindings(bindings);
            schema_instance.timeWritten =
                DbField::new(format!("{}.{}", ___________graph_traversal_string, "in"))
                    .____________update_many_bindings(bindings);
            schema_instance.timeWritten =
                DbField::new(format!("{}.{}", ___________graph_traversal_string, "out"))
                    .____________update_many_bindings(bindings);
            schema_instance.timeWritten = DbField::new(format!(
                "{}.{}",
                ___________graph_traversal_string, "timeWritten"
            ))
            .____________update_many_bindings(bindings);
            schema_instance
        }
    }
}
fn test_writes_edge_name() {}

// ==============================================
// Recursive expansion of the SurrealdbNode macro
// ==============================================

impl surrealdb_macros::SurrealdbModel for Book {
    fn table_name() -> ::surrealdb::sql::Table {
        "book".into()
    }

    fn get_serializable_field_names() -> Vec<&'static str> {
        todo!()
    }
}
impl surrealdb_macros::SurrealdbNode for Book {
    type TableNameChecker = book::TableNameStaticChecker;
    type Schema = book::Book;
    fn schema() -> Self::Schema {
        book::Book::new()
    }
    fn get_table_name() -> ::surrealdb::sql::Table {
        "book".into()
    }
    fn get_key<T: From<surrealdb_macros::RecordId>>(self) -> ::std::option::Option<T> {
        let record_id = self
            .id
            .map(|id| surrealdb_macros::RecordId::from(id).into());
        record_id
    }
}
pub mod book {
    use ::serde::Serialize;
    use surrealdb_macros::{
        db_field::{BindingsList, Parametric},
        DbField,
    };
    pub struct TableNameStaticChecker {
        pub book: String,
    }
    #[derive(Debug)]
    pub struct Book {
        pub id: surrealdb_macros::DbField,
        pub title: surrealdb_macros::DbField,
        pub(crate) ___________graph_traversal_string: ::std::string::String,
        ___________bindings: BindingsList,
    }

    impl Parametric for Book {
        fn get_bindings(&self) -> surrealdb_macros::db_field::BindingsList {
            self.___________bindings.to_vec()
        }
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
                ___________graph_traversal_string: "".into(),
                ___________bindings: vec![],
            }
        }
        pub fn empty() -> Self {
            Self {
                id: "".into(),
                title: "".into(),
                ___________graph_traversal_string: "".into(),
                ___________bindings: vec![],
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
            filterable: impl Into<surrealdb_macros::DbFilter>,
            existing_bindings: BindingsList,
        ) -> Self {
            let mut schema_instance = Self::empty();
            let filter: surrealdb_macros::DbFilter = filterable.into();
            // let bindings = [
            //     existing_bindings.as_slice(),
            //     filter.get_bindings().as_slice(),
            // ]
            // .concat();
            let bindings = [&existing_bindings[..], &filter.get_bindings()[..]].concat();
            let bindings = bindings.as_slice();

            schema_instance
                .___________bindings
                .extend_from_slice(bindings);

            let connection = format!(
                "{}{}{}",
                store,
                "book",
                surrealdb_macros::format_filter(filter)
            );
            schema_instance
                .___________graph_traversal_string
                .push_str(connection.as_str());
            let ___________graph_traversal_string =
                &schema_instance.___________graph_traversal_string;
            schema_instance
                .id
                .push_str(format!("{}.{}", ___________graph_traversal_string, "id").as_str());
            // schema_instance
            //     .title
            //     .push_str(format!("{}.{}", ___________graph_traversal_string, "title").as_str());

            // schema_instance.title = schema_instance.title.__update_many_bindings(bindings);
            schema_instance.title =
                DbField::new(format!("{}.{}", ___________graph_traversal_string, "title"))
                    .____________update_many_bindings(bindings);
            // schema_instance.title
            schema_instance
        }
        pub fn __as__<'a, T>(&self, alias: T) -> ::std::string::String
        where
            T: Into<::std::borrow::Cow<'a, surrealdb_macros::DbField>>,
        {
            let alias: &surrealdb_macros::DbField = &alias.into();
            format!("{} AS {}", self, alias.to_string())
        }
    }
}
fn test_book_edge_name() {}

// ==============================================
// Recursive expansion of the SurrealdbNode macro
// ==============================================

impl surrealdb_macros::SurrealdbModel for Blog {
    fn table_name() -> ::surrealdb::sql::Table {
        "blog".into()
    }

    fn get_serializable_field_names() -> Vec<&'static str> {
        todo!()
    }
}

impl surrealdb_macros::SurrealdbNode for Blog {
    type TableNameChecker = blog::TableNameStaticChecker;
    type Schema = blog::Blog;
    fn schema() -> Self::Schema {
        blog::Blog::new()
    }
    fn get_table_name() -> ::surrealdb::sql::Table {
        "blog".into()
    }
    fn get_key<T: From<surrealdb_macros::RecordId>>(self) -> ::std::option::Option<T> {
        let record_id = self
            .id
            .map(|id| surrealdb_macros::RecordId::from(id).into());
        record_id
    }
}
pub mod blog {
    use ::serde::Serialize;
    pub struct TableNameStaticChecker {
        pub blog: String,
    }
    #[derive(Debug)]
    pub struct Blog {
        pub id: surrealdb_macros::DbField,
        pub content: surrealdb_macros::DbField,
        pub(crate) ___________graph_traversal_string: ::std::string::String,
    }
    impl ::std::fmt::Display for Blog {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{}", self.___________graph_traversal_string))
        }
    }
    impl Blog {
        pub fn new() -> Self {
            Self {
                id: "id".into(),
                content: "content".into(),
                ___________graph_traversal_string: "".into(),
            }
        }
        pub fn empty() -> Self {
            Self {
                id: "".into(),
                content: "".into(),
                ___________graph_traversal_string: "".into(),
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
            filter: impl Into<surrealdb_macros::DbFilter>,
        ) -> Self {
            let mut schema_instance = Self::empty();
            let filter: surrealdb_macros::DbFilter = filter.into();
            let connection = format!(
                "{}{}{}",
                store,
                "blog",
                surrealdb_macros::format_filter(filter)
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
                .content
                .push_str(format!("{}.{}", ___________graph_traversal_string, "content").as_str());
            schema_instance
        }
        pub fn __as__<'a, T>(&self, alias: T) -> ::std::string::String
        where
            T: Into<::std::borrow::Cow<'a, surrealdb_macros::DbField>>,
        {
            let alias: &surrealdb_macros::DbField = &alias.into();
            format!("{} AS {}", self, alias.to_string())
        }
    }
}
fn test_blog_edge_name() {}

fn main() {
    let x = Student::schema()
        .writes__(
            StudentWritesBook::schema()
                .timeWritten
                .greater_than(453)
                .less_than(98)
                .like("Oyelowo"),
        )
        .book(
            Book::schema()
                .id
                .equal(SurrealId::try_from("book:blaze").unwrap()),
        )
        .title;

    // let x = Student::schema().unoBook(cond(
    //     Book::schema().id.equal(RecordId::from(("book", "blaze"))),
    // ));
    // .title;

    println!("XAM {x}");
    let m = x.get_bindings();
    println!("Bindingzzzz {m:?}");
    // assert_eq!(
    //     x.to_string(),
    //     // "->writes->book[WHERE id = book:blaze].title".to_string()
    //     "->writes->book[WHERE id = $_param_00000000].title".to_string()
    // );
    //
    // let m = x.get_bindings();
    // assert_eq!(format!("{m:?}"), "".to_string());
}
