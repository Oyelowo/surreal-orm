/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// use super::studentwithgranularattributes;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use surreal_orm::{
    sql,
    statements::{
        define_field, define_table, for_permission, select, DefineFieldStatement,
        DefineTableStatement, Permissions, SelectStatement,
    },
    *,
};
use typed_builder::TypedBuilder;
use CrudType::*;


fn age_permissions() -> Permissions {
    let student_with_granular_attributes::Schema {
        ageInlineExpr,
        firstName,
        ..
    } = StudentWithGranularAttributes::schema();

    [
        for_permission([Create, Delete]).where_(firstName.is("Oyelowo")),
        for_permission(Update).where_(ageInlineExpr.less_than_or_equal(130)),
    ]
    .into()
}

fn student_permissions() -> Permissions {
    let student_with_granular_attributes::Schema {
        ageInlineExpr,
        firstName,
        ..
    } = StudentWithGranularAttributes::schema();

    Permissions::from(vec![
        for_permission([Select, Update]).where_(firstName.is("Oyedayo")),
        for_permission([Create, Delete]).where_(ageInlineExpr.lte(57)),
    ])
}

// use Duration;
fn default_duration_value() -> sql::Duration {
    Duration::from_secs(60 * 60 * 24 * 7).into()
}

fn age_define_external_fn_path() -> DefineFieldStatement {
    let student_with_define_fn_attr::Schema {
        ref ageDefineInline,
        ref firstName,
        ..
    } = StudentWithDefineFnAttr::schema();

    use FieldType::*;

    define_field(ageDefineInline)
        .on_table(Student::table())
        .type_(Int)
        .value("oyelowo@codebreather.com")
        .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
        .permissions(for_permission(Select).where_(ageDefineInline.greater_than_or_equal(18))) // Single works
        .permissions(for_permission([Create, Update]).where_(firstName.is("Oyedayo"))) //Multiple
        .permissions([
            for_permission([Create, Delete]).where_(firstName.is("Oyelowo")),
            for_permission(Update).where_(ageDefineInline.less_than_or_equal(130)),
        ])
}

fn define_age_define_external_fn_path() -> DefineFieldStatement {
    use CrudType::*;
    let student_with_define_attr::Schema {
        ref ageDefineInline,
        ref firstName,
        ..
    } = StudentWithDefineAttr::schema();

    use FieldType::*;

    // let statement = define_field(Student::schema().age)

    define_field(ageDefineInline)
        .on_table(Student::table())
        .type_(Int)
        .value("oyelowo@codebreather.com")
        .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
        .permissions(for_permission(Select).where_(ageDefineInline.greater_than_or_equal(18))) // Single works
        .permissions(for_permission([Create, Update]).where_(firstName.is("Oyedayo"))) //Multiple
        .permissions([
            for_permission([Create, Delete]).where_(firstName.is("Oyelowo")),
            for_permission(Update).where_(ageDefineInline.less_than_or_equal(130)),
        ])
}

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

fn as_fn() -> SelectStatement {
    // would copy from student table to destination table.
    select(All).from(Student::table())
}

#[derive(Node, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table = student_fn_attrs,
    drop,
    flexible,
    schemafull,
    as_ = as_fn(),
    permissions = student_permissions
)]
struct StudentFnAttrs {
    id: SurrealId<StudentFnAttrs, String>,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table = student_with_granular_attributes,
    drop,
    flexible,
    schemafull,
    as_ = select(All).from(Student::table()),
    permissions = student_permissions()
)]
pub struct StudentWithGranularAttributes {
    id: SurrealId<Self, String>,
    first_name: String,
    last_name: String,
    #[surreal_orm(
        ty = int,
        value = 18,
        assert = cond(value().is_not(NONE)).and(value().gte(18)),
        permissions = for_permission([CrudType::Create, CrudType::Delete]).where_(StudentWithGranularAttributes::schema().firstName.is("Oyelowo"))
    )]
    age_inline_expr: u8,

    #[surreal_orm(
        ty = int,
        value = get_age_default_value(),
        assert = get_age_assertion,
        permissions = age_permissions
    )]
    age_default_external_function_invoked_expr: u8,

    #[surreal_orm(
        ty = int,
        value = get_age_by_group_default_value(AgeGroup::Teen),
        assert = get_age_assertion,
        permissions = age_permissions()
    )]
    age_teen_external_function_invoked_expr: u8,

    #[surreal_orm(
        ty = int,
        value = get_age_by_group_default_value(AgeGroup::Senior),
        assert = get_age_assertion
    )]
    age_senior_external_function_invoked_expr: u8,

    #[surreal_orm(
        ty = "int",
        value = get_age_by_group_default_value(AgeGroup::Child),
        permissions = age_permissions
    )]
    age_child_external_function_invoked_expr: u8,

    #[surreal_orm(
        ty = "int",
        value = get_age_by_group_default_value(AgeGroup::Adult)
    )]
    age_adult_external_function_invoked_expr: u8,

    #[surreal_orm(
        ty = "int",
        value = get_age_default_value,
        assert = get_age_assertion,
        permissions = age_permissions
    )]
    age_external_fn_attrs: u8,
    #[surreal_orm(
        ty = "int",
        value = get_age_default_value,
        assert = get_age_assertion,
        permissions = age_permissions
    )]
    age_mix_and_match_external_fn_inline_attrs: u8,

    #[surreal_orm(
        ty = duration,
        value = default_duration_value,
        assert = value().is_not(NONE)
    )]
    time_to_kelowna: Duration,

    #[surreal_orm(
        ty = "duration",
        value = sql::Duration(Duration::from_secs(60 * 60 * 24 * 7)),
        assert = value().is_not(NONE)
    )]
    time_to_kelowna_inline: Duration,
    // Even if ypu dont list the type for all links, the types are autogenerated at compile time
    // becuase I have enough info from the annotation to derive it
    #[surreal_orm(
        link_self = StudentWithGranularAttributes,
        ty = "record<student_with_granular_attributes>"
    )]
    best_friend: LinkSelf<StudentWithGranularAttributes>,

    #[surreal_orm(link_one = Book)]
    #[serde(rename = "unoBook")]
    fav_book: LinkOne<Book>,

    #[surreal_orm(link_one = Book, skip_serializing)]
    course: LinkOne<Book>,

    #[surreal_orm(link_many = Book, ty = "array<record<book>>")]
    #[serde(rename = "semesterCourses")]
    all_semester_courses: LinkMany<Book>,

    #[surreal_orm(relate(
        model = StudentWithGranularAttributesWritesBook,
        connection = "->writes->book"
    ))]
    #[serde(skip_serializing)]
    _written_books: Relate<Book>,

    #[surreal_orm(relate(
        model = StudentWithGranularAttributesWritesBlog,
        connection = "->writes->blog"
    ))]
    #[serde(skip_serializing)]
    _blogs: Relate<Blog>,
}

pub type StudentWithGranularAttributesWritesBook = Writes<StudentWithGranularAttributes, Book>;
pub type StudentWithGranularAttributesWritesBlog = Writes<StudentWithGranularAttributes, Blog>;

fn define_first_name(field: impl Into<Field>, table: Table) -> DefineFieldStatement {
    use CrudType::*;
    let student_with_define_attr::Schema {
        ref ageDefineInline,
        ref firstName,
        ..
    } = StudentWithDefineAttr::schema();

    define_field(field)
        .on_table(table)
        .type_(FieldType::String)
        .value("Oyelowo")
        .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
        .permissions([
            for_permission(Select).where_(ageDefineInline.gte(18)),
            for_permission([Create, Update]).where_(firstName.is("Oyedayo")),
        ])
}

fn define_last_name() -> DefineFieldStatement {
    use CrudType::*;
    let student_with_define_attr::Schema {
        ref ageDefineInline,
        ref lastName,
        ..
    } = StudentWithDefineAttr::schema();

    define_field(lastName)
        .on_table(StudentWithDefineAttr::table())
        .type_(FieldType::String)
        .value("Oyedayo")
        .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
        .permissions([
            for_permission(Select).where_(ageDefineInline.gte(18)),
            for_permission([Create, Update]).where_(lastName.is("Oyedayo")),
        ])
}

fn define_last_name_external_fn_attr() -> DefineFieldStatement {
    use CrudType::*;
    let student_with_define_attr::Schema {
        ref ageDefineInline,
        ref lastName,
        ..
    } = StudentWithDefineAttr::schema();

    define_field(lastName)
        .on_table(StudentWithDefineAttr::table())
        .type_(FieldType::String)
        .value("Oyedayo")
        .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
        .permissions([
            for_permission(Select).where_(ageDefineInline.gte(18)),
            for_permission([Create, Update]).where_(lastName.is("Oyedayo")),
        ])
}
fn define_student_with_define_attr() -> DefineTableStatement {
    let student::Schema {
        ref age,
        ref firstName,
        ref lastName,
        ..
    } = Student::schema();
    use CrudType::*;

    define_table(StudentWithDefineAttr::table())
        .drop()
        .as_(
            select(All)
                .from(Student::table())
                .where_(firstName.is("Rust"))
                .order_by(age.numeric().desc())
                .limit(20)
                .start(5),
        )
        .schemafull()
        .permissions(for_permission(Select).where_(age.greater_than_or_equal(18))) // Single works
        .permissions(for_permission([Create, Delete]).where_(lastName.is("Oye"))) //Multiple
        .permissions([
            for_permission([Create, Delete]).where_(lastName.is("Oyedayo")),
            for_permission(Update).where_(age.less_than_or_equal(130)),
        ])
}

fn define_age(field: impl Into<Field>) -> DefineFieldStatement {
    use CrudType::*;
    let student::Schema { age, firstName, .. } = Student::schema();

    use FieldType::*;

    define_field(field)
        .on_table(Student::table())
        .type_(Int)
        .value("oyelowo@codebreather.com")
        .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
        .permissions(for_permission(Select).where_(age.greater_than_or_equal(18))) // Single works
        .permissions(for_permission([Create, Update]).where_(firstName.is("Oyedayo"))) //Multiple
        .permissions([
            for_permission([Create, Delete]).where_(firstName.is("Oyelowo")),
            for_permission(Update).where_(age.less_than_or_equal(130)),
        ])
}
#[derive(Node, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table = student_with_define_attr,
    define = define_student_with_define_attr()
)]
pub struct StudentWithDefineAttr {
    // When using Typedbuilder, you cannot use Self like other
    // places as TypedBuilder does not support that, in that
    // case, just use the struct name explicitly.
    // So, 'SurrealId<StudentWithDefineAttr, String>,' instead of
    // SurrealId<Self, String>,
    id: SurrealId<StudentWithDefineAttr, String>,
    #[surreal_orm(
        ty = string,
        define = define_first_name(StudentWithDefineAttr::schema().firstName, StudentWithDefineAttr::table())
    )]
    first_name: String,

    #[surreal_orm(ty = string, define = define_last_name)]
    last_name: String,

    #[surreal_orm(ty = string, define = define_last_name_external_fn_attr)]
    last_name_external_fn_attr: String,

    #[surreal_orm(
        ty = int,
        define = define_field(StudentWithDefineAttr::schema().ageDefineInline).on_table(Student::table()).type_(FieldType::Int).value("oyelowo@codebreather.com")
    )]
    age_define_inline: u8,

    #[surreal_orm(
        ty = int,
        define = define_age(StudentWithDefineAttr::schema().ageDefineExternalInvoke)
    )]
    age_define_external_invoke: u8,

    #[surreal_orm(ty = "int", define = define_age_define_external_fn_path)]
    age_define_external_fn_path: u8,

    // Even if ypu dont list the type for all links, the types are autogenerated at compile time
    // becuase I have enough info from the annotation to derive it
    #[surreal_orm(
        link_self = StudentWithDefineAttr,
        ty = "record<student_with_define_attr>"
    )]
    best_friend: LinkSelf<StudentWithDefineAttr>,

    #[surreal_orm(link_one = Book)]
    #[serde(rename = "unoBook")]
    fav_book: LinkOne<Book>,

    #[surreal_orm(link_one = Book, skip_serializing)]
    course: LinkOne<Book>,

    #[surreal_orm(link_many = Book, ty = "array<record<book>>")]
    #[serde(rename = "semesterCourses")]
    all_semester_courses: LinkMany<Book>,

    #[surreal_orm(relate(
        model = StudentWithDefineAttrWritesBook,
        connection = "->writes->book"
    ))]
    #[serde(skip_serializing)]
    _written_books: Relate<Book>,

    #[surreal_orm(relate(
        model = StudentWithDefineAttrWritesBlog,
        connection = "->writes->blog"
    ))]
    #[serde(skip_serializing)]
    _blogs: Relate<Blog>,
}

pub type StudentWithDefineAttrWritesBook = Writes<StudentWithDefineAttr, Book>;
pub type StudentWithDefineAttrWritesBlog = Writes<StudentWithDefineAttr, Blog>;

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table = student_with_define_fn_attr,
    define = define_student_with_define_attr
)]
pub struct StudentWithDefineFnAttr {
    id: SurrealId<Self, String>,
    // id: SurrealId<StudentWithDefineFnAttr, String>,
    // can be as simple as this
    #[surreal_orm(ty = string, define = define_last_name)]
    last_name: String,

    #[surreal_orm(ty = string, define = define_last_name)]
    last_name_external_fn_attr: String,

    // or go even crazier
    #[surreal_orm(
        ty = string,
        define = define_first_name(StudentWithDefineFnAttr::schema().firstName, StudentWithDefineFnAttr::table())
    )]
    first_name: String,

    #[surreal_orm(
        ty = int,
        define = define_field(StudentWithDefineFnAttr::schema().ageDefineInline).on_table(Student::table()).type_(FieldType::Int).value("oyelowo@codebreather.com")
    )]
    age_define_inline: u8,

    #[surreal_orm(
        ty = int,
        define = define_age(StudentWithDefineFnAttr::schema().ageDefineExternalInvoke)
    )]
    age_define_external_invoke: u8,

    #[surreal_orm(ty = int, define = age_define_external_fn_path)]
    age_define_external_fn_path: u8,

    // Even if ypu dont list the type for all links, the types are autogenerated at compile time
    // becuase I have enough info from the annotation to derive it
    #[surreal_orm(
        link_self = StudentWithDefineFnAttr,
        ty = "record<student_with_define_fn_attr>"
    )]
    best_friend: LinkSelf<StudentWithDefineFnAttr>,

    #[surreal_orm(link_one = Book)]
    #[serde(rename = "unoBook")]
    fav_book: LinkOne<Book>,

    #[surreal_orm(link_one = "Book", skip_serializing)]
    course: LinkOne<Book>,

    #[surreal_orm(link_many = Book, ty = "array<record<book>>")]
    #[serde(rename = "semesterCourses")]
    all_semester_courses: LinkMany<Book>,

    #[surreal_orm(relate(
        model = StudentWithDefineFnAttrWritesBook,
        connection = "->writes->book"
    ))]
    #[serde(skip_serializing)]
    _written_books: Relate<Book>,

    #[surreal_orm(relate(
        model = StudentWithDefineFnAttrWritesBlog,
        connection = "->writes->blog"
    ))]
    #[serde(skip_serializing)]
    _blogs: Relate<Blog>,
}

pub type StudentWithDefineFnAttrWritesBook = Writes<StudentWithDefineFnAttr, Book>;
pub type StudentWithDefineFnAttrWritesBlog = Writes<StudentWithDefineFnAttr, Blog>;

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = student)]
pub struct Student {
    id: SurrealId<Self, String>,
    // id: SurrealId<Student, String>,
    first_name: String,
    last_name: String,
    #[surreal_orm(
        ty = int,
        value = 18,
        assert = cond(value().is_not(NONE)).and(value().gte(18)),
        permissions = age_permissions
    )]
    age: u8,

    #[surreal_orm(ty = "int")]
    score: u8,

    // Even if ypu dont list the type for all links, the types are autogenerated at compile time
    // becuase I have enough info from the annotation to derive it
    #[surreal_orm(link_self = Student, ty = "record<student>")]
    best_friend: LinkSelf<Student>,

    #[surreal_orm(link_one = Book)]
    #[serde(rename = "unoBook")]
    fav_book: LinkOne<Book>,

    #[surreal_orm(link_one = Book, skip_serializing)]
    course: LinkOne<Book>,

    #[surreal_orm(link_many = Book, ty = "array<record<book>>")]
    // #[surreal_orm(link_many = "Book", ty = "array", item_type = "record(book)")]
    #[serde(rename = "semesterCourses")]
    all_semester_courses: LinkMany<Book>,

    #[surreal_orm(relate(model = StudentWritesBook, connection = "->writes->book"))]
    #[serde(skip_serializing)]
    _written_books: Relate<Book>,

    #[surreal_orm(relate(model = StudentWritesBlog, connection = "->writes->blog"))]
    #[serde(skip_serializing)]
    _blogs: Relate<Blog>,
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
            _written_books: Default::default(),
            _blogs: Default::default(),
        }
    }
}

#[derive(surreal_orm::Edge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = writes)]
pub struct Writes<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Self>,
    // pub id: SurrealSimpleId<Writes<In, Out>>,
    #[serde(skip_serializing)]
    #[surreal_orm(link_many="In")]
    pub r#in: LinkMany<In>,
    #[surreal_orm(link_many="Out")]
    #[serde(skip_serializing)]
    pub out: LinkMany<Out>,
    pub time_written: Duration,
    pub count: i32,
}

pub type StudentWritesBook = Writes<Student, Book>;
pub type StudentWritesBlog = Writes<Student, Blog>;

#[derive(Edge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = likes)]
pub struct Likes<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Self>,
    // pub id: SurrealSimpleId<Likes<In, Out>>,
    #[serde(rename = "in", skip_serializing)]
    #[surreal_orm(link_many = In)]
    pub in_: LinkMany<In>,

    #[serde(skip_serializing)]
    #[surreal_orm(link_many = Out)]
    pub out: LinkMany<Out>,
    pub likes_count: u64,
}
pub type StudentLiksBook = Likes<Student, Book>;

#[derive(Node, TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = book)]
pub struct Book {
    id: SurrealSimpleId<Book>,
    title: String,
    content: String,
}

#[derive(Node, TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = blog)]
pub struct Blog {
    id: SurrealSimpleId<Blog>,
    title: String,
    content: String,
}

// #[derive(Object, TypedBuilder, Serialize, Deserialize, Debug, Clone, Default)]
// #[serde(rename_all = "camelCase")]
// // #[surreal_orm(table = rocket)]
// pub struct Rocket {
//     // id: SurrealSimpleId<Rocket>,
//     name: String,
//     sama: &'static str,
// }
//
//

