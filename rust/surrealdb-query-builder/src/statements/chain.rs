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

use crate::traits::{BindingsList, Buildable, Erroneous, ErrorList, Parametric, Queryable};

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
        filter::cond,
        sql::{All, SurrealId},
        Field, Operatable,
    };

    use super::*;

    #[test]
    #[cfg(feature = "mock")]
    fn test_transaction_commit() {
        use crate::statements::select;

        let name = Field::new("name");
        let age = Field::new("age");
        let country = Field::new("country");
        let city = Field::new("city");
        let fake_id = SurrealId::try_from("user:oyelowo").unwrap();
        let fake_id2 = SurrealId::try_from("user:oyedayo").unwrap();
        let fake_id3 = SurrealId::try_from("user:4").unwrap();

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

        let transaction = chain(statement1)
            .chain(statement2)
            .chain(select(All).from(fake_id3));

        assert_debug_snapshot!(transaction.get_bindings());
        assert_display_snapshot!(transaction);
    }

    #[test]
    #[cfg(feature = "raw")]
    fn test_transaction_cancel() {
        use crate::statements::select::order;

        let name = Field::new("name");
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

        assert_debug_snapshot!(transaction.get_bindings());
        assert_display_snapshot!(transaction);
    }
}
