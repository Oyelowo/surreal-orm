/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
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
use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql;

use crate::{
    derive_binding_and_errors_from_value, Binding, BindingsList, Buildable, Conditional,
    DurationLike, Erroneous, ErrorList, Filter, Model, Parametric, PatchOp, Queryable, ReturnType,
    ReturnableDefault, ReturnableStandard, Setter, SurrealId, SurrealSimpleId, SurrealUlid,
    SurrealUuid, ToRaw,
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
/// # use surreal_query_builder as surreal_orm;
/// use std::time::Duration;
/// use surreal_orm::{*, statements::update};
///
/// // Update using set method
/// update::<User>(user)
///     .set(object_partial!(User { score: 5 }))
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
pub fn update<T>(targettables: impl Into<TargettablesForUpdate>) -> UpdateStatementInit<T>
where
    T: Serialize + DeserializeOwned + Model,
{
    let table_name = T::table();
    let targettables: TargettablesForUpdate = targettables.into();
    let mut bindings = vec![];
    let mut errors = vec![];
    let param = match targettables {
        TargettablesForUpdate::Table(table) => {
            let table = table.to_string();
            if table != table_name.to_string() {
                errors.push(format!(
                    "table name -{table} does not match the surreal model struct type which belongs to {table} table"
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

    UpdateStatementInit {
        target: param,
        is_only: false,
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

/// Creates a new UPDATE statement and returns a single object.
/// The UPDATE statement can be used to update or modify records in the database.
///
/// # Arguments
///
/// * `connection` - built using `with` method on a node. e.g `Student::with(..).writes(..).book(..)`
/// # Examples
///
/// ```rust, ignore
/// # use surreal_query_builder as surreal_orm;
/// use std::time::Duration;
/// use surreal_orm::{*, statements::update};
///
/// // Update using set method
/// update_only::<User>(user)
///     .set(object_partial!(
///         User {
///             score: 5
///         }))
///     .where_(age.greater_than(18));
///
/// // Update many records that match the filter using content method in user table
/// update_only::<User>(user)
///     .content(
///          User {
///             team: "Codebreather",
///             ...
///          }
///     ).where_(cond(age.greater_than(18)).and(name.like("codebreather"));
///     
/// // Update many records that match the filter using merge method in user table
/// update_only::<User>(user)
///     .merge(
///          UserDocument {
///             hobbies: vec!["music production", "problem solving", "rust"],
///             ...
///          }
///     ).where_(cond(age.greater_than(18)).and(name.like("codebreather"));
///
/// // Update specific record using content method
/// update_only::<User>(user1)
///     .content(
///          User {
///             name: "Oyelowo".into(),
///             age: 198,
///             ...
///          }
///     );
///     
/// // Update using content method
/// update_only::<User>(user2)
///     .merge(
///          UserDocument {
///             hobbies: vec!["music production", "problem solving", "rust"],
///             ...
///          }
///     );
/// ```
pub fn update_only<T>(targettables: impl Into<TargettablesForUpdate>) -> UpdateStatementInit<T>
where
    T: Serialize + DeserializeOwned + Model,
{
    let mut statement = update(targettables);
    statement.is_only = true;
    statement
}

/// Update statement initializer
#[derive(Debug, Clone)]
pub struct UpdateStatementInit<T>
where
    T: Serialize + DeserializeOwned + Model,
{
    target: String,
    is_only: bool,
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

impl<T> Queryable for UpdateStatement<T> where T: Serialize + DeserializeOwned + Model {}
impl<T> Erroneous for UpdateStatement<T>
where
    T: Serialize + DeserializeOwned + Model,
{
    fn get_errors(&self) -> ErrorList {
        self.0.errors.to_vec()
    }
}

pub enum TargettablesForUpdate {
    Table(sql::Table),
    SurrealId(sql::Thing),
}

impl<T> From<T> for TargettablesForUpdate
where
    T: Serialize + DeserializeOwned + Model,
{
    fn from(value: T) -> Self {
        Self::SurrealId(value.get_id_as_thing())
    }
}

impl From<crate::Table> for TargettablesForUpdate {
    fn from(value: crate::Table) -> Self {
        Self::Table(value.into())
    }
}

impl From<&crate::Table> for TargettablesForUpdate {
    fn from(value: &crate::Table) -> Self {
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
        Self::SurrealId(value.to_owned())
    }
}

impl From<sql::Thing> for TargettablesForUpdate {
    fn from(value: sql::Thing) -> Self {
        Self::SurrealId(value)
    }
}

impl<T, Id> From<SurrealId<T, Id>> for TargettablesForUpdate
where
    T: Model,
    Id: Into<sql::Id>,
{
    fn from(value: SurrealId<T, Id>) -> Self {
        Self::SurrealId(value.to_thing())
    }
}

impl<T, Id> From<&SurrealId<T, Id>> for TargettablesForUpdate
where
    T: Model,
    Id: Into<sql::Id>,
{
    fn from(value: &SurrealId<T, Id>) -> Self {
        Self::SurrealId(value.to_owned().to_thing())
    }
}

impl<T> From<SurrealSimpleId<T>> for TargettablesForUpdate
where
    T: Model,
{
    fn from(value: SurrealSimpleId<T>) -> Self {
        Self::SurrealId(value.to_thing())
    }
}

impl<T> From<&SurrealSimpleId<T>> for TargettablesForUpdate
where
    T: Model,
{
    fn from(value: &SurrealSimpleId<T>) -> Self {
        Self::SurrealId(value.to_owned().to_thing())
    }
}

impl<T> From<SurrealUuid<T>> for TargettablesForUpdate
where
    T: Model,
{
    fn from(value: SurrealUuid<T>) -> Self {
        Self::SurrealId(value.to_thing())
    }
}

impl<T> From<&SurrealUuid<T>> for TargettablesForUpdate
where
    T: Model,
{
    fn from(value: &SurrealUuid<T>) -> Self {
        Self::SurrealId(value.to_owned().to_thing())
    }
}

impl<T> From<SurrealUlid<T>> for TargettablesForUpdate
where
    T: Model,
{
    fn from(value: SurrealUlid<T>) -> Self {
        Self::SurrealId(value.to_thing())
    }
}

impl<T> From<&SurrealUlid<T>> for TargettablesForUpdate
where
    T: Model,
{
    fn from(value: &SurrealUlid<T>) -> Self {
        Self::SurrealId(value.to_owned().to_thing())
    }
}

impl From<sql::Table> for TargettablesForUpdate {
    fn from(value: sql::Table) -> Self {
        Self::Table(value)
    }
}

impl<T> UpdateStatementInit<T>
where
    T: Serialize + DeserializeOwned + Model,
{
    /// Caution! Overrides all data even with default. Use with care. You may prefer `merge` with Updater instead e.g `UserUpdater`.
    /// Specify the full record data using the CONTENT keyword. The content must be serializable
    /// and implement Model trait.
    pub fn content(mut self, content: T) -> UpdateStatement<T> {
        let (binding, errors) = derive_binding_and_errors_from_value(&content);
        self.content = Some(binding.get_param_dollarised());
        self.bindings.push(binding);
        self.errors.extend(errors);
        self.into()
    }

    /// merge-update only specific fields by using the MERGE keyword and specifying only the fields which are to be updated.
    pub fn merge(mut self, merge: impl Serialize) -> UpdateStatement<T> {
        let (binding, errors) = derive_binding_and_errors_from_value(&merge);
        self.merge = Some(binding.get_param_dollarised());
        self.bindings.push(binding);
        self.errors.extend(errors);
        self.into()
    }

    /// Caution!
    /// Fully replaces weapon table with completely new object and data. This will remove all fields
    /// that are not present in the new object. This is a destructive operation.
    pub fn replace(mut self, replacement: impl Serialize) -> UpdateStatement<T> {
        let (binding, errors) = derive_binding_and_errors_from_value(&replacement);
        self.replace = Some(binding.get_param_dollarised());
        self.bindings.push(binding);
        self.errors.extend(errors);
        self.into()
    }

    /// When specifying fields to update using the SET clause,
    /// it is possible to increment and decrement numeric values,
    /// and add or remove values from arrays.
    /// To increment a numeric value, or to add an item to an array,
    /// use the `append` or incremenet (i.e +=) operator. To decrement a numeric value,
    /// or to remove an value from an array, use the `decrement` or `remove` (i.e -=) operator.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// // Set fields using a helper macro function:
    /// .set(object_partial!(Weapon {
    ///     id: weapon_id.clone(),
    ///     name: "Laser".to_string()
    /// }))
    ///
    /// // Set multiple fields as an array or vector:
    /// .set([name.equal_to("Laser"), damage.increment_by(100)]);
    ///
    /// // set a single field number. Generates  =
    /// .set(score.equal_to(5))
    ///
    /// // increment a field number. Generates  +=
    /// .set(score.increment_by(5))
    ///
    /// // decrement a field number. Generates  -=
    /// .set(score.decrement_by(5))
    ///
    /// // add to an array. Generates  +=
    /// .set(friends_names.append("Oyelowo"))
    ///
    /// // remove value from an array. Generates  -=
    /// .set(friends_names.remove("Oyedayo"))
    pub fn set(mut self, settables: impl Into<Vec<Setter>>) -> UpdateStatement<T> {
        let settable: Vec<Setter> = settables.into();

        let (settable, bindings) = settable.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut settable, mut bindings), s| {
                settable.push(s.build());
                bindings.extend(s.get_bindings());
                (settable, bindings)
            },
        );

        self.bindings.extend(bindings);
        self.set.extend(settable);
        self.into()
    }

    /// Specify the patch operations to be applied to the record using the PATCH keyword.
    ///
    /// # Arguments
    /// * `patch_op` - A patch operation to be applied to the record. Use the `patch`
    /// helper function.
    ///
    /// # Example
    /// ```rust, ignore
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, statements::patch};
    /// // Typically, you would use the schema to get the field(e.g `User::schema().name(E).first`) but using this as an example.
    /// name.patch_replace("Oyelowo");
    /// name.patch_add("Oyelowo");
    /// name.patch_remove();
    /// // regex search and replace
    /// name.patch_change("@@ -1,4 +1,4 @@\n te\n-s\n+x\n t\n");
    pub fn patch(mut self, patch_op: impl Into<Vec<PatchOp>>) -> UpdateStatement<T> {
        let patch_op: Vec<PatchOp> = patch_op.into();
        for patch_op in patch_op {
            self.bindings.extend(patch_op.get_bindings());
            self.errors.extend(patch_op.get_errors());
            self.patch_ops.push(patch_op.build());
        }

        self.into()
    }
}

/// A builder for update statements.
pub struct UpdateStatement<T>(UpdateStatementInit<T>)
where
    T: Serialize + DeserializeOwned + Model;

impl<T> From<UpdateStatementInit<T>> for UpdateStatement<T>
where
    T: Serialize + DeserializeOwned + Model,
{
    fn from(value: UpdateStatementInit<T>) -> Self {
        Self(value)
    }
}

impl<T> UpdateStatement<T>
where
    T: Serialize + DeserializeOwned + Model,
{
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
        self.0.where_ = Some(condition.build());
        self
    }

    fn update_bindings(&mut self, bindings: BindingsList) -> &mut Self {
        self.0.bindings.extend(bindings);
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
        self.0.return_type = Some(return_type);
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
        self.0.timeout = Some(duration.to_raw().build());
        self
    }

    /// Indicates that the query should be executed in parallel.
    pub fn parallel(mut self) -> Self {
        self.0.parallel = true;
        self
    }
}

impl<T> Buildable for UpdateStatement<T>
where
    T: Serialize + DeserializeOwned + Model,
{
    fn build(&self) -> String {
        let statement = &self.0;
        let mut query = format!("UPDATE {}", statement.target);

        if statement.is_only {
            query = format!("{query} ONLY");
        }

        if let Some(content) = &statement.content {
            query = format!("{query} CONTENT  {content}",);
        } else if let Some(merge) = &statement.merge {
            query = format!("{query} MERGE {merge}");
        } else if let Some(replace) = &statement.replace {
            query = format!("{query} REPLACE {replace}");
        } else if !statement.set.is_empty() {
            let set_vec = statement.set.join(", ");
            query = format!("{query} SET {set_vec}");
        } else if !statement.patch_ops.is_empty() {
            let patch_vec = statement.patch_ops.join(", ");
            query = format!("{query} PATCH [{patch_vec}]");
        }

        if let Some(condition) = &statement.where_ {
            query = format!("{query} WHERE {condition}");
        }

        if let Some(return_type) = &statement.return_type {
            query = format!("{query} {return_type}");
        }

        if let Some(timeout) = &statement.timeout {
            query = format!("{query} TIMEOUT {timeout}");
        }

        if statement.parallel {
            query.push_str(" PARALLEL");
        }

        format!("{query};")
    }
}

impl<T> std::fmt::Display for UpdateStatement<T>
where
    T: Serialize + DeserializeOwned + Model,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl<T> Parametric for UpdateStatement<T>
where
    T: Serialize + DeserializeOwned + Model,
{
    fn get_bindings(&self) -> BindingsList {
        self.0.bindings.to_vec()
    }
}

impl<T> ReturnableDefault<T> for UpdateStatement<T> where T: Serialize + DeserializeOwned + Model {}

impl<T> ReturnableStandard<T> for UpdateStatement<T>
where
    T: Serialize + DeserializeOwned + Model + Send + Sync,
{
    fn set_return_type(mut self, return_type: ReturnType) -> Self {
        self.0.return_type = Some(return_type);
        self
    }

    fn get_return_type(&self) -> ReturnType {
        self.0.return_type.clone().unwrap_or(ReturnType::None)
    }
}
