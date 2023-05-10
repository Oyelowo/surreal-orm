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
    expression::Expression,
    traits::{BindingsList, Buildable, Conditional, Erroneous, Parametric, Queryable},
    types::Filter,
};

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
/// # let fake_id = TestUser::create_id("oyelowo");
/// # let fake_id2 = TestUser::create_id("oyedayo");
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

pub struct IfElseExpression(Expression);

impl From<Expression> for IfElseExpression {
    fn from(value: Expression) -> Self {
        Self(value)
    }
}

impl Buildable for IfElseExpression {
    fn build(&self) -> String {
        self.0.build()
    }
}

impl Parametric for IfElseExpression {
    fn get_bindings(&self) -> BindingsList {
        self.0.get_bindings()
    }
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
        let expression: IfElseExpression = expression.into().into();
        self.flow_data.else_data = ExpressionContent::new(expression.build());
        self.bindings.extend(expression.get_bindings());

        ElseStatement {
            flow_data: self.flow_data,
            bindings: self.bindings,
        }
    }

    pub fn end(self) -> IfElseStatement {
        IfElseStatement {
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
    pub fn end(self) -> IfElseStatement {
        IfElseStatement {
            flow_data: self.flow_data,
            bindings: self.bindings,
        }
    }
}

#[derive(Default)]
struct ExpressionContent(String);

impl ExpressionContent {
    pub fn new(expr: impl Into<String>) -> Self {
        Self(expr.into())
    }
}

impl Buildable for ExpressionContent {
    fn build(&self) -> String {
        self.0.to_string()
    }
}

impl Display for ExpressionContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
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

#[derive(Default)]
struct Flows {
    conditions: Vec<Filter>,
    expressions: Vec<IfElseExpression>,
}

#[derive(Default)]
struct Flow {
    condition: Option<Filter>,
    expression: Option<ExpressionContent>,
}

pub struct ElseStatement {
    flow_data: FlowStatementData,
    bindings: BindingsList,
}

impl ElseIfStatement {
    pub fn then(mut self, expression: impl Into<Expression>) -> ThenExpression {
        let expression: IfElseExpression = expression.into().into();
        self.bindings.extend(expression.get_bindings());
        self.flow_data.else_if_data.expressions.push(expression);

        ThenExpression {
            flow_data: self.flow_data,
            bindings: self.bindings,
        }
    }
}

/// if flow builder
#[derive(Debug, Clone)]
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

        let expression: IfElseExpression = expression.into().into();
        let bindings = vec![if_condition.get_bindings(), expression.get_bindings()].concat();

        let mut flow_data = FlowStatementData::default();
        flow_data.if_data.condition = Some(if_condition);
        flow_data.if_data.expression = Some(ExpressionContent(expression.build()));

        ThenExpression {
            flow_data,
            bindings,
        }
    }
}

/// if else flow builder
pub struct IfElseStatement {
    flow_data: FlowStatementData,
    bindings: BindingsList,
}

impl Parametric for IfElseStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Erroneous for IfElseStatement {}
impl Buildable for IfElseStatement {
    fn build(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!(
            "IF {} THEN\n\t{}",
            self.flow_data
                .if_data
                .condition
                .as_ref()
                .expect("condition must be provided"),
            self.flow_data
                .if_data
                .expression
                .as_ref()
                .expect("expression must be provided")
        ));

        for i in 0..self.flow_data.else_if_data.conditions.len() {
            output.push_str(&format!(
                "\nELSE IF {} THEN\n\t{}",
                self.flow_data.else_if_data.conditions[i].build(),
                self.flow_data.else_if_data.expressions[i].build()
            ));
        }

        if !&self.flow_data.else_data.is_empty() {
            output.push_str(&format!("\nELSE\n\t{}", self.flow_data.else_data.build()));
        }

        output.push_str("\nEND");

        output
    }
}

impl Queryable for IfElseStatement {}

impl fmt::Display for IfElseStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_display_snapshot;
    use surrealdb::sql;

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

        assert_eq!(if_statement1.get_bindings().len(), 3);
        assert_eq!(
            if_statement1.fine_tune_params(),
            "IF age >= $_param_00000001 <= $_param_00000002 THEN\n\t\
                $_param_00000003\n\
                END"
        );
        assert_eq!(
            if_statement1.to_raw().build(),
            "IF age >= 18 <= 120 THEN\n\t\
                'Valid'\n\
                END"
        );
    }

    #[test]
    fn test_if_statement2() {
        let age = Field::new("age");
        let if_statement2 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then("Valid")
            .else_("Invalid")
            .end();
        assert_eq!(if_statement2.get_bindings().len(), 4);

        assert_eq!(
            if_statement2.fine_tune_params(),
            "IF age >= $_param_00000001 <= $_param_00000002 THEN\n\t\
                $_param_00000003\n\
                ELSE\n\t$_param_00000004\n\
                END"
        );

        assert_eq!(
            if_statement2.to_raw().build(),
            "IF age >= 18 <= 120 THEN\n\t'Valid'\nELSE\n\t'Invalid'\nEND"
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

        assert_eq!(if_statement.get_bindings().len(), 5);

        assert_eq!(
        if_statement.fine_tune_params(),
            "IF age >= $_param_00000001 <= $_param_00000002 THEN\n\t$_param_00000003\nELSE IF name ~ $_param_00000004 THEN\n\t$_param_00000005\nEND"
        );

        assert_eq!(
        if_statement.to_raw().build(),
            "IF age >= 18 <= 120 THEN\n\t'Valid'\nELSE IF name ~ 'Oyelowo Oyedayo' THEN\n\t'The Alien!'\nEND"
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
        assert_eq!(if_statement4.get_bindings().len(), 6);

        assert_eq!(
            if_statement4.fine_tune_params(),
            "IF age >= $_param_00000001 <= $_param_00000002 THEN\n\t\
                $_param_00000003\n\
                ELSE IF name ~ $_param_00000004 THEN\n\t\
                $_param_00000005\n\
                ELSE\n\t$_param_00000006\n\
                END"
        );

        assert_eq!(
            if_statement4.to_raw().build(),
            "IF age >= 18 <= 120 THEN\n\t\
                'Valid'\n\
                ELSE IF name ~ 'Oyelowo Oyedayo' THEN\n\t\
                'The Apple!'\n\
                ELSE\n\t\
                'The Mango!'\n\
                END"
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

        assert_eq!(if_statement5.get_bindings().len(), 9);

        assert_eq!(
            if_statement5.fine_tune_params(),
            "IF age >= $_param_00000001 <= $_param_00000002 THEN\n\t\
                $_param_00000003\n\
                ELSE IF name ~ $_param_00000004 THEN\n\t\
                $_param_00000005\n\
                ELSE IF (country IS $_param_00000006) OR (country IS $_param_00000007) THEN\n\t\
                $_param_00000008\nELSE\n\t$_param_00000009\nEND"
        );

        assert_eq!(
            if_statement5.to_raw().build(),
            "IF age >= 18 <= 120 THEN\n\t\
                'Valid'\n\
                ELSE IF name ~ 'Oyelowo Oyedayo' THEN\n\t\
                'The Alien!'\n\
                ELSE IF (country IS 'Canada') OR (country IS 'Norway') THEN\n\t\
                'Cold'\n\
                ELSE\n\t\
                'Hot'\n\
                END"
        );
    }

    #[test]
    fn test_if_statement6() {
        let name = Field::new("name");
        let age = Field::new("age");
        let country = Field::new("country");
        let city = Field::new("city");
        let fake_id = sql::Thing::from(("user".to_string(), "oyelowo".to_string()));
        let fake_id2 = sql::Thing::from(("user".to_string(), "oyedayo".to_string()));

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

        assert_eq!(if_statement5.get_bindings().len(), 9);
        assert_display_snapshot!(if_statement5.fine_tune_params());
        assert_display_snapshot!(if_statement5.to_raw().build());
        assert_eq!(
            if_statement5.fine_tune_params(),
            "IF age >= $_param_00000001 <= $_param_00000002 THEN\n\
                \t$_param_00000003\n\
                ELSE IF name ~ $_param_00000004 THEN\n\
                \t$_param_00000005\n\
                ELSE IF (country IS $_param_00000006) OR (country IS $_param_00000007) THEN\n\
                \t$_param_00000008\nELSE\n\t$_param_00000009\nEND"
        );

        assert_eq!(
            if_statement5.to_raw().build(),
            "IF age >= 18 <= 120 THEN\n\
                \t(SELECT * FROM user:oyelowo \
                WHERE (city = 'Prince Edward Island') AND (city = 'NewFoundland') OR (city ~ 'Toronto') \
                ORDER BY age NUMERIC LIMIT 153 START 10 PARALLEL)\n\
                ELSE IF name ~ 'Oyelowo Oyedayo' THEN\n\
                \t(SELECT * FROM user:oyedayo WHERE country = 'INDONESIA' \
                ORDER BY age NUMERIC LIMIT 20 START 5)\n\
                ELSE IF (country IS 'Canada') OR (country IS 'Norway') THEN\n\
                \t'Cold'\n\
                ELSE\n\
                \t'Hot'\n\
                END"
        );
    }
}
