/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::{self, Display};

use surrealdb::sql;

use crate::{
    traits::{BindingsList, Parametric},
    traits::{Buildable, Erroneous, Queryable},
};
/// Creates a Transaction statement starting with BEGIN TRANSACTION and ends with COMMIT
/// TRANSACTION or END TRANSACTION.
///
/// Transactions
/// Each statement within SurrealDB is run within its own transaction.
/// If a set of changes need to be made together, then groups of statements can be run
/// together as a single transaction, either succeeding as a whole,
/// or failing without leaving any residual data modifications.
///
/// Starting a transaction
/// The BEGIN TRANSACTION statement can be used to run a group of statements together,
/// either succeeding as a whole, or failing. If all of the statements within a transaction succeed,
/// and the transaction is successful, then all of the data modifications made during the transaction
/// are committed and become a permanent part of the database. If a transaction encounters errors
/// and must be cancelled or rolled back, then any data modification made within the transaction is rolledback,
/// and will not be visible within the database.
///
/// Example
///
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use std::time::Duration;
/// use surrealdb::sql;
/// use surreal_orm::{*, statements::{begin_transaction, order, select}};
/// # let name = Field::new("name");
/// # let age = Field::new("age");
/// # let country = Field::new("country");
/// # let city = Field::new("city");
/// # let fake_id = sql::Thing::from(("user".to_string(), "oyelowo".to_string()));
/// # let fake_id2 = sql::Thing::from(("user".to_string(), "oyedayo".to_string()));
///
/// let statement1 = select(All)
///     .from(fake_id)
///     .where_(cond(city.is("Prince Edward Island"))
///             .and(city.is("NewFoundland"))
///             .or(city.like("Toronto")),
///     )
///     .order_by(order(&age).numeric())
///     .limit(153)
///     .start(10)
///     .parallel();
///
/// let statement2 = select(All)
///     .from(fake_id2)
///     .where_(country.is("INDONESIA"))
///     .order_by(order(&age).numeric())
///     .limit(20)
///     .start(5);
///
/// let transaction = begin_transaction()
///     .query(statement1)
///     .query(statement2)
///     .commit_transaction();
/// ```
pub fn begin_transaction() -> QueryTransaction {
    BeginTransactionStatement::begin()
}

/// Creates a Transaction statement starting
#[macro_export]
macro_rules! transaction_deprecated {
    (BEGIN TRANSACTION; $($query:expr;)* COMMIT TRANSACTION;) => {
        {
            $crate::statements::begin_transaction().
            $(
            query(&$query).
            )*

            .commit_transaction()
        }
    };
    (begin transaction; $($query:expr;)* commit transaction;) => {
        {
            $crate::statements::begin_transaction().
            $(
            query(&$query).
            )*

            .commit_transaction()
        }
    };
    (BEGIN TRANSACTION; $($query:expr;)* CANCEL TRANSACTION;) => {
        {
            $crate::statements::begin_transaction().
            $(
            query(&$query).
            )*

            .cancel_transaction()
        }
    };
    (begin transaction; $($query:expr;)* cancel transaction;) => {
        {
            $crate::statements::begin_transaction().
            $(
            query(&$query).
            )*

            .cancel_transaction()
        }
    };
}
pub use transaction_deprecated;

pub struct QueryTransaction {
    data: TransactionData,
}

impl QueryTransaction {
    /// takes a statement as an argument
    pub fn query(mut self, query: impl Queryable + Display) -> Self {
        self.data.bindings.extend(query.get_bindings());
        self.data.queries.push(query.build());
        self
    }

    pub fn commit_transaction(self) -> TransactionCompletion {
        let mut transaction = TransactionCompletion { data: self.data };
        transaction.data.transaction_completion_type =
            Some(TranctionCompletionType::CommitTransaction);
        transaction
    }

    pub fn cancel_transaction(self) -> TransactionCompletion {
        let mut transaction = TransactionCompletion { data: self.data };
        transaction.data.transaction_completion_type =
            Some(TranctionCompletionType::CancelTransaction);
        transaction
    }
}

/// Transaction statement initialization builder
pub struct BeginTransactionStatement;

impl BeginTransactionStatement {
    pub(crate) fn begin() -> QueryTransaction {
        QueryTransaction {
            data: TransactionData {
                transaction_completion_type: None,
                queries: vec![],
                bindings: vec![],
            },
        }
    }
}

#[derive(Debug, Clone)]
enum TranctionCompletionType {
    CommitTransaction,
    CancelTransaction,
}

#[derive(Debug, Clone)]
pub struct TransactionData {
    transaction_completion_type: Option<TranctionCompletionType>,
    queries: Vec<String>,
    bindings: BindingsList,
}

/// Completed Transaction statements. Either COMMIT or CANCEL Transaction.
#[derive(Debug, Clone)]
pub struct TransactionCompletion {
    data: TransactionData,
}

impl Parametric for TransactionCompletion {
    fn get_bindings(&self) -> BindingsList {
        self.data.bindings.to_vec()
    }
}

impl Queryable for TransactionCompletion {}
impl Erroneous for TransactionCompletion {}

impl Buildable for TransactionCompletion {
    fn build(&self) -> String {
        let mut output = String::new();
        output.push_str("BEGIN TRANSACTION;\n");

        self.data.queries.iter().for_each(|q| {
            output.push_str(&format!("\n{};\n", q.trim().trim_end_matches(';')));
        });

        if let Some(completion_type) = &self.data.transaction_completion_type {
            let com_type = match completion_type {
                TranctionCompletionType::CommitTransaction => {
                    sql::statements::CommitStatement.to_string()
                }
                TranctionCompletionType::CancelTransaction => {
                    sql::statements::CancelStatement.to_string()
                }
            };

            output.push_str(&format!("\n{com_type};\n\t"));
        }

        output
    }
}

impl fmt::Display for TransactionCompletion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use select::CanOrder;

    use crate::{statements::select, *};

    use super::*;

    #[test]
    fn test_transaction_commit() {
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
            .order_by(age.numeric().asc())
            .limit(153)
            .start(10)
            .parallel();

        let statement2 = select(All)
            .from(fake_id2)
            .where_(country.is("INDONESIA"))
            .order_by(age.numeric().asc())
            .limit(20)
            .start(5);

        let transaction = begin_transaction()
            .query(statement1)
            .query(statement2)
            .commit_transaction();

        assert_eq!(transaction.get_bindings().len(), 10);

        insta::assert_snapshot!(transaction.fine_tune_params());
        insta::assert_snapshot!(transaction.to_raw().build());
    }

    #[test]
    fn test_transaction_cancel() {
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
            .order_by(age.numeric().asc())
            .limit(153)
            .start(10)
            .parallel();

        let statement2 = select(All)
            .from(fake_id2)
            .where_(country.is("INDONESIA"))
            .order_by(age.numeric().asc())
            .limit(20)
            .start(5);

        let transaction = begin_transaction()
            .query(statement1)
            .query(statement2)
            .cancel_transaction();

        assert_eq!(transaction.get_bindings().len(), 10);

        insta::assert_snapshot!(transaction.fine_tune_params());
        insta::assert_snapshot!(transaction.to_raw().build());
    }
}
