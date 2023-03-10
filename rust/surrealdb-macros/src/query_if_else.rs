use std::fmt;

use surrealdb::sql;

use crate::{query_select::SelectStatement, BindingsList, DbFilter, Parametric};

enum Expression {
    SelectStatement(SelectStatement),
    Value(sql::Value),
}

impl Parametric for Expression {
    fn get_bindings(&self) -> BindingsList {
        todo!()
    }
}

impl From<SelectStatement> for Expression {
    fn from(value: SelectStatement) -> Self {
        Self::SelectStatement(value)
    }
}

impl<T: Into<sql::Value>> From<T> for Expression {
    fn from(value: T) -> Self {
        Self::Value(value.into())
    }
}

pub struct IfElseStatementBuilder {
    condition: String,
    then_expression: String,
    else_if_conditions: Vec<String>,
    else_if_expressions: Vec<String>,
    else_expression: Option<String>,
    bindings: BindingsList,
}

impl Parametric for IfElseStatementBuilder {
    fn get_bindings(&self) -> crate::BindingsList {
        todo!()
    }
}

impl IfElseStatementBuilder {
    pub fn new() -> Self {
        Self {
            condition: "".to_string(),
            then_expression: "".to_string(),
            else_if_conditions: vec![],
            else_if_expressions: vec![],
            else_expression: None,
            bindings: todo!(),
        }
    }

    pub fn if_then(
        self,
        condition: impl Into<DbFilter>,
        then_expression: impl Into<Expression>,
    ) -> Self {
        Self {
            condition: condition.into().to_string(),
            then_expression: then_expression.into().to_string(),
            ..self
        }
    }

    pub fn else_if(
        self,
        condition: impl Into<DbFilter>,
        then_expression: impl Into<Expression>,
    ) -> Self {
        Self {
            condition: condition.into().to_string(),
            then_expression: then_expression.to_string(),
            ..self
        }
    }

    pub fn else_expr<E>(mut self, expression: E) -> Self
    where
        E: ToString,
    {
        self.else_expression = Some(expression.to_string());
        self
    }

    pub fn build(self) -> Result<IfElseStatement, String> {
        if self.condition.is_empty() {
            return Err("Condition is missing".to_string());
        }

        if self.then_expression.is_empty() {
            return Err("Then expression is missing".to_string());
        }

        Ok(IfElseStatement {
            condition: self.condition,
            then_expression: self.then_expression,
            else_if_conditions: self.else_if_conditions,
            else_if_expressions: self.else_if_expressions,
            else_expression: self.else_expression,
        })
    }
}

impl fmt::Display for IfElseStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IF {} THEN\n\t{}", self.condition, self.then_expression)?;
        for i in 0..self.else_if_conditions.len() {
            write!(
                f,
                "\nELSE IF {} THEN\n\t{}",
                self.else_if_conditions[i], self.else_if_expressions[i]
            )?;
        }
        if let Some(else_expr) = &self.else_expression {
            write!(f, "\nELSE\n\t{}", else_expr)?;
        }
        write!(f, "\nEND")
    }
}
fn main() {
    let statement = IfElseStatementBuilder::new()
        .if_then("$scope = 'admin'", "SELECT * FROM account")
        .else_if("$scope = 'user'", "SELECT * FROM $auth.account")
        .else_expr("[]")
        .build()
        .unwrap();

    println!("{}", statement);
}
