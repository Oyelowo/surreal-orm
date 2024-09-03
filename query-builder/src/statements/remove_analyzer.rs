/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */
use std::fmt::{self, Display};

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    Binding,
};

/// Creates a REMOVE ANALYZER statement builder.
///
/// Examples
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::remove_analyzer};
/// remove_analyzer("analyzer::standard").build();
/// ```
pub fn remove_analyzer(name: impl Display) -> RemoveAnalyzerStatement {
    let name: String = name.to_string();
    let name = Binding::new(name).as_raw();
    RemoveAnalyzerStatement {
        name: name.get_param_dollarised(),
        bindings: vec![name],
        errors: vec![],
    }
}

/// Represents the REMOVE ANALYZER statement.
pub struct RemoveAnalyzerStatement {
    name: String,
    bindings: BindingsList,
    errors: crate::ErrorList,
}

impl Queryable for RemoveAnalyzerStatement {}

impl Erroneous for RemoveAnalyzerStatement {
    fn get_errors(&self) -> crate::ErrorList {
        self.errors.clone()
    }
}

impl Parametric for RemoveAnalyzerStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.clone()
    }
}

impl Buildable for RemoveAnalyzerStatement {
    fn build(&self) -> String {
        format!("REMOVE ANALYZER {};", self.name)
    }
}

impl fmt::Display for RemoveAnalyzerStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{traits::Buildable, Field, Param, ToRaw};

    #[test]
    fn test_remove_analyzer_build() {
        let statement = remove_analyzer("analyzer::standard");
        assert_eq!(
            statement.to_raw().build(),
            "REMOVE ANALYZER analyzer::standard;"
        );
        assert_eq!(
            statement.fine_tune_params(),
            "REMOVE ANALYZER $_param_00000001;"
        );
        assert_eq!(statement.get_bindings().len(), 1);
    }

    #[test]
    fn test_remove_analyzer_build_with_field() {
        let field = Field::new("field");
        let statement = remove_analyzer(field);
        assert_eq!(statement.to_raw().build(), "REMOVE ANALYZER field;");
        assert_eq!(
            statement.fine_tune_params(),
            "REMOVE ANALYZER $_param_00000001;"
        );
        assert_eq!(statement.get_bindings().len(), 1);
    }

    #[test]
    fn test_remove_analyzer_build_with_param() {
        let param_analyzer_to_remove = Param::new("param_analyzer_to_remove");
        let statement = remove_analyzer(param_analyzer_to_remove.clone());
        assert_eq!(
            statement.to_raw().build(),
            "REMOVE ANALYZER $param_analyzer_to_remove;"
        );
        assert_eq!(
            statement.fine_tune_params(),
            "REMOVE ANALYZER $_param_00000001;"
        );
        assert_eq!(statement.get_bindings().len(), 1);
    }
}
