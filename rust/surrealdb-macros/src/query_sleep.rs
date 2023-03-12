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
        self.0.to_string()
    }
}

impl Runnable for SleepStatement {}

impl Queryable for SleepStatement {}
