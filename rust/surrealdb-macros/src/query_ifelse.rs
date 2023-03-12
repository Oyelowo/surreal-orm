/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

use std::fmt::{self, Display};

use insta::{assert_debug_snapshot, assert_display_snapshot};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::sql;

use crate::{
    db_field::{cond, Binding},
    query_insert::Buildable,
    query_select::SelectStatement,
    BindingsList, DbField, DbFilter, Parametric,
};

#[derive(Clone)]
pub enum Expression {
    SelectStatement(SelectStatement),
    Value(sql::Value),
}

impl Into<ExpressionContent> for Expression {
    fn into(self) -> ExpressionContent {
        let expression: Expression = self.into();
        ExpressionContent(format!("{expression}"))
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expression = match self {
            Expression::SelectStatement(s) => format!("({s})"),
            // Expression::SelectStatement(s) => s.get_bindings().first().unwrap().get_raw(),
            Expression::Value(v) => {
                let bindings = self.get_bindings();
                assert_eq!(bindings.len(), 1);
                format!("{}", self.get_bindings().first().expect("Param must have been generated for value. This is a bug. Please report here: ").get_param())
            }
        };
        write!(f, "{}", expression)
    }
}

impl Parametric for Expression {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Expression::SelectStatement(s) => s
                .get_bindings()
                .into_iter()
                // query must have already been built and bound
                .map(|b| b.with_raw(format!("({s})")))
                .collect::<_>(),
            Expression::Value(sql_value) => {
                // let sql_value = sql::json(&serde_json::to_string(&v).unwrap()).unwrap();
                let sql_value: sql::Value = sql_value.to_owned();
                vec![Binding::new(sql_value.clone()).with_raw(sql_value.to_raw_string())]
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
        Self::Value(value.into())
    }
}

pub fn if_(condition: impl Into<DbFilter>) -> IfStatement {
    IfStatement::new(condition)
}

pub struct ThenExpression {
    flow_data: FlowStatementData,
    bindings: BindingsList,
}

impl ThenExpression {
    pub fn else_if(mut self, condition: impl Into<DbFilter>) -> ElseIfStatement {
        let condition: DbFilter = condition.into();
        self.bindings.extend(condition.get_bindings());
        self.flow_data.else_if_data.conditions.push(condition);

        ElseIfStatement {
            flow_data: self.flow_data,
            bindings: self.bindings,
        }
    }

    pub fn else_(mut self, expression: impl Into<Expression>) -> ElseStatement {
        let expression: Expression = expression.into();
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

impl Display for ExpressionContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ExpressionContent {
    fn empty() -> Self {
        Self("".into())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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

pub struct ElseStatement {
    flow_data: FlowStatementData,
    bindings: BindingsList,
}

impl ElseIfStatement {
    pub fn then(mut self, expression: impl Into<Expression>) -> ThenExpression {
        let expression: Expression = expression.into();
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

    pub fn then(mut self, expression: impl Into<Expression>) -> ThenExpression {
        let if_condition = self.condition;

        let expression: Expression = expression.into();
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

pub struct End {
    flow_data: FlowStatementData,
    bindings: BindingsList,
}

impl Parametric for End {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Buildable for End {
    fn build(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!(
            "IF {} THEN\n\t{}",
            self.flow_data.if_data.condition, self.flow_data.if_data.expression
        ));

        for i in 0..self.flow_data.else_if_data.conditions.len() {
            output.push_str(&format!(
                "\nELSE IF {} THEN\n\t{}",
                self.flow_data.else_if_data.conditions[i],
                self.flow_data.else_if_data.expressions[i]
            ));
        }

        if !&self.flow_data.else_data.is_empty() {
            output.push_str(&format!("\nELSE\n\t{}", self.flow_data.else_data));
        }

        output.push_str("\nEND");

        output
    }
}

impl fmt::Display for End {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_statement1() {
        let name = DbField::new("name");
        let age = DbField::new("age");
        let country = DbField::new("country");

        let if_statement1 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then("Valid".to_string())
            .end();

        assert_debug_snapshot!(if_statement1.get_bindings());
        assert_display_snapshot!(if_statement1);
        assert_eq!(
            format!("{if_statement1}"),
            "IF age >= $_param_00000000 <= $_param_00000000 THEN\n\t_param_00000000\nEND"
        );
    }

    #[test]
    fn test_if_statement2() {
        let age = DbField::new("age");
        let if_statement2 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then("Valid")
            .else_("Invalid")
            .end();
        assert_debug_snapshot!(if_statement2.get_bindings());
        assert_display_snapshot!(if_statement2);
        assert_eq!(
            format!("{if_statement2}"),
            "IF age >= $_param_00000000 <= $_param_00000000 THEN\n\t_param_00000000\nELSE\n\t_param_00000000\nEND"
        );
    }

    #[test]
    fn test_if_statement3() {
        let name = DbField::new("name");
        let age = DbField::new("age");

        let if_statement = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then("Valid")
            .else_if(name.like("Oyelowo Oyedayo"))
            .then("The man!")
            .end();

        assert_debug_snapshot!(if_statement.get_bindings());
        assert_display_snapshot!(if_statement);
        assert_eq!(
            format!("{if_statement}"),
            "IF age >= $_param_00000000 <= $_param_00000000 THEN\n\t_param_00000000\nELSE IF name ~ $_param_00000000 THEN\n\t_param_00000000\nEND"
        );
    }

    #[test]
    fn test_if_statement4() {
        let name = DbField::new("name");
        let age = DbField::new("age");

        let if_statement4 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then("Valid")
            .else_if(name.like("Oyelowo Oyedayo"))
            .then("The Apple!")
            .else_("The Mango!")
            .end();
        assert_debug_snapshot!(if_statement4.get_bindings());
        assert_display_snapshot!(if_statement4);
        assert_eq!(
            format!("{if_statement4}"),
            "IF age >= $_param_00000000 <= $_param_00000000 THEN\n\t_param_00000000\nELSE IF name ~ $_param_00000000 THEN\n\t_param_00000000\nELSE\n\t_param_00000000\nEND"
        );
    }

    #[test]
    fn test_if_statement5() {
        let name = DbField::new("name");
        let age = DbField::new("age");
        let country = DbField::new("country");

        let if_statement5 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then("Valid")
            .else_if(name.like("Oyelowo Oyedayo"))
            .then("The man!")
            .else_if(cond(country.is("Canada")).or(country.is("Norway")))
            .then("Cold")
            .else_("Hot")
            .end();
        assert_debug_snapshot!(if_statement5.get_bindings());
        assert_display_snapshot!(if_statement5);
        assert_eq!(
            format!("{if_statement5}"),
            "IF age >= $_param_00000000 <= $_param_00000000 THEN\n\t_param_00000000\nELSE IF name ~ $_param_00000000 THEN\n\t_param_00000000\nELSE IF (country IS $_param_00000000) OR (country IS $_param_00000000) THEN\n\t_param_00000000\nELSE\n\t_param_00000000\nEND"
        );
    }
}
