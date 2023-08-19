use serde::{Deserialize, Serialize};
use surreal_orm::{
    statements::{for_, select, Permissions},
    *,
};

fn get_age_default_value() -> u8 {
    18
}

fn get_age_assertion() -> Filter {
    cond(value().is_not(NONE))
}

fn age_permissions() -> Permissions {
    for_([CrudType::Create, CrudType::Delete])
        .where_(StudentTest2::schema().firstName.is("Oyelowo"))
        .into()
}

fn student_permissions() -> Permissions {
    for_([CrudType::Create, CrudType::Delete])
        .where_(StudentTest2::schema().firstName.is("Oyelowo"))
        .into()
}

// when define is used, no other attributes should exist except table_name and relax_table_name
#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table_name = "student_test_0",
    drop,
    // flexible,
    schemafull,
    as = "select(All).from(Student::table_name())",
    permissions = "student_permissions()",
    define = "define_student()"
)]
pub struct StudentTest0 {
    id: SurrealSimpleId<Self>,
}

// when define is used, no other attributes should exist except table_name and relax_table_name
#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table_name = "student_test_1",
    drop,
    // flexible,
    schemafull,
    as = "select(All).from(Student::table_name())",
    permissions = "student_permissions()",
    define_fn = "define_student"
)]
pub struct StudentTest1 {
    id: SurrealSimpleId<Self>,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student")]
pub struct Student {
    id: SurrealSimpleId<Self>,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student_test_2", drop, define = "define_student()")]
pub struct StudentTest2 {
    id: SurrealSimpleId<Self>,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student_test_3", schemafull, define = "define_student()")]
pub struct StudentTest3 {
    id: SurrealSimpleId<Self>,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table_name = "student_test_4",
    as = "select(All).from(Student::table_name())",
    // permissions = "student_permissions()",
    define = "define_student()"
)]
pub struct StudentTest4 {
    id: SurrealSimpleId<Self>,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table_name = "student_test_5",
    permissions = "student_permissions()",
    define = "define_student()"
)]
pub struct StudentTest5 {
    id: SurrealSimpleId<Self>,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table_name = "student_test_6",
    define_fn = "define_student",
    define = "define_student()"
)]
pub struct StudentTest6 {
    id: SurrealSimpleId<Self>,
}

// do same for define_fn. When define_fn exists, no other attributes should exist except table_name and relax_table_name.
#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student_test_7", drop, define_fn = "define_student")]
pub struct StudentTest7 {
    id: SurrealSimpleId<Self>,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table_name = "student_test_8",
    schemafull,
    define_fn = "define_student"
)]
pub struct StudentTest8 {
    id: SurrealSimpleId<Self>,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table_name = "student_test_9",
    as = "select(All).from(Student::table_name())",
    define_fn = "define_student"
)]
pub struct StudentTest9 {
    id: SurrealSimpleId<Self>,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(
    table_name = "student_test_10",
    permissions = "student_permissions()",
    define_fn = "define_student"
)]
pub struct StudentTest10 {
    id: SurrealSimpleId<Self>,
}
