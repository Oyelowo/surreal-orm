/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

use std::fmt::{self, Display};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::sql;

use crate::{
    db_field::{cond, Binding},
    query_select::SelectStatement,
    BindingsList, DbField, DbFilter, Parametric,
};

#[derive(Clone)]
pub enum Expression<T>
where
    T: Serialize + DeserializeOwned,
{
    SelectStatement(SelectStatement),
    Value(T),
    // Value(sql::Value),
}

impl<T> Into<ExpressionContent> for Expression<T>
where
    T: Serialize + DeserializeOwned,
{
    fn into(self) -> ExpressionContent {
        let expression: Expression<T> = self.into();
        ExpressionContent(format!("{expression}"))
    }
}

impl<T> Display for Expression<T>
where
    T: Serialize + DeserializeOwned,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match self {
            Expression::SelectStatement(s) => format!("({s})"),
            // Expression::SelectStatement(s) => s.get_bindings().first().unwrap().get_raw(),
            Expression::Value(v) => {
                let bindings = self.get_bindings();
                assert_eq!(bindings.len(), 1);
                format!("{}", self.get_bindings().first().expect("Param must have been generated for value. This is a bug. Please report here: ").get_param())
            }
        };
        write!(f, "{}", x)
    }
}

impl<T: Serialize + DeserializeOwned> Parametric for Expression<T> {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Expression::SelectStatement(s) => s
                .get_bindings()
                .into_iter()
                // query must have already been built and bound
                .map(|b| b.with_raw(format!("({s})")))
                .collect::<_>(),
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

fn if_(condition: impl Into<DbFilter>) -> IfStatement {
    todo!()
}

pub struct ThenExpression {
    flow_data: FlowStatementData,
    bindings: BindingsList,
}

impl ThenExpression {
    fn else_if(mut self, condition: impl Into<DbFilter>) -> ElseIfStatement {
        let condition: DbFilter = condition.into();
        self.bindings.extend(condition.get_bindings());
        self.flow_data.else_if_data.conditions.push(condition);

        ElseIfStatement {
            flow_data: self.flow_data,
            bindings: self.bindings,
        }
    }

    fn else_<T>(mut self, expression: impl Into<Expression<T>>) -> ElseStatement
    where
        T: Serialize + DeserializeOwned,
    {
        let expression: Expression<T> = expression.into();
        self.flow_data.else_data = ExpressionContent(format!("{expression}"));

        ElseStatement {
            flow_data: self.flow_data,
            bindings: self.bindings,
        }
    }

    pub fn end(mut self) -> End {
        End {
            flow_data: self.flow_data,
            bindings: self.bindings,
        }
    }
}

pub struct ElseIfStatement {
    flow_data: FlowStatementData,
    bindings: BindingsList,
}

impl ElseStatement {
    pub fn end(mut self) -> End {
        End {
            flow_data: self.flow_data,
            bindings: self.bindings,
        }
    }
}

#[derive(Default)]
struct ExpressionContent(String);

impl ExpressionContent {
    fn empty() -> Self {
        Self("".into())
    }
}

#[derive(Default)]
struct FlowStatementData {
    if_data: Flow,
    else_if_data: Flows,
    else_data: ExpressionContent,
}
// enum FlowStatementData {
//     If(Flow),
//     ElseIfs(Vec<Flow>),
//     Else(ExpressionContent),
//     End,
// }

impl FlowStatementData {
    fn update_if(mut self, condition: DbFilter) -> Self {
        self.if_data.condition = condition;
        self
    }
}

#[derive(Default)]
struct Flows {
    conditions: Vec<DbFilter>,
    expressions: Vec<ExpressionContent>,
}

#[derive(Default)]
struct Flow {
    condition: DbFilter,
    expression: ExpressionContent,
}

pub struct End {
    flow_data: FlowStatementData,
    bindings: BindingsList,
}

pub struct ElseStatement {
    flow_data: FlowStatementData,
    bindings: BindingsList,
}

impl ElseIfStatement {
    pub fn then<T>(mut self, expression: impl Into<Expression<T>>) -> ThenExpression
    where
        T: Serialize + DeserializeOwned,
    {
        let expression: Expression<T> = expression.into();
        self.flow_data
            .else_if_data
            .expressions
            .push(ExpressionContent(format!("{expression}")));

        self.bindings.extend(expression.get_bindings());

        ThenExpression {
            flow_data: self.flow_data,
            bindings: self.bindings,
        }
    }
}

pub struct IfStatement {
    condition: DbFilter,
}

impl IfStatement {
    pub(crate) fn new(condition: impl Into<DbFilter>) -> Self {
        Self {
            condition: condition.into(),
        }
    }

    pub fn then<T>(mut self, expression: impl Into<Expression<T>>) -> ThenExpression
    where
        T: Serialize + DeserializeOwned,
    {
        let if_condition = self.condition;

        let expression: Expression<T> = expression.into();
        let bindings = vec![if_condition.get_bindings(), expression.get_bindings()].concat();

        let mut flow_data = FlowStatementData::default();
        flow_data.if_data.condition = if_condition;
        flow_data.if_data.expression = expression.into();

        ThenExpression {
            flow_data,
            bindings,
        }
    }
}

// impl fmt::Display for IfStatement {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "IF {} THEN\n\t{}", self.condition, self.then_expression)?;
//         for i in 0..self.else_if_conditions.len() {
//             write!(
//                 f,
//                 "\nELSE IF {} THEN\n\t{}",
//                 self.else_if_conditions[i], self.else_if_expressions[i]
//             )?;
//         }
//         if let Some(else_expr) = &self.else_expression {
//             write!(f, "\nELSE\n\t{}", else_expr)?;
//         }
//         write!(f, "\nEND")
//     }
// }
fn main() {
    // let statement = IfElseStatement::new()
    //     .if_then("$scope = 'admin'", "SELECT * FROM account")
    //     .else_if("$scope = 'user'", "SELECT * FROM $auth.account")
    //     .else_expr("[]")
    //     .build()
    //     .unwrap();

    // println!("{}", statement);
}

#[test]
fn test() {
    let name = DbField::new("name");
    let age = DbField::new("age");
    let country = DbField::new("country");

    if_(age.greater_than_or_equal(18).less_than_or_equal(120))
        .then("Valid".to_string())
        .end();

    if_(age.greater_than_or_equal(18).less_than_or_equal(120))
        .then("Valid")
        .else_("Invalid")
        .end();

    if_(age.greater_than_or_equal(18).less_than_or_equal(120))
        .then("Valid")
        .else_if(name.like("Oyelowo Oyedayo"))
        .then("The man!")
        .end();

    if_(age.greater_than_or_equal(18).less_than_or_equal(120))
        .then("Valid")
        .else_if(name.like("Oyelowo Oyedayo"))
        .then("The Apple!")
        .else_("The Mango!")
        .end();

    if_(age.greater_than_or_equal(18).less_than_or_equal(120))
        .then("Valid")
        .else_if(name.like("Oyelowo Oyedayo"))
        .then("The man!")
        .else_if(cond(country.is("Canada")).or(country.is("Norway")))
        .then("Cold")
        .else_("Hot")
        .end();
}
