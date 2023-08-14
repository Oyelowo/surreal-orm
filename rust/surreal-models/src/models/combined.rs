#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use _core::ops::Deref;
use insta;
use regex;
use serde::{Deserialize, Serialize};
use static_assertions::*;
use std::time::Duration;
use surrealdb::{
    engine::local::{Db, Mem},
    opt::IntoResource,
    sql::{self, Id},
    Result, Surreal,
};
// use surreal_derive::{SurrealEdge, SurrealNode};

use std::fmt::{Debug, Display};
use surreal_orm::{
    functions::*,
    statements::{
        define_field, define_table, for_, order, select, DefineFieldStatement,
        DefineTableStatement, For, PermissionType, SelectStatement,
    },
    *,
};

use test_case::test_case;
use typed_builder::TypedBuilder;

fn gama() -> SelectStatement {
    crypto::argon2::compare!("Rer", "Erer");
    // All

    select(All).from(Table::new("user"))
}
fn full() -> u32 {
    54
}
// fn perm() -> RawStatement {
//     use CrudType::*;
//     let name = Field::new("name");
//     let age = Field::new("age");
//     // vec![
//     //     for_(&[Create, Delete]).where_(name.is("Oyelowo")),
//     //     for_(Update).where_(age.less_than_or_equal(130)),
//     // ]
//     // .into_iter()
//     // .map(|e| e.to_raw())
//     // .collect::<Vec<_>>()
//     // .to_vec()
//     PermissionType::from(vec![
//         for_(&[Create, Delete]).where_(name.is("Oyelowo")),
//         for_(Update).where_(age.less_than_or_equal(130)),
//     ])
//     // .to_raw()
// }

fn define_student() -> DefineTableStatement {
    use CrudType::*;
    let name = Field::new("name");
    let _user_table = Table::from("user");
    let age = Field::new("age");
    let country = Field::new("country");
    let fake_id2 = sql::Thing::from(("user".to_string(), "oyedayo".to_string()));

    let statement = define_table(Student::table_name())
        .drop()
        .as_(
            select(All)
                .from(fake_id2)
                .where_(country.is("INDONESIA"))
                .order_by(order(&age).numeric().desc())
                .limit(20)
                .start(5),
        )
        .schemafull()
        .permissions(for_(Select).where_(age.greater_than_or_equal(18))) // Single works
        .permissions(for_(&[Create, Delete]).where_(name.is("Oyedayo"))) //Multiple
        .permissions(&[
            for_(&[Create, Delete]).where_(name.is("Oyedayo")),
            for_(Update).where_(age.less_than_or_equal(130)),
        ]);

    statement
}
// use Duration;
fn we() -> sql::Value {
    surrealdb::sql::Value::Duration(Duration::from_secs(60 * 60 * 24 * 7).into())
}

fn erer() -> Filter {
    cond(value().is_not(NONE)).and(value().like("email"))
}
fn define_age() -> DefineFieldStatement {
    use surreal_orm::{SurrealModel, SurrealNode};
    use CrudType::*;
    let student_schema::Student { age, firstName, .. } = Student::schema();

    use FieldType::*;

    // let statement = define_field(Student::schema().age)
    //     .on_table(Student::table_name())
    //     .type_(String)
    //     .value("example@codebreather.com")
    //     .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
    //     // .permissions_for(for_(Select).where_(age.greater_than_or_equal(18))) // Single works
    //     .permissions_for(PermissionForables::from(
    //         for_(&[Create, Update])
    //             .where_(firstName.is("Oyedayo"))
    //             .to_raw(),
    //     )) //Multiple
    //     .permissions_for(
    //         PermissionForables::from(&[
    //             for_(&[Create, Delete]).where_(firstName.is("Oyelowo")),
    //             for_(Update).where_(age.less_than_or_equal(130)),
    //         ])
    //         .to_raw(),
    //     );
    let statement = define_field(Student::schema().age)
        .on_table(Student::table_name())
        .type_(String)
        .value("example@codebreather.com")
        .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
        .permissions(for_(Select).where_(age.greater_than_or_equal(18))) // Single works
        .permissions(for_(&[Create, Update]).where_(firstName.is("Oyedayo"))) //Multiple
        .permissions(
            &[
                for_(&[Create, Delete]).where_(firstName.is("Oyelowo")),
                for_(Update).where_(age.less_than_or_equal(130)),
            ], // .into_iter()
               // .map(|e| e.to_raw())
               // .collect::<Vec<_>>()
               // .to_vec(),
        );
    statement
}

#[derive(SurrealNode, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table_name = "student"
    // drop,
    // schemafull,
    // as = "select(All)",
    // permissions = "perm()",
    // permissions_fn = "perm",
    // define = "define_student()",
    // define_fn = "define_student"
)]
pub struct Student {
    id: SurrealId<Student, String>,

    first_name: String,
    last_name: String,
    #[surreal_orm(
        // type="array(int)",
        // type = "geometry(feature, point, collection, polygon)",
        // value = "we()",
        // value = "Duration::from_secs(54)",
        // assert_fn = "erer",
        // assert = "erer()",
        // assert_fn = "erer",
        // assert = "cond(value().is_not(NONE))",
        // assert = "cond(value().is_not(NONE)).and(value().like("is_email"))",
        // permissions = "perm()",
        // permissions_fn = "perm",
        // define = "define_age()",
        define_fn = "define_age"
    )]
    age: u8,

    // Even if ypu dont list the type for all links, the types are autogenerated at compile time
    // becuase I have enough info from the annotation to derive it
    #[surreal_orm(link_self = "Student", type = "record(student)")]
    best_friend: LinkSelf<Student>,

    #[surreal_orm(link_one = "Book")]
    #[serde(rename = "unoBook")]
    fav_book: LinkOne<Book>,

    #[surreal_orm(link_one = "Book", skip_serializing)]
    course: LinkOne<Book>,

    #[surreal_orm(
        link_many = "Book",
        type = "array",
        content_type = "record(book)",
        content_assert_fn = "erer"
    )]
    #[serde(rename = "semCoures")]
    all_semester_courses: LinkMany<Book>,

    #[surreal_orm(relate(model = "StudentWritesBook", connection = "->writes->book"))]
    #[serde(skip_serializing)]
    written_books: Relate<Book>,

    // #[surreal_orm(relate(model = "StudentWritesBook", connection = "->writes->book"))]
    // #[serde(skip_serializing)]
    // prof_book: Relate<Book>,
    #[surreal_orm(relate(model = "StudentWritesBlog", connection = "->writes->blog"))]
    #[serde(skip_serializing)]
    blogsssss: Relate<Blog>,
}

impl Default for Student {
    fn default() -> Self {
        let id = Self::create_id(sql::Id::rand().to_raw());
        Self {
            id,
            first_name: Default::default(),
            last_name: Default::default(),
            age: Default::default(),
            best_friend: Default::default(),
            fav_book: Default::default(),
            course: Default::default(),
            all_semester_courses: Default::default(),
            written_books: Default::default(),
            blogsssss: Default::default(),
        }
    }
}

#[derive(SurrealEdge, TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "writes")]
pub struct Writes<In: SurrealNode, Out: SurrealNode> {
    pub id: SurrealSimpleId<Writes<In, Out>>,

    #[serde(rename = "in", skip_serializing)]
    pub in_: LinkOne<In>,
    #[serde(skip_serializing)]
    pub out: LinkOne<Out>,
    pub time_written: Duration,
    pub count: i32,
}

pub type StudentWritesBook = Writes<Student, Book>;
pub type StudentWritesBlog = Writes<Student, Blog>;

#[derive(SurrealEdge, TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "likes")]
pub struct Likes<In: SurrealNode, Out: SurrealNode> {
    pub id: SurrealSimpleId<Likes<In, Out>>,

    #[serde(rename = "in", skip_serializing)]
    pub in_: LinkOne<In>,
    #[serde(skip_serializing)]
    pub out: LinkOne<Out>,
    pub likes_count: u64,
}
pub type StudentLiksBook = Likes<Student, Book>;

#[derive(SurrealNode, TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "book")]
pub struct Book {
    id: SurrealSimpleId<Book>,
    title: String,
    content: String,
}

#[derive(SurrealNode, TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "blog")]
pub struct Blog {
    id: SurrealSimpleId<Blog>,
    title: String,
    content: String,
}

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
