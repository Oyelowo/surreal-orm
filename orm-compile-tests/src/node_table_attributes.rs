/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

// August 25, 2023.
// When any of these checks stops compiling, make sure
// to update the corresponding doc tests in the file.
// The doc tests are expected to fail to compile.
// So, we want to be sure that the failures are coming from the right place
// with field attributes and not from imports or other places we do not expect
// or from misspelled attributes names or functions names etc.
// We are mostly testing for invalid attribute combinations here.
// e.g
// define and define_fn should not be used together,
// as_ and as_fn should not be used together.
// permissions and permissions_fn should not be used together.
// <attr_name> and <attr_name>_fn should not be used together.
// NOTE: Change this if the logic changes in the future.

mod check1 {
    // When checking the doctest for updates,
    // just make sure that it uses the package name
    // e.g `surreal_compile_tests` in place of `crate`
    // i.e `use surreal_compile_tests::*;` instead of `use crate::*;`
    use crate::*;

    #[derive(Node, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(
        table = student,
        flexible,
        drop,
        schemafull,
        as_ = as_fn(),
        permissions = permissions_fn()
    )]
    pub struct Student {
        id: SurrealSimpleId<Self>,
        age: u8,
    }
}

mod check2 {
    use crate::*;

    #[derive(Node, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(table = student, define = define_table_fn())]
    pub struct Student {
        id: SurrealSimpleId<Self>,
    }
}

mod check3 {
    use crate::*;

    #[derive(Node, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(table = student, define = define_table_fn)]
    pub struct Student {
        id: SurrealSimpleId<Self>,
    }
}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = "student_invalid_by_default")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
/// }
/// ```
fn _no_table_should_be_snake_case_version_of_struct_name() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = student,
///     flexible,
///     drop,
///     schemafull,
///     as_ = "as_fn()",
///     permissions = permissions_fn(),
///     define = define_table_fn()
/// )]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
/// }
/// ```
fn _no_other_attributes_when_define_attr_is_used_1() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = student,
///     flexible,
///     drop,
///     schemafull,
///     as_ = as_fn(),
///     define = define_table_fn()
/// )]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
/// }
/// ```
fn _no_other_attributes_when_define_attr_is_used_2() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = student,
///     flexible,
///     drop,
///     schemafull,
///     define = define_table_fn()
/// )]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
/// }
/// ```
fn _no_other_attributes_when_define_attr_is_used_3() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = student,
///     flexible,
///     drop,
///     define = define_table_fn()
/// )]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
/// }
/// ```
fn _no_other_attributes_when_define_attr_is_used_4() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = student,
///     flexible,
///     define = define_table_fn()
/// )]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
/// }
/// ```
fn _no_other_attributes_when_define_attr_is_used_5() {}

// DO SAME AS ABOVE WITH define attr for define_fn ATTRIBUTE

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = student,
///     flexible,
///     drop,
///     schemafull,
///     as_ = as_fn(),
///     permissions = permissions_fn(),
///     define = define_table_fn
/// )]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
/// }
/// ```
fn _no_other_attributes_when_define_fn_attr_is_used_1() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = student,
///     flexible,
///     drop,
///     schemafull,
///     as_ = as_fn(),
///     define = define_table_fn
/// )]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
/// }
/// ```
fn _no_other_attributes_when_define_fn_attr_is_used_2() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = student,
///     flexible,
///     drop,
///     schemafull,
///     define = define_table_fn
/// )]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
/// }
/// ```
fn _no_other_attributes_when_define_fn_attr_is_used_3() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = student,
///     flexible,
///     drop,
///     define = define_table_fn
/// )]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
/// }
/// ```
fn _no_other_attributes_when_define_fn_attr_is_used_4() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")] // this is the only attribute that should be allowed
/// #[surreal_orm(
///     table = student,
///     flexible,
///     define = define_table_fn
/// )]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
/// }
/// ```
fn _no_other_attributes_when_define_fn_attr_is_used_5() {}

// <attr_name> and <attr_name>_fn should not be used together

// TODO: Remove.. was previously checking define & define_fn collision. Same with others below:
/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[surreal_orm(
///     table = student,
///     define = define_table_fn(),
/// )]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
/// }
/// ```
fn _define_and_define_fn_cannot_be_used_together_1() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[surreal_orm(table = student, permissions = permissions_fn())]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
/// }
/// ```
fn _permissions_and_permissions_fn_cannot_be_used_together_2() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[surreal_orm(table = student, as_ = as_fn())]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
/// }
/// ```
fn _as_and_as_fn_cannot_be_used_together_3() {}
