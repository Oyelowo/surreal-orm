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
// as_ and as_fn should not be used together.
// permissions and permissions_fn should not be used together.
// <attr_name> and <attr_name>_fn should not be used together.
// NOTE: Change this if the logic changes in the future.

mod check1 {
    use crate::*;

    #[derive(Edge, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(
        table = likes,
        flexible,
        drop,
        schemafull,
        as_ = as_fn(),
        permissions = permissions_fn()
    )]
    pub struct Likes<In: Node, Out: Node> {
        pub id: SurrealSimpleId<Likes<In, Out>>,

        #[serde(rename = "in", skip_serializing)]
        pub in_: LinkOne<In>,
        #[serde(skip_serializing)]
        pub out: LinkOne<Out>,
        pub likes_count: u64,
    }
}

mod check2 {
    use crate::*;

    #[derive(Edge, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(table = likes, define = "define_table_fn()")]
    pub struct Likes<In: Node, Out: Node> {
        pub id: SurrealSimpleId<Likes<In, Out>>,

        #[serde(rename = "in", skip_serializing)]
        pub in_: LinkOne<In>,
        #[serde(skip_serializing)]
        pub out: LinkOne<Out>,
        pub likes_count: u64,
    }
}

mod check3 {
    use crate::*;

    #[derive(Edge, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(table = likes, define = define_table_fn)]
    pub struct Likes<In: Node, Out: Node> {
        pub id: SurrealSimpleId<Likes<In, Out>>,

        #[serde(rename = "in", skip_serializing)]
        pub in_: LinkOne<In>,
        #[serde(skip_serializing)]
        pub out: LinkOne<Out>,
        pub likes_count: u64,
    }
}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = likes,
///     flexible,
///     drop,
///     schemafull,
///     as_ = as_fn(),
///     permissions = permissions_fn(),
///     define = define_table_fn()
/// )]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     pub likes_count: u64,
/// }
/// ```
fn _no_other_attributes_when_define_attr_is_used_1() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = likes,
///     flexible,
///     drop,
///     schemafull,
///     as_ = as_fn(),
///     define = define_table_fn()
/// )]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     pub likes_count: u64,
/// }
/// ```
fn _no_other_attributes_when_define_attr_is_used_2() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = likes,
///     flexible,
///     drop,
///     schemafull,
///     define = define_table_fn()
/// )]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     pub likes_count: u64,
/// }
/// ```
fn _no_other_attributes_when_define_attr_is_used_3() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = likes,
///     flexible,
///     drop,
///     define = define_table_fn()
/// )]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     pub likes_count: u64,
/// }
/// ```
fn _no_other_attributes_when_define_attr_is_used_4() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = likes,
///     flexible,
///     define = "define_table_fn()"
/// )]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     pub likes_count: u64,
/// }
/// ```
fn _no_other_attributes_when_define_attr_is_used_5() {}

// DO SAME AS ABOVE WITH define attr for define_fn ATTRIBUTE

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = likes,
///     flexible,
///     drop,
///     schemafull,
///     as_ = as_fn(),
///     permissions = permissions_fn(),
///     define_fn = define_table_fn
/// )]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     pub likes_count: u64,
/// }
/// ```
fn _no_other_attributes_when_define_fn_attr_is_used_1() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = likes,
///     flexible,
///     drop,
///     schemafull,
///     as_ = as_fn(),
///     define = define_table_fn
/// )]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     pub likes_count: u64,
/// }
/// ```
fn _no_other_attributes_when_define_fn_attr_is_used_2() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = likes,
///     flexible,
///     drop,
///     schemafull,
///     define = define_table_fn
/// )]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     pub likes_count: u64,
/// }
/// ```
fn _no_other_attributes_when_define_fn_attr_is_used_3() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(
///     table = likes,
///     flexible,
///     drop,
///     define = define_table_fn
/// )]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     pub likes_count: u64,
/// }
/// ```
fn _no_other_attributes_when_define_fn_attr_is_used_4() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")] // this is the only attribute that should be allowed
/// #[surreal_orm(
///     table = likes,
///     flexible,
///     define = define_table_fn
/// )]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///     #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///    #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     pub likes_count: u64,
/// }
/// ```
fn _no_other_attributes_when_define_fn_attr_is_used_5() {}

// <attr_name> and <attr_name>_fn should not be used together

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[surreal_orm(
///     table = likes,
///     define = define_table_fn(),
/// )]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///    #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///     #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     pub likes_count: u64,
/// }
/// ```
fn _define_and_define_fn_cannot_be_used_together_1() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[surreal_orm(
///     table = likes,
///     permissions = permissions_fn(),
/// )]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///    #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///    #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     pub likes_count: u64,
/// }
/// ```
fn _permissions_and_permissions_fn_cannot_be_used_together_2() {}

/// Test for `Likes`
/// ```rust, compile_fail
/// use surreal_compile_tests::*;
///
/// #[derive(Edge, Serialize, Deserialize)]
/// #[surreal_orm(
///     table = likes,
///     as_ = as_fn()
/// )]
/// pub struct Likes<In: Node, Out: Node> {
///     pub id: SurrealSimpleId<Likes<In, Out>>,
///
///    #[serde(rename = "in", skip_serializing)]
///     pub in_: LinkOne<In>,
///    #[serde(skip_serializing)]
///     pub out: LinkOne<Out>,
///     pub likes_count: u64,
/// }
/// ```
fn _as_and_as_fn_cannot_be_used_together_3() {}
