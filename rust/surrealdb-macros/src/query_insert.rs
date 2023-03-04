use std::{collections::HashMap, mem::BikeshedIntrinsicFrom};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use surrealdb::{
    engine::local::Db,
    method::Query,
    opt::QueryResult,
    sql::{self, Operator},
    Response, Surreal,
};

use crate::{
    db_field::Binding, query_select::QueryBuilderSelect, BindingsList, DbField, Parametric,
    SurrealdbNode,
};

pub struct InsertStatement<T: Serialize + DeserializeOwned + SurrealdbNode> {
    // table: String,
    values: Vec<T>,
    on_duplicate_key_update: Vec<String>,
    bindings: BindingsList,
}

pub enum Insertables<T: Serialize + DeserializeOwned + SurrealdbNode> {
    Node(T),
    Nodes(Vec<T>),
    FromQuery(QueryBuilderSelect),
}

impl<N: SurrealdbNode + DeserializeOwned + Serialize> Parametric for N {
    fn get_bindings(&self) -> BindingsList {
        let value = self;
        let field_names = get_field_names(value);
        // let mut variables = Vec::new();
        // let mut values = String::new();

        let xx = field_names
            .into_iter()
            .map(|field_name| {
                let field_value = get_field_value(value, &field_name)
                    .expect("Unable to get value name. This should never happen!");
                Binding::new(field_value).with_name(field_name)
            })
            .collect::<Vec<_>>();
        // let mut row_values = Vec::new();
        // for field_name in &field_names {
        //     let field_value = get_field_value(value, field_name)
        //         .expect("Unable to get value name. This should never happen!");
        //
        //     // let placeholder_var_names = format!("{}_{}", field_name, i);
        //     let placeholder_var_names = format!("{}_{}", field_name, i);
        //     variables.push((placeholder_var_names.clone(), field_value));
        //     // row_values.push(format!("${}", variables.len()));
        //     row_values.push(format!("${}", placeholder_var_names));
        // }
        // // let mut variables = Vec::new();
        // let mut values = String::new();
        //
        // for (i, value) in self.values.iter().enumerate() {
        //     let mut row_values = Vec::new();
        //     for field_name in &field_names {
        //         let field_value = get_field_value(value, field_name)?;
        //         let placeholder_var_names = format!("{}_{}", field_name, i);
        //         variables.push((placeholder_var_names.clone(), field_value));
        //         // row_values.push(format!("${}", variables.len()));
        //         row_values.push(format!("${}", placeholder_var_names));
        //     }
        xx
    }
}

impl<T: Serialize + DeserializeOwned + SurrealdbNode> Parametric for Insertables<T> {
    fn get_bindings(&self) -> crate::BindingsList {
        match self {
            Insertables::Node(node) => node.get_bindings(),
            Insertables::Nodes(nodes) => nodes
                .into_iter()
                .flat_map(|n| n.get_bindings())
                .collect::<Vec<_>>(),
            Insertables::FromQuery(query) => query.get_bindings(),
        }
    }
}

// impl<T: SurrealdbNode + Into<sql::Value>, N: SurrealdbNode + Serialize> From<N> for Insertables<T> {
//     fn from(value: N) -> Self {
//         let v = serde_json::to_string(&value).unwrap();
//         let v = sql::json(&v).unwrap();
//         let v: sql::Value = v.into();
//         Self::Node { field1: v }
//     }
// }

impl<T: Serialize + DeserializeOwned + SurrealdbNode> InsertStatement<T> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            on_duplicate_key_update: Vec::new(),
            bindings: vec![],
        }
    }

    pub fn insert<V: Into<Insertables<T>>>(mut self, value: V) -> Self {
        let value: Insertables<T> = value.into();
        let bindings = value.get_bindings();
        self.bindings.extend(bindings);
        // self.values.push(value);
        self
    }

    pub fn insert_many(&mut self, values: Vec<T>) -> &mut Self {
        self.values = values;
        self
    }

    fn on_duplicate_key_update(mut self, updateables: impl Into<Updateables>) -> Self {
        let updates: Updateables = updateables.into();
        self.bindings.extend(updates.get_bindings());
        // let update_map: HashMap<String, String> = updates
        //     .iter()
        //     .map(|(k, v)| (String::from(*k), String::from(*v)))
        //     .collect();
        // self.on_duplicate_key_update = (update_map);
        let updater_query = match updates {
            Updateables::Updater(up) => vec![up.get_updater_string()],
            Updateables::Updaters(ups) => ups
                .into_iter()
                .map(|u| u.get_updater_string())
                .collect::<Vec<_>>(),
        };
        self.on_duplicate_key_update.extend(updater_query);
        self
    }

    // pub fn build(&self) -> Result<(String, Vec<(String, Value)>), String> {
    pub fn build(self) -> String {
        // if self.values.is_empty() {
        //     return Err(String::from("No values to insert"));
        // }

        let bindings = self.bindings;
        // let first_value = self.values.get(0).unwrap();
        // let field_names = get_field_names(first_value);
        let field_names = bindings
            .iter()
            .map(|b| b.get_original_name().to_owned())
            .collect::<Vec<_>>();

        let mut query = String::new();
        query.push_str("INSERT INTO ");
        let table_name = T::get_table_name();
        query.push_str(&T::get_table_name());
        // query.push_str(&self.table);
        query.push_str(" (");
        query.push_str(&field_names.join(", "));
        query.push_str(") VALUES ");

        // let mut variables = Vec::new();
        // let mut values = String::new();

        let values = self
            .bindings
            .iter()
            .map(|b| format!("({}, {})", b.get_param(), b.get_value()))
            .collect::<Vec<_>>()
            .join(", ");

        // fmr (i, value) in self.values.iter().enumerate() {
        //     let mut row_values = Vec::new();
        //     for field_name in &field_names {
        //         let field_value = get_field_value(value, field_name)?;
        //         let placeholder_var_names = format!("{}_{}", field_name, i);
        //         variables.push((placeholder_var_names.clone(), field_value));
        //         // row_values.push(format!("${}", variables.len()));
        //         row_values.push(format!("${}", placeholder_var_names));
        //     }
        //     if i > 0 {
        //         values.push_str(", ");
        //     }
        //     values.push_str("(");
        //     values.push_str(&row_values.join(", "));
        //     values.push_str(")");
        // }

        query.push_str(&values);

        if !&self.on_duplicate_key_update.is_empty() {
            let updates_str = self.on_duplicate_key_update.join(", ");

            query.push_str(" ON DUPLICATE KEY UPDATE ");
            query.push_str(&updates_str);
        }

        query.push_str(";");
        query
    }

    pub async fn get_one(&self, db: Surreal<Db>) -> surrealdb::Result<T> {
        let (query, variables) = self.build().unwrap();
        let response = variables
            .clone()
            .iter()
            .fold(db.query(query), |acc, val| acc.bind(val))
            .await?
            .take::<Option<T>>(0)?;

        // TODO:: Handle error if nothing is returned
        Ok(response.unwrap())
    }

    pub async fn get_many(&self, db: Surreal<Db>) -> surrealdb::Result<Vec<T>> {
        let (query, variables) = self.build().unwrap();
        let response = variables
            .clone()
            .iter()
            .fold(db.query(query), |acc, val| acc.bind(val))
            .await?
            .take::<Vec<T>>(0)?;

        Ok(response)
    }
}

fn get_field_names<T>(value: &T) -> Vec<String>
where
    T: serde::Serialize,
{
    serde_json::to_value(value)
        .unwrap()
        .as_object()
        .unwrap()
        .keys()
        .map(ToString::to_string)
        .collect()
}

fn get_field_value<T: Serialize>(
    value: &T,
    field_name: &str,
) -> Result<surrealdb::sql::Value, String>
where
    T: serde::Serialize,
{
    let whole_struct = json!(value);
    // TODO: Improve error handling
    Ok(sql::json(&whole_struct[field_name].to_string())?)
}

/// A helper struct for generating SQL update statements.
pub struct Updater {
    column_updater_string: String,
    ____bindings: BindingsList,
}

impl Parametric for Updater {
    fn get_bindings(&self) -> BindingsList {
        todo!()
    }
}

pub fn updater(field: impl Into<DbField>) -> Updater {
    Updater::new(field)
}

impl std::fmt::Display for Updater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.column_updater_string))
    }
}

enum Updateables {
    Updater(Updater),
    Updaters(Vec<Updater>),
}

impl Parametric for Updateables {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Updateables::Updater(up) => up.get_bindings(),
            Updateables::Updaters(ups) => ups
                .into_iter()
                .flat_map(|u| u.get_bindings())
                .collect::<Vec<_>>(),
        }
    }
}

impl Updater {
    /// Creates a new `Updater` instance with the given column update string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("score = score + 1".to_string());
    /// ```
    pub fn new(db_field: impl Into<DbField>) -> Self {
        let db_field = db_field.into();
        Self {
            column_updater_string: db_field.to_string(),
            ____bindings: vec![],
        }
    }
    /// Returns a new `Updater` instance with the string to increment the column by the given value.
    /// Alias for plus_equal but idiomatically for numbers
    ///
    /// # Arguments
    ///
    /// * `value` - The value to increment the column by.
    ///
    /// # Examples
    ///
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("score".to_string());
    /// let updated_updater = updater.increment_by(2);
    /// assert_eq!(updated_updater.to_string(), "score += 2");
    /// ```
    pub fn increment_by(&self, value: impl Into<sql::Number>) -> Self {
        let value: sql::Number = value.into();
        self._____update_field(Operator::Inc, value)
    }

    /// Returns a new `Updater` instance with the string to append the given value to a column that stores an array.
    /// Alias for plus_equal but idiomatically for an array
    ///
    /// # Arguments
    ///
    /// * `value` - The value to append to the column's array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("tags += 'rust'".to_string());
    /// let updated_updater = updater.remove("python");
    /// assert_eq!(updated_updater.to_string(), "tags += 'rust'");
    /// ```
    pub fn append(&self, value: impl Into<sql::Value>) -> Self {
        self._____update_field(Operator::Inc, value)
    }

    /// Returns a new `Updater` instance with the string to decrement the column by the given value.
    /// Alias for minus_equal but idiomatically for an number
    ///
    /// # Arguments
    ///
    /// * `value` - The value to decrement the column by.
    ///
    /// # Examples
    ///
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("score".to_string());
    /// let updated_updater = updater.decrement_by(2);
    /// assert_eq!(updated_updater.to_string(), "score -= 2");
    /// ```
    pub fn decrement_by(&self, value: impl Into<sql::Number>) -> Self {
        let value: sql::Number = value.into();
        self._____update_field(Operator::Dec, value)
    }

    /// Returns a new `Updater` instance with the string to remove the given value from a column that stores an array.
    /// Alias for minus_equal but idiomatically for an array
    ///
    /// # Arguments
    ///
    /// * `value` - The value to remove from the column's array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("tags -= 'rust'".to_string());
    /// let updated_updater = updater.remove("python");
    /// assert_eq!(updated_updater.to_string(), "tags -= 'rust'");
    /// ```
    pub fn remove(&self, value: impl Into<sql::Value>) -> Self {
        self._____update_field(Operator::Dec, value)
    }

    /// Returns a new `Updater` instance with the string to add the given value to the column.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add to the column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("score = 5".to_string());
    /// let updated_updater = updater.plus_equal(2);
    /// assert_eq!(updated_updater.to_string(), "score = 5 + 2");
    /// ```
    pub fn plus_equal(&self, value: impl Into<sql::Value>) -> Self {
        self._____update_field(Operator::Inc, value)
    }

    /// Returns a new `Updater` instance with the string to remove the given value from the column.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to remove from the column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("name = 'John'".to_string());
    /// let updated_updater = updater.minus_equal("ohn");
    /// assert_eq!(updated_updater.to_string(), "name = 'J'");
    /// ```
    pub fn minus_equal(&self, value: impl Into<sql::Value>) -> Self {
        self._____update_field(Operator::Dec, value)
    }

    /// Returns the string representation of the column update statement.
    ///
    /// # Examples
    ///
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("score = score + 1".to_string());
    /// assert_eq!(updater.to_string(), "score = score + 1");
    /// ```
    pub fn get_updater_string(&self) -> String {
        self.column_updater_string
    }

    fn _____update_field(&self, operator: sql::Operator, value: impl Into<sql::Value>) -> Updater {
        let value: sql::Value = value.into();
        let binding = Binding::new(value);
        let column_updater_string = format!("{self} {operator} {}", binding.get_param());
        Self {
            column_updater_string,
            ____bindings: vec![binding],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_by() {
        let updater = Updater::new("score".to_string());
        let updated_updater = updater.increment_by(10);
        assert_eq!(updated_updater.get_updater_string(), "score += 10");
    }

    #[test]
    fn test_append() {
        let updater = Updater::new("names".to_string());
        let updated_updater = updater.append("Alice");
        assert_eq!(updated_updater.get_updater_string(), "names += 'Alice'");
    }

    #[test]
    fn test_decrement_by() {
        let updater = Updater::new("score".to_string());
        let updated_updater = updater.decrement_by(5);
        assert_eq!(updated_updater.get_updater_string(), "score -= 5");
    }

    #[test]
    fn test_remove() {
        let updater = Updater::new("names".to_string());
        let updated_updater = updater.remove("Alice");
        assert_eq!(updated_updater.get_updater_string(), "names -= 'Alice'");
    }

    #[test]
    fn test_plus_equal() {
        let updater = Updater::new("score".to_string());
        let updated_updater = updater.plus_equal(10);
        assert_eq!(updated_updater.get_updater_string(), "score += 10");
    }

    #[test]
    fn test_minus_equal() {
        let updater = Updater::new("names".to_string());
        let updated_updater = updater.minus_equal("Alice");
        assert_eq!(updated_updater.get_updater_string(), "names -= 'Alice'");
    }
}
