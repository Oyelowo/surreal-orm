use std::fmt::Display;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{engine::local::Db, Surreal};

use crate::{Erroneous, Parametric};

pub trait Buildable {
    fn build(&self) -> String;
}

pub trait Queryable: Parametric + Buildable + Display + Erroneous {}

#[async_trait]
pub trait Runnable<T>
where
    Self: Parametric + Buildable,
    T: Serialize + DeserializeOwned,
{
    async fn return_one(&self, db: Surreal<Db>) -> surrealdb::Result<T> {
        let query = self.build();
        let mut response = self
            .get_bindings()
            .iter()
            .fold(db.query(query), |acc, val| {
                acc.bind((val.get_param(), val.get_value()))
            })
            .await?;

        // If it errors, try to check if multiple entries have been inputed, hence, suurealdb
        // trying to return Vec<T> rather than Option<T>, then pick the first of the returned
        // Ok<T>.
        let mut returned_val = match response.take::<Option<T>>(0) {
            Ok(one) => vec![one.unwrap()],
            Err(err) => response.take::<Vec<T>>(0)?,
        };

        // TODO:: Handle error if nothing is returned
        let only_or_last = returned_val.pop().unwrap();
        Ok(only_or_last)
    }

    async fn return_many(&self, db: Surreal<Db>) -> surrealdb::Result<Vec<T>> {
        let query = self.build();
        let mut response = self
            .get_bindings()
            .iter()
            .fold(db.query(query), |acc, val| {
                acc.bind((val.get_param(), val.get_value()))
            })
            .await?;

        // This does the reverse of get_one
        // If it errors, try to check if only single entry has been inputed, hence, suurealdb
        // trying to return Option<T>, then pick the return the only item as Vec<T>.
        let mut returned_val = match response.take::<Vec<T>>(0) {
            Ok(many) => many,
            Err(err) => vec![response.take::<Option<T>>(0)?.unwrap()],
        };

        // TODO:: Handle error if nothing is returned
        Ok(returned_val)
    }
}

#[async_trait::async_trait]
pub trait RunnableSelect
where
    Self: Parametric + Buildable,
{
    async fn return_one<T: Serialize + DeserializeOwned>(
        &self,
        db: surrealdb::Surreal<surrealdb::engine::local::Db>,
    ) -> surrealdb::Result<T> {
        let query = self.build();
        println!("XXXX {query}");
        let mut response = self
            .get_bindings()
            .iter()
            .fold(db.query(query), |acc, val| {
                acc.bind((val.get_param(), val.get_value()))
            })
            .await?;

        // If it errors, try to check if multiple entries have been inputed, hence, suurealdb
        // trying to return Vec  rather than Option , then pick the first of the returned
        // Ok .
        let mut returned_val = match response.take::<Option<T>>(0) {
            Ok(one) => vec![one.unwrap()],
            Err(err) => response.take::<Vec<T>>(0)?,
        };

        // TODO:: Handle error if nothing is returned
        let only_or_last = returned_val.pop().unwrap();
        Ok(only_or_last)
    }

    async fn return_many<T: Serialize + DeserializeOwned>(
        &self,
        db: surrealdb::Surreal<surrealdb::engine::local::Db>,
    ) -> surrealdb::Result<Vec<T>> {
        let query = self.build();
        println!("XXXX {query}");
        let mut response = self
            .get_bindings()
            .iter()
            .fold(db.query(query), |acc, val| {
                acc.bind((val.get_param(), val.get_value()))
            })
            .await?;

        println!("mmmmm {response:?}");
        // This does the reverse of get_one
        // If it errors, try to check if only single entry has been inputed, hence, suurealdb
        // trying to return Option , then pick the return the only item as Vec .
        let mut returned_val = match response.take::<Vec<T>>(0) {
            Ok(many) => many,
            Err(err) => vec![response.take::<Option<T>>(0)?.unwrap()],
        };

        // TODO:: Handle error if nothing is returned
        Ok(returned_val)
    }
}

#[async_trait::async_trait]
pub trait Runnables
where
    Self: Buildable,
{
    async fn run(
        &self,
        db: surrealdb::Surreal<surrealdb::engine::local::Db>,
    ) -> surrealdb::Result<()> {
        let query = self.build();
        db.query(query).await?;
        Ok(())
    }
}
