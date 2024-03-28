/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
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

// Edge attributes
// define
// define_fn
// value
// value_fn
// assert
// assert_fn
// permissions
// permissions_fn

mod check1 {
    use crate::*;

    #[derive(Edge, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(table = likes)]
    pub struct Likes<In: Node, Out: Node> {
        pub id: SurrealSimpleId<Likes<In, Out>>,

        #[serde(rename = "in", skip_serializing)]
        pub in_: LinkOne<In>,
        #[serde(skip_serializing)]
        pub out: LinkOne<Out>,
        #[surreal_orm(define = "define_field_fn()")]
        pub likes_count: u64,
    }
}

mod check2 {
    use crate::*;

    #[derive(Edge, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(table = likes)]
    pub struct Likes<In: Node, Out: Node> {
        pub id: SurrealSimpleId<Likes<In, Out>>,

        #[serde(rename = "in", skip_serializing)]
        pub in_: LinkOne<In>,
        #[serde(skip_serializing)]
        pub out: LinkOne<Out>,
        #[surreal_orm(define = define_field_fn)]
        pub likes_count: u64,
    }
}

mod check3 {
    use crate::*;

    #[derive(Edge, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(table = likes)]
    pub struct Likes<In: Node, Out: Node> {
        pub id: SurrealSimpleId<Likes<In, Out>>,

        #[serde(rename = "in", skip_serializing)]
        pub in_: LinkOne<In>,
        #[serde(skip_serializing)]
        pub out: LinkOne<Out>,
        #[surreal_orm(value = "18", assert = assert_fn(), permissions = permissions_fn())]
        pub likes_count: u64,
    }
}

mod check4 {
    use crate::*;

    #[derive(Edge, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(table = likes)]
    pub struct Likes<In: Node, Out: Node> {
        pub id: SurrealSimpleId<Likes<In, Out>>,

        #[serde(rename = "in", skip_serializing)]
        pub in_: LinkOne<In>,
        #[serde(skip_serializing)]
        pub out: LinkOne<Out>,
        #[surreal_orm(
            value = "18",
            assert= assert_fn,
            permissions= permissions_fn
        )]
        pub likes_count: u64,
    }
}

// <attr_name> and <attr_name>_fn should not be used together

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = likes)]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(
///         value = "18",
///         assert = assert_fn(),
///         permissions = permissions_fn(),
///         define = define_field_fn()
///     )]
///     likes_count: u64,
/// }
fn _no_other_attributes_when_define_attr_is_used_in_edge_1() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = likes)]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(
///         value = "18",
///         define = define_field_fn()
///     )]
///     likes_count: u64,
/// }
fn _no_other_attributes_when_define_attr_is_used_in_edge_2() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = likes)]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(
///         assert = assert_fn(),
///         define = define_field_fn()
///     )]
///     likes_count: u64,
/// }
fn _no_other_attributes_when_define_attr_is_used_in_edge_3() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = likes)]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(
///         permissions = permissions_fn(),
///         define = define_field_fn()
///     )]
///     likes_count: u64,
/// }
fn _no_other_attributes_when_define_attr_is_used_in_edge_4() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = likes)]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(
///         value = "18",
///         assert = assert_fn(),
///         define = define_field_fn()
///     )]
///     likes_count: u64,
/// }
fn _no_other_attributes_when_define_attr_is_used_in_edge_5() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = likes)]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(
///         assert = assert_fn(),
///         permissions = permissions_fn(),
///         define = define_field_fn()
///     )]
///     likes_count: u64,
/// }
fn _no_other_attributes_when_define_attr_is_used_in_edge_7() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = likes)]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(
///         value = "18",
///         permissions = permissions_fn(),
///         define = define_field_fn()
///     )]
///     likes_count: u64,
/// }
fn _no_other_attributes_when_define_attr_is_used_in_edge_8() {}

// same for define_fn

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = likes)]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(
///         value = "18",
///         assert = assert_fn(),
///         permissions = permissions_fn(),
///         define_fn = define_field_fn
///     )]
///     likes_count: u64,
/// }
fn _no_other_attributes_when_define_fn_attr_is_used_in_edge_1() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = likes)]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(
///         value = "18",
///         define_fn = define_field_fn
///     )]
///     likes_count: u64,
/// }
fn _no_other_attributes_when_define_fn_attr_is_used_in_edge_2() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = likes)]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(
///         assert = assert_fn(),
///         define_fn = define_field_fn
///     )]
///     likes_count: u64,
/// }
fn _no_other_attributes_when_define_fn_attr_is_used_in_edge_3() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = "likes")]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(
///         permissions = permissions_fn(),
///         define_fn = define_field_fn
///     )]
///     likes_count: u64,
/// }
fn _no_other_attributes_when_define_fn_attr_is_used_in_edge_4() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = "likes")]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(
///         value = "18",
///         assert = assert_fn,
///         define_fn = define_field_fn
///     )]
///     likes_count: u64,
/// }
fn _no_other_attributes_when_define_fn_attr_is_used_in_edge_5() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = likes)]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(
///         assert = assert_fn(),
///         permissions = permissions_fn(),
///         define_fn = define_field_fn
///     )]
///     likes_count: u64,
/// }
fn _no_other_attributes_when_define_fn_attr_is_used_in_edge_7() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
///
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = likes)]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,

///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(
///         value = "18",
///         permissions = permissions_fn(),
///         define_fn = define_field_fn
///     )]
///     likes_count: u64,
/// }
fn _no_other_attributes_when_define_fn_attr_is_used_in_edge_8() {}

// define
// define_fn
// value
// value_fn
// assert
// assert_fn
// permissions
// permissions_fn

// define and define_fn

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = likes)]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(define = define_field_fn(), define_fn = define_field_fn)]
///     pub likes_count: u64,
/// }
fn _define_and_define_fn_attrs_should_not_be_used_together_in_edge_1() {}

// do the same for value and value_fn, assert and assert_fn, permissions and permissions_fn

// TODO: Probably remove or add new case since we dont allow <attr>_fn counterpart
// anymore and just use same attribute to accept a path or an expression and
// conditionally handle that to simplify the API.
/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = likes)]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(value = value_fn())]
///     pub likes_count: u64,
/// }
fn _value_and_value_fn_attrs_should_not_be_used_together_in_edge() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = likes)]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(assert = assert_fn())]
///     pub likes_count: u64,
/// }
fn _assert_and_assert_fn_attrs_should_not_be_used_together_in_edge() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table = likes)]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     #[surreal_orm(permissions = permissions_fn())]
///     pub likes_count: u64,
/// }
fn _permissions_and_permissions_fn_attrs_should_not_be_used_together_in_edge() {}
