/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// August 23, 2023.
// When any of these checks stops compiling, make sure
// to update the corresponding doc tests in the file.
// The doc tests are expected to fail to compile.
// So, we want to be sure that the failures are coming from the right place
// with field attributes and not from imports or other places we do not expect
// or from misspelled attributes names or functions names etc.
// We are mostly testing for invalid attribute combinations here.
// e.g
// define and define_fn should not be used together,
// value and value_fn should not be used together.
// assert and assert_fn should not be used together.
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
    #[surreal_orm(table_name = "student")]
    pub struct Student {
        id: SurrealSimpleId<Self>,
        #[surreal_orm(type_ = "int", define = "field_define_fn()")]
        age: u8,
    }
}

mod check2 {
    use crate::*;

    #[derive(Node, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(table_name = "student")]
    pub struct Student {
        id: SurrealSimpleId<Self>,
        #[surreal_orm(type_ = "int", define_fn = "field_define_fn")]
        age: u8,
    }
}

mod check3 {
    use crate::*;

    #[derive(Node, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(table_name = "student")]
    pub struct Student {
        id: SurrealSimpleId<Self>,
        #[surreal_orm(
            type_ = "int",
            value = "18",
            assert = "assert_fn()",
            permissions = "permissions_fn()"
        )]
        age: u8,
    }
}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "18",
///         assert = "assert_fn()",
///         permissions = "permissions_fn()",
///         define = "field_define_fn()"
///     )]
///     age: u8,
/// }
fn _no_other_attributes_when_define_attr_is_used_1() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "18",
///         define = "define_fn()"
///     )]
///     age: u8,
/// }
///
fn _no_other_attributes_when_define_attr_is_used_2() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         assert = "assert_fn()",
///         define = "define_fn()"
///     )]
///     age: u8,
/// }
fn _no_other_attributes_when_define_attr_is_used_3() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         permissions = "permissions_fn()",
///         define = "define_fn()"
///     )]
///     age: u8,
/// }
fn _no_other_attributes_when_define_attr_is_used_4() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "18",
///         assert = "assert_fn()",
///         define = "define_fn()"
///     )]
///     age: u8,
/// }
fn _no_other_attributes_when_define_attr_is_used_5() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         assert = "assert_fn()",
///         permissions = "permissions_fn()",
///         define = "define_fn()"
///     )]
///     age: u8,
/// }
fn _no_other_attributes_when_define_attr_is_used_7() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "18",
///         permissions = "permissions_fn()",
///         define = "define_fn()"
///     )]
///     age: u8,
/// }
fn _no_other_attributes_when_define_attr_is_used_8() {}

// Do the same for define_fn

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "18",
///         assert = "assert_fn()",
///         permissions = "permissions_fn()",
///         define_fn = "define_fn"
///     )]
///     age: u8,
/// }
fn _no_other_attributes_when_define_fn_attr_is_used_1() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "18",
///         define_fn = "define_fn"
///     )]
///     age: u8,
/// }
///
fn _no_other_attributes_when_define_fn_attr_is_used_2() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         assert = "assert_fn()",
///         define_fn = "define_fn"
///     )]
///     age: u8,
/// }
fn _no_other_attributes_when_define_fn_attr_is_used_3() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         permissions = "permissions_fn()",
///         define_fn = "define_fn"
///     )]
///     age: u8,
/// }
fn _no_other_attributes_when_define_fn_attr_is_used_4() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "18",
///         assert = "assert_fn",
///         define_fn = "define_fn"
///     )]
///     age: u8,
/// }
fn _no_other_attributes_when_define_fn_attr_is_used_5() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         assert = "assert_fn()",
///         permissions = "permissions_fn()",
///         define_fn = "define_fn"
///     )]
///     age: u8,
/// }
fn _no_other_attributes_when_define_fn_attr_is_used_7() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "18",
///         permissions = "permissions_fn()",
///         define_fn = "define_fn"
///     )]
///     age: u8,
/// }
fn _no_other_attributes_when_define_fn_attr_is_used_8() {}

// <attr_name> and <attr_name>_fn should not be used together

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         define = "define_fn()",
///         define_fn = "define_fn"
///     )]
///     age: u8,
/// }
fn _define_and_define_fn_attrs_should_not_be_used_together_1() {}

// do the same for value and value_fn, assert and assert_fn, permissions and permissions_fn

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "value_fn()",
///         value_fn = "value_fn"
///     )]
///     age: u8,
/// }
fn _value_and_value_fn_attrs_should_not_be_used_together() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         assert = "assert_fn()",
///         assert_fn = "assert_fn"
///     )]
///     age: u8,
/// }
fn _assert_and_assert_fn_attrs_should_not_be_used_together() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         permissions = "permissions_fn()",
///         permissions_fn = "permissions_fn",
///     )]
///     age: u8,
/// }
fn _permissions_and_permissions_fn_attrs_should_not_be_used_together() {}
