use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql;

use crate::{
    db_field::Binding,
    query_insert::{Buildable, Runnable, Updateables},
    query_relate::Return,
    query_update::{self, Targettable},
    BindingsList, Parametric, SurrealdbNode,
};

pub fn update<T>(targettables: impl Into<Targettable>) -> CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    // TODO: Pass this to UpdateStatement constructor and gather the errors to be handled when
    // query is run using one of the run methods.
    let table_name = T::table_name();
    let targettables: Targettable = targettables.into();
    if !targettables
        .get_bindings()
        .first()
        .unwrap()
        .get_value()
        .to_raw_string()
        .starts_with(&table_name.to_string())
    {
        panic!("You're trying to update into the wrong table");
    }
    // let errors: String = connection.get_errors();
    let mut builder = CreateStatement::<T>::new(targettables);
    builder
    // builder.new(targettables)
}

pub struct CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    target: String,
    content: Option<String>,
    set: Vec<String>,
    return_type: Option<Return>,
    timeout: Option<String>,
    parallel: bool,
    bindings: BindingsList,
    __model_return_type: PhantomData<T>,
}

impl<T> CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    pub fn new(targettables: impl Into<query_update::Targettable>) -> Self {
        let targets: Targettable = targettables.into();
        let targets_bindings = targets.get_bindings();

        let mut target_names = match targets {
            Targettable::Table(table) => vec![table.to_string()],
            Targettable::SurrealId(_) => targets_bindings
                .iter()
                .map(|b| format!("${}", b.get_param()))
                .collect::<Vec<_>>(),
        };

        Self {
            target: target_names
                .pop()
                .expect("Table or record id must exist here. this is a bug"),
            content: None,
            set: Vec::new(),
            return_type: None,
            timeout: None,
            parallel: false,
            bindings: targets_bindings,
            __model_return_type: PhantomData,
        }
    }

    pub fn content(mut self, content: T) -> Self {
        let sql_value = sql::json(&serde_json::to_string(&content).unwrap()).unwrap();
        let binding = Binding::new(sql_value);
        self.content = Some(binding.get_param().to_owned());
        self.bindings.push(binding);
        self
    }

    pub fn set(mut self, settables: impl Into<Updateables>) -> Self {
        let settable: Updateables = settables.into();
        self.bindings.extend(settable.get_bindings());

        let setter_query = match settable {
            Updateables::Updater(up) => vec![up.get_updater_string()],
            Updateables::Updaters(ups) => ups
                .into_iter()
                .map(|u| u.get_updater_string())
                .collect::<Vec<_>>(),
        };
        self.set.extend(setter_query);
        self
    }

    pub fn return_(mut self, return_type: impl Into<Return>) -> Self {
        let return_type = return_type.into();
        self.return_type = Some(return_type);
        self
    }

    /// Sets the timeout duration for the query.
    ///
    /// # Arguments
    ///
    /// * `duration` - a string slice that specifies the timeout duration. It can be expressed in any format that the database driver supports.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_db_client::{Query, QueryBuilder};
    ///
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.timeout("5s");
    /// ```
    ///
    /// ---
    ///
    /// Indicates that the query should be executed in parallel.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_db_client::{Query, QueryBuilder};
    ///
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.parallel();
    /// ```
    pub fn timeout(mut self, duration: impl Into<crate::query_select::Duration>) -> Self {
        let duration: crate::query_select::Duration = duration.into();
        let duration = sql::Duration::from(duration);
        self.timeout = Some(duration.to_string());
        self
    }

    pub fn parallel(mut self) -> Self {
        self.parallel = true;
        self
    }
}

impl<T> Buildable for CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn build(&self) -> String {
        let mut query = String::new();

        query.push_str("CREATE ");
        query.push_str(&self.target);

        if let Some(content) = &self.content {
            query.push_str(" CONTENT ");
            query.push_str(&content);
        } else if !&self.set.is_empty() {
            query += "SET ";
            let set_vec = self.set.join(", ");
            query += &set_vec;
            query += " ";
        }

        if let Some(return_type) = &self.return_type {
            query.push_str(" RETURN ");
            match return_type {
                Return::None => query.push_str("NONE"),
                Return::Before => query.push_str("BEFORE"),
                Return::After => query.push_str("AFTER"),
                Return::Diff => query.push_str("DIFF"),
                Return::Projections(projections) => {
                    for (i, projection) in projections.iter().enumerate() {
                        if i > 0 {
                            query.push_str(", ");
                        }
                        query.push_str(&projection.to_string());
                    }
                }
            }
        }

        if let Some(timeout) = &self.timeout {
            query.push_str(" TIMEOUT ");
            query.push_str(&timeout);
        }

        if self.parallel {
            query.push_str(" PARALLEL");
        }

        query.push_str(";");

        query
    }
}

impl<T> std::fmt::Display for CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.build()))
    }
}

impl<T> Parametric for CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn get_bindings(&self) -> crate::BindingsList {
        self.bindings.to_vec()
    }
}

impl<T> Runnable<T> for CreateStatement<T> where T: Serialize + DeserializeOwned + SurrealdbNode {}
