/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{
    BindingsList, Block, Buildable, Erroneous, ErrorList, FieldType, Param, Parametric, Queryable,
};

#[derive(Debug, Clone)]
pub struct FunctionArgument {
    name: Param,
    type_: FieldType,
}

/// A function definition statement
#[derive(Debug, Clone)]
pub struct DefineFunctionStatement {
    name: String,
    params: Vec<FunctionArgument>,
    body: Option<Block>,
    bindings: BindingsList,
    errors: ErrorList,
}

impl DefineFunctionStatement {
    /// Create a new function definition statement
    ///
    /// # Arguments
    /// * `name` - The name of the function
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: vec![],
            body: None,
            bindings: vec![],
            errors: vec![],
        }
    }

    /// Sets the parameters for the function
    pub fn arguments(mut self, params: Vec<FunctionArgument>) -> Self {
        self.params = params;
        self
    }

    /// Sets the body of the function
    pub fn body(mut self, body: Block) -> Self {
        self.bindings.extend(body.get_bindings());
        self.errors.extend(body.get_errors());
        self.body = Some(body);
        self
    }
}

/// Create a new function definition statement
pub fn define_function(name: impl Into<String>) -> DefineFunctionStatement {
    DefineFunctionStatement {
        name: name.into(),
        params: vec![],
        body: None,
        bindings: vec![],
        errors: vec![],
    }
}

impl Parametric for DefineFunctionStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Erroneous for DefineFunctionStatement {
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl Queryable for DefineFunctionStatement {}

impl Buildable for DefineFunctionStatement {
    fn build(&self) -> String {
        let mut build = format!("DEFINE FUNCTION {}(", self.name);
        build.push_str(
            &self
                .params
                .iter()
                .map(|FunctionArgument { name, type_ }| format!("{}: {}", name.build(), type_))
                .collect::<Vec<String>>()
                .join(", "),
        );
        build.push_str(") ");
        if let Some(body) = &self.body {
            build.push_str(&body.build());
        }
        format!("{build};")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    define_function!(get_it(first: bool, last: string, birthday: string) {
        let person = "43";
        return person;
    });

    #[test]
    fn test_define_function() {
        let fn_statement = get_it_statement();

        insta::assert_display_snapshot!(fn_statement.to_raw().build());
        insta::assert_display_snapshot!(fn_statement.fine_tune_params());
        assert_eq!(
            fn_statement.to_raw().build(),
            "DEFINE FUNCTION get_it($first: bool, $last: string, $birthday: string) {\n\
                LET $person = '43';\n\n\
                RETURN $person;\n\
                };"
        );
        assert_eq!(
            fn_statement.fine_tune_params(),
            "DEFINE FUNCTION get_it($first: bool, $last: string, $birthday: string) {\n\
            LET $person = $_param_00000001;\n\n\
            RETURN $person;\n\
            };"
        );
        let get_it_function = get_it(false, "3".to_string(), "3".to_string());
        assert_eq!(get_it_function.to_raw().build(), "get_it(false, '3', '3')");
        assert_eq!(
            get_it_function.fine_tune_params(),
            "get_it($_param_00000001, $_param_00000002, $_param_00000003)"
        );
    }
}
