use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{engine::local::Db, Surreal};

use crate::ReturnType;

use super::{Buildable, Parametric};

// Create, Update, Relate, Delete
// [ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]

#[async_trait::async_trait]
pub trait Runnable
where
    Self: Parametric + Buildable,
{
    async fn run(&self, db: Surreal<Db>) -> Result<surrealdb::Response, surrealdb::Error> {
        let query = self.build();
        let query = db.query(query);
        let mut query = self.get_bindings().iter().fold(query, |acc, val| {
            acc.bind((val.get_param(), val.get_value()))
        });
        let mut response = query.await?;
        Ok(response)
    }
}

#[async_trait]
pub trait RunnableStandard<T>: Runnable + RunnableDefault<T>
where
    Self: Parametric + Buildable,
    T: Serialize + DeserializeOwned,
{
    async fn return_one_before(&self, db: Surreal<Db>) -> surrealdb::Result<T> {
        self.set_return_type(ReturnType::Before);
        let response = self.run(db).await?;
        let mut value = response.take::<Vec<T>>(0)?;
        // Maybe check if there are multiple items and return error if there are more than one;
        Ok(value.pop().unwrap())
    }

    async fn return_one_after(&self, db: Surreal<Db>) -> surrealdb::Result<T> {
        let response = self.run(db).await?;
        let mut value = response.take::<Vec<T>>(0)?;
        // Maybe check if there are multiple items and return error if there are more than one;
        Ok(value.pop().unwrap())
    }

    async fn return_one_diff(&self, db: Surreal<Db>) -> surrealdb::Result<T> {
        let response = self.run(db).await?;
        let mut value = response.take::<Vec<T>>(0)?;
        // Maybe check if there are multiple items and return error if there are more than one;
        Ok(value.pop().unwrap())
    }

    async fn return_one_projections(&self, db: Surreal<Db>) -> surrealdb::Result<T> {
        let response = self.run(db).await?;
        let mut value = response.take::<Vec<T>>(0)?;
        // Maybe check if there are multiple items and return error if there are more than one;
        Ok(value.pop().unwrap())
    }

    async fn return_many_before(&self, db: Surreal<Db>) -> surrealdb::Result<Vec<T>> {
        let response = self.run(db).await?;
        let mut value = response.take::<Vec<T>>(0)?;
        Ok(value)
    }

    async fn return_many_after(&self, db: Surreal<Db>) -> surrealdb::Result<Vec<T>> {
        let response = self.run(db).await?;
        let mut value = response.take::<Vec<T>>(0)?;
        Ok(value)
    }

    async fn return_many_diff(&self, db: Surreal<Db>) -> surrealdb::Result<Vec<T>> {
        let response = self.run(db).await?;
        let mut value = response.take::<Vec<T>>(0)?;
        Ok(value)
    }

    async fn return_many_projections(&self, db: Surreal<Db>) -> surrealdb::Result<Vec<T>> {
        let response = self.run(db).await?;
        let mut value = response.take::<Vec<T>>(0)?;
        Ok(value)
    }

    fn set_return_type(&self, return_type: ReturnType);
}

#[async_trait::async_trait]
pub trait RunnableDefault<T>: Runnable
where
    Self: Parametric + Buildable,
    T: Serialize + DeserializeOwned,
{
    async fn return_none(&self, db: Surreal<Db>) -> surrealdb::Result<()> {
        self.run(db).await?;
        Ok(())
    }

    async fn return_one(&self, db: Surreal<Db>) -> surrealdb::Result<T> {
        let response = self.run(db).await?;
        let mut value = response.take::<Vec<T>>(0)?;
        // Maybe check if there are multiple items and return error if there are more than one;
        Ok(value.pop().unwrap())
    }

    async fn return_many(&self, db: Surreal<Db>) -> surrealdb::Result<Vec<T>> {
        let response = self.run(db).await?;
        let mut value = response.take::<Vec<T>>(0)?;
        Ok(value)
    }
}

#[async_trait::async_trait]
pub trait RunnableSelect: Runnable
where
    Self: Parametric + Buildable,
{
    async fn return_none(&self, db: Surreal<Db>) -> surrealdb::Result<()> {
        self.run(db).await?;
        Ok(())
    }

    async fn return_one<T>(&self, db: Surreal<Db>) -> surrealdb::Result<T>
    where
        T: Serialize + DeserializeOwned,
    {
        let response = self.run(db).await?;
        let mut value = response.take::<Vec<T>>(0)?;
        // Maybe check if there are multiple items and return error if there are more than one;
        Ok(value.pop().unwrap())
    }

    async fn return_many<T>(&self, db: Surreal<Db>) -> surrealdb::Result<T>
    where
        T: Serialize + DeserializeOwned + IntoIterator,
    {
        let response = self.run(db).await?;
        let mut value = response.take::<T>(0)?;
        Ok(value)
    }
}
