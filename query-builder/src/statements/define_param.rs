/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::{fmt::Display, ops::Deref};

use crate::{
    BindingsList, Buildable, Erroneous, ErrorList, Param, Parametric, Queryable, ValueLike,
};

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
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, CrudType::*, statements::{define_param}};
///   // First create the param name as rust function i.e endpoint_base()
///   create_param_name_fn!(endpoint_base);
///   // Can also we wwriten as below if you want to add a doc comment.
///   // create_param_name_fn!(
///   //  /// endpoint of codebreather.com
///   //  =>
///   //  endpoint_base
///   // );
/// let statement = define_param(endpoint_base()).value("https://dummyjson.com");
/// assert!(!statement.build().is_empty());
/// ```
pub fn define_param(param_name: impl Deref<Target = Param>) -> DefineParamStatementBuilder {
    let param_name: &Param = param_name.deref();
    let define_param_statement = DefineParamStatement {
        name: param_name.to_string(),
        value: None,
        bindings: vec![],
        errors: vec![],
    };
    DefineParamStatementBuilder(define_param_statement)
}

pub struct DefineParamStatementBuilder(DefineParamStatement);

/// Define param statement
pub struct DefineParamStatement {
    name: String,
    value: Option<String>,
    bindings: BindingsList,
    errors: ErrorList,
}

impl DefineParamStatementBuilder {
    /// Set the value of the parameter.
    pub fn value(mut self, value: impl Into<ValueLike>) -> DefineParamStatement {
        let value: ValueLike = value.into();
        self.0.bindings.extend(value.get_bindings());
        self.0.errors.extend(value.get_errors());
        self.0.value = Some(value.build());
        self.0
    }
}

impl Buildable for DefineParamStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE PARAM {}", self.name);

        if let Some(value) = &self.value {
            query = format!("{query} VALUE {value}");
        }

        format!("{query};")
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
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use crate::{create_param_name_fn, ToRaw};

    use super::*;
    // fn endpoint_base() -> Param {
    //     Param::new("endpoint_base")
    // }

    create_param_name_fn!(endpoint_base_without_doc);

    #[test]
    fn test_define_param_statement_without_doc() {
        let statement = define_param(endpoint_base_without_doc()).value("https://codebreather.com");
        assert_eq!(
            statement.to_raw().build(),
            "DEFINE PARAM $endpoint_base_without_doc VALUE 'https://codebreather.com';"
        );

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE PARAM $endpoint_base_without_doc VALUE $_param_00000001;"
        );
    }

    create_param_name_fn!(
        /// endpoint of codebreather.com
        =>
        endpoint_base_with_doc
    );

    #[test]
    fn test_define_param_statement() {
        let statement = define_param(endpoint_base_with_doc()).value("https://codebreather.com");
        assert_eq!(
            statement.to_raw().build(),
            "DEFINE PARAM $endpoint_base_with_doc VALUE 'https://codebreather.com';"
        );

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE PARAM $endpoint_base_with_doc VALUE $_param_00000001;"
        );
    }
}
