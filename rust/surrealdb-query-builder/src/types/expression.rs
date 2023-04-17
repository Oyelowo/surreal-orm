/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::{self, Display};

use surrealdb::sql;

use crate::{
    statements::SelectStatement,
    traits::{Binding, BindingsList, Parametric},
    Buildable,
};

#[derive(Clone, Debug)]
pub struct ValueWithBinding {
    value: String,
    binding: Binding,
}

// impl Buildable for ValueWithBinding {
//     fn build(&self) -> String {
//         self.binding.to_vec
//     }
// }
#[derive(Clone, Debug)]
pub enum Expression {
    SelectStatement(SelectStatement),
    Value(ValueWithBinding),
}

impl Buildable for Expression {
    fn build(&self) -> String {
        match self {
            Expression::SelectStatement(s) => s.build().trim_end_matches(";").to_string(),
            // Expression::SelectStatement(s) => format!("( {} )", s.build().trim_end_matches(";")),
            Expression::Value(v) => v.binding.get_param_dollarised(),
        }
    }
}

// impl Display for Expression {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let expression = match self {
//             Expression::SelectStatement(s) => s.build(),
//             // Expression::SelectStatement(s) => s.get_bindings().first().unwrap().get_raw(),
//             Expression::Value(v) => {
//                 // let bindings = self.get_bindings();
//                 let bindings = v.to_string();
//                 // assert_eq!(bindings.len(), 1);
//                 format!("{}", self.get_bindings().first().expect("Param must have been generated for value. This is a bug. Please report here: ").get_param())
//             }
//         };
//         write!(f, "{}", expression)
//     }
// }
//
impl Parametric for Expression {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Expression::SelectStatement(s) => s.get_bindings(),
            Expression::Value(sql_value) => {
                // let sql_value = sql::json(&serde_json::to_string(&v).unwrap()).unwrap();
                // let sql_value: sql::Value = sql_value.to_owned();
                vec![sql_value.binding.clone()]
            }
        }
    }
}

impl From<SelectStatement> for Expression {
    fn from(value: SelectStatement) -> Self {
        Self::SelectStatement(value)
    }
}

impl<T: Into<sql::Value>> From<T> for Expression {
    fn from(value: T) -> Self {
        let binding = Binding::new(value.into());
        Self::Value(ValueWithBinding {
            value: binding.get_param_dollarised(),
            binding,
        })
    }
}
