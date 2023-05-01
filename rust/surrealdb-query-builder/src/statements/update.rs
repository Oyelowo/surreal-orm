/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Statement syntax
// UPDATE @targets
// 	[ CONTENT @value
// 	  | MERGE @value
// 	  | PATCH @value
// 	  | SET @field = @value ...
// 	]
// 	[ WHERE @condition ]
// 	[ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]
// 	[ TIMEOUT @duration ]
// 	[ PARALLEL ]
// ;
use std::{fmt::Display, marker::PhantomData};

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql;

use crate::{
    traits::{Binding, BindingsList, Buildable, Erroneous, Parametric, Queryable, SurrealdbModel},
    types::{DurationLike, Filter, ReturnType, SurrealId, Updateables},
    Conditional, ErrorList, Field, ReturnableDefault, ReturnableStandard, ToRaw,
};

/// Creates a new UPDATE statement.
/// The UPDATE statement can be used to update or modify records in the database.

///
/// # Arguments
///
/// * `connection` - built using `with` method on a node. e.g `Student::with(..).writes(..).book(..)`
/// # Examples
///
/// ```rust, ignore
/// # use surrealdb_query_builder as surrealdb_orm;
/// use std::time::Duration;
/// use surrealdb_orm::{*, statements::update};
///
/// // Update using set method
/// update::<User>(user)
///     .set(updater(score).equal(5))
///     .where_(age.greater_than(18));
///
/// // Update many records that match the filter using content method in user table
/// update::<User>(user)
///     .content(
///          User {
///             team: "Codebreather",
///             ...
///          }
///     ).where_(cond(age.greater_than(18)).and(name.like("codebreather"));
///     
/// // Update many records that match the filter using merge method in user table
/// update::<User>(user)
///     .merge(
///          UserDocument {
///             hobbies: vec!["music production", "problem solving", "rust"],
///             ...
///          }
///     ).where_(cond(age.greater_than(18)).and(name.like("codebreather"));
///
/// // Update specific record using content method
/// update::<User>(user1)
///     .content(
///          User {
///             name: "Oyelowo".into(),
///             age: 198,
///             ...
///          }
///     );
///     
/// // Update using content method
/// update::<User>(user2)
///     .merge(
///          UserDocument {
///             hobbies: vec!["music production", "problem solving", "rust"],
///             ...
///          }
///     );
/// ```
pub fn update<T>(targettables: impl Into<TargettablesForUpdate>) -> UpdateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    let table_name = T::table_name();
    let targettables: TargettablesForUpdate = targettables.into();
    let mut bindings = vec![];
    let mut errors = vec![];
    let param = match targettables {
        TargettablesForUpdate::Table(table) => {
            let table = table.to_string();
            if &table != &table_name.to_string() {
                errors.push(format!(
                    "table name -{table} does not match the surreal model struct type which belongs to {table_name} table"
                ));
            }
            table
        }
        TargettablesForUpdate::SurrealId(id) => {
            if !id
                .to_string()
                .starts_with(format!("{table_name}:").as_str())
            {
                errors.push(format!(
                    "id - {id} does not belong to {table_name} table from the surreal model struct provided"
                ));
            }
            let binding = Binding::new(id);
            let param = binding.get_param_dollarised();
            bindings.push(binding);
            param
        }
    };

    UpdateStatement {
        target: param,
        content: None,
        merge: None,
        replace: None,
        patch_ops: vec![],
        set: vec![],
        where_: None,
        return_type: None,
        timeout: None,
        parallel: false,
        bindings,
        errors,
        __model_return_type: PhantomData,
    }
}

#[derive(Clone, Debug)]
enum OpType {
    /// Adds values along the path using JSON patch operation
    Add,
    /// Removes values along the path using JSON patch operation
    Remove,
    /// Replaces values along the path using JSON patch operation
    Replace,
    /// Moves values along the path using JSON patch operation
    Change,
}
impl Display for OpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            OpType::Add => "add",
            OpType::Remove => "remove",
            OpType::Replace => "replace",
            OpType::Change => "change",
        };
        write!(f, "{}", op)
    }
}

#[derive(Clone, Debug)]
pub struct PatchOpInit {
    path: String,
    op: OpType,
    value: Option<String>,
    bindings: BindingsList,
    errors: ErrorList,
}

///
pub struct PatchOp(PatchOpInit);

impl std::ops::Deref for PatchOp {
    type Target = PatchOpInit;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Buildable for PatchOp {
    fn build(&self) -> String {
        let mut patch = vec![];
        let path = self.path.to_string();
        let op = self.op.to_string();
        let value = self.value.clone();
        let value = match value {
            Some(value) => format!(", value: {}", value),
            None => "".to_string(),
        };
        patch.push(format!("{{ op: '{}', path: {}{} }}", op, path, value));
        patch.join(", ")
    }
}

impl From<PatchOp> for Vec<PatchOp> {
    fn from(patch_op: PatchOp) -> Self {
        vec![patch_op]
    }
}

impl Parametric for PatchOp {
    fn get_bindings(&self) -> BindingsList {
        self.0.bindings.to_vec()
    }
}

impl Erroneous for PatchOp {
    fn get_errors(&self) -> ErrorList {
        self.0.errors.to_vec()
    }
}

/// Json patch operation
/// # Arguments
/// * `path` - The path to the field to be patched. Use the same field operation
/// you use in the library for accessing top level or nested fields. It automatically
/// converts that to a valid json path to the field.
///
/// # Examples
/// ```
/// # use surrealdb_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::{patch}};
/// let ref name = Field::new("name");
/// let name_first = Field::new("name.first");
///
/// let patch_op = patch(name).add("Oyelowo");
/// let patch_op = patch(name_first).change("Oyelowo");
/// let patch_op = patch(name).replace("Oyelowo");
/// let patch_op = patch(name).remove();
/// ```
pub fn patch(path: impl Into<Field>) -> PatchOpInit {
    let path: Field = path.into();
    let path = path.build();
    let path = path.split('.').collect::<Vec<&str>>();
    // Check if any of the item in the array contains invalid identifier
    // i.e not start with aplhabet, contains only alphanumeric and underscore
    // if any of the item is invalid, return error
    // Must be e.g name, name.first, name.first.second, so that we can easily replace `.` with `/`
    let bad_path = path.iter().filter(|item| {
        item.starts_with(|c: char| !c.is_alphabetic() && c != '_')
            || item
                .chars()
                .any(|c: char| !(c.is_alphanumeric() || c == '_'))
    });

    let mut errors = vec![];
    if bad_path.count() > 0 {
        errors.push("The path you have provided is invalid. Make sure that there are no clauses or conditions included. Valid path include e.g name, name(E).first, name(E).first(E).second, etc.".to_string());
    }

    let path = format!("/{}", path.join("/"));
    let path_binding = Binding::new(sql::Value::from(path));

    PatchOpInit {
        path: path_binding.get_param_dollarised(),
        op: OpType::Add,
        value: None,
        bindings: vec![path_binding],
        errors,
    }
}

impl PatchOpInit {
    pub fn add(self, value: impl Serialize) -> PatchOp {
        let sql_value = sql::json(&serde_json::to_string(&value).unwrap()).unwrap();
        let binding = Binding::new(sql_value);

        PatchOp(Self {
            op: OpType::Add,
            value: Some(binding.get_param_dollarised()),
            bindings: self.bindings.into_iter().chain(vec![binding]).collect(),
            ..self
        })
    }

    pub fn remove(self) -> PatchOp {
        PatchOp(Self {
            op: OpType::Remove,
            ..self
        })
    }

    pub fn replace(self, value: impl Serialize) -> PatchOp {
        let sql_value = sql::json(&serde_json::to_string(&value).unwrap()).unwrap();
        let binding = Binding::new(sql_value);

        PatchOp(Self {
            op: OpType::Replace,
            value: Some(binding.get_param_dollarised()),
            bindings: self.bindings.into_iter().chain(vec![binding]).collect(),
            ..self
        })
    }

    pub fn change(self, value: impl Serialize) -> PatchOp {
        let sql_value = sql::json(&serde_json::to_string(&value).unwrap()).unwrap();
        let binding = Binding::new(sql_value);

        PatchOp(Self {
            op: OpType::Change,
            value: Some(binding.get_param_dollarised()),
            bindings: self.bindings.into_iter().chain(vec![binding]).collect(),
            ..self
        })
    }
}

/// Update statement builder
pub struct UpdateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    target: String,
    content: Option<String>,
    merge: Option<String>,
    replace: Option<String>,
    set: Vec<String>,
    patch_ops: Vec<String>,
    where_: Option<String>,
    return_type: Option<ReturnType>,
    timeout: Option<String>,
    bindings: BindingsList,
    errors: ErrorList,
    parallel: bool,
    __model_return_type: PhantomData<T>,
}

impl<T> Queryable for UpdateStatement<T> where T: Serialize + DeserializeOwned + SurrealdbModel {}
impl<T> Erroneous for UpdateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

pub enum TargettablesForUpdate {
    Table(sql::Table),
    SurrealId(sql::Thing),
}

impl From<crate::Table> for TargettablesForUpdate {
    fn from(value: crate::Table) -> Self {
        Self::Table(value.into())
    }
}

impl From<&sql::Table> for TargettablesForUpdate {
    fn from(value: &sql::Table) -> Self {
        Self::Table(value.to_owned())
    }
}

impl From<&sql::Thing> for TargettablesForUpdate {
    fn from(value: &sql::Thing) -> Self {
        Self::SurrealId(value.to_owned().into())
    }
}

impl From<sql::Thing> for TargettablesForUpdate {
    fn from(value: sql::Thing) -> Self {
        Self::SurrealId(value.into())
    }
}

impl<T: SurrealdbModel> From<&SurrealId<T>> for TargettablesForUpdate {
    fn from(value: &SurrealId<T>) -> Self {
        Self::SurrealId(value.to_owned().to_thing())
    }
}

impl<T: SurrealdbModel> From<SurrealId<T>> for TargettablesForUpdate {
    fn from(value: SurrealId<T>) -> Self {
        Self::SurrealId(value.to_thing())
    }
}

// impl<T: SurrealdbModel> From<SurrealId<T>> for TargettablesForUpdate {
//     fn from(value: SurrealId<T>) -> Self {
//         Self::SurrealId(value.to_thing().into())
//     }
// }

impl From<sql::Table> for TargettablesForUpdate {
    fn from(value: sql::Table) -> Self {
        Self::Table(value)
    }
}

impl<T> UpdateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    /// Caution! Overrides all data even with default. Use with care. You may prefer `merge` with Updater instead e.g `UserUpdater`.
    /// Specify the full record data using the CONTENT keyword. The content must be serializable
    /// and implement SurrealdbModel trait.
    pub fn content(mut self, content: T) -> Self {
        let sql_value = sql::json(&serde_json::to_string(&content).unwrap()).unwrap();
        let binding = Binding::new(sql_value);
        self.content = Some(binding.get_param_dollarised());
        self.bindings.push(binding);
        self
    }

    /// merge-update only specific fields by using the MERGE keyword and specifying only the fields which are to be updated.
    pub fn merge(mut self, merge: impl Serialize) -> Self {
        let sql_value = sql::json(&serde_json::to_string(&merge).unwrap()).unwrap();
        let binding = Binding::new(sql_value);
        self.merge = Some(binding.get_param_dollarised());
        self.bindings.push(binding);
        self
    }

    /// Caution!
    /// Fully replaces weapon table with completely new object and data. This will remove all fields
    /// that are not present in the new object. This is a destructive operation.
    pub fn replace(mut self, merge: impl Serialize) -> Self {
        let sql_value = sql::json(&serde_json::to_string(&merge).unwrap()).unwrap();
        let binding = Binding::new(sql_value);
        self.replace = Some(binding.get_param_dollarised());
        self.bindings.push(binding);
        self
    }

    /// When specifying fields to update using the SET clause, it is possible to increment and decrement numeric values, and add or remove values from arrays. To increment a numeric value, or to add an item to an array, use the += operator. To decrement a numeric value, or to remove an value from an array, use the -= operator.
    pub fn set(mut self, settables: impl Into<Updateables>) -> Self {
        let settable: Updateables = settables.into();
        self.bindings.extend(settable.get_bindings());

        let setter_query = match settable {
            Updateables::Updater(up) => vec![up.build()],
            Updateables::Updaters(ups) => ups.into_iter().map(|u| u.build()).collect::<Vec<_>>(),
        };
        self.set.extend(setter_query);
        self
    }

    /// Specify the patch operations to be applied to the record using the PATCH keyword.
    ///
    /// # Arguments
    /// * `patch_op` - A patch operation to be applied to the record. Use the `patch`
    /// helper function.
    ///
    /// # Example
    /// ```rust
    /// # use surrealdb_query_builder as surreal_orm;
    /// use surreal_orm::{*, statements::patch};
    /// // Typically, you would use the schema to get the field(e.g `User::schema().name(E).first`) but using this as an example.
    /// let name = Field::new("name.first");
    /// let patch_op = patch(name).change("Oyelowo");
    /// assert_eq!(patch_op.to_raw().build(), "{ op: 'change', path: '/name/first', value: 'Oyelowo' }");
    pub fn patch(mut self, patch_op: impl Into<Vec<PatchOp>>) -> Self {
        let patch_op: Vec<PatchOp> = patch_op.into();
        for patch_op in patch_op {
            self.bindings.extend(patch_op.get_bindings());
            self.errors.extend(patch_op.get_errors());
            self.patch_ops.push(patch_op.build());
        }

        self
    }

    /// Adds a condition to the `` clause of the query.
    ///
    /// # Arguments
    ///
    /// * `condition` - Filter for the query.
    ///
    /// # Example
    ///
    /// ```rust, ignore
    /// // You can use a simple filter without the `cond` helper function
    /// .where_(age.greater_than_or_equal(18)
    ///
    /// // or with the `cond` helper function for multiple `AND` or `OR` conditions
    /// .where_(cond(age.greater_than_or_equal(18)).and(age.less_than_or_equal(90)))
    /// ```
    pub fn where_(mut self, condition: impl Conditional) -> Self {
        self.update_bindings(condition.get_bindings());
        let condition = Filter::new(condition);
        self.where_ = Some(condition.build());
        self
    }

    fn update_bindings(&mut self, bindings: BindingsList) -> &mut Self {
        self.bindings.extend(bindings);
        self
    }

    /// Sets the return type for the query.
    ///
    /// # Arguments
    ///
    /// * `return_type` - The type of return to set.
    ///
    /// # Examples
    ///
    /// Set the return type to `None`:
    ///
    /// ```rust,ignore
    /// statement.return_type(ReturnType::None);
    /// ```
    ///
    /// Set the return type to `Before`:
    ///
    /// ```rust,ignore
    /// statement.return_type(ReturnType::Before);
    /// ```
    ///
    /// Set the return type to `After`:
    ///
    /// ```rust,ignore
    /// statement.return_type(ReturnType::After);
    /// ```
    ///
    /// Set the return type to `Diff`:
    ///
    /// ```rust,ignore
    /// statement.return_type(ReturnType::Diff);
    /// ```
    ///
    /// Set the return type to a projection of specific fields:
    ///
    /// ```rust,ignore
    /// statement.return_type(ReturnType::Projections(vec![...]));
    /// ```
    pub fn return_type(mut self, return_type: impl Into<ReturnType>) -> Self {
        let return_type = return_type.into();
        self.return_type = Some(return_type);
        self
    }

    /// Sets the timeout duration for the query.
    ///
    /// # Arguments
    ///
    /// * `duration` - a value that can represent a duration for the timeout. This can be one of the following:
    ///
    ///   * `Duration` - a standard Rust `Duration` value.
    ///
    ///   * `Field` - an identifier for a specific field in the query, represented by an `Idiom` value.
    ///
    ///   * `Param` - a named parameter in the query, represented by a `Param` value.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let query = query.timeout(Duration::from_secs(30));
    ///
    /// assert_eq!(query.to_raw().to_string(), "30s");
    /// ```
    pub fn timeout(mut self, duration: impl Into<DurationLike>) -> Self {
        let duration: DurationLike = duration.into();
        self.timeout = Some(duration.to_raw().build());
        self
    }

    /// Indicates that the query should be executed in parallel.
    pub fn parallel(mut self) -> Self {
        self.parallel = true;
        self
    }
}

impl<T> Buildable for UpdateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    fn build(&self) -> String {
        let mut query = format!("UPDATE {}", self.target);

        if let Some(content) = &self.content {
            query = format!("{query} CONTENT  {content}",);
        } else if let Some(merge) = &self.merge {
            query = format!("{query} MERGE {merge}");
        } else if let Some(replace) = &self.replace {
            query = format!("{query} REPLACE {replace}");
        } else if !self.set.is_empty() {
            let set_vec = self.set.join(", ");
            query = format!("{query} SET {set_vec}");
        } else if !self.patch_ops.is_empty() {
            let patch_vec = self.patch_ops.join(", ");
            query = format!("{query} PATCH [{patch_vec}]");
        }

        if let Some(condition) = &self.where_ {
            query = format!("{query} WHERE {condition}");
        }

        if let Some(return_type) = &self.return_type {
            query = format!("{query} {return_type}");
        }

        if let Some(timeout) = &self.timeout {
            query = format!("{query} TIMEOUT {timeout}");
        }

        if self.parallel {
            query.push_str(" PARALLEL");
        }

        format!("{query};")
    }
}

impl<T> std::fmt::Display for UpdateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl<T> Parametric for UpdateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl<T> ReturnableDefault<T> for UpdateStatement<T> where
    T: Serialize + DeserializeOwned + SurrealdbModel
{
}

impl<T> ReturnableStandard<T> for UpdateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel + Send + Sync,
{
    fn set_return_type(mut self, return_type: ReturnType) -> Self {
        self.return_type = Some(return_type);
        self
    }

    fn get_return_type(&self) -> ReturnType {
        self.return_type.clone().unwrap_or(ReturnType::None)
    }
}

#[cfg(test)]
mod tests {
    use super::patch;
    use crate::*;

    #[test]
    fn can_build_patch_operation() {
        let email = Field::new("_email");

        let patch_op = patch(email).add("oyelowo@example.com");
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 2);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001, value: $_param_00000002 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'add', path: '/_email', value: 'oyelowo@example.com' }"
        );
    }

    #[test]
    fn gathers_errors_when_invalid_path_is_provided() {
        let email = Field::new("email[WHERE id = 1]");

        let patch_op = patch(email).add("Lowo");

        assert_eq!(patch_op.get_errors().len(), 1);
        assert_eq!(
            patch_op.get_errors().first().unwrap(),
            "The path you have provided is invalid. \
            Make sure that there are no clauses or conditions included. Valid path include \
            e.g name, name(E).first, name(E).first(E).second, etc."
        );
        assert_eq!(patch_op.get_bindings().len(), 2);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001, value: $_param_00000002 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'add', path: '/email[WHERE id = 1]', value: 'Lowo' }"
        );
    }

    #[test]
    fn gathers_error_when_clauses_uses() {
        get_invalid_paths(Field::new("name[WHERE id = 1]"));
        get_invalid_paths(Field::new("name[WHERE id = 1].first"));
        get_invalid_paths(Field::new("name[0]"));
        get_invalid_paths(Field::new("name[1]"));
        get_invalid_paths(Field::new("name[$]"));
        get_invalid_paths(Field::new("name[*]"));
        get_invalid_paths(Field::new("name->"));
        get_invalid_paths(Field::new("name->writes"));
        get_invalid_paths(Field::new("->writes"));
        get_invalid_paths(Field::new("->"));
        get_invalid_paths(Field::new("->->"));
        get_invalid_paths(Field::new("-"));
        get_invalid_paths(Field::new("_-"));
        get_invalid_paths(Field::new("-something"));
        get_invalid_paths(Field::new("name->writes->book"));
        get_invalid_paths(Field::new("->writes->book"));
        get_invalid_paths(Field::new("user:oye->write->blog:mars"));
        get_invalid_paths(Field::new(
            "->knows->person->(knows WHERE influencer = true)",
        ));
        get_invalid_paths(Field::new("5book"));
        get_invalid_paths(Field::new("-book_"));
        get_invalid_paths(Field::new("*book_"));
        get_invalid_paths(Field::new("$book_"));
        get_invalid_paths(Field::new("%book_"));
        get_invalid_paths(Field::new("&book_"));
        get_invalid_paths(Field::new("#book_"));
        get_invalid_paths(Field::new("@book_"));
        get_invalid_paths(Field::new("(book_"));
        get_invalid_paths(Field::new(")book_"));
        get_invalid_paths(Field::new("book*"));
        get_invalid_paths(Field::new("bo$ok"));
    }

    fn get_invalid_paths(field: Field) {
        let patch_op = patch(field).add("Lowo");

        assert_eq!(patch_op.get_errors().len(), 1);
        assert_eq!(
            patch_op.get_errors().first().unwrap(),
            "The path you have provided is invalid. \
            Make sure that there are no clauses or conditions included. Valid path include \
            e.g name, name(E).first, name(E).first(E).second, etc."
        );
    }

    #[test]
    fn can_build_add_operation() {
        let name = Field::new("_name.first");

        let patch_op = patch(name).add("Oyelowo");
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 2);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001, value: $_param_00000002 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'add', path: '/_name/first', value: 'Oyelowo' }"
        );
    }

    #[test]
    fn can_build_change_operation() {
        let name = Field::new("name.first");

        let patch_op = patch(name).change("Oyelowo");
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 2);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'change', path: $_param_00000001, value: $_param_00000002 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'change', path: '/name/first', value: 'Oyelowo' }"
        );
    }

    #[test]
    fn can_build_remove_operation() {
        let name = Field::new("name.first");

        let patch_op = patch(name).remove();
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 1);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'remove', path: $_param_00000001 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'remove', path: '/name/first' }"
        );
    }

    #[test]
    fn can_build_replace_operation() {
        let name = Field::new("name.first.title");

        let patch_op = patch(name).replace("Alien");
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 2);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'replace', path: $_param_00000001, value: $_param_00000002 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'replace', path: '/name/first/title', value: 'Alien' }"
        );
    }
}
