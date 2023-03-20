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
use surrealdb_derive::{SurrealdbEdge, SurrealdbNode};

use std::fmt::{Debug, Display};
use surrealdb_macros::{
    links::{LinkMany, LinkOne, LinkSelf, Reference, Relate},
    RecordId, SurrealdbEdge, SurrealdbNode,
};
use typed_builder::TypedBuilder;

// ::static_assertions::assert_impl_one!()
#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone /* , Default */)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "student")]
pub struct Student {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<RecordId>,
    first_name: String,
    last_name: String,

    #[surrealdb(link_self = "Student")]
    best_class_mate: LinkSelf<Student>,

    #[surrealdb(link_one = "Book")]
    #[serde(rename = "lowo_na")]
    fav_book: LinkOne<Book>,

    #[surrealdb(link_one = "Blog")]
    course: LinkOne<Blog>,

    #[surrealdb(link_many = "Book")]
    #[serde(rename = "lowo")]
    all_semester_courses: LinkMany<Book>,

    #[surrealdb(relate(model = "StudentWritesBook", connection = "->rites->book"))]
    written_blogs: Relate<Book>,
}

#[derive(SurrealdbEdge, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "rites", relax_table_name)]
pub struct Writes<In: SurrealdbNode, Out: SurrealdbNode> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<RecordId>,

    // #[surrealdb(link_one = "Book", skip_serializing)]
    #[serde(skip_serializing)]
    r#in: Option<In>,
    #[serde(skip_serializing)]
    out: Option<Out>,
    time_written: String,
}

type StudentWritesBook = Writes<Student, Book>;

#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "book")]
pub struct Book {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<RecordId>,
    title: String,
}

#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "blog")]
pub struct Blog {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<RecordId>,
    content: String,
}

// fn eerer() {
//     Student::schema()
//         .rites__(Clause::All)
//         .book(Clause::All)
//         .title;
// }
fn main() {
    use serde::{Deserialize, Serialize};
    use serde_json::Result;
    use serde_json::{Map, Value};

    #[derive(Serialize, Deserialize)]
    struct Country {
        title: String,
        continent: String,
    }

    #[derive(Serialize, Deserialize)]
    struct Person {
        name: String,
        age: u8,
        countries: Vec<Country>,
    }

    #[derive(Serialize, Deserialize)]
    struct Address {
        street: String,
        city: String,
        countries: Vec<Country>,
        owner: Person,
    }

    fn print_an_address() -> Result<String> {
        // Some data structure.

        let address = Address {
            street: "10 Downing Street".to_owned(),
            city: "London".to_owned(),
            countries: vec![
                Country {
                    title: "Canada".into(),
                    continent: "NA".into(),
                },
                Country {
                    title: "finland".into(),
                    continent: "EU".into(),
                },
            ],
            owner: Person {
                name: "Oyelowo".into(),
                age: 90,
                countries: vec![
                    Country {
                        title: "Canada".into(),
                        continent: "NA".into(),
                    },
                    Country {
                        title: "finland".into(),
                        continent: "EU".into(),
                    },
                ],
            },
        };

        //
        // insert.values(address);
        //
        // insert(Company {
        //     name: "SurrealDB".into(),
        //     founded: Date(2021-09-10)
        // })
        // [("name", String("SurrealDB")), ("founded", Date(2021-09-10))]
        //
        // INSERT INTO company (name, founded) VALUES ($name, $founded);
        //
        //
        // INSERT INTO company (name, founded) VALUES ('SurrealDB', '2021-09-10');
        //
        // INSERT INTO company (name, founded) VALUES ($name, $founded);
        // bindings: [
        //           (name, 'SurrealDB')
        //           (founded, '2021-09-10')
        // ]
        //
        // INSERT INTO company (name, founded) VALUES ($arg1, $arg2);
        // bindings: [
        //           (arg1, 'SurrealDB')
        //           (arg2, '2021-09-10')
        // ]
        //
        // [("street", String("10 Downing Street")), ("city", String("London")), ("countries", Array [Object {"title": String("Canada"), "continent": String("NA")}, Object {"title": String("finland"), "continent": String("E
        // U")}]), ("owner", Object {"name": String("Oyelowo"), "age": Number(90), "countries": Array [Object {"title": String("Canada"), "continent": String("NA")}, Object {"title": String("finland"), "continent": String("
        // EU")}]})]                                                                                                                                      git:(204-surrealdb-orm-implement-fully-compliant-insert-query|✚2⚑20
        //
        //
        //
        //
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&address)?;

        // Print, write to a file, or send to an HTTP server.

        // println!("ADDRES = {}", j);

        Ok(j)
    }
    // print_an_address().unwrap();
    fn json_to_vec(json: &str) -> Vec<(String, Value)> {
        let parsed: Map<String, Value> = serde_json::from_str(json).unwrap();

        parsed
            .into_iter()
            .map(|(key, value)| (key, value))
            .collect()
    }

    // fn dmain() {
    let json = r#"{"key1": "value1", "key2": 42}"#;

    let result = json_to_vec(print_an_address().unwrap().as_str());

    println!("{:?}", result);
    // }
    // let book = Book {
    //     id: Some("book:1".try_into().unwrap()),
    //     title: "ere".into(),
    // };
    // let x1 = Student {
    //     id: None,
    //     first_name: "".into(),
    //     last_name: "".into(),
    //     fav_book: book.clone().into(),
    //     // fav_book: LinkOne::from(book),
    //     // written_blogs: vec![].into(),
    //     written_blogs: Relate::null(),
    //     // best_class_mate: Default::default(),
    //     best_class_mate: LinkSelf::null(),
    //     course: LinkOne::null(),
    //     all_semester_courses: LinkMany::null(),
    // };
    //
    // let xx = Student {
    //     id: None,
    //     first_name: "".into(),
    //     last_name: "".into(),
    //     fav_book: book.into(),
    //     // fav_book: LinkOne::from(book),
    //     written_blogs: vec![].into(),
    //     all_semester_courses: vec![].into(),
    //     best_class_mate: x1.into(),
    //     course: LinkOne::null(),
    // };
    // let xxo = xx.clone().best_class_mate.value().unwrap();
    //
    // // Returns either:
    // // the foreign values if fetched
    // // id keys of the foreign Field if not fetched
    // // empty Vec if not available
    // let xcv = xx.all_semester_courses.clone();
    //
    // // Returns just the fully fetched values if fetched and available, otherwise, None
    // let xcv = xx.all_semester_courses.values();
    // //
    // // Returns just the keys of the foreign field if available, otherwise, None
    // let xcv = xx.all_semester_courses.keys();
    // // xx.all_semester_courses
    // //     .into_iter()
    // //     .map(|x| x.value().unwrap());
    //
    // let xcv = xx.written_blogs.clone();
    //
    // // Returns just the fully fetched values if fetched and available, otherwise, None
    // let xcv = xx.written_blogs.values();
    // //
    // // Returns just the keys of the foreign field if available, otherwise, None
    // let xcv = xx.written_blogs.keys();
    // // xx.fav_book.value_owned().unwrap()
    // xx.best_class_mate
    //     .value()
    //     .unwrap()
    //     .fav_book
    //     .value()
    //     .unwrap();
    //
    // let x = xx.clone().get_key();
    // let cc = xx.clone().get_key();
    // println!("areore:{xx:?}");
    //
    // // xx.get_key()
    // let x1 = Student::schema().firstName.__as__("lowo");
    // println!("x1 --- {x1}");
    //
    // let bee = Student::schema()
    //     .lowo_na(Clause::Where(
    //         query().where_(Student::schema().lastName.equals("lowo")),
    //     ))
    //     .title
    //     .contains("bee");
    //
    // Student::schema().rites__(Clause::All).book(Clause::All);
    //
    // let xx = Student::schema()
    //     .rites__(Clause::All)
    //     .book(Clause::Where(
    //         query().where_(Student::schema().lastName.contains("Dayo")),
    //     ))
    //     .title
    //     .__as__("meorm");
    // println!("xx --- {xx}");
    //
    // let x2 = Student::schema();
    // println!("x2 --- {x2}");
    //
    // // Student::schema().favBook(Clause::All);
    // let xx = Student::schema().__with_id__("Student:lowo");
    // // Student::schema().favBook(Clause::All);
    // let xx = Student::schema();
    // println!("dfmoaef --- {xx}");
    // // Student::schema().__with_id__("student:3434").lastName;
    // // Student::schema().__with_id__("student:3434");
}
// Student::schema().__with_id__("student:3434").lastName;
// Student::schema().__with_id__("student:3434");
// struct Writes<In: SurrealdbNode, Out: SurrealdbNode> {
//     id: Option<String>,
//     #[surrealdb(link_one = "Student")]
//     r#in: LinkOne<In>,
//
//     #[surrealdb(link_one = "Blog")]
//     out: LinkOne<Out>,
//     when: String,
//     destination: String,
// }
//
// // impl<In: SurrealdbNode, Out: SurrealdbNode> Writes<In, Out> {
// //     // const Nama: &'static str = "Writes";
// // }
//
// #[derive(SurrealdbNode, TypedBuilder, Default, Serialize, Deserialize, Debug, Clone)]
// /* #[surrealdb(rename_all = "camelCase")] */
// pub struct Blog {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[builder(default, setter(strip_option))]
//     id: Option<String>,
//     title: String,
//     #[serde(skip_serializing)]
//     content: String,
// }
//
// #[derive(SurrealdbNode, TypedBuilder, Default, Serialize, Deserialize, Debug, Clone)]
// /* #[surrealdb(rename_all = "camelCase")] */
// pub struct Book {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[builder(default, setter(strip_option))]
//     id: Option<String>,
//     title: String,
// }
//
// static DB: Surreal<Db> = Surreal::init();
//
// pub fn nama() {
//     // Student::new().book_written().chapters().verse
//     // Field("df".into())
//     // "".contains_not()
//     // let rela = Student::new()
//     //     .book_written_cond(Cond("WHERE pages > 5".into()))
//     //     .writer();
//     // println!("rela...{:?}", rela.store);
//
//     let rela = student_schema::Student::new()
//         .writes__(Clause::Where(
//             query()
//                 .and_where("pages > 5")
//                 .and("time_done = yesterday")
//                 .build(),
//         ))
//         .book(Clause::Id("book:akkaka".into()))
//         .__writes(Clause::None)
//         .student(Clause::Id("student:lowo".into()))
//         .writes__(Clause::None)
//         .book(Clause::None)
//         .__writes(Clause::None)
//         .student(Clause::None)
//         .drinks__(Clause::None)
//         .juice(Clause::None)
//         .__as__("kula");
//
//     println!("rela...{:?}", rela);
//
//     let rela = student_schema::Student::new()
//         .writes__(Clause::Where(
//             query()
//                 .and_where("pages > 5")
//                 .and("time_done = yesterday")
//                 .build(),
//         ))
//         .book(Clause::Id("book:akkaka".into()))
//         .__writes(Clause::Id("writes:pram".into()))
//         .time_written
//         .__as__("xxx");
//     // .student(Clause::None)
//     // .drunk_water
//     // .__as__("wara");
//     // .__as__(Student::book_written);
//     // .blog(Clause::Id("blog:akkaka".into()));
//     // .as_alias(Blog)
//     // .intro
//     // .__as__("dfdf");
//
//     println!("rela...{}", rela);
//
//     // Student.favorite_book.title
//     let rela = student_schema::Student::new()
//         .favorite_book(Clause::Id("book:janta".into()))
//         .title;
//     println!("rela...{}", rela);
//
//     // println!("rela...{}", StudentEnum::book_written);
//     let rela = Student::schema()
//         .__with_id__("Student:lowo")
//         .writes__(Clause::None)
//     let rela = Student::schema()
//         .__with_id__("Book:maow");
//     println!("rela...{}", rela);
// }
// // impl Book {
// //     fn writer(&self) -> Student {
// //         todo!()
// //     }
// // }
// // Student->writes->Book->has->Chaper
// // let rela = Student::new().book_written().chapters();
// // // ->writes->Book->
// // let xx = Student::new()
// //     .book_written()
// //     .writer()
// //     .book_written()
// //     .writer()
// //     .book_written()
// //     .writer()
// //     .book_written();
//
// #[tokio::main]
// async fn main() {
//     // let xx =S
//     nama();
// }
//
// // LET $from = (SELECT users FROM company:surrealdb);
// // LET $devs = (SELECT * FROM user WHERE tags CONTAINS 'developer');
// // RELATE $from->like->$devs SET time.connected = time::now();
// // struct Company {
// //   users: LinkMany<User>
// // }
// //
// // struct User {
// //     tags: Vec<String>,
// //     company: LinkOne<Company>,
// //     companies: LinkMany<Company>,
// // }
// // RELATE User[where company.id == company:surrealdb]->like->User[where tags contains 'developer']
// //
// //
// /* #[derive(SurrealdbModel, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Student {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[builder(default, setter(strip_option))]
//     id: Option<String>,
//     first_name: String,
//
//     #[surrealdb(link_one = "Course", skip_serializing)]
//     course: LinkOne<Course>,
//
//     #[surrealdb(link_many = "Course", skip_serializing)]
//     #[serde(rename = "lowo")]
//     all_semester_courses: LinkMany<Course>,
//
//     #[surrealdb(relate(edge = "StudentWritesBlog", link = "->writes->Blog"))]
//     written_blogs: Relate<Blog>,
// } */
// // Account::with_id(SuId(""))
//
// /*
// ========RELATE===========
//  * -- Add a graph edge between two specific records
// RELATE user:tobie->write->article:surreal SET time.written = time::now();
//
// -- Add a graph edge between multiple specific users and devs
// LET $from = (SELECT users FROM company:surrealdb);
// LET $devs = (SELECT * FROM user WHERE tags CONTAINS 'developer');
// RELATE $from->like->$devs SET time.connected = time::now();/
//
// RELATE user:tobie->write->article:surreal CONTENT {
//     source: 'Apple notes',
//     tags: ['notes', 'markdown'],
//     time: {
//         written: time::now(),
//     },
// };
//
// ========SELECT===========
// -- Select a remote field from connected out graph edges
// SELECT ->like->friend.name AS friends FROM person:tobie;
//
//
// -- Conditional filtering based on graph edges
// SELECT * FROM profile WHERE count(->experience->organisation) > 3;
//
// SELECT * FROM person WHERE ->knows->person->(knows WHERE influencer = true) TIMEOUT 5s;
// PREFERRED: SELECT * FROM person WHERE ->knows[WHERE influencer = true]->person
//
// #[derive(SurrealdbModel, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// struct Person {
//     #[surrealdb(relate(edge = "PersonKnowsPerson", link = "->knows->Person"))]
//    known_persons: Relate<Person>
// }
//
// #[derive(SurrealdbModel, Debug, Serialize, Deserialize)]
// #[surrealdb(relation_name = "knows")]
// struct PersonKnowsPerson {
//     id: Option<String>,
//     #[surrealdb(link_one = "Person", skip_serializing)]
//     r#in: LinkOne<Person>,
//     #[surrealdb(link_one = "Person", skip_serializing)]
//     out: LinkOne<Person>,
//     influencer: bool,
// }
//
// SELECT ->purchased->product<-purchased<-person->purchased->product FROM person:tobie PARALLEL;
//
//
// ========DELETE===========
// // DELETE person WHERE ->knows->person->(knows WHERE influencer = false) TIMEOUT 5s;
//
// ========UPDATE===========
// // UPDATE person SET important = true WHERE ->knows->person->(knows WHERE influencer = true) TIMEOUT 5s;
// // PREFERRED: UPDATE person SET important = true WHERE ->knows->person[WHERE influencer = true] TIMEOUT 5s;
// */
