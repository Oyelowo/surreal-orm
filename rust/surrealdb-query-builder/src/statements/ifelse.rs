/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Statement syntax
// IF @condition THEN
// 	@expression
// [ ELSE IF @condition THEN
// 	@expression ... ]
// [ ELSE
// 	@expression ]
// END

use std::fmt::{self, Display};

use crate::{
    traits::{BindingsList, Buildable, Conditional, Erroneous, Parametric, Queryable},
    types::{expression::Expression, Filter},
};

impl Into<ExpressionContent> for Expression {
    fn into(self) -> ExpressionContent {
        let expression: Expression = self.into();
        ExpressionContent(format!("{expression}"))
    }
}
/// Creates an IF ELSE statement with compile-time valid transition.
/// The IF ELSE statement can be used as a main statement, or within a parent statement,
/// to return a value depending on whether a condition, or a series of conditions match.
/// The statement allows for multiple ELSE IF expressions, and a final ELSE expression,
/// with no limit to the number of ELSE IF conditional expressions.
///
/// Examples
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, statements::{if_, order, select}};
///         # let age = Field::new("age");
/// // Can create simple if statement
/// if_(age.greater_than_or_equal(18))
///     .then("Valid".to_string())
///     .end();
///
/// // A bit complex if else statement
/// # let age = Field::new("age");
/// # let name = Field::new("name");
/// # let country = Field::new("country");
/// if_(age.greater_than_or_equal(18).less_than_or_equal(120))
///     .then("Valid")
///     .else_if(name.like("Oyelowo Oyedayo"))
///     .then("The Alien!")
///     .else_if(cond(country.is("Canada")).or(country.is("Norway")))
///     .then("Cold")
///     .else_("Hot")
///     .end();
///
/// // And even with nested statements
/// # let name = Field::new("name");
/// # let age = Field::new("age");
/// # let country = Field::new("country");
/// # let city = Field::new("city");
/// # let fake_id = SurrealId::try_from("user:oyelowo").unwrap();
/// # let fake_id2 = SurrealId::try_from("user:oyedayo").unwrap();
///
/// let select1 = select(All)
///     .from(fake_id)
///     .where_(cond(city.is("Prince Edward Island"))
///                 .and(city.is("NewFoundland"))
///                 .or(city.like("Toronto"))
///     )
///     .order_by(order(&age).numeric())
///     .limit(153)
///     .start(10)
///     .parallel();
///
/// let select2 = select(All)
///     .from(fake_id2)
///     .where_(country.is("INDONESIA"))
///     .order_by(order(&age).numeric())
///     .limit(20)
///     .start(5);
///
///  if_(cond(age.greater_than_or_equal(18)).and(age.less_than_or_equal(120)))
///     .then(select1)
///     .else_if(name.like("Oyelowo Oyedayo"))
///     .then(select2)
///     .else_if(cond(country.is("Canada")).or(country.is("Norway")))
///     .then("Cold")
///     .else_("Hot")
///     .end();
///
pub fn if_(condition: impl Conditional) -> IfStatement {
    IfStatement::new(condition)
}

pub struct ThenExpression {
    flow_data: FlowStatementData,
    bindings: BindingsList,
}

impl ThenExpression {
    pub fn else_if(mut self, condition: impl Conditional) -> ElseIfStatement {
        let condition = Filter::new(condition);
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

impl FlowStatementData {
    fn update_if(mut self, condition: Filter) -> Self {
        self.if_data.condition = condition;
        self
    }
}

#[derive(Default)]
struct Flows {
    conditions: Vec<Filter>,
    expressions: Vec<ExpressionContent>,
}

#[derive(Default)]
struct Flow {
    condition: Filter,
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

/// if flow builder
pub struct IfStatement {
    condition: Filter,
}

impl IfStatement {
    pub(crate) fn new(condition: impl Conditional) -> Self {
        Self {
            condition: Filter::new(condition),
        }
    }

    /// Can be a select statment or any other valid surrealdb Value
    pub fn then(self, expression: impl Into<Expression>) -> ThenExpression {
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

impl Erroneous for End {}
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

impl Queryable for End {}

impl fmt::Display for End {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use insta::{assert_debug_snapshot, assert_display_snapshot};

    use crate::{
        statements::{order, select},
        *,
    };

    use super::*;

    #[test]
    fn test_if_statement1() {
        let age = Field::new("age");

        let if_statement1 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then("Valid".to_string())
            .end();

        assert_debug_snapshot!(if_statement1.get_bindings());
        assert_display_snapshot!(if_statement1);
        assert_eq!(
            if_statement1.build(),
            "IF age >= $_param_00000000 <= $_param_00000000 THEN\n\t_param_00000000\nEND"
        );
        assert_eq!(
            if_statement1.to_raw().build(),
            "IF age >= $_param_00000000 <= $_param_00000000 THEN\n\t_param_00000000\nEND"
        );
    }

    #[test]
    fn test_if_statement2() {
        let age = Field::new("age");
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
        let name = Field::new("name");
        let age = Field::new("age");

        let if_statement = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then("Valid")
            .else_if(name.like("Oyelowo Oyedayo"))
            .then("The Alien!")
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
        let name = Field::new("name");
        let age = Field::new("age");

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
        let name = Field::new("name");
        let age = Field::new("age");
        let country = Field::new("country");

        let if_statement5 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then("Valid")
            .else_if(name.like("Oyelowo Oyedayo"))
            .then("The Alien!")
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

    #[test]
    fn test_if_statement6() {
        let name = Field::new("name");
        let age = Field::new("age");
        let country = Field::new("country");
        let city = Field::new("city");
        let fake_id = SurrealId::try_from("user:oyelowo").unwrap();
        let fake_id2 = SurrealId::try_from("user:oyedayo").unwrap();

        let statement1 = select(All)
            .from(fake_id)
            .where_(
                cond(city.is("Prince Edward Island"))
                    .and(city.is("NewFoundland"))
                    .or(city.like("Toronto")),
            )
            .order_by(order(&age).numeric())
            .limit(153)
            .start(10)
            .parallel();

        let statement2 = select(All)
            .from(fake_id2)
            .where_(country.is("INDONESIA"))
            .order_by(order(&age).numeric())
            .limit(20)
            .start(5);

        let if_statement5 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then(statement1)
            .else_if(name.like("Oyelowo Oyedayo"))
            .then(statement2)
            .else_if(cond(country.is("Canada")).or(country.is("Norway")))
            .then("Cold")
            .else_("Hot")
            .end();

        assert_debug_snapshot!(if_statement5.get_bindings());
        assert_display_snapshot!(if_statement5);
        assert_eq!(
            format!("{if_statement5}"),
            "IF age >= $_param_00000000 <= $_param_00000000 THEN\n\t(SELECT * FROM $_param_00000000 WHERE city IS $_param_00000000 AND $_param_00000000 OR $_param_00000000 ORDER BY age NUMERIC ASC LIMIT 153 START AT 10 PARALLEL;)\nELSE IF name ~ $_param_00000000 THEN\n\t(SELECT * FROM $_param_00000000 WHERE country IS $_param_00000000 ORDER BY age NUMERIC ASC LIMIT 20 START AT 5;)\nELSE IF (country IS $_param_00000000) OR (country IS $_param_00000000) THEN\n\t_param_00000000\nELSE\n\t_param_00000000\nEND"
        );
    }
}
