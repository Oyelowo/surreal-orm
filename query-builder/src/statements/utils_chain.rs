/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt;

use crate::{
    BindingsList, Block, Buildable, Erroneous, ErrorList, Parametric, Queryable, ValueLike,
};

/// Chains together multiple queries into a single `QueryChain`.
///
/// # Arguments
///
/// * `query` - The first query in the chain. Subsequent queries can be added using the `chain` method on the returned `QueryChain`.
///
/// # Example Usage
///
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::select};
///
/// // Create a query chain with a single query
/// let user_lowo = TestUser::create_id("oyelowo");
/// let user_dayo = TestUser::create_id("oyedayo");
///
/// // Append a new query to the chain
/// let query1 = select(All).from(user_lowo);
/// let query2 = select(All).from(user_dayo).limit(10);
/// let chain = chain(query1).chain(query2);
///
/// // The resulting chain contains both queries
/// assert!(!chain.build().is_empty());
/// assert_eq!(chain.fine_tune_params(), "SELECT * FROM $_param_00000001;\n\nSELECT * FROM $_param_00000002 LIMIT $_param_00000003;");
/// assert_eq!(chain.to_raw().to_string(), "SELECT * FROM user:oyelowo;\n\nSELECT * FROM user:oyedayo LIMIT 10;");
/// ```
pub fn chain(query: impl Queryable + Parametric + Buildable) -> QueryChain {
    QueryChain {
        queries: vec![query.build()],
        bindings: query.get_bindings(),
        errors: query.get_errors(),
        paranthesized: false,
    }
}

/// Chains together multiple queries into a single `QueryChain`.
///
/// A `QueryChain` is created with an initial query, and additional queries can be added to the chain using the `chain` method. A `QueryChain` can be built into a single SQL query using the `build` method.
///
#[derive(Debug, Clone)]
pub struct QueryChain {
    queries: Vec<String>,
    bindings: BindingsList,
    errors: ErrorList,
    paranthesized: bool,
}

/// A chainable query.
#[derive(Debug, Clone)]
pub struct Chainable(ValueLike);

impl<T: Erroneous + Parametric + Buildable> From<T> for Chainable {
    fn from(query: T) -> Self {
        Self(ValueLike {
            string: query.build(),
            bindings: query.get_bindings(),
            errors: query.get_errors(),
        })
    }
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
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, statements::select};
    /// use surrealdb::sql;
    ///
    /// // Create a query chain with a single query
    /// let user_lowo = sql::Thing::from(("user".to_string(), "oyelowo".to_string()));
    /// let user_dayo = sql::Thing::from(("user".to_string(), "oyedayo".to_string()));
    ///
    /// // Append a new query to the chain
    /// let query1 = select(All).from(user_lowo);
    /// let query2 = select(All).from(user_dayo).limit(10);
    /// let chain = chain(query1).chain(query2);
    ///
    /// // The resulting chain contains both queries
    /// assert!(!chain.build().is_empty());
    /// assert_eq!(chain.fine_tune_params(), "SELECT * FROM $_param_00000001;\n\nSELECT * FROM $_param_00000002 LIMIT $_param_00000003;");
    /// assert_eq!(chain.to_raw().to_string(), "SELECT * FROM user:oyelowo;\n\nSELECT * FROM user:oyedayo LIMIT 10;");
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic.
    pub fn chain(mut self, query: impl Into<Chainable>) -> Self {
        let query = query.into().0;
        self.bindings.extend(query.get_bindings());
        self.errors.extend(query.get_errors());
        self.queries.push(query.build());
        self
    }

    pub fn paranthesized(mut self) -> Self {
        self.paranthesized = true;
        self
    }

    /// Surrounds the query chain with a curly brace.
    pub fn as_block(self) -> Block {
        self.into()
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
        let queries = self.queries.join("\n\n");
        if self.paranthesized {
            format!("({})", queries)
        } else {
            queries
        }
    }
}

impl fmt::Display for QueryChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl From<Vec<ValueLike>> for QueryChain {
    fn from(values: Vec<ValueLike>) -> Self {
        let mut bindings = BindingsList::new();
        let mut errors = ErrorList::new();
        let mut queries = Vec::new();

        for query in values {
            bindings.extend(query.get_bindings());
            errors.extend(query.get_errors());
            queries.push(query.build());
        }

        Self {
            queries,
            bindings,
            errors,
            paranthesized: false,
        }
    }
}

impl From<Vec<Chainable>> for QueryChain {
    fn from(values: Vec<Chainable>) -> Self {
        let mut bindings = BindingsList::new();
        let mut errors = ErrorList::new();
        let mut queries = Vec::new();

        for query in values {
            bindings.extend(query.0.get_bindings());
            errors.extend(query.0.get_errors());
            queries.push(query.0.build());
        }

        Self {
            queries,
            bindings,
            errors,
            paranthesized: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        chain, cond, statements::select, traits::Buildable, All, Field, Operatable, ToRaw,
    };
    use insta::assert_display_snapshot;
    use select::CanOrder;
    use surrealdb::sql;

    #[test]
    fn test_chaining() {
        let age = Field::new("age");
        let country = Field::new("country");
        let city = Field::new("city");
        let fake_id = sql::Thing::from(("user".to_string(), "oyelowo".to_string()));
        let fake_id2 = sql::Thing::from(("user".to_string(), "oyedayo".to_string()));
        let fake_id3 = sql::Thing::from(("user".to_string(), "4".to_string()));

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

        let transaction = chain(statement1)
            .chain(statement2)
            .chain(select(All).from(fake_id3));

        assert_display_snapshot!(transaction.fine_tune_params());
        assert_display_snapshot!(transaction.to_raw().to_string());
    }
}
