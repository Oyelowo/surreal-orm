/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use surrealdb::sql;

use crate::{query_insert::Buildable, query_remove::Runnable, query_select::Duration, Queryable};

pub fn sleep(duration: impl Into<Duration>) -> SleepStatement {
    SleepStatement::new(duration)
}

pub struct SleepStatement(String);

impl SleepStatement {
    fn new(duration: impl Into<Duration>) -> Self {
        let duration: Duration = duration.into();
        let duration = sql::Duration::from(duration);
        // self.timeout = Some(duration.to_string());
        Self(duration.to_string())
    }
}
impl Buildable for SleepStatement {
    fn build(&self) -> String {
        format!("SLEEP {};", self.0.to_string())
    }
}
impl Queryable for SleepStatement {}

impl Runnable for SleepStatement {}

impl Display for SleepStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {

    use std::time::Duration;

    use super::*;

    #[test]
    fn test_sleep_statement() {
        assert_eq!(sleep(Duration::from_secs(43)).build(), "SLEEP 43s;");
    }
}
