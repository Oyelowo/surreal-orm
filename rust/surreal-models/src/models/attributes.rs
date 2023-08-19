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
// use surreal_derive::{Edge, Node};

use std::fmt::{Debug, Display};
use surreal_orm::{
    functions::*,
    statements::{
        define_field, define_table, for_, order, select, DefineFieldStatement,
        DefineTableStatement, For, Permissions, SelectStatement,
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
fn age_permissions() -> Permissions {
    use CrudType::*;
    let name = Field::new("name");
    let age = Field::new("age");
    // vec![
    //     for_(&[Create, Delete]).where_(name.is("Oyelowo")),
    //     for_(Update).where_(age.less_than_or_equal(130)),
    // ]
    // .into_iter()
    // .map(|e| e.to_raw())
    // .collect::<Vec<_>>()
    // .to_vec()
    Permissions::from(vec![
        for_([Create, Delete]).where_(name.is("Oyelowo")),
        for_(Update).where_(age.less_than_or_equal(130)),
    ])
}

fn student_permissions() -> Permissions {
    use CrudType::*;
    let name = Field::new("name");
    let age = Field::new("age");

    Permissions::from(vec![
        for_([Select, Update]).where_(name.is("Oyedayo")),
        for_([Create, Delete]).where_(age.lte(57)),
    ])
}

// use Duration;
fn default_duration_value() -> Duration {
    Duration::from_secs(60 * 60 * 24 * 7)
}

fn erer() -> Filter {
    cond(value().is_not(NONE)).and(value().like("email"))
}
// fn define_age() -> DefineFieldStatement {
//     use surreal_orm::{Model, Node};
//     use CrudType::*;
//     let student_schema::Student { age, firstName, .. } = Student::schema();
//
//     use FieldType::*;
//
//     // let statement = define_field(Student::schema().age)
//     //     .on_table(Student::table_name())
//     //     .type_(String)
//     //     .value("example@codebreather.com")
//     //     .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
//     //     // .permissions(for_(Select).where_(age.greater_than_or_equal(18))) // Single works
//     //     .permissions(PermissionForables::from(
//     //         for_(&[Create, Update])
//     //             .where_(firstName.is("Oyedayo"))
//     //             .to_raw(),
//     //     )) //Multiple
//     //     .permissions(
//     //         PermissionForables::from(&[
//     //             for_(&[Create, Delete]).where_(firstName.is("Oyelowo")),
//     //             for_(Update).where_(age.less_than_or_equal(130)),
//     //         ])
//     //         .to_raw(),
//     //     );
//     let statement = define_field(Student::schema().age)
//         .on_table(Student::table_name())
//         .type_(String)
//         .value("example@codebreather.com")
//         .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
//         .permissions(for_(Select).where_(age.greater_than_or_equal(18))) // Single works
//         .permissions(for_(&[Create, Update]).where_(firstName.is("Oyedayo"))) //Multiple
//         .permissions(
//             &[
//                 for_(&[Create, Delete]).where_(firstName.is("Oyelowo")),
//                 for_(Update).where_(age.less_than_or_equal(130)),
//             ], // .into_iter()
//                // .map(|e| e.to_raw())
//                // .collect::<Vec<_>>()
//                // .to_vec(),
//         );
//     statement
// }

fn get_age_default_value() -> u8 {
    18
}

fn get_age_assertion() -> Filter {
    cond(value().is_not(NONE)).and(value().gte(18))
}

enum AgeGroup {
    Child,
    Teen,
    Adult,
    Senior,
}

fn get_age_by_group_default_value(group: AgeGroup) -> u8 {
    match group {
        AgeGroup::Child => 10,
        AgeGroup::Teen => 18,
        AgeGroup::Adult => 30,
        AgeGroup::Senior => 60,
    }
}

#[derive(Node, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table_name = "student_with_granular_attributes",
    drop,
    schemafull,
    as = "select(All)",
    permissions = "student_permissions()",
    // define = "define_student()"
)]
pub struct StudentWithGranularAttributes {
    id: SurrealId<StudentWithGranularAttributes, String>,
    first_name: String,
    last_name: String,
    #[surreal_orm(
        type = "int",
        value = "18",
        assert = "cond(value().is_not(NONE)).and(value().gte(18))",
        permissions = "for_([CrudType::Create, CrudType::Delete]).where_(StudentWithGranularAttributes::schema().firstName.is(\"Oyelowo\"))",
        // define = "define_age()"
    )]
    age_inline_expr: u8,

    #[surreal_orm(
        type = "int",
        value = "get_age_default_value()",
        assert = "get_age_assertion()",
        permissions = "age_permissions()",
        // define = "define_age()"
    )]
    age_default_external_function_invoked_expr: u8,

    #[surreal_orm(
        type = "int",
        value = "get_age_by_group_default_value(AgeGroup::Teen)",
        assert = "get_age_assertion()",
        permissions = "age_permissions()",
        // define = "define_age()"
    )]
    age_teen_external_function_invoked_expr: u8,

    #[surreal_orm(
        type = "int",
        value = "get_age_by_group_default_value(AgeGroup::Senior)",
        assert = "get_age_assertion()",
        // define = "define_age()"
    )]
    age_senior_external_function_invoked_expr: u8,

    #[surreal_orm(
        type = "int",
        value = "get_age_by_group_default_value(AgeGroup::Child)",
        permissions = "age_permissions()",
        // define = "define_age()"
    )]
    age_child_external_function_invoked_expr: u8,

    #[surreal_orm(
        type = "int",
        value = "get_age_by_group_default_value(AgeGroup::Adult)",
        // define = "define_age()"
    )]
    age_adult_external_function_invoked_expr: u8,

    #[surreal_orm(
        type = "int",
        value_fn = "get_age_default_value",
        assert_fn = "get_age_assertion",
        permissions_fn = "age_permissions",
        // define = "define_age()"
    )]
    age_external_fn_attrs: u8,
    #[surreal_orm(
        type = "int",
        value= "get_age_default_value()",
        assert_fn = "get_age_assertion",
        permissions_fn = "age_permissions",
        // define = "define_age()"
    )]
    age_mix_and_match_external_fn_inline_attrs: u8,

    // age_closure_external_fn_attrs: u8,
    #[surreal_orm(
        type = "duration",
        value = "default_duration_value()",
        assert = "value().is_not(NONE)",
        // define = "define_age()"
    )]
    time_to_kelowna: Duration,

    #[surreal_orm(
        type = "duration",
        value = "Duration::from_secs(60 * 60 * 24 * 7)",
        assert = "value().is_not(NONE)",
        // define = "define_age()"
    )]
    time_to_kelowna_inline: Duration,
    // Even if ypu dont list the type for all links, the types are autogenerated at compile time
    // becuase I have enough info from the annotation to derive it
    #[surreal_orm(
        link_self = "StudentWithGranularAttributes",
        type = "record(student_with_granular_attributes)"
    )]
    best_friend: LinkSelf<StudentWithGranularAttributes>,

    #[surreal_orm(link_one = "Book")]
    #[serde(rename = "unoBook")]
    fav_book: LinkOne<Book>,

    #[surreal_orm(link_one = "Book", skip_serializing)]
    course: LinkOne<Book>,

    #[surreal_orm(link_many = "Book", type = "array", item_type = "record(book)")]
    #[serde(rename = "semesterCourses")]
    all_semester_courses: LinkMany<Book>,

    #[surreal_orm(relate(
        model = "StudentWithGranularAttributesWritesBook",
        connection = "->writes->book"
    ))]
    #[serde(skip_serializing)]
    written_books: Relate<Book>,

    #[surreal_orm(relate(
        model = "StudentWithGranularAttributesWritesBlog",
        connection = "->writes->blog"
    ))]
    #[serde(skip_serializing)]
    blogsssss: Relate<Blog>,
}

pub type StudentWithGranularAttributesWritesBook = Writes<StudentWithGranularAttributes, Book>;
pub type StudentWithGranularAttributesWritesBlog = Writes<StudentWithGranularAttributes, Blog>;

#[derive(Node, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table_name = "student",
    // drop,
    // schemafull,
    // as = "select(All)",
    // permissions = "perm()",
    // define = "define_student()"
)]
pub struct Student {
    id: SurrealId<Student, String>,
    first_name: String,
    last_name: String,
    #[surreal_orm(
        type = "int",
        // value = "we()",
        value = "18",
        // assert = "erer()",
        assert = "cond(value().is_not(NONE)).and(value().gte(18))",
        permissions = "age_permissions()",
        // define = "define_age()"
    )]
    age: u8,

    #[surreal_orm(
        type = "int",
        // value = "we()",
        // value = "18",
        // assert = "erer()",
        // assert = "cond(value().is_not(NONE)).and(value().gte(18))",
        // define = "define_age()"
    )]
    score: u8,

    // Even if ypu dont list the type for all links, the types are autogenerated at compile time
    // becuase I have enough info from the annotation to derive it
    #[surreal_orm(link_self = "Student", type = "record(student)")]
    best_friend: LinkSelf<Student>,

    #[surreal_orm(link_one = "Book")]
    #[serde(rename = "unoBook")]
    fav_book: LinkOne<Book>,

    #[surreal_orm(link_one = "Book", skip_serializing)]
    course: LinkOne<Book>,

    #[surreal_orm(link_many = "Book", type = "array", item_type = "record(book)")]
    #[serde(rename = "semesterCourses")]
    all_semester_courses: LinkMany<Book>,

    #[surreal_orm(relate(model = "StudentWritesBook", connection = "->writes->book"))]
    #[serde(skip_serializing)]
    written_books: Relate<Book>,

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
            score: Default::default(),
            best_friend: Default::default(),
            fav_book: Default::default(),
            course: Default::default(),
            all_semester_courses: Default::default(),
            written_books: Default::default(),
            blogsssss: Default::default(),
        }
    }
}

#[derive(surreal_orm::Edge, TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "writes")]
pub struct Writes<In: Node, Out: Node> {
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

#[derive(Edge, TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "likes")]
pub struct Likes<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Likes<In, Out>>,

    #[serde(rename = "in", skip_serializing)]
    pub in_: LinkOne<In>,
    #[serde(skip_serializing)]
    pub out: LinkOne<Out>,
    pub likes_count: u64,
}
pub type StudentLiksBook = Likes<Student, Book>;

#[derive(Node, TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "book")]
pub struct Book {
    id: SurrealSimpleId<Book>,
    title: String,
    content: String,
}

#[derive(Node, TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "blog")]
pub struct Blog {
    id: SurrealSimpleId<Blog>,
    title: String,
    content: String,
}
//
// #[derive(Node, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// #[surreal_orm(
//     table_name = "student"
//     drop,
//     schemafull,
//     as = "select(All)",
//     permissions = "perm()",
//     // permissions_fn = "perm",
//     // define = "define_student()",
//     // define_fn = "define_student"
// )]
// pub struct StudentWithFn {
//     id: SurrealId<Student, String>,
//
//     first_name: String,
//     last_name: String,
//     #[surreal_orm(
//         // type="array(int)",
//         // type = "geometry(feature, point, collection, polygon)",
//         // value = "we()",
//         // value = "Duration::from_secs(54)",
//         // assert_fn = "erer",
//         // assert = "erer()",
//         // assert_fn = "erer",
//         // assert = "cond(value().is_not(NONE))",
//         // assert = "cond(value().is_not(NONE)).and(value().like("is_email"))",
//         // permissions = "perm()",
//         // permissions_fn = "perm",
//         // define = "define_age()",
//         define_fn = "define_age"
//     )]
//     age: u8,
//
//     // Even if ypu dont list the type for all links, the types are autogenerated at compile time
//     // becuase I have enough info from the annotation to derive it
//     #[surreal_orm(link_self = "Student", type = "record(student)")]
//     best_friend: LinkSelf<Student>,
//
//     #[surreal_orm(link_one = "Book")]
//     #[serde(rename = "unoBook")]
//     fav_book: LinkOne<Book>,
//
//     #[surreal_orm(link_one = "Book", skip_serializing)]
//     course: LinkOne<Book>,
//
//     #[surreal_orm(
//         link_many = "Book",
//         type = "array",
//         item_type = "record(book)",
//         item_assert_fn = "erer"
//     )]
//     #[serde(rename = "semesterCourses")]
//     all_semester_courses: LinkMany<Book>,
//
//     #[surreal_orm(relate(model = "StudentWritesBook", connection = "->writes->book"))]
//     #[serde(skip_serializing)]
//     written_books: Relate<Book>,
//
//     // #[surreal_orm(relate(model = "StudentWritesBook", connection = "->writes->book"))]
//     // #[serde(skip_serializing)]
//     // prof_book: Relate<Book>,
//     #[surreal_orm(relate(model = "StudentWritesBlog", connection = "->writes->blog"))]
//     #[serde(skip_serializing)]
//     blogsssss: Relate<Blog>,
// }
//
// fn define_student() -> DefineTableStatement {
//     use CrudType::*;
//     let name = Field::new("name");
//     let _user_table = Table::from("user");
//     let age = Field::new("age");
//     let country = Field::new("country");
//     let fake_id2 = sql::Thing::from(("user".to_string(), "oyedayo".to_string()));
//
//     let statement = define_table(Student::table_name())
//         .drop()
//         .as_(
//             select(All)
//                 .from(fake_id2)
//                 .where_(country.is("INDONESIA"))
//                 .order_by(order(&age).numeric().desc())
//                 .limit(20)
//                 .start(5),
//         )
//         .schemafull()
//         .permissions(for_(Select).where_(age.greater_than_or_equal(18))) // Single works
//         .permissions(for_([Create, Delete]).where_(name.is("Oyedayo"))) //Multiple
//         .permissions([
//             for_([Create, Delete]).where_(name.is("Oyedayo")),
//             for_(Update).where_(age.less_than_or_equal(130)),
//         ]);
//
//     statement
// }
//
// fn define_score() -> DefineFieldStatement {
//     use surreal_orm::{Model, Node};
//     use CrudType::*;
//     let student_schema::Student { age, firstName, .. } = Student::schema();
//
//     use FieldType::*;
//
//     let statement = define_field(Student::schema().age)
//         .on_table(Student::table_name())
//         .type_(FieldType::String)
//         .value(18)
//         .permissions(for_(FieldType::Select).where_(age.gte(50))) // Single works
//         .permissions(
//             for_([Create, Update])
//                 .where_(firstName.is("Oyedayo"))
//                 .to_raw(),
//         ) //Multiple
//         .permissions(
//             [
//                 for_([Create, Delete]).where_(firstName.is("Oyelowo")),
//                 for_(Update).where_(age.less_than_or_equal(130)),
//             ]
//             .to_raw(),
//         );
//     statement
// }
// #[derive(Node, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// #[surreal_orm(table_name = "student_external_defs", define_fn = "define_student")]
// pub struct StudentExternalDefs {
//     id: SurrealId<StudentExternalDefs, String>,
//     first_name: String,
//     last_name: String,
//     #[surreal_orm(
//         // type="array(int)",
//         // type = "geometry(feature, point, collection, polygon)",
//         // value = "we()",
//         // value = "Duration::from_secs(54)",
//         // assert_fn = "erer",
//         // assert = "erer()",
//         // assert_fn = "erer",
//         // assert = "cond(value().is_not(NONE))",
//         // assert = "cond(value().is_not(NONE)).and(value().like("is_email"))",
//         // permissions = "perm()",
//         // permissions_fn = "perm",
//         // define = "define_age()",
//         define_fn = "define_age"
//     )]
//     age: u8,
//
//     #[surreal_orm(
//         link_self = "StudentExternalDefs",
//         type = "record(student_external_defs)"
//     )]
//     best_friend: LinkSelf<StudentExternalDefs>,
//
//     #[surreal_orm(link_one = "Book")]
//     #[serde(rename = "unoBook")]
//     fav_book: LinkOne<Book>,
//
//     #[surreal_orm(link_one = "Book", skip_serializing)]
//     course: LinkOne<Book>,
//
//     #[surreal_orm(
//         link_many = "Book",
//         type = "array",
//         item_type = "record(book)",
//         item_assert_fn = "erer"
//     )]
//     #[serde(rename = "semesterCourses")]
//     all_semester_courses: LinkMany<Book>,
//
//     #[surreal_orm(relate(model = "StudentWritesBook", connection = "->writes->book"))]
//     #[serde(skip_serializing)]
//     written_books: Relate<Book>,
//
//     // #[surreal_orm(relate(model = "StudentWritesBook", connection = "->writes->book"))]
//     // #[serde(skip_serializing)]
//     // prof_book: Relate<Book>,
//     #[surreal_orm(relate(model = "StudentWritesBlog", connection = "->writes->blog"))]
//     #[serde(skip_serializing)]
//     blogsssss: Relate<Blog>,
// }
