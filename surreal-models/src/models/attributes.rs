use serde::{Deserialize, Serialize};
use std::time::Duration;
use surreal_orm::{
    statements::{
        define_field, define_table, for_, select, DefineFieldStatement, DefineTableStatement,
        Permissions, SelectStatement,
    },
    *,
};
use surreal_orm::{Model, Node};
use surrealdb::sql;

use typed_builder::TypedBuilder;

fn age_permissions() -> Permissions {
    use CrudType::*;
    let studentwithgranularattributes_schema::StudentWithGranularAttributes {
        ageInlineExpr,
        firstName,
        ..
    } = StudentWithGranularAttributes::schema();

    [
        for_([Create, Delete]).where_(firstName.is("Oyelowo")),
        for_(Update).where_(ageInlineExpr.less_than_or_equal(130)),
    ]
    .into()
}

fn student_permissions() -> Permissions {
    use CrudType::*;
    let studentwithgranularattributes_schema::StudentWithGranularAttributes {
        ageInlineExpr,
        firstName,
        ..
    } = StudentWithGranularAttributes::schema();

    Permissions::from(vec![
        for_([Select, Update]).where_(firstName.is("Oyedayo")),
        for_([Create, Delete]).where_(ageInlineExpr.lte(57)),
    ])
}

// use Duration;
fn default_duration_value() -> Duration {
    Duration::from_secs(60 * 60 * 24 * 7)
}

fn age_define_external_fn_path() -> DefineFieldStatement {
    use CrudType::*;
    let studentwithdefinefnattr_schema::StudentWithDefineFnAttr {
        ref ageDefineInline,
        ref firstName,
        ..
    } = StudentWithDefineFnAttr::schema();

    use FieldType::*;

    define_field(ageDefineInline)
        .on_table(Student::table_name())
        .type_(Int)
        .value("oyelowo@codebreather.com")
        .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
        .permissions(for_(Select).where_(ageDefineInline.greater_than_or_equal(18))) // Single works
        .permissions(for_([Create, Update]).where_(firstName.is("Oyedayo"))) //Multiple
        .permissions([
            for_([Create, Delete]).where_(firstName.is("Oyelowo")),
            for_(Update).where_(ageDefineInline.less_than_or_equal(130)),
        ])
}

fn define_age_define_external_fn_path() -> DefineFieldStatement {
    use CrudType::*;
    let studentwithdefineattr_schema::StudentWithDefineAttr {
        ref ageDefineInline,
        ref firstName,
        ..
    } = StudentWithDefineAttr::schema();

    use FieldType::*;

    // let statement = define_field(Student::schema().age)

    define_field(ageDefineInline)
        .on_table(Student::table_name())
        .type_(Int)
        .value("oyelowo@codebreather.com")
        .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
        .permissions(for_(Select).where_(ageDefineInline.greater_than_or_equal(18))) // Single works
        .permissions(for_([Create, Update]).where_(firstName.is("Oyedayo"))) //Multiple
        .permissions([
            for_([Create, Delete]).where_(firstName.is("Oyelowo")),
            for_(Update).where_(ageDefineInline.less_than_or_equal(130)),
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
    select(All).from(Student::table_name())
}

#[derive(Node, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table_name = "student_fn_attrs",
    drop,
    flexible,
    schemafull,
    as_fn = "as_fn",
    permissions_fn = "student_permissions"
)]
pub struct StudentFnAttrs {
    id: SurrealId<StudentFnAttrs, String>,
}

#[derive(Node, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table_name = "student_with_granular_attributes",
    drop,
    flexible,
    schemafull,
    as_ = "select(All).from(Student::table_name())",
    permissions = "student_permissions()"
)]
pub struct StudentWithGranularAttributes {
    id: SurrealId<StudentWithGranularAttributes, String>,
    first_name: String,
    last_name: String,
    #[surreal_orm(
        type_ = "int",
        value = "18",
        assert = "cond(value().is_not(NONE)).and(value().gte(18))",
        permissions = "for_([CrudType::Create, CrudType::Delete]).where_(StudentWithGranularAttributes::schema().firstName.is(\"Oyelowo\"))"
    )]
    age_inline_expr: u8,

    #[surreal_orm(
        type_ = "int",
        value = "get_age_default_value()",
        assert = "get_age_assertion()",
        permissions = "age_permissions()"
    )]
    age_default_external_function_invoked_expr: u8,

    #[surreal_orm(
        type_ = "int",
        value = "get_age_by_group_default_value(AgeGroup::Teen)",
        assert = "get_age_assertion()",
        permissions = "age_permissions()"
    )]
    age_teen_external_function_invoked_expr: u8,

    #[surreal_orm(
        type_ = "int",
        value = "get_age_by_group_default_value(AgeGroup::Senior)",
        assert = "get_age_assertion()"
    )]
    age_senior_external_function_invoked_expr: u8,

    #[surreal_orm(
        type_ = "int",
        value = "get_age_by_group_default_value(AgeGroup::Child)",
        permissions = "age_permissions()"
    )]
    age_child_external_function_invoked_expr: u8,

    #[surreal_orm(
        type_ = "int",
        value = "get_age_by_group_default_value(AgeGroup::Adult)"
    )]
    age_adult_external_function_invoked_expr: u8,

    #[surreal_orm(
        type_ = "int",
        value_fn = "get_age_default_value",
        assert_fn = "get_age_assertion",
        permissions_fn = "age_permissions"
    )]
    age_external_fn_attrs: u8,
    #[surreal_orm(
        type_ = "int",
        value = "get_age_default_value()",
        assert_fn = "get_age_assertion",
        permissions_fn = "age_permissions"
    )]
    age_mix_and_match_external_fn_inline_attrs: u8,

    #[surreal_orm(
        type_ = "duration",
        value = "default_duration_value()",
        assert = "value().is_not(NONE)"
    )]
    time_to_kelowna: Duration,

    #[surreal_orm(
        type_ = "duration",
        value = "Duration::from_secs(60 * 60 * 24 * 7)",
        assert = "value().is_not(NONE)"
    )]
    time_to_kelowna_inline: Duration,
    // Even if ypu dont list the type for all links, the types are autogenerated at compile time
    // becuase I have enough info from the annotation to derive it
    #[surreal_orm(
        link_self = "StudentWithGranularAttributes",
        type_ = "record(student_with_granular_attributes)"
    )]
    best_friend: LinkSelf<StudentWithGranularAttributes>,

    #[surreal_orm(link_one = "Book")]
    #[serde(rename = "unoBook")]
    fav_book: LinkOne<Book>,

    #[surreal_orm(link_one = "Book", skip_serializing)]
    course: LinkOne<Book>,

    #[surreal_orm(link_many = "Book", type_ = "array", item_type = "record(book)")]
    #[serde(rename = "semesterCourses")]
    all_semester_courses: LinkMany<Book>,

    #[surreal_orm(relate(
        model = "StudentWithGranularAttributesWritesBook",
        connection = "->writes->book"
    ))]
    #[serde(skip_serializing)]
    _written_books: Relate<Book>,

    #[surreal_orm(relate(
        model = "StudentWithGranularAttributesWritesBlog",
        connection = "->writes->blog"
    ))]
    #[serde(skip_serializing)]
    _blogs: Relate<Blog>,
}

pub type StudentWithGranularAttributesWritesBook = Writes<StudentWithGranularAttributes, Book>;
pub type StudentWithGranularAttributesWritesBlog = Writes<StudentWithGranularAttributes, Blog>;

fn define_first_name(field: impl Into<Field>, table: Table) -> DefineFieldStatement {
    use CrudType::*;
    let studentwithdefineattr_schema::StudentWithDefineAttr {
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
            for_(Select).where_(ageDefineInline.gte(18)),
            for_([Create, Update]).where_(firstName.is("Oyedayo")),
        ])
}

fn define_last_name() -> DefineFieldStatement {
    use CrudType::*;
    let studentwithdefineattr_schema::StudentWithDefineAttr {
        ref ageDefineInline,
        ref lastName,
        ..
    } = StudentWithDefineAttr::schema();

    define_field(lastName)
        .on_table(StudentWithDefineAttr::table_name())
        .type_(FieldType::String)
        .value("Oyedayo")
        .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
        .permissions([
            for_(Select).where_(ageDefineInline.gte(18)),
            for_([Create, Update]).where_(lastName.is("Oyedayo")),
        ])
}

fn define_last_name_external_fn_attr() -> DefineFieldStatement {
    use CrudType::*;
    let studentwithdefineattr_schema::StudentWithDefineAttr {
        ref ageDefineInline,
        ref lastName,
        ..
    } = StudentWithDefineAttr::schema();

    define_field(lastName)
        .on_table(StudentWithDefineAttr::table_name())
        .type_(FieldType::String)
        .value("Oyedayo")
        .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
        .permissions([
            for_(Select).where_(ageDefineInline.gte(18)),
            for_([Create, Update]).where_(lastName.is("Oyedayo")),
        ])
}
fn define_student_with_define_attr() -> DefineTableStatement {
    let student_schema::Student {
        ref age,
        ref firstName,
        ref lastName,
        ..
    } = Student::schema();
    use CrudType::*;

    define_table(StudentWithDefineAttr::table_name())
        .drop()
        .as_(
            select(All)
                .from(Student::table_name())
                .where_(firstName.is("Rust"))
                .order_by(age.numeric().desc())
                .limit(20)
                .start(5),
        )
        .schemafull()
        .permissions(for_(Select).where_(age.greater_than_or_equal(18))) // Single works
        .permissions(for_([Create, Delete]).where_(lastName.is("Oye"))) //Multiple
        .permissions([
            for_([Create, Delete]).where_(lastName.is("Oyedayo")),
            for_(Update).where_(age.less_than_or_equal(130)),
        ])
}

fn define_age(field: impl Into<Field>) -> DefineFieldStatement {
    use CrudType::*;
    let student_schema::Student { age, firstName, .. } = Student::schema();

    use FieldType::*;

    define_field(field)
        .on_table(Student::table_name())
        .type_(Int)
        .value("oyelowo@codebreather.com")
        .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
        .permissions(for_(Select).where_(age.greater_than_or_equal(18))) // Single works
        .permissions(for_([Create, Update]).where_(firstName.is("Oyedayo"))) //Multiple
        .permissions([
            for_([Create, Delete]).where_(firstName.is("Oyelowo")),
            for_(Update).where_(age.less_than_or_equal(130)),
        ])
}
#[derive(Node, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table_name = "student_with_define_attr",
    define = "define_student_with_define_attr()"
)]
pub struct StudentWithDefineAttr {
    id: SurrealId<StudentWithDefineAttr, String>,
    #[surreal_orm(
        type_ = "string",
        define = "define_first_name(StudentWithDefineAttr::schema().firstName, StudentWithDefineAttr::table_name())"
    )]
    first_name: String,

    #[surreal_orm(type_ = "string", define = "define_last_name()")]
    last_name: String,

    #[surreal_orm(type_ = "string", define_fn = "define_last_name_external_fn_attr")]
    last_name_external_fn_attr: String,

    #[surreal_orm(
        type_ = "int",
        define = "define_field(StudentWithDefineAttr::schema().ageDefineInline).on_table(Student::table_name()).type_(FieldType::Int).value(\"oyelowo@codebreather.com\")"
    )]
    age_define_inline: u8,

    #[surreal_orm(
        type_ = "int",
        define = "define_age(StudentWithDefineAttr::schema().ageDefineExternalInvoke)"
    )]
    age_define_external_invoke: u8,

    #[surreal_orm(type_ = "int", define_fn = "define_age_define_external_fn_path")]
    age_define_external_fn_path: u8,

    // Even if ypu dont list the type for all links, the types are autogenerated at compile time
    // becuase I have enough info from the annotation to derive it
    #[surreal_orm(
        link_self = "StudentWithDefineAttr",
        type_ = "record(student_with_define_attr)"
    )]
    best_friend: LinkSelf<StudentWithDefineAttr>,

    #[surreal_orm(link_one = "Book")]
    #[serde(rename = "unoBook")]
    fav_book: LinkOne<Book>,

    #[surreal_orm(link_one = "Book", skip_serializing)]
    course: LinkOne<Book>,

    #[surreal_orm(link_many = "Book", type_ = "array", item_type = "record(book)")]
    #[serde(rename = "semesterCourses")]
    all_semester_courses: LinkMany<Book>,

    #[surreal_orm(relate(
        model = "StudentWithDefineAttrWritesBook",
        connection = "->writes->book"
    ))]
    #[serde(skip_serializing)]
    _written_books: Relate<Book>,

    #[surreal_orm(relate(
        model = "StudentWithDefineAttrWritesBlog",
        connection = "->writes->blog"
    ))]
    #[serde(skip_serializing)]
    _blogs: Relate<Blog>,
}

pub type StudentWithDefineAttrWritesBook = Writes<StudentWithDefineAttr, Book>;
pub type StudentWithDefineAttrWritesBlog = Writes<StudentWithDefineAttr, Blog>;

#[derive(Node, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table_name = "student_with_define_fn_attr",
    define_fn = "define_student_with_define_attr"
)]
pub struct StudentWithDefineFnAttr {
    id: SurrealId<StudentWithDefineFnAttr, String>,
    // can be as simple as this
    #[surreal_orm(type_ = "string", define = "define_last_name()")]
    last_name: String,

    #[surreal_orm(type_ = "string", define_fn = "define_last_name")]
    last_name_external_fn_attr: String,

    // or go even crazier
    #[surreal_orm(
        type_ = "string",
        define = "define_first_name(StudentWithDefineFnAttr::schema().firstName, StudentWithDefineFnAttr::table_name())"
    )]
    first_name: String,

    #[surreal_orm(
        type_ = "int",
        define = "define_field(StudentWithDefineFnAttr::schema().ageDefineInline).on_table(Student::table_name()).type_(FieldType::Int).value(\"oyelowo@codebreather.com\")"
    )]
    age_define_inline: u8,

    #[surreal_orm(
        type_ = "int",
        define = "define_age(StudentWithDefineFnAttr::schema().ageDefineExternalInvoke)"
    )]
    age_define_external_invoke: u8,

    #[surreal_orm(type_ = "int", define_fn = "age_define_external_fn_path")]
    age_define_external_fn_path: u8,

    // Even if ypu dont list the type for all links, the types are autogenerated at compile time
    // becuase I have enough info from the annotation to derive it
    #[surreal_orm(
        link_self = "StudentWithDefineFnAttr",
        type_ = "record(student_with_define_fn_attr)"
    )]
    best_friend: LinkSelf<StudentWithDefineFnAttr>,

    #[surreal_orm(link_one = "Book")]
    #[serde(rename = "unoBook")]
    fav_book: LinkOne<Book>,

    #[surreal_orm(link_one = "Book", skip_serializing)]
    course: LinkOne<Book>,

    #[surreal_orm(link_many = "Book", type_ = "array", item_type = "record(book)")]
    #[serde(rename = "semesterCourses")]
    all_semester_courses: LinkMany<Book>,

    #[surreal_orm(relate(
        model = "StudentWithDefineFnAttrWritesBook",
        connection = "->writes->book"
    ))]
    #[serde(skip_serializing)]
    _written_books: Relate<Book>,

    #[surreal_orm(relate(
        model = "StudentWithDefineFnAttrWritesBlog",
        connection = "->writes->blog"
    ))]
    #[serde(skip_serializing)]
    _blogs: Relate<Blog>,
}

pub type StudentWithDefineFnAttrWritesBook = Writes<StudentWithDefineFnAttr, Book>;
pub type StudentWithDefineFnAttrWritesBlog = Writes<StudentWithDefineFnAttr, Blog>;

#[derive(Node, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student")]
pub struct Student {
    id: SurrealId<Student, String>,
    first_name: String,
    last_name: String,
    #[surreal_orm(
        type_ = "int",
        value = "18",
        assert = "cond(value().is_not(NONE)).and(value().gte(18))",
        permissions = "age_permissions()"
    )]
    age: u8,

    #[surreal_orm(type_ = "int")]
    score: u8,

    // Even if ypu dont list the type for all links, the types are autogenerated at compile time
    // becuase I have enough info from the annotation to derive it
    #[surreal_orm(link_self = "Student", type_ = "record(student)")]
    best_friend: LinkSelf<Student>,

    #[surreal_orm(link_one = "Book")]
    #[serde(rename = "unoBook")]
    fav_book: LinkOne<Book>,

    #[surreal_orm(link_one = "Book", skip_serializing)]
    course: LinkOne<Book>,

    #[surreal_orm(link_many = "Book", type_ = "array", item_type = "record(book)")]
    #[serde(rename = "semesterCourses")]
    all_semester_courses: LinkMany<Book>,

    #[surreal_orm(relate(model = "StudentWritesBook", connection = "->writes->book"))]
    #[serde(skip_serializing)]
    _written_books: Relate<Book>,

    #[surreal_orm(relate(model = "StudentWritesBlog", connection = "->writes->blog"))]
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
