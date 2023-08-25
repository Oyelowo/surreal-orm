/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     statements::{define_field, for_, DefineFieldStatement, Permissions},
///     *,
/// };
///
/// fn define_age() -> DefineFieldStatement {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "18",
///         assert = "cond(value().is_not(NONE)).and(value().gte(18))",
///         permissions = "for_([CrudType::Create, CrudType::Delete]).where_(Student::schema().agt.gt(18))",
///         define = "define_age()"
///     )]
///     age: u8,
/// }
fn no_other_attributes_when_define_attr_is_used_1() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     statements::{define_field, for_, DefineFieldStatement, Permissions},
///     *,
/// };
///
/// fn define_age() -> DefineFieldStatement {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "18",
///         define = "define_age()"
///     )]
///     age: u8,
/// }
///
fn no_other_attributes_when_define_attr_is_used_2() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     statements::{define_field, for_, DefineFieldStatement, Permissions},
///     *,
/// };
///
/// fn define_age() -> DefineFieldStatement {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         assert = "cond(value().is_not(NONE)).and(value().gte(18))",
///         define = "define_age()"
///     )]
///     age: u8,
/// }
fn no_other_attributes_when_define_attr_is_used_3() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     statements::{define_field, for_, DefineFieldStatement, Permissions},
///     *,
/// };
///
/// fn define_age() -> DefineFieldStatement {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         permissions = "for_([CrudType::Create, CrudType::Delete]).where_(Student::schema().age.is(18))",
///         define = "define_age()"
///     )]
///     age: u8,
/// }
fn no_other_attributes_when_define_attr_is_used_4() {}
/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     statements::{define_field, for_, DefineFieldStatement, Permissions},
///     *,
/// };
///
/// fn define_age() -> DefineFieldStatement {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "18",
///         assert = "cond(value().is_not(NONE)).and(value().gte(18))",
///         define = "define_age()"
///     )]
///     age: u8,
/// }
fn no_other_attributes_when_define_attr_is_used_5() {}

fn no_other_attributes_when_define_attr_is_used_6() {}
/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     statements::{define_field, for_, DefineFieldStatement, Permissions},
///     *,
/// };
///
/// fn define_age() -> DefineFieldStatement {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         assert = "cond(value().is_not(NONE)).and(value().gte(18))",
///         permissions = "for_([CrudType::Create, CrudType::Delete]).where_(Student::schema().age.gt(18))",
///         define = "define_age()"
///     )]
///     age: u8,
/// }
fn no_other_attributes_when_define_attr_is_used_7() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     statements::{define_field, for_, DefineFieldStatement, Permissions},
///     *,
/// };
///
/// fn define_age() -> DefineFieldStatement {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "18",
///         permissions = "for_([CrudType::Create, CrudType::Delete]).where_(Student::schema().age(18))",
///         define = "define_age()"
///     )]
///     age: u8,
/// }
fn no_other_attributes_when_define_attr_is_used_8() {}

// Do the same for define_fn

/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     statements::{define_field, for_, DefineFieldStatement, Permissions},
///     *,
/// };
///
/// fn define_age() -> DefineFieldStatement {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "18",
///         assert = "cond(value().is_not(NONE)).and(value().gte(18))",
///         permissions = "for_([CrudType::Create, CrudType::Delete]).where_(Student::schema().age(18))",
///         define_fn = "define_age"
///     )]
///     age: u8,
/// }
fn no_other_attributes_when_define_fn_attr_is_used_1() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     statements::{define_field, for_, DefineFieldStatement, Permissions},
///     *,
/// };
///
/// fn define_age() -> DefineFieldStatement {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "18",
///         define_fn = "define_age"
///     )]
///     age: u8,
/// }
///
fn no_other_attributes_when_define_fn_attr_is_used_2() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     statements::{define_field, for_, DefineFieldStatement, Permissions},
///     *,
/// };
///
/// fn define_age() -> DefineFieldStatement {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         assert = "cond(value().is_not(NONE)).and(value().gte(18))",
///         define_fn = "define_age"
///     )]
///     age: u8,
/// }
fn no_other_attributes_when_define_fn_attr_is_used_3() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     statements::{define_field, for_, DefineFieldStatement, Permissions},
///     *,
/// };
///
/// fn define_age() -> DefineFieldStatement {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         permissions = "for_([CrudType::Create, CrudType::Delete]).where_(Student::schema().age.is(18))",
///         define_fn = "define_age"
///     )]
///     age: u8,
/// }
fn no_other_attributes_when_define_fn_attr_is_used_4() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     statements::{define_field, for_, DefineFieldStatement, Permissions},
///     *,
/// };
///
/// fn define_age() -> DefineFieldStatement {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "18",
///         assert = "cond(value().is_not(NONE)).and(value().gte(18))",
///         define_fn = "define_age"
///     )]
///     age: u8,
/// }
fn no_other_attributes_when_define_fn_attr_is_used_5() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     statements::{define_field, for_, DefineFieldStatement, Permissions},
///     *,
/// };
///
/// fn define_age() -> DefineFieldStatement {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         assert = "cond(value().is_not(NONE)).and(value().gte(18))",
///         permissions = "for_([CrudType::Create, CrudType::Delete]).where_(Student::schema().age.gt(18))",
///         define_fn = "define_age"
///     )]
///     age: u8,
/// }
fn no_other_attributes_when_define_fn_attr_is_used_7() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     statements::{define_field, for_, DefineFieldStatement, Permissions},
///     *,
/// };
///
/// fn define_age() -> DefineFieldStatement {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "18",
///         permissions = "for_([CrudType::Create, CrudType::Delete]).where_(Student::schema().age.gt(18))",
///         define_fn = "define_age"
///     )]
///     age: u8,
/// }
fn no_other_attributes_when_define_fn_attr_is_used_8() {}

// <attr_name> and <attr_name>_fn should not be used together

/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     statements::{define_field, DefineFieldStatement},
///     *,
/// };
///
/// fn define_age() -> DefineFieldStatement {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         define = "define_age()",
///         define_fn = "define_age"
///     )]
///     age: u8,
/// }
fn define_and_define_fn_attrs_should_not_be_used_together_1() {}

// do the same for value and value_fn, assert and assert_fn, permissions and permissions_fn

/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     *,
/// };
///
/// fn get_age_default_value() -> u8 {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         value = "get_age_default_value()",
///         value_fn = "get_age_default_value"
///     )]
///     age: u8,
/// }
fn value_and_value_fn_attrs_should_not_be_used_together() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     *,
/// };
///
/// fn get_age_assertion() -> Filter {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         assert = "get_age_assertion()",
///         assert_fn = "get_age_assertion"
///     )]
///     age: u8,
/// }
fn assert_and_assert_fn_attrs_should_not_be_used_together() {}

/// Test for `Student`
/// ```rust, compile_fail
/// use serde::{Deserialize, Serialize};
/// use surreal_orm::{
///     *, statements::Permissions
/// };
///
/// fn age_permissions() -> Permissions {
///    unimplemented!()
/// }
///
/// #[derive(Node, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[surreal_orm(table_name = "student")]
/// pub struct Student {
///     id: SurrealSimpleId<Self>,
///     #[surreal_orm(
///         type_ = "int",
///         permissions = "age_permissions()",
///         permissions_fn = "age_permissions",
///     )]
///     age: u8,
/// }
fn permissions_and_permissions_fn_attrs_should_not_be_used_together() {}
