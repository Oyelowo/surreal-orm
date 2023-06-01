use std::fmt::Display;

use crate::{BindingsList, Buildable, Erroneous, ErrorList, Param, Parametric, Queryable, Valuex};

// DEFINE PARAM statement
// The DEFINE PARAM statement allows you to define global (database-wide) parameters that are available to every client.
//
// Requirements
// You must be authenticated as a root, namespace, or database user before you can use the DEFINE PARAM statement.
// You must select your namespace and database before you can use the DEFINE PARAM statement.
// Statement syntax
// DEFINE PARAM $@name VALUE @value;
// Example usage
// Below shows how you can define a global parameter using the DEFINE PARAM statement.
//
// DEFINE PARAM $endpointBase VALUE "https://dummyjson.com";
// Then, simply use the global parameter like you would with any variable.
//
// RETURN http::get($endpointBase + "/products");

/// Define a new parameter. DEFINE PARAM statement allows you to define global (database-wide) parameters that are available to every client.
///
/// Requirements
/// You must be authenticated as a root, namespace, or database user before you can use the DEFINE PARAM statement.
/// You must select your namespace and database before you can use the DEFINE PARAM statement.
///
/// Arguments
/// name: The name of the parameter.
/// value: The value of the parameter.
///
/// Example
///
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, CrudType::*, statements::{define_param}};
/// let endpoint_base = Param::new("endpointBase");
/// let statement = define_param(endpoint_base).value("https://dummyjson.com");
/// assert!(!statement.build().is_empty());
/// ```
pub fn define_param(param_name: impl Into<Param>) -> DefineParamStatement {
    let param_name: Param = param_name.into();
    DefineParamStatement {
        name: param_name.to_string(),
        value: None,
        bindings: vec![],
        errors: vec![],
    }
}

/// Define param statement
pub struct DefineParamStatement {
    name: String,
    value: Option<String>,
    bindings: BindingsList,
    errors: ErrorList,
}

impl DefineParamStatement {
    /// Set the value of the parameter.
    pub fn value(mut self, value: impl Into<Valuex>) -> Self {
        let value: Valuex = value.into();
        self.bindings.extend(value.get_bindings());
        self.errors.extend(value.get_errors());
        self.value = Some(value.build());
        self
    }
}

impl Buildable for DefineParamStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE PARAM {}", self.name);

        if let Some(value) = &self.value {
            query = format!("{query} VALUE {value}");
        }

        query
    }
}

impl Parametric for DefineParamStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Erroneous for DefineParamStatement {
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl Queryable for DefineParamStatement {}

impl Display for DefineParamStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};", self.build())
    }
}
