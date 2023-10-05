use std::fmt::{self, Display};

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    Binding,
};

/// Creates a REMOVE PARAM statement builder.
///
/// Examples
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::remove_param};
/// remove_param("website_name");
/// ```
pub fn remove_param(name: impl Display) -> RemoveParamStatement {
    let name: String = name.to_string();
    let name = Binding::new(name).as_raw();

    RemoveParamStatement {
        name: name.get_param_dollarised(),
        bindings: vec![name],
        errors: vec![],
    }
}

/// Represents the REMOVE PARAM statement.
pub struct RemoveParamStatement {
    name: String,
    bindings: BindingsList,
    errors: crate::ErrorList,
}

impl Queryable for RemoveParamStatement {}

impl Erroneous for RemoveParamStatement {
    fn get_errors(&self) -> crate::ErrorList {
        self.errors.clone()
    }
}

impl Parametric for RemoveParamStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.clone()
    }
}

impl Buildable for RemoveParamStatement {
    fn build(&self) -> String {
        format!("REMOVE PARAM {};", self.name)
    }
}

impl fmt::Display for RemoveParamStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{traits::Buildable, Field, Param, ToRaw};

    #[test]
    fn test_remove_param_build() {
        let statement = remove_param("website_name");
        assert_eq!(statement.to_raw().build(), "REMOVE PARAM website_name;");
        assert_eq!(
            statement.fine_tune_params(),
            "REMOVE PARAM $_param_00000001;"
        );
        assert_eq!(statement.get_bindings().len(), 1);
    }

    #[test]
    fn test_remove_param_build_with_param() {
        let param_variable = Param::new("param_variable");
        let statement = remove_param(param_variable);
        assert_eq!(statement.to_raw().build(), "REMOVE PARAM $param_variable;");
        assert_eq!(
            statement.fine_tune_params(),
            "REMOVE PARAM $_param_00000001;"
        );
        assert_eq!(statement.get_bindings().len(), 1);
    }

    #[test]
    fn test_remove_param_build_with_field() {
        let field = Field::new("field");
        let statement = remove_param(field);

        assert_eq!(statement.to_raw().build(), "REMOVE PARAM field;");
        assert_eq!(
            statement.fine_tune_params(),
            "REMOVE PARAM $_param_00000001;"
        );
        assert_eq!(statement.get_bindings().len(), 1);
    }
}
