/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::{self, Display};

use crate::traits::{BindingsList, Buildable, Erroneous, ErrorList, Parametric, Queryable};

/// Chains together multiple queries into a single `QueryChain`.
///
/// # Arguments
///
/// * `query` - The first query in the chain. Subsequent queries can be added using the `chain` method on the returned `QueryChain`.
///
/// # Example Usage
///
/// ```rust
/// use surrealdb_query_builder::{chain, SurrealId, statements::select, All, Field, cond};
///
/// let user_id = Field::new("user_id");
/// let age = Field::new("age");
///
/// let query1 = select(All).from(users).where_(cond(age.gt(30)));
/// let query2 = select(All).from(orders).where_(cond(user_id.eq(123)));
///
/// let query_chain = chain(query1).chain(query2);
/// ```
pub fn chain(query: impl Queryable + Parametric + Display) -> QueryChain {
    QueryChain {
        queries: vec![query.to_string()],
        bindings: query.get_bindings(),
        errors: query.get_errors(),
    }
}

pub struct QueryChain {
    queries: Vec<String>,
    bindings: BindingsList,
    errors: ErrorList,
}

impl QueryChain {
    /// Appends a new query to the end of the chain.
    ///
    /// This method takes a query that implements the `Queryable`, `Parametric`, and `Display` traits
    /// and appends it to the end of the query chain. The `get_bindings()` and `get_errors()` methods of
    /// the provided query are called to retrieve its bindings and errors, which are then merged with
    /// those of the current query chain. The string representation of the query is also added to the
    /// list of queries in the chain.
    ///
    /// # Examples
    ///
    /// ```
    /// use surrealdb_query_builder::{
    ///     Queryable,
    ///     Parametric,
    ///     chain,
    ///     statements::select,
    ///     SurrealId,
    ///     All
    /// };
    ///
    /// // Create a query chain with a single query
    /// let user_lowo = SurrealId::try_from("user:oyelowo").unwrap();
    /// let user_dayo = SurrealId::try_from("user:oyedayo").unwrap();
    ///
    /// // Append a new query to the chain
    /// let query1 = select(All).from(user_lowo);
    /// let query2 = select(All).from(user_dayo).limit(10);
    /// let chain = chain(query1).chain(query2);
    ///
    /// // The resulting chain contains both queries
    /// assert_eq!(chain.build(), "SELECT * FROM users;\nSELECT * FROM orders LIMIT 10;");
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn chain(mut self, query: impl Queryable + Parametric + Display) -> Self {
        self.bindings.extend(query.get_bindings());
        self.errors.extend(query.get_errors());
        self.queries.push(query.to_string());
        self
    }
}

impl Parametric for QueryChain {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Queryable for QueryChain {}
impl Erroneous for QueryChain {
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl Buildable for QueryChain {
    fn build(&self) -> String {
        self.queries.join("\n")
    }
}

impl fmt::Display for QueryChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cond,
        statements::{chain, select},
        traits::Buildable,
        All, Field, Operatable, SurrealId, ToRaw,
    };
    use insta::assert_display_snapshot;
    use select::order;

    #[test]
    fn test_chaining() {
        let age = Field::new("age");
        let country = Field::new("country");
        let city = Field::new("city");
        let fake_id = SurrealId::try_from("user:oyelowo").unwrap();
        let fake_id2 = SurrealId::try_from("user:oyedayo").unwrap();
        let fake_id3 = SurrealId::try_from("user:4").unwrap();

        let statement1 = select(All)
            .from(fake_id)
            .where_(cond(
                cond(city.is("Prince Edward Island"))
                    .and(city.is("NewFoundland"))
                    .or(city.like("Toronto")),
            ))
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

        let transaction = chain(statement1)
            .chain(statement2)
            .chain(select(All).from(fake_id3));

        assert_display_snapshot!(transaction.fine_tune_params());
        assert_display_snapshot!(transaction.to_raw().to_string());
    }
}
