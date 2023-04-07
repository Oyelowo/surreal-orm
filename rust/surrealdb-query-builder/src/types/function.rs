use std::fmt::Display;

use crate::{
    traits::{BindingsList, Buildable, Parametric},
    Aliasable, Erroneous, ErrorList,
};

#[derive(Debug, Clone)]
pub struct Function {
    pub query_string: String,
    pub bindings: BindingsList,
    // pub errors: ErrorList,
}

impl Parametric for Function {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Buildable for Function {
    fn build(&self) -> String {
        self.query_string.clone()
    }
}

impl Erroneous for Function {
    // fn get_errors(&self) -> ErrorList {
    // self.errors.to_vec()
    // }
}

impl Aliasable for Function {}
