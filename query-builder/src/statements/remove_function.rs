/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::{self, Display};

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    Binding,
};

/// Creates a REMOVE FUNCTION statement builder.
///
/// Examples
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::remove_function};
/// remove_function("fn::update_author").build();
/// ```
pub fn remove_function(name: impl Display) -> RemoveFunctionStatement {
    let name: String = name.to_string();
    let name = Binding::new(name).as_raw();
    RemoveFunctionStatement {
        name: name.get_param_dollarised(),
        bindings: vec![name],
        errors: vec![],
    }
}

/// Represents the REMOVE FUNCTION statement.
pub struct RemoveFunctionStatement {
    name: String,
    bindings: BindingsList,
    errors: crate::ErrorList,
}

impl Queryable for RemoveFunctionStatement {}

impl Erroneous for RemoveFunctionStatement {
    fn get_errors(&self) -> crate::ErrorList {
        self.errors.clone()
    }
}

impl Parametric for RemoveFunctionStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.clone()
    }
}

impl Buildable for RemoveFunctionStatement {
    fn build(&self) -> String {
        format!(
            "REMOVE FUNCTION fn::{};",
            self.name.trim_start_matches("fn::")
        )
    }
}

impl fmt::Display for RemoveFunctionStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{traits::Buildable, Param, ToRaw};

    #[test]
    fn test_remove_function_build() {
        let statement = remove_function("fn::update_author");
        assert_eq!(
            statement.to_raw().build(),
            "REMOVE FUNCTION fn::update_author;"
        );
        assert_eq!(
            statement.fine_tune_params(),
            "REMOVE FUNCTION $_param_00000001;"
        );
        assert_eq!(statement.get_bindings().len(), 1);
    }

    #[test]
    fn test_remove_function_build_error() {
        let param_function_to_remove = Param::new("param_function_to_remove");
        let statement = remove_function(param_function_to_remove);
        assert_eq!(
            statement.to_raw().build(),
            "REMOVE FUNCTION $param_function_to_remove;"
        );
        assert_eq!(
            statement.fine_tune_params(),
            "REMOVE FUNCTION $_param_00000001;"
        );
        assert_eq!(statement.get_bindings().len(), 1);
    }
}
