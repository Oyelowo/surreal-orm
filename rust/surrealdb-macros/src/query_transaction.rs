/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::{self, Display};

use insta::{assert_debug_snapshot, assert_display_snapshot};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::sql;

use crate::{
    db_field::{cond, Binding},
    query_create::CreateStatement,
    query_insert::{Buildable, InsertStatement},
    query_relate::RelateStatement,
    query_select::SelectStatement,
    query_update::UpdateStatement,
    BindingsList, DbField, DbFilter, Parametric,
};

#[derive(Clone)]
// pub enum Query<TModel, TNode, TEdge> {
pub enum Query {
    // CreateStatement(CreateStatement<TNode>),
    // InsertStatement(InsertStatement<TModel>),
    // UpdateStatement(UpdateStatement<TModel>),
    // RelateStatement(RelateStatement<TEdge>),
    SelectStatement(SelectStatement),
    // Value(sql::Value),
}

// impl<TModel, TNode, TEdge> Display for Query<TModel, TNode, TEdge> {
impl Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expression = match self {
            Query::SelectStatement(s) => format!("({s})"),
            // Expression::SelectStatement(s) => s.get_bindings().first().unwrap().get_raw(),
            // Query::Value(v) => {
            //     let bindings = self.get_bindings();
            //     assert_eq!(bindings.len(), 1);
            //     format!("{}", self.get_bindings().first().expect("Param must have been generated for value. This is a bug. Please report here: ").get_param())
            // }
        };
        write!(f, "{}", expression)
    }
}

// impl<TModel, TNode, TEdge> Parametric for Query<TModel, TNode, TEdge> {
impl Parametric for Query {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Query::SelectStatement(s) => s
                .get_bindings()
                .into_iter()
                // query must have already been built and bound
                .map(|b| b.with_raw(format!("({s})")))
                .collect::<_>(),
            // Query::Value(sql_value) => {
            //     // let sql_value = sql::json(&serde_json::to_string(&v).unwrap()).unwrap();
            //     let sql_value: sql::Value = sql_value.to_owned();
            //     vec![Binding::new(sql_value.clone()).with_raw(sql_value.to_raw_string())]
            // }
        }
    }
}

// impl<TModel, TNode, TEdge> From<SelectStatement> for Query<TModel, TNode, TEdge> {
impl From<SelectStatement> for Query {
    fn from(value: SelectStatement) -> Self {
        Self::SelectStatement(value)
    }
}

// impl<<TModel, TNode, TEdge>,T: Into<sql::Value>> From<T> for Query<TModel, TNode, TEdge> {
// impl<T: Into<sql::Value>> From<T> for Query {
//     fn from(value: T) -> Self {
//         Self::Value(value.into())
//     }
// }

pub fn begin_transaction() -> BeginTransaction {
    // BeginTransaction::new(condition)
    todo!()
}

#[derive(Default)]
pub struct QueryTransaction {
    data: TransactionData,
}

impl QueryTransaction {
    pub fn query(mut self, condition: impl Into<DbFilter>) -> Self {
        // let condition: DbFilter = condition.into();
        // self.bindings.extend(condition.get_bindings());
        // self.flow_data.else_if_data.conditions.push(condition);
        //
        // ElseIfStatement {
        //     flow_data: self.flow_data,
        //     bindings: self.bindings,
        // }
        todo!()
    }

    pub fn commit_transaction(mut self) -> CommitTransaction {
        // CommitTransaction {
        // }
        todo!()
    }

    pub fn cancel_transaction(mut self) -> CommitTransaction {
        // CommitTransaction {
        // }
        todo!()
    }
}

pub struct BeginTransaction;

impl BeginTransaction {
    pub(crate) fn new() -> QueryTransaction {
        // Self {
        //     begin_transaction: sql::statements::BeginStatement.to_string(),
        // }
        todo!()
    }
}

#[derive(Default)]
pub struct TransactionData {
    begin_transaction: bool,
    cancel_transaction: bool,
    commit_transaction: bool,
    queries: Vec<Query>,
    bindings: BindingsList,
}

pub struct CancelTransaction {
    data: TransactionData,
}

pub struct CommitTransaction {
    data: TransactionData,
}

impl Parametric for CommitTransaction {
    fn get_bindings(&self) -> BindingsList {
        self.data.bindings.to_vec()
    }
}

impl Buildable for CommitTransaction {
    fn build(&self) -> String {
        todo!()
    }
}

impl fmt::Display for CommitTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        query_select::{order, select, All},
        value_type_wrappers::SurrealId,
    };

    use super::*;

    #[test]
    fn test_if_statement6() {
        // let name = DbField::new("name");
        // let age = DbField::new("age");
        // let country = DbField::new("country");
        // let city = DbField::new("city");
        // let fake_id = SurrealId::try_from("user:oyelowo").unwrap();
        // let fake_id2 = SurrealId::try_from("user:oyedayo").unwrap();
        //
        // let statement1 = select(All)
        //     .from(fake_id)
        //     .where_(cond(
        //         city.is("Prince Edward Island")
        //             .and(city.is("NewFoundland"))
        //             .or(city.like("Toronto")),
        //     ))
        //     .order_by(order(&age).numeric())
        //     .limit(153)
        //     .start(10)
        //     .parallel();
        //
        // let statement2 = select(All)
        //     .from(fake_id2)
        //     .where_(country.is("INDONESIA"))
        //     .order_by(order(&age).numeric())
        //     .limit(20)
        //     .start(5);
        //
        // let if_statement5 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
        //     .then(statement1)
        //     .else_if(name.like("Oyelowo Oyedayo"))
        //     .then(statement2)
        //     .else_if(cond(country.is("Canada")).or(country.is("Norway")))
        //     .then("Cold")
        //     .else_("Hot")
        //     .end();
        //
        // assert_debug_snapshot!(if_statement5.get_bindings());
        // assert_display_snapshot!(if_statement5);
        // assert_eq!(
        //     format!("{if_statement5}"),
        //     "IF age >= $_param_00000000 <= $_param_00000000 THEN\n\t(SELECT * FROM $_param_00000000 WHERE city IS $_param_00000000 AND $_param_00000000 OR $_param_00000000 ORDER BY age NUMERIC ASC LIMIT 153 START AT 10 PARALLEL;)\nELSE IF name ~ $_param_00000000 THEN\n\t(SELECT * FROM $_param_00000000 WHERE country IS $_param_00000000 ORDER BY age NUMERIC ASC LIMIT 20 START AT 5;)\nELSE IF (country IS $_param_00000000) OR (country IS $_param_00000000) THEN\n\t_param_00000000\nELSE\n\t_param_00000000\nEND"
        // );
    }
}
