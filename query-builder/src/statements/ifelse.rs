/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
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

use std::fmt;

use crate::{
    expression::Expression,
    traits::{BindingsList, Buildable, Conditional, Erroneous, Parametric, Queryable},
    types::Filter,
    QueryChain,
};

/// Creates an IF ELSE statement with compile-time valid transition.
/// The IF ELSE statement can be used as a main statement, or within a parent statement,
/// to return a value depending on whether a condition, or a series of conditions match.
/// The statement allows for multiple ELSE IF expressions, and a final ELSE expression,
/// with no limit to the number of ELSE IF conditional expressions.
///
/// Examples
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::{if_, order, select}};
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
/// ```
pub fn if_(condition: impl Conditional) -> IfStatement {
    IfStatement::new(condition)
}

#[derive(Debug, Clone)]
pub enum Body {
    Expression(Expression),
    QueryChain(QueryChain),
}

impl<T: Into<Expression>> From<T> for Body {
    fn from(value: T) -> Self {
        Self::Expression(value.into())
    }
}

impl From<QueryChain> for Body {
    fn from(value: QueryChain) -> Self {
        Self::QueryChain(value)
    }
}

impl Buildable for Body {
    fn build(&self) -> String {
        match self {
            Self::Expression(expr) => expr.build(),
            Self::QueryChain(body) => body.build(),
        }
    }
}

impl Parametric for Body {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Self::Expression(expr) => expr.get_bindings(),
            Self::QueryChain(body) => body.get_bindings(),
        }
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

        // since condition has to come first, we need to initialize the last element
        // with it.
        let new_cond_meta = CondMeta {
            condition,
            body: None,
        };
        self.flow_data.else_if_data.push(new_cond_meta);

        ElseIfStatement {
            flow_data: self.flow_data,
            bindings: self.bindings,
        }
    }

    pub fn else_(mut self, body: impl Into<Body>) -> ElseStatement {
        let body: Body = body.into();

        self.bindings.extend(body.get_bindings());
        self.flow_data.else_data = Some(body);

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

impl ElseStatement {
    pub fn end(self) -> IfElseStatement {
        IfElseStatement {
            flow_data: self.flow_data,
            bindings: self.bindings,
        }
    }
}

#[derive(Debug, Clone)]
struct FlowStatementData {
    if_data: CondMeta,
    else_if_data: Vec<CondMeta>,
    else_data: Option<Body>,
}

impl FlowStatementData {
    fn new(if_data: CondMeta) -> Self {
        Self {
            if_data,
            else_if_data: vec![],
            else_data: None,
        }
    }
}

#[derive(Debug, Clone)]
struct CondMeta {
    condition: Filter,
    body: Option<Body>,
}

pub struct ElseStatement {
    flow_data: FlowStatementData,
    bindings: BindingsList,
}

pub struct ElseIfStatement {
    flow_data: FlowStatementData,
    bindings: BindingsList,
}

impl ElseIfStatement {
    pub fn then(mut self, body: impl Into<Body>) -> ThenExpression {
        let body: Body = body.into();

        if let Some(latest_else_if_cond_meta) = self.flow_data.else_if_data.last_mut() {
            self.bindings.extend(body.get_bindings());
            latest_else_if_cond_meta.body = Some(body);
        }

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
    pub fn then(self, body: impl Into<Body>) -> ThenExpression {
        let if_condition = self.condition;

        let body: Body = body.into();
        let bindings = [if_condition.get_bindings(), body.get_bindings()].concat();

        let if_cond_meta = CondMeta {
            condition: if_condition,
            body: Some(body),
        };

        let flow_data_init = FlowStatementData::new(if_cond_meta);
        ThenExpression {
            bindings,
            flow_data: flow_data_init,
        }
    }
}

/// if else flow builder
#[derive(Debug, Clone)]
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
            "IF {} \n\t{{ {} }}",
            self.flow_data.if_data.condition,
            self.flow_data
                .if_data
                .body
                .as_ref()
                .expect("if body must be provided")
                .build()
        ));

        for cond in &self.flow_data.else_if_data {
            output.push_str(&format!(
                "\nELSE IF {} \n\t{{ {} }}",
                cond.condition.build(),
                cond.body
                    .as_ref()
                    .expect("else if must have a body. Please report this bug")
                    .build()
            ));
        }

        if let Some(else_body) = &self.flow_data.else_data {
            output.push_str(&format!("\nELSE\n\t{{ {} }}", else_body.build()));
        }

        output.push(';');
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
    use insta::assert_snapshot;
    use surrealdb::sql;

    use crate::{
        statements::{order, select},
        *,
    };

    use super::*;

    #[test]
    fn test_if_statement_primitive() {
        let age = Field::new("age");

        let if_statement = if_(cond(age.gte(18)).and(age.lte(120)))
            .then("Valid".to_string())
            .else_("Invalid".to_string())
            .end();

        assert_eq!(if_statement.get_bindings().len(), 4);
        insta::assert_snapshot!(if_statement.fine_tune_params());
        insta::assert_snapshot!(if_statement.to_raw().build());
    }

    #[test]
    fn test_if_statement1() {
        let age = Field::new("age");
        let user_table = Table::new("user");

        let if_statement1 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then(select(All).from(user_table))
            .end();

        assert_eq!(if_statement1.get_bindings().len(), 3);
        insta::assert_snapshot!(if_statement1.fine_tune_params());
        insta::assert_snapshot!(if_statement1.to_raw().build());
    }

    #[test]
    fn test_if_statement2() {
        let age = Field::new("age");
        let user_table = Table::new("user");
        let book_table = Table::new("book");

        let if_statement2 = if_(cond(age.gte(18)).and(age.lte(120)))
            .then(select(All).from(user_table).where_(age.gt(34)))
            .else_(select(All).from(book_table))
            .end();
        assert_eq!(if_statement2.get_bindings().len(), 4);

        insta::assert_snapshot!(if_statement2.fine_tune_params());
        insta::assert_snapshot!(if_statement2.to_raw().build());
    }

    #[test]
    fn test_if_statement3() {
        let name = Field::new("name");
        let age = Field::new("age");
        let user_table = Table::new("user");
        let book_table = Table::new("book");

        let if_statement = if_(cond(age.gte(18)).and(age.lte(120)))
            .then(select(All).from(user_table))
            .else_if(name.like("Oyelowo Oyedayo"))
            .then(select(All).from(book_table))
            .end();

        assert_eq!(if_statement.get_bindings().len(), 5);
        assert_snapshot!(if_statement.fine_tune_params());
        assert_snapshot!(if_statement.to_raw().build());
    }

    #[test]
    fn test_if_statement4() {
        let name = Field::new("name");
        let age = Field::new("age");

        let user_table = Table::new("user");
        let book_table = Table::new("book");
        let fruit_table = Table::new("fruit");

        let if_statement4 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then(select(All).from(user_table))
            .else_if(name.like("Oyelowo Oyedayo"))
            .then(select(All).from(book_table))
            .else_(select(All).from(fruit_table))
            .end();

        assert_eq!(if_statement4.get_bindings().len(), 6);
        assert_snapshot!(if_statement4.fine_tune_params());
        assert_snapshot!(if_statement4.to_raw().build());
    }

    #[test]
    fn test_if_statement5() {
        let name = Field::new("name");
        let age = Field::new("age");
        let country = Field::new("country");

        let user_table = Table::new("user");
        let book_table = Table::new("book");
        let fruit_table = Table::new("fruit");

        let if_statement5 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then(select(All).from(user_table.clone()))
            .else_if(name.like("Oyelowo Oyedayo"))
            .then(select(All).from(book_table))
            .else_if(cond(country.is("Canada")).or(country.is("Norway")))
            .then(select(All).from(fruit_table))
            .else_(select(All).from(user_table))
            .end();

        assert_eq!(if_statement5.get_bindings().len(), 9);
        assert_snapshot!(if_statement5.fine_tune_params());
        assert_snapshot!(if_statement5.to_raw().build());
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

        let book_table = Table::new("book");
        let fruit_table = Table::new("fruit");

        let if_statement5 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then(statement1)
            .else_if(name.like("Oyelowo Oyedayo"))
            .then(statement2)
            .else_if(cond(country.is("Canada")).or(country.is("Norway")))
            .then(select(All).from(fruit_table))
            .else_(select(All).from(book_table))
            .end();

        assert_eq!(if_statement5.get_bindings().len(), 9);
        assert_snapshot!(if_statement5.fine_tune_params());
        assert_snapshot!(if_statement5.to_raw().build());
    }
}
