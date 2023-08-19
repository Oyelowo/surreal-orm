use serde::{Deserialize, Serialize};
use surreal_orm::{
    statements::{define_field, for_, select, DefineFieldStatement, Permissions},
    *,
};

// There should nne 12 errors in this file.
// And there were as at August 19, 2023. If you add more, please update this comment.
// The errors are from StudentTest0 to StudentTest11:
// 1. 1 Invalid value and value_fn combination
// 2. 1 Invalid assert and assert_fn combination
// 3. 1 Invalid permissions and permissions_fn combination
// 4. 1 Invalid define and define_fn combination
// 5. 8 Invalid combinations. Other attributes should not be used when define or define_fn is used
//    except for type

fn get_age_default_value() -> u8 {
    18
}

fn get_age_assertion() -> Filter {
    cond(value().is_not(NONE))
}

fn age_permissions() -> Permissions {
    for_([CrudType::Create, CrudType::Delete])
        .where_(StudentTest0::schema().firstName.is("Oyelowo"))
        .into()
}

fn student_permissions() -> Permissions {
    for_([CrudType::Create, CrudType::Delete])
        .where_(StudentTest0::schema().firstName.is("Oyelowo"))
        .into()
}

fn define_age() -> DefineFieldStatement {
    use surreal_orm::{Model, Node};
    use CrudType::*;
    let studenttest0_schema::StudentTest0 {
        ref age,
        ref firstName,
        ..
    } = StudentTest0::schema();

    use FieldType::*;

    let statement = define_field(age)
        .on_table(StudentTest0::table_name())
        .type_(Int)
        .value("oyelowo@codebreather.com")
        .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
        .permissions(for_(Select).where_(age.greater_than_or_equal(18))) // Single works
        .permissions(for_([Create, Update]).where_(firstName.is("Oyedayo"))) //Multiple
        .permissions([
            for_([Create, Delete]).where_(firstName.is("Oyelowo")),
            for_(Update).where_(age.less_than_or_equal(130)),
        ]);
    statement
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student_test_0")]
pub struct StudentTest0 {
    id: SurrealSimpleId<Self>,
    #[surreal_orm(
        type = "int",
        value = "18",
        assert = "cond(value().is_not(NONE)).and(value().gte(18))",
        permissions = "for_([CrudType::Create, CrudType::Delete]).where_(StudentTest0::schema().firstName.is(\"Oyelowo\"))",
        define = "define_age()"
    )]
    age: u8,
    // #[surreal_orm(
    //     type = "int",
    //     value = "get_age_default_value()",
    //     assert = "get_age_assertion()",
    //     permissions = "age_permissions()",
    //     // define = "define_age()"
    // )]
    // age_default_external_function_invoked_expr: u8,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student_test_1")]
pub struct StudentTest1 {
    id: SurrealSimpleId<Self>,
    #[surreal_orm(type = "int", value = "18", define = "define_age()")]
    age: u8,
}

// do for others
#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student_test_2")]
pub struct StudentTest2 {
    id: SurrealSimpleId<Self>,
    #[surreal_orm(
        type = "int",
        assert = "cond(value().is_not(NONE)).and(value().gte(18))",
        define = "define_age()"
    )]
    age: u8,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student_test_3")]
pub struct StudentTest3 {
    id: SurrealSimpleId<Self>,
    #[surreal_orm(
        type = "int",
        permissions = "for_([CrudType::Create, CrudType::Delete]).where_(StudentTest3::schema().firstName.is(\"Oyelowo\"))",
        define = "define_age()"
    )]
    age: u8,
}

// Do same with define_fn
#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student_test_4")]
pub struct StudentTest4 {
    id: SurrealSimpleId<Self>,
    #[surreal_orm(
        type = "int",
        value = "18",
        assert = "cond(value().is_not(NONE)).and(value().gte(18))",
        permissions = "for_([CrudType::Create, CrudType::Delete]).where_(StudentTest4::schema().firstName.is(\"Oyelowo\"))",
        define = "define_age()"
    )]
    age: u8,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student_test_5")]
pub struct StudentTest5 {
    id: SurrealSimpleId<Self>,
    #[surreal_orm(type = "int", value = "18", define_fn = "define_age")]
    age: u8,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student_test_6")]
pub struct StudentTest6 {
    id: SurrealSimpleId<Self>,
    #[surreal_orm(
        type = "int",
        assert = "cond(value().is_not(NONE)).and(value().gte(18))",
        define_fn = "define_age"
    )]
    age: u8,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student_test_7")]
pub struct StudentTest7 {
    id: SurrealSimpleId<Self>,
    #[surreal_orm(
        type = "int",
        permissions = "for_([CrudType::Create, CrudType::Delete]).where_(StudentTest7::schema().firstName.is(\"Oyelowo\"))",
        define_fn = "define_age"
    )]
    age: u8,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student_test_8")]
pub struct StudentTest8 {
    id: SurrealSimpleId<Self>,
    #[surreal_orm(
        type = "int",
        define = "define_age()"
        define_fn = "define_age"
    )]
    age: u8,
}

// normal attribute and its counterpart suffixed with _fn should not be allowed
#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student_test_9")]
pub struct StudentTest9 {
    #[surreal_orm(
        type = "int",
        value = "get_age_default_value()",
        value_fn = "get_age_default_value"
    )]
    age: u8,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student_test_10")]
pub struct StudentTest10 {
    #[surreal_orm(
        type = "int",
        assert = "get_age_assertion()",
        assert_fn = "get_age_assertion"
    )]
    age: u8,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student_test_11")]
pub struct StudentTest11 {
    #[surreal_orm(
        type = "int",
        permissions = "age_permissions()",
        permissions_fn = "age_permissions"
    )]
    age: u8,
}
