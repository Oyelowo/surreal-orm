/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{statements::SelectStatement, Buildable, Parametric, Valuex};

#[derive(Clone, Debug)]
pub struct Expression(Valuex);

impl std::ops::Deref for Expression {
    type Target = Valuex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for Expression
where
    T: Into<Valuex>,
{
    fn from(value: T) -> Self {
        Expression(value.into())
    }
}

impl From<SelectStatement> for Expression {
    fn from(select_statement: SelectStatement) -> Self {
        Expression(Valuex {
            string: format!("( {} )", select_statement.build().trim_end_matches(";")),
            bindings: select_statement.get_bindings(),
        })
    }
}
