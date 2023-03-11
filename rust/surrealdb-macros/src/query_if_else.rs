use std::fmt::{self, Display};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::sql;

use crate::{db_field::Binding, query_select::SelectStatement, BindingsList, DbFilter, Parametric};

enum Expression<T>
where
    T: Serialize + DeserializeOwned,
{
    SelectStatement(SelectStatement),
    Value(T),
    // Value(sql::Value),
}

impl<T> Display for Expression<T>
where
    T: Serialize + DeserializeOwned,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match self {
            Expression::SelectStatement(s) => format!("({s})"),
            Expression::Value(v) => format!("{}", self.get_bindings().first().unwrap().get_param()),
        };
        write!(f, "{}", x)
    }
}

impl<T: Serialize + DeserializeOwned> Parametric for Expression<T> {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Expression::SelectStatement(s) => s.get_bindings(),
            Expression::Value(v) => {
                let sql_value = sql::json(&serde_json::to_string(&v).unwrap()).unwrap();
                vec![Binding::new(sql_value)]
            }
        }
    }
}

impl<T: Serialize + DeserializeOwned> From<SelectStatement> for Expression<T> {
    fn from(value: SelectStatement) -> Self {
        Self::SelectStatement(value)
    }
}

// impl<T: Into<sql::Value>> From<T> for Expression {
impl<T: Serialize + DeserializeOwned> From<T> for Expression<T> {
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

    pub fn if_then<T: Serialize + DeserializeOwned>(
        mut self,
        condition: impl Into<DbFilter>,
        then_expression: impl Into<Expression<T>>,
    ) -> Self {
        let condition: DbFilter = condition.into();
        self.condition = condition.into();
        let then_expression: Expression<T> = then_expression.into();
        let xx = then_expression
            .get_bindings()
            .into_iter()
            .map(|x| x.get_param());
        let param = match then_expression {
            Expression::SelectStatement(s) => format!("({s})"),
            Expression::Value(v) => v.get_bindings(),
        };
        self.then_expression = then_expression.into().to_string();
        self.bindings.extend(condition.get_bindings());
        self.bindings.extend(then_expression.into().get_bindings());
        self
    }

    pub fn else_if<T: Serialize + DeserializeOwned>(
        self,
        condition: impl Into<DbFilter>,
        then_expression: impl Into<Expression<T>>,
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
