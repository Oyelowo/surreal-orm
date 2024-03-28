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
    statements::{
        define_field, define_table, for_permission, select, DefineFieldStatement,
        DefineTableStatement, Permissions, SelectStatement,
    },
    *,
};
use surreal_orm::{Model, Node};

use surrealdb::sql;

use typed_builder::TypedBuilder;
use CrudType::*;

fn age_permissions() -> Permissions {
    let StudentWithGranularAttributes::Schema {
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
    let StudentWithGranularAttributes::Schema {
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
fn default_duration_value() -> Duration {
    Duration::from_secs(60 * 60 * 24 * 7)
}

fn age_define_external_fn_path() -> DefineFieldStatement {
    let StudentWithDefineFnAttr::Schema {
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
    as_ = as_fn,
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
        value = Duration::from_secs(60 * 60 * 24 * 7),
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
    #[serde(rename = "in", skip_serializing)]
    pub in_: LinkOne<In>,
    #[serde(skip_serializing)]
    pub out: LinkOne<Out>,
    pub time_written: Duration,
    pub count: i32,
}

pub type StudentWritesBook = Writes<Student, Book>;
pub type StudentWritesBlog = Writes<Student, Blog>;

#[derive( Edge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = likes)]
pub struct Likes<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Self>,
    // pub id: SurrealSimpleId<Likes<In, Out>>,
    #[serde(rename = "in", skip_serializing)]
    pub in_: LinkOne<In>,
    #[serde(skip_serializing)]
    pub out: LinkOne<Out>,
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


// =====================================
// Recursive expansion of the Edge macro
// =====================================

::core::compile_error!{
  "Edge struct must include 'in' and 'out'"
}use surreal_orm::{
  ToRaw as _
};
impl <In:Node,Out:Node>Likes<In,Out>{
  pub const fn __get_serializable_field_names() -> [&'static str;
  4usize]{
    unimplemented!()
  }
  }
impl <In:Node,Out:Node>surreal_orm::SchemaGetter for Likes<In,Out>{
  type Schema = ________internal_likes_schema::Likes<In,Out> ;
  fn schema() -> Self::Schema {
    likes::Schema:: <In,Out> ::new()
  }
  fn schema_prefixed(prefix:impl ::std::convert::Into<surreal_orm::ValueLike>) -> Self::Schema {
    likes::Schema:: <In,Out> ::new_prefixed(prefix)
  }
  }
impl <In:Node,Out:Node>surreal_orm::PartialUpdater for Likes<In,Out>{
  type StructPartial = LikesPartial<In,Out> ;
  type PartialBuilder = LikesPartialBuilder<In,Out> ;
  fn partial_builder() -> Self::PartialBuilder {
    LikesPartialBuilder::default()
  }
  }
#[allow(non_snake_case)]
impl <In:Node,Out:Node>surreal_orm::Edge for Likes<In,Out>{
  type In = In;
  type Out = Out;
  type TableNameChecker = ________internal_likes_schema::TableNameStaticChecker;
  #[allow(non_snake_case)]
  fn get_table() -> surreal_orm::Table {
    "likes".into()
  }
  }
#[allow(non_snake_case)]
#[derive(surreal_orm::serde::Serialize,Debug,Clone,Default)]
pub struct LikesPartial<In:Node,Out:Node>{
  #[serde(skip)]
  _____struct_marker_ident:surreal_orm::Maybe< ::std::marker::PhantomData<(In,Out)> > , #[serde(skip_serializing_if = "surreal_orm::Maybe::is_none",rename = "likesCount")]
  pub likes_count:surreal_orm::Maybe<u64>
}
#[derive(surreal_orm::serde::Serialize,Debug,Clone,Default)]
pub struct LikesPartialBuilder<In:Node,Out:Node>(LikesPartial<In,Out>);
impl <In:Node,Out:Node>LikesPartialBuilder<In,Out>{
  pub fn in_(mut self,value:LinkOne<In>) -> Self {
    self.0.in_ = surreal_orm::Maybe::Some(value);
    self
  }
  pub fn out(mut self,value:LinkOne<Out>) -> Self {
    self.0.out = surreal_orm::Maybe::Some(value);
    self
  }
  pub fn likes_count(mut self,value:u64) -> Self {
    self.0.likes_count = surreal_orm::Maybe::Some(value);
    self
  }
  pub fn build(self) -> LikesPartial<In,Out>{
    self.0
  }
  }
#[allow(non_snake_case)]
#[derive(surreal_orm::serde::Serialize, Debug,Clone)]
pub struct LikesRenamedCreator {
  pub r#in: &'static str,pub out: &'static str,pub likesCount: &'static str
}
#[allow(non_snake_case)]
impl <In:Node,Out:Node>surreal_orm::Model for Likes<In,Out>{
  type Id = SurrealSimpleId<Self> ;
  type StructRenamedCreator = LikesRenamedCreator;
  fn table() -> surreal_orm::Table {
    "likes".into()
  }
  fn get_id(self) -> Self::Id {
    self.id
  }
  fn get_id_as_thing(&self) -> surreal_orm::sql::Thing {
    surreal_orm::sql::thing(self.id.to_raw().as_str()).unwrap()
  }
  fn get_serializable_fields() ->  ::std::vec::Vec<surreal_orm::Field>{
    return::std::vec!["id".into(),"in".into(),"out".into(),"likesCount".into()]
  }
  fn get_linked_fields() ->  ::std::vec::Vec<surreal_orm::Field>{
    return::std::vec![]
  }
  fn get_link_one_fields() ->  ::std::vec::Vec<surreal_orm::Field>{
    return::std::vec![]
  }
  fn get_link_self_fields() ->  ::std::vec::Vec<surreal_orm::Field>{
    return::std::vec![]
  }
  fn get_link_one_and_self_fields() ->  ::std::vec::Vec<surreal_orm::Field>{
    return::std::vec![]
  }
  fn get_link_many_fields() ->  ::std::vec::Vec<surreal_orm::Field>{
    return::std::vec![]
  }
  fn define_table() -> surreal_orm::Raw {
    surreal_orm::statements::define_table(Self::table()).to_raw()
  }
  fn define_fields() ->  ::std::vec::Vec<surreal_orm::Raw>{
    vec![surreal_orm::statements::define_field(surreal_orm::Field::new("id")).on_table(surreal_orm::Table::from(Self::table())).to_raw(),surreal_orm::statements::define_field(surreal_orm::Field::new("in")).on_table(surreal_orm::Table::from(Self::table())).to_raw(),surreal_orm::statements::define_field(surreal_orm::Field::new("out")).on_table(surreal_orm::Table::from(Self::table())).to_raw(),surreal_orm::statements::define_field(surreal_orm::Field::new("likesCount")).on_table(surreal_orm::Table::from(Self::table())).to_raw()]
  }
  fn get_field_meta() ->  ::std::vec::Vec<surreal_orm::FieldMetadata>{
    return vec![surreal_orm::FieldMetadata {
      name:"id".into(),old_name: ::std::option::Option::None,definition: ::std::vec![surreal_orm::statements::define_field(surreal_orm::Field::new("id")).on_table(surreal_orm::Table::from(Self::table())).to_raw()]
    },surreal_orm::FieldMetadata {
      name:"in".into(),old_name: ::std::option::Option::None,definition: ::std::vec![surreal_orm::statements::define_field(surreal_orm::Field::new("in")).on_table(surreal_orm::Table::from(Self::table())).to_raw()]
    },surreal_orm::FieldMetadata {
      name:"out".into(),old_name: ::std::option::Option::None,definition: ::std::vec![surreal_orm::statements::define_field(surreal_orm::Field::new("out")).on_table(surreal_orm::Table::from(Self::table())).to_raw()]
    },surreal_orm::FieldMetadata {
      name:"likesCount".into(),old_name: ::std::option::Option::None,definition: ::std::vec![surreal_orm::statements::define_field(surreal_orm::Field::new("likesCount")).on_table(surreal_orm::Table::from(Self::table())).to_raw()]
    }]
  }
  }
#[allow(non_snake_case)]
pub mod likes {
  pub use super::________internal_likes_schema::_____schema_def::Schema;
}#[allow(non_snake_case)]
mod ________internal_likes_schema {
  use surreal_orm::Node;
  use surreal_orm::Parametric as _;
  use surreal_orm::Buildable as _;
  use surreal_orm::Erroneous as _;
  pub struct TableNameStaticChecker {
    pub likes: ::std::string::String,
  }
  pub(super)mod _____field_names {
    use super::super:: * ;
    use surreal_orm::Parametric as _;
    use surreal_orm::Buildable as _;
    #[derive(Debug,Clone)]
    pub struct __Id__(pub surreal_orm::Field);
    
    impl ::std::convert::From< &str>for __Id__ {
      fn from(field_name: &str) -> Self {
        Self(surreal_orm::Field::new(field_name))
      }
    
      }
    impl ::std::convert::From<surreal_orm::Field>for __Id__ {
      fn from(field_name:surreal_orm::Field) -> Self {
        Self(field_name)
      }
    
      }
    impl ::std::convert::From< &__Id__>for surreal_orm::ValueLike {
      fn from(value: &__Id__) -> Self {
        let field:surreal_orm::Field = value.into();
        field.into()
      }
    
      }
    impl ::std::convert::From<__Id__>for surreal_orm::ValueLike {
      fn from(value:__Id__) -> Self {
        let field:surreal_orm::Field = value.into();
        field.into()
      }
    
      }
    impl ::std::convert::From< &__Id__>for surreal_orm::Field {
      fn from(field_name: &__Id__) -> Self {
        field_name.0.clone()
      }
    
      }
    impl ::std::convert::From<__Id__>for surreal_orm::Field {
      fn from(field_name:__Id__) -> Self {
        field_name.0
      }
    
      }
    impl ::std::ops::Deref for __Id__ {
      type Target = surreal_orm::Field;
      fn deref(&self) ->  &Self::Target {
        &self.0
      }
    
      }
    impl ::std::ops::DerefMut for __Id__ {
      fn deref_mut(&mut self) ->  &mut Self::Target {
        &mut self.0
      }
    
      }
    impl <T:surreal_orm::serde::Serialize> ::std::convert::From<self::__Id__>for surreal_orm::SetterArg<T>{
      fn from(value:self::__Id__) -> Self {
        Self::Field(value.into())
      }
    
      }
    impl <T:surreal_orm::serde::Serialize> ::std::convert::From< &self::__Id__>for surreal_orm::SetterArg<T>{
      fn from(value: &self::__Id__) -> Self {
        Self::Field(value.into())
      }
    
      }
    #[derive(Debug,Clone)]
    pub struct __In__(pub surreal_orm::Field);
    
    impl ::std::convert::From< &str>for __In__ {
      fn from(field_name: &str) -> Self {
        Self(surreal_orm::Field::new(field_name))
      }
    
      }
    impl ::std::convert::From<surreal_orm::Field>for __In__ {
      fn from(field_name:surreal_orm::Field) -> Self {
        Self(field_name)
      }
    
      }
    impl ::std::convert::From< &__In__>for surreal_orm::ValueLike {
      fn from(value: &__In__) -> Self {
        let field:surreal_orm::Field = value.into();
        field.into()
      }
    
      }
    impl ::std::convert::From<__In__>for surreal_orm::ValueLike {
      fn from(value:__In__) -> Self {
        let field:surreal_orm::Field = value.into();
        field.into()
      }
    
      }
    impl ::std::convert::From< &__In__>for surreal_orm::Field {
      fn from(field_name: &__In__) -> Self {
        field_name.0.clone()
      }
    
      }
    impl ::std::convert::From<__In__>for surreal_orm::Field {
      fn from(field_name:__In__) -> Self {
        field_name.0
      }
    
      }
    impl ::std::ops::Deref for __In__ {
      type Target = surreal_orm::Field;
      fn deref(&self) ->  &Self::Target {
        &self.0
      }
    
      }
    impl ::std::ops::DerefMut for __In__ {
      fn deref_mut(&mut self) ->  &mut Self::Target {
        &mut self.0
      }
    
      }
    impl <T:surreal_orm::serde::Serialize> ::std::convert::From<self::__In__>for surreal_orm::SetterArg<T>{
      fn from(value:self::__In__) -> Self {
        Self::Field(value.into())
      }
    
      }
    impl <T:surreal_orm::serde::Serialize> ::std::convert::From< &self::__In__>for surreal_orm::SetterArg<T>{
      fn from(value: &self::__In__) -> Self {
        Self::Field(value.into())
      }
    
      }
    impl<In: Node> surreal_orm::SetterAssignable<LinkOne<In> >for self::__In__{}
    
    impl surreal_orm::Patchable<LinkOne<In> >for self::__In__{}
    
    #[derive(Debug,Clone)]
    pub struct __Out__(pub surreal_orm::Field);
    
    impl ::std::convert::From< &str>for __Out__ {
      fn from(field_name: &str) -> Self {
        Self(surreal_orm::Field::new(field_name))
      }
    
      }
    impl ::std::convert::From<surreal_orm::Field>for __Out__ {
      fn from(field_name:surreal_orm::Field) -> Self {
        Self(field_name)
      }
    
      }
    impl ::std::convert::From< &__Out__>for surreal_orm::ValueLike {
      fn from(value: &__Out__) -> Self {
        let field:surreal_orm::Field = value.into();
        field.into()
      }
    
      }
    impl ::std::convert::From<__Out__>for surreal_orm::ValueLike {
      fn from(value:__Out__) -> Self {
        let field:surreal_orm::Field = value.into();
        field.into()
      }
    
      }
    impl ::std::convert::From< &__Out__>for surreal_orm::Field {
      fn from(field_name: &__Out__) -> Self {
        field_name.0.clone()
      }
    
      }
    impl ::std::convert::From<__Out__>for surreal_orm::Field {
      fn from(field_name:__Out__) -> Self {
        field_name.0
      }
    
      }
    impl ::std::ops::Deref for __Out__ {
      type Target = surreal_orm::Field;
      fn deref(&self) ->  &Self::Target {
        &self.0
      }
    
      }
    impl ::std::ops::DerefMut for __Out__ {
      fn deref_mut(&mut self) ->  &mut Self::Target {
        &mut self.0
      }
    
      }
    impl <T:surreal_orm::serde::Serialize> ::std::convert::From<self::__Out__>for surreal_orm::SetterArg<T>{
      fn from(value:self::__Out__) -> Self {
        Self::Field(value.into())
      }
    
      }
    impl <T:surreal_orm::serde::Serialize> ::std::convert::From< &self::__Out__>for surreal_orm::SetterArg<T>{
      fn from(value: &self::__Out__) -> Self {
        Self::Field(value.into())
      }
    
      }
    impl surreal_orm::SetterAssignable<LinkOne<Out> >for self::__Out__{}
    
    impl surreal_orm::Patchable<LinkOne<Out> >for self::__Out__{}
    
    #[derive(Debug,Clone)]
    pub struct __LikesCount__(pub surreal_orm::Field);
    
    impl ::std::convert::From< &str>for __LikesCount__ {
      fn from(field_name: &str) -> Self {
        Self(surreal_orm::Field::new(field_name))
      }
    
      }
    impl ::std::convert::From<surreal_orm::Field>for __LikesCount__ {
      fn from(field_name:surreal_orm::Field) -> Self {
        Self(field_name)
      }
    
      }
    impl ::std::convert::From< &__LikesCount__>for surreal_orm::ValueLike {
      fn from(value: &__LikesCount__) -> Self {
        let field:surreal_orm::Field = value.into();
        field.into()
      }
    
      }
    impl ::std::convert::From<__LikesCount__>for surreal_orm::ValueLike {
      fn from(value:__LikesCount__) -> Self {
        let field:surreal_orm::Field = value.into();
        field.into()
      }
    
      }
    impl ::std::convert::From< &__LikesCount__>for surreal_orm::Field {
      fn from(field_name: &__LikesCount__) -> Self {
        field_name.0.clone()
      }
    
      }
    impl ::std::convert::From<__LikesCount__>for surreal_orm::Field {
      fn from(field_name:__LikesCount__) -> Self {
        field_name.0
      }
    
      }
    impl ::std::ops::Deref for __LikesCount__ {
      type Target = surreal_orm::Field;
      fn deref(&self) ->  &Self::Target {
        &self.0
      }
    
      }
    impl ::std::ops::DerefMut for __LikesCount__ {
      fn deref_mut(&mut self) ->  &mut Self::Target {
        &mut self.0
      }
    
      }
    impl <T:surreal_orm::serde::Serialize> ::std::convert::From<self::__LikesCount__>for surreal_orm::SetterArg<T>{
      fn from(value:self::__LikesCount__) -> Self {
        Self::Field(value.into())
      }
    
      }
    impl <T:surreal_orm::serde::Serialize> ::std::convert::From< &self::__LikesCount__>for surreal_orm::SetterArg<T>{
      fn from(value: &self::__LikesCount__) -> Self {
        Self::Field(value.into())
      }
    
      }
    impl surreal_orm::SetterAssignable<u64>for self::__LikesCount__{}
    
    impl surreal_orm::Patchable<u64>for self::__LikesCount__{}
    
    impl surreal_orm::SetterNumeric<u64>for self::__LikesCount__{}
    
    impl ::std::convert::From<self::__LikesCount__>for surreal_orm::NumberLike {
      fn from(val:self::__LikesCount__) -> Self {
        val.0.into()
      }
    
      }
    impl ::std::convert::From< &self::__LikesCount__>for surreal_orm::NumberLike {
      fn from(val: &self::__LikesCount__) -> Self {
        val.clone().0.into()
      }
    
      }
    impl <T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Add<T>for __LikesCount__ {
      type Output = surreal_orm::Operation;
      fn add(self,rhs:T) -> Self::Output {
        let rhs:surreal_orm::NumberLike = rhs.into();
        surreal_orm::Operation {
          query_string:format!("{} + {}",self.build(),rhs.build()),bindings:[self.get_bindings(),rhs.get_bindings()].concat(),errors:vec![],
        }
      }
    
      }
    impl <T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Sub<T>for __LikesCount__ {
      type Output = surreal_orm::Operation;
      fn sub(self,rhs:T) -> Self::Output {
        let rhs:surreal_orm::NumberLike = rhs.into();
        surreal_orm::Operation {
          query_string:format!("{} - {}",self.build(),rhs.build()),bindings:[self.get_bindings(),rhs.get_bindings()].concat(),errors:vec![],
        }
      }
    
      }
    impl <T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Mul<T>for __LikesCount__ {
      type Output = surreal_orm::Operation;
      fn mul(self,rhs:T) -> Self::Output {
        let rhs:surreal_orm::NumberLike = rhs.into();
        surreal_orm::Operation {
          query_string:format!("{} * {}",self.build(),rhs.build()),bindings:[self.get_bindings(),rhs.get_bindings()].concat(),errors:vec![],
        }
      }
    
      }
    impl <T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Div<T>for __LikesCount__ {
      type Output = surreal_orm::Operation;
      fn div(self,rhs:T) -> Self::Output {
        let rhs:surreal_orm::NumberLike = rhs.into();
        surreal_orm::Operation {
          query_string:format!("{} / {}",self.build(),rhs.build()),bindings:[self.get_bindings(),rhs.get_bindings()].concat(),errors:vec![],
        }
      }
    
      }
    impl <T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Add<T>for&__LikesCount__ {
      type Output = surreal_orm::Operation;
      fn add(self,rhs:T) -> Self::Output {
        let rhs:surreal_orm::NumberLike = rhs.into();
        surreal_orm::Operation {
          query_string:format!("{} + {}",self.build(),rhs.build()),bindings:[self.get_bindings(),rhs.get_bindings()].concat(),errors:vec![],
        }
      }
    
      }
    impl <T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Sub<T>for&__LikesCount__ {
      type Output = surreal_orm::Operation;
      fn sub(self,rhs:T) -> Self::Output {
        let rhs:surreal_orm::NumberLike = rhs.into();
        surreal_orm::Operation {
          query_string:format!("{} - {}",self.build(),rhs.build()),bindings:[self.get_bindings(),rhs.get_bindings()].concat(),errors:vec![],
        }
      }
    
      }
    impl <T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Mul<T>for&__LikesCount__ {
      type Output = surreal_orm::Operation;
      fn mul(self,rhs:T) -> Self::Output {
        let rhs:surreal_orm::NumberLike = rhs.into();
        surreal_orm::Operation {
          query_string:format!("{} * {}",self.build(),rhs.build()),bindings:[self.get_bindings(),rhs.get_bindings()].concat(),errors:vec![],
        }
      }
    
      }
    impl <T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Div<T>for&__LikesCount__ {
      type Output = surreal_orm::Operation;
      fn div(self,rhs:T) -> Self::Output {
        let rhs:surreal_orm::NumberLike = rhs.into();
        surreal_orm::Operation {
          query_string:format!("{} / {}",self.build(),rhs.build()),bindings:[self.get_bindings(),rhs.get_bindings()].concat(),errors:vec![],
        }
      }
    
      }
  
    }pub mod _____schema_def {
    use super::_____field_names;
    #[allow(non_snake_case)]
    #[derive(Debug,Clone)]
    pub struct Schema<In,Out>{
      pub id:_____field_names::__Id__,pub in:_____field_names::__In__,pub out:_____field_names::__Out__,pub likesCount:_____field_names::__LikesCount__,pub(super)___________graph_traversal_string: ::std::string::String,pub(super)___________bindings:surreal_orm::BindingsList,pub(super)___________errors: ::std::vec::Vec< ::std::string::String> ,pub(super)_____struct_marker_ident: ::std::marker::PhantomData<(In,Out)> ,
    }
  
    }pub type Likes<In,Out>  = _____schema_def::Schema<In,Out> ;
  impl <In:Node,Out:Node>surreal_orm::Buildable for Likes<In,Out>{
    fn build(&self) ->  ::std::string::String {
      self.___________graph_traversal_string.to_string()
    }
  
    }
  impl <In:Node,Out:Node>surreal_orm::Parametric for Likes<In,Out>{
    fn get_bindings(&self) -> surreal_orm::BindingsList {
      self.___________bindings.to_vec()
    }
  
    }
  impl <In:Node,Out:Node>surreal_orm::Erroneous for Likes<In,Out>{
    fn get_errors(&self) ->  ::std::vec::Vec< ::std::string::String>{
      self.___________errors.to_vec()
    }
  
    }
  impl <In:Node,Out:Node>surreal_orm::Aliasable for Likes<In,Out>{}
  
  impl <In:Node,Out:Node>surreal_orm::Parametric for&Likes<In,Out>{
    fn get_bindings(&self) -> surreal_orm::BindingsList {
      self.___________bindings.to_vec()
    }
  
    }
  impl <In:Node,Out:Node>surreal_orm::Buildable for&Likes<In,Out>{
    fn build(&self) ->  ::std::string::String {
      self.___________graph_traversal_string.to_string()
    }
  
    }
  impl <In:Node,Out:Node>surreal_orm::Erroneous for&Likes<In,Out>{
    fn get_errors(&self) ->  ::std::vec::Vec< ::std::string::String>{
      self.___________errors.to_vec()
    }
  
    }
  impl <In:Node,Out:Node>Likes<In,Out>{
    pub fn new() -> Self {
      Self {
        id:"id".into(),in:"in".into(),out:"out".into(),likesCount:"likesCount".into(),___________graph_traversal_string:"".into(),___________bindings: ::std::vec![],___________errors: ::std::vec![],_____struct_marker_ident: ::std::marker::PhantomData,
      }
    }
    pub fn new_prefixed(prefix:impl ::std::convert::Into<surreal_orm::ValueLike>) -> Self {
      let prefix:surreal_orm::ValueLike = prefix.into();
      Self {
        id:surreal_orm::Field::new(format!("{}.{}",prefix.build(),"id")).with_bindings(prefix.get_bindings()).into(),in:surreal_orm::Field::new(format!("{}.{}",prefix.build(),"in")).with_bindings(prefix.get_bindings()).into(),out:surreal_orm::Field::new(format!("{}.{}",prefix.build(),"out")).with_bindings(prefix.get_bindings()).into(),likesCount:surreal_orm::Field::new(format!("{}.{}",prefix.build(),"likesCount")).with_bindings(prefix.get_bindings()).into(),___________graph_traversal_string:prefix.build(),___________bindings:prefix.get_bindings(),___________errors: ::std::vec![],_____struct_marker_ident: ::std::marker::PhantomData,
      }
    }
    pub fn empty() -> Self {
      Self {
        id:"".into(),in:"".into(),out:"".into(),likesCount:"".into(),___________graph_traversal_string:"".into(),___________bindings: ::std::vec![],___________errors: ::std::vec![],_____struct_marker_ident: ::std::marker::PhantomData,
      }
    }
    pub fn __________connect_edge_to_graph_traversal_string(connection:impl surreal_orm::Buildable+surreal_orm::Parametric+surreal_orm::Erroneous,clause:impl ::std::convert::Into<surreal_orm::EdgeClause> ,) -> Self {
      let mut schema_instance = Self::empty();
      let clause:surreal_orm::EdgeClause = clause.into();
      let bindings = [connection.get_bindings().as_slice(),clause.get_bindings().as_slice()].concat();
      let bindings = bindings.as_slice();
      schema_instance.___________bindings = bindings.into();
      let errors = [connection.get_errors().as_slice(),clause.get_errors().as_slice()].concat();
      let errors = errors.as_slice();
      schema_instance.___________errors = errors.into();
      let schema_edge_str_with_arrow = format!("{}{}",connection.build(),clause.build(),);
      schema_instance.___________graph_traversal_string.push_str(schema_edge_str_with_arrow.as_str());
      let ___________graph_traversal_string =  &schema_instance.___________graph_traversal_string;
      schema_instance.id = schema_instance.id.set_graph_string(format!("{}.{}",___________graph_traversal_string,"id")).____________update_many_bindings(bindings).into();
      schema_instance.in = schema_instance.in.set_graph_string(format!("{}.{}",___________graph_traversal_string,"in")).____________update_many_bindings(bindings).into();
      schema_instance.out = schema_instance.out.set_graph_string(format!("{}.{}",___________graph_traversal_string,"out")).____________update_many_bindings(bindings).into();
      schema_instance.likesCount = schema_instance.likesCount.set_graph_string(format!("{}.{}",___________graph_traversal_string,"likesCount")).____________update_many_bindings(bindings).into();
      schema_instance
    }
  
    }
  }#[allow(non_snake_case)]
fn _________test_________internal_likes_schema_static_funcs_name__________<In:Node,Out:Node>(){
  surreal_orm::validators::assert_impl_one!(SurrealSimpleId<Likes<In,Out> > : ::std::convert::Into<surreal_orm::sql::Thing>);
  surreal_orm::validators::assert_impl_one!(LinkOne<In> : ::std::convert::Into<surreal_orm::sql::Thing>);
  surreal_orm::validators::assert_impl_one!(LinkOne<Out> : ::std::convert::Into<surreal_orm::sql::Thing>);
  surreal_orm::validators::assert_type_is_int:: <u64>();
}


