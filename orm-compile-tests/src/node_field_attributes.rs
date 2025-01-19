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
    #[orm(table = student)]
    pub struct Student {
        id: SurrealSimpleId<Self>,
        #[orm(ty = "int", define = define_field_fn())]
        age: u8,
    }
}

mod check2 {
    use crate::*;

    // Ordinary path should also work for define attribute
    #[derive(Node, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[orm(table = student)]
    pub struct Student {
        id: SurrealSimpleId<Self>,
        #[orm(ty = "int", define = define_field_fn)]
        age: u8,
    }
}

mod check3 {
    use crate::*;

    #[derive(Node, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[orm(table = student)]
    pub struct Student {
        id: SurrealSimpleId<Self>,
        #[orm(
            ty = "int",
            value = "18",
            assert = assert_fn(),
            permissions = permissions_fn()
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
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(
///         ty = "int",
///         value = "18",
///         assert = assert_fn(),
///         permissions = permissions_fn(),
///         define = define_field_fn()
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
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(
///         ty = "int",
///         value = "18",
///         define = define_field_fn()
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
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(
///         ty = "int",
///         assert = assert_fn(),
///         define = define_field_fn()
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
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(
///         ty = "int",
///         permissions = permissions_fn(),
///         define = define_field_fn()
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
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(
///         ty = "int",
///         value = "18",
///         assert = assert_fn(),
///         define = define_field_fn()
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
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(
///         ty = "int",
///         assert = assert_fn(),
///         permissions = permissions_fn(),
///         define = define_field_fn()
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
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(
///         ty = "int",
///         value = "18",
///         permissions = permissions_fn(),
///         define = define_field_fn()
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
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(
///         ty = "int",
///         value = "18",
///         assert = assert_fn(),
///         permissions = permissions_fn(),
///         define = define_field_fn
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
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(
///         ty = "int",
///         value = "18",
///         define = define_field_fn
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
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(
///         ty = "int",
///         assert = assert_fn(),
///         define = define_field_fn
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
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(
///         ty = "int",
///         permissions = permissions_fn(),
///         define = define_field_fn
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
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(
///         ty = "int",
///         value = "18",
///         assert = assert_fn,
///         define = define_field_fn
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
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(
///         ty = "int",
///         assert = assert_fn(),
///         permissions = permissions_fn(),
///         define = define_field_fn
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
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(
///         ty = "int",
///         value = "18",
///         permissions = permissions_fn(),
///         define = define_field_fn
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
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(ty = "int", define = define_field_fn())]
///     age: u8,
/// }
/// ```
fn _define_and_define_fn_attrs_should_not_be_used_together_1() {}

// do the same for value and value_fn, assert and assert_fn, permissions and permissions_fn

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(ty = "int", value = value_fn())]
///     age: u8,
/// }
/// ```
fn _value_and_value_fn_attrs_should_not_be_used_together() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(ty = "int", assert = assert_fn())]
///     age: u8,
/// }
/// ```
fn _assert_and_assert_fn_attrs_should_not_be_used_together() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(ty = "int", permissions = permissions_fn())]
///     age: u8,
/// }
/// ```
fn _permissions_and_permissions_fn_attrs_should_not_be_used_together() {}

/// Test type mismatch. age - Int is not string.
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(ty = "string")]
///     age: u8,
/// }
/// ```
fn _type_mismatch_age_int_is_not_string() {}

/// Test type mismatch. age - String is not int.
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(ty = "int")]
///     age: String,
/// }
/// ```
fn _type_mismatch_age_string_is_not_int() {}

/// Test type mismatch. age - Int is not float.
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(ty = "float")]
///     age: u8,
/// }
/// ```
fn _type_mismatch_age_int_is_not_float() {}

/// Test type mismatch. age - Float is not int.
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(ty = "int")]
///     age: f64,
/// }
/// ```
fn _type_mismatch_age_float_is_not_int() {}

/// Test type mismatch. age - Float is not string.
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(ty = "string")]
///     age: f64,
/// }
/// ```
fn _type_mismatch_age_float_is_not_string() {}

/// Test type mismatch. age - String is not float.
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[orm(table = student)]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[orm(ty = "float")]
///     age: String,
/// }
/// ```
fn _type_mismatch_age_string_is_not_float() {}

/// Test type mismatch. age - bool is not int.
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[orm(table = student)]
/// pub struct Student {
///    id: SurrealSimpleId<Self>,
///   #[orm(ty = "int")]
///    age: bool,
/// }
/// ```
fn _type_mismatch_age_boolean_is_not_int() {}

/// Test type mismatch. age - bool is not string.
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///  #[derive(Node, Serialize, Deserialize)]
///  #[serde(rename_all = "camelCase")]
///  #[orm(table = student)]
///  pub struct Student {
///      id: SurrealSimpleId<Self>,
///      #[orm(ty = "string")]
///      age: bool,
///  }
/// ```
fn _type_mismatch_age_boolean_is_not_string() {}

/// Test type mismatch. age - bool is not float.
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///  #[derive(Node, Serialize, Deserialize)]
///  #[serde(rename_all = "camelCase")]
///  #[orm(table = student)]
///  pub struct Student {
///      id: SurrealSimpleId<Self>,
///      #[orm(ty = "float")]
///      age: bool,
///  }
/// ```
fn _type_mismatch_age_boolean_is_not_float() {}

/// Test type mismatch. age - bool is not decimal.
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///  #[derive(Node, Serialize, Deserialize)]
///  #[serde(rename_all = "camelCase")]
///  #[orm(table = student)]
///  pub struct Student {
///      id: SurrealSimpleId<Self>,
///      #[orm(ty = "decimal")]
///      age: bool,
///  }
/// ```
fn _type_mismatch_age_boolean_is_not_decimal() {}

/// Test type mismatch. age - bool is not datetime.
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///  #[derive(Node, Serialize, Deserialize)]
///  #[serde(rename_all = "camelCase")]
///  #[orm(table = student)]
///  pub struct Student {
///      id: SurrealSimpleId<Self>,
///      #[orm(ty = "datetime")]
///      age: bool,
///  }
/// ```
fn _type_mismatch_age_boolean_is_not_datetime() {}

/// Test type mismatch. age - bool is not duration.
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///  #[derive(Node, Serialize, Deserialize)]
///  #[serde(rename_all = "camelCase")]
///  #[urreal_orm(table = student)]
/// pub struct Student {
///    id: SurrealSimpleId<Self>,
///   #[orm(ty = "duration")]
///   age: bool,
/// }
/// ```
fn _type_mismatch_age_boolean_is_not_duration() {}

/// Test type mismatch. age - bool is not geometry.
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[orm(table = student)]
/// pub struct Student {
///   id: SurrealSimpleId<Self>,
///  #[orm(ty = "geometry(line)")]
/// age: bool,
/// }
/// ```
fn _type_mismatch_age_boolean_is_not_geometry() {}
