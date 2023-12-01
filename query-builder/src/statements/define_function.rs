/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{
    BindingsList, Block, Buildable, Erroneous, ErrorList, FieldType, Param, Parametric, Queryable,
};

/// Represents a surrealdb define function statement argument
#[derive(Debug, Clone)]
pub struct FunctionArgument {
    /// The name of the argument
    pub name: Param,
    /// The type of the argument
    pub type_: FieldType,
}

/// A function definition statement
#[derive(Debug, Clone)]
pub struct DefineFunctionStatement {
    name: String,
    args: Vec<FunctionArgument>,
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
            args: vec![],
            body: None,
            bindings: vec![],
            errors: vec![],
        }
    }

    /// Sets the arguments for the function
    pub fn arguments(mut self, args: Vec<FunctionArgument>) -> Self {
        self.args = args;
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
        args: vec![],
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
                .args
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
mod tests {}
