use super::{Buildable, Parametric};
use crate::{
    AllGetter, Field, Projections, Queryable, ReturnType, SurrealdbModel, SurrealdbOrmError,
    SurrealdbOrmResult, ToRaw,
};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{engine::local::Db, Surreal};

// Create, Update, Relate, Delete
// [ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]

/// A trait that represents a statement that can be run against the database.
#[async_trait::async_trait]
pub trait Runnable
where
    Self: Queryable,
{
    /// Runs the statement against the database.
    async fn run(&self, db: Surreal<Db>) -> SurrealdbOrmResult<surrealdb::Response> {
        let query_builder_error = self.get_errors();
        if !query_builder_error.is_empty() {
            return Err(SurrealdbOrmError::QueryBuilder(
                query_builder_error.join(". \n"),
            ));
        }
        let query = self.build();
        let query = db.query(query);
        let query = self.get_bindings().iter().fold(query, |acc, val| {
            acc.bind((val.get_param(), val.get_value()))
        });

        Ok(query.await.map_err(SurrealdbOrmError::QueryRun)?)
    }
}

impl<Q> Runnable for Q where Q: Queryable {}

/// A trait that represents a statement that can be run against the database and return a single
#[async_trait]
pub trait ReturnableStandard<T>
where
    Self: Parametric + Buildable + Sized + Send + Sync + ReturnableDefault<T> + Runnable,
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    /// Runs the statement against the database and returns the first result before the change.
    async fn return_first_before(self, db: Surreal<Db>) -> SurrealdbOrmResult<Option<T>> {
        let query = self.set_return_type(ReturnType::Before);
        query.return_first(db).await
    }

    /// Runs the statement against the database and returns the first result after the change.
    async fn return_first_after(self, db: Surreal<Db>) -> SurrealdbOrmResult<Option<T>> {
        let query = self.set_return_type(ReturnType::After);
        query.return_first(db).await
    }

    /// Runs the statement against the database and returns the first result of the change.
    async fn return_first_diff(self, db: Surreal<Db>) -> SurrealdbOrmResult<Option<T>> {
        let query = self.set_return_type(ReturnType::Diff);
        query.return_first(db).await
    }

    /// Runs the statement against the database and returns the first result of the change with the
    /// specified projections or list of fields.
    async fn return_first_projections<P>(
        self,
        db: Surreal<Db>,
        projections: impl Send + Into<Projections>,
    ) -> SurrealdbOrmResult<Option<P>>
    where
        P: Serialize + DeserializeOwned,
    {
        let mut query = self;
        let projections: Projections = projections.into();
        query = query.set_return_type(ReturnType::Projections(projections));

        let response = query.run(db).await?;
        get_first::<P>(response)
    }
    /// Runs the statement against the database and returns the one result before the change.
    async fn return_one_before(self, db: Surreal<Db>) -> SurrealdbOrmResult<Option<T>> {
        let query = self.set_return_type(ReturnType::Before);
        query.return_one(db).await
    }

    /// Runs the statement against the database and returns the one result after the change.
    async fn return_one_after(self, db: Surreal<Db>) -> SurrealdbOrmResult<Option<T>> {
        let query = self.set_return_type(ReturnType::After);
        query.return_one(db).await
    }

    /// Runs the statement against the database and returns the one result of the change.
    async fn return_one_diff(self, db: Surreal<Db>) -> SurrealdbOrmResult<Option<T>> {
        let query = self.set_return_type(ReturnType::Diff);
        query.return_one(db).await
    }

    /// Runs the statement against the database and returns the one result of the change with the
    /// specified projections or list of fields.
    async fn return_one_projections<P>(
        self,
        db: Surreal<Db>,
        projections: impl Send + Into<Projections>,
    ) -> SurrealdbOrmResult<Option<P>>
    where
        P: Serialize + DeserializeOwned,
    {
        let mut query = self;
        let projections: Projections = projections.into();
        query = query.set_return_type(ReturnType::Projections(projections));

        let response = query.run(db).await?;
        get_one::<P>(response)
    }

    /// Runs the statement against the database and returns the many result of the change with the
    /// specified projections or list of fields.
    async fn return_many_projections<P>(
        self,
        db: Surreal<Db>,
        projections: impl Send + Into<Projections>,
    ) -> SurrealdbOrmResult<Vec<P>>
    where
        P: Serialize + DeserializeOwned,
    {
        let mut query = self;
        let projections: Projections = projections.into();
        query = query.set_return_type(ReturnType::Projections(projections));

        let response = query.run(db).await?;
        get_many::<P>(response)
    }

    /// fetch all links for all fields
    async fn return_one_and_fetch_all_links_default(
        self,
        db: Surreal<Db>,
    ) -> SurrealdbOrmResult<Option<T>> {
        let mut query = self;
        query = query.set_return_type(ReturnType::Projections(
            vec![Field::new("*")]
                .into_iter()
                .chain(
                    T::get_linked_fields()
                        .into_iter()
                        // We use double i.e `.*.*` to also work for fetching array of links since it
                        // also works for a single link as well. If we do it once i.e `.*` then
                        // it will not work for array of links but only for a single link fields
                        // fetching.
                        // For array of list, the first set of `.*` says that we want all
                        // array items and the second set of `.*` says that we want all fields of
                        // all the arrays. If you want only a specific index, you would do link[0].*
                        // to get all fields of the first link and link[0].name to get only the name.
                        .map(|field| field.all().all())
                        .collect::<Vec<_>>(),
                )
                .collect::<Vec<_>>()
                .into(),
        ));

        query.return_one(db).await
    }

    /// Runs the statement against the database and returns the one result. In addition, specified
    /// fields' value are loaded. So, they return the value rather than the id if present.
    async fn return_one_and_fetch_all_links(
        self,
        db: Surreal<Db>,
        fields_to_fetch: Vec<Field>,
    ) -> SurrealdbOrmResult<Option<T>> {
        let mut query = self;
        query = query.set_return_type(ReturnType::Projections(
            vec![Field::new("*")]
                .into_iter()
                .chain(
                    fields_to_fetch
                        .into_iter()
                        // We use double i.e `.*.*` to also work for fetching array of links since it
                        // also works for a single link as well. If we do it once i.e `.*` then
                        // it will not work for array of links but only for a single link fields
                        // fetching.
                        // For array of list, the first set of `.*` says that we want all
                        // array items and the second set of `.*` says that we want all fields of
                        // all the arrays. If you want only a specific index, you would do link[0].*
                        // to get all fields of the first link and link[0].name to get only the name.
                        .map(|field| field.all().all())
                        .collect::<Vec<_>>(),
                )
                .collect::<Vec<_>>()
                .into(),
        ));

        query.return_one(db).await
    }

    /// Fetch all fields' links of all items in the returned array
    async fn return_many_and_fetch_all_links_default(
        self,
        db: Surreal<Db>,
    ) -> SurrealdbOrmResult<Vec<T>> {
        let mut query = self;
        query = query.set_return_type(ReturnType::Projections(
            vec![Field::new("*")]
                .into_iter()
                .chain(
                    T::get_linked_fields()
                        .into_iter()
                        // We use double i.e `.*.*` to also work for fetching array of links since it
                        // also works for a single link as well. If we do it once i.e `.*` then
                        // it will not work for array of links but only for a single link fields
                        // fetching.
                        // For array of list, the first set of `.*` says that we want all
                        // array items and the second set of `.*` says that we want all fields of
                        // all the arrays. If you want only a specific index, you would do link[0].*
                        // to get all fields of the first link and link[0].name to get only the name.
                        .map(|field| field.all().all())
                        .collect::<Vec<_>>(),
                )
                .collect::<Vec<_>>()
                .into(),
        ));

        query.return_many(db).await
    }

    /// Fetch all fields' links of all items in the returned array
    async fn return_many_and_fetch_all_links(
        self,
        db: Surreal<Db>,
        fields_to_fetch: Vec<Field>,
    ) -> SurrealdbOrmResult<Vec<T>> {
        let mut query = self;
        query = query.set_return_type(ReturnType::Projections(
            vec![Field::new("*")]
                .into_iter()
                .chain(
                    fields_to_fetch
                        .into_iter()
                        // We use double i.e `.*.*` to also work for fetching array of links since it
                        // also works for a single link as well. If we do it once i.e `.*` then
                        // it will not work for array of links but only for a single link fields
                        // fetching.
                        // For array of list, the first set of `.*` says that we want all
                        // array items and the second set of `.*` says that we want all fields of
                        // all the arrays. If you want only a specific index, you would do link[0].*
                        // to get all fields of the first link and link[0].name to get only the name.
                        .map(|field| field.all().all())
                        .collect::<Vec<_>>(),
                )
                .collect::<Vec<_>>()
                .into(),
        ));

        query.return_many(db).await
    }

    /// Return only the non-null loaded values of the linked fields.
    async fn return_one_and_fetch_non_null_links(
        self,
        db: Surreal<Db>,
        fields_to_fetch: Vec<Field>,
    ) -> SurrealdbOrmResult<Option<T>> {
        let mut query = self;
        dbg!(query.to_raw().build());
        dbg!(query.fine_tune_params());
        query = query.set_return_type(ReturnType::Projections(
            vec![Field::new("*")]
                .into_iter()
                .chain(
                    fields_to_fetch
                        .into_iter()
                        // Fetch only where the link is not null.
                        .map(|field| {
                            format!("{}[WHERE type::thing(id) IS NOT NULL].*", field).into()
                        })
                        .collect::<Vec<_>>(),
                )
                .collect::<Vec<_>>()
                .into(),
        ));

        query.return_one(db).await
    }

    /// Return only the non-null loaded values of the linked fields.
    async fn return_many_and_fetch_non_null_links(
        self,
        db: Surreal<Db>,
        fields_to_fetch: Vec<Field>,
    ) -> SurrealdbOrmResult<Vec<T>> {
        let mut query = self;
        query = query.set_return_type(ReturnType::Projections(
            vec![Field::new("*")]
                .into_iter()
                .chain(
                    fields_to_fetch
                        .into_iter()
                        // Fetch only where the link is not null.
                        .map(|field| {
                            format!("{}[WHERE type::thing(id) IS NOT NULL].*", field).into()
                        })
                        .collect::<Vec<_>>(),
                )
                .collect::<Vec<_>>()
                .into(),
        ));

        query.return_many(db).await
    }

    /// Runs the statement against the database and returns the many results before the change.
    async fn return_many_before(self, db: Surreal<Db>) -> SurrealdbOrmResult<Vec<T>> {
        let query = self.set_return_type(ReturnType::Before);
        query.return_many(db).await
    }

    /// Runs the statement against the database and returns the many results after the change.
    async fn return_many_after(self, db: Surreal<Db>) -> SurrealdbOrmResult<Vec<T>> {
        let query = self.set_return_type(ReturnType::After);
        query.return_many(db).await
    }

    /// Runs the statement against the database and returns the many results of the change.
    async fn return_many_diff(self, db: Surreal<Db>) -> SurrealdbOrmResult<Vec<T>> {
        let query = self.set_return_type(ReturnType::Diff);
        query.return_many(db).await
    }

    /// Internal method to set the surrealdb return type of the statement.
    fn set_return_type(self, return_type: ReturnType) -> Self;
}

/// A trait that represents a statement that can be run against the database
#[async_trait::async_trait]
pub trait ReturnableDefault<T>
where
    Self: Parametric + Buildable + Runnable,
    T: Serialize + DeserializeOwned,
{
    /// Runs the statement against the database and returns no result.
    async fn return_none(&self, db: Surreal<Db>) -> SurrealdbOrmResult<()> {
        self.run(db).await?;
        Ok(())
    }

    /// Runs the statement against the database and returns the first result.
    async fn return_first(&self, db: Surreal<Db>) -> SurrealdbOrmResult<Option<T>> {
        let response = self.run(db).await?;
        get_first::<T>(response)
    }

    /// Runs the statement against the database and returns the one result.
    async fn return_one(&self, db: Surreal<Db>) -> SurrealdbOrmResult<Option<T>> {
        let response = self.run(db).await?;
        get_one::<T>(response)
    }

    /// Runs the statement against the database and returns the one result with custom specified
    /// return type.
    async fn return_one_explicit<V>(&self, db: Surreal<Db>) -> SurrealdbOrmResult<Option<V>>
    where
        V: Serialize + DeserializeOwned,
    {
        let response = self.run(db).await?;
        get_one::<V>(response)
    }

    /// Runs the statement against the database and returns the many results.
    async fn return_many(&self, db: Surreal<Db>) -> SurrealdbOrmResult<Vec<T>> {
        let response = self.run(db).await?;
        get_many::<T>(response)
    }

    /// Runs the statement against the database and returns the many results with custom
    /// specified.
    async fn return_many_explicit<V>(&self, db: Surreal<Db>) -> SurrealdbOrmResult<Vec<V>>
    where
        V: Serialize + DeserializeOwned,
    {
        let response = self.run(db).await?;
        get_many::<V>(response)
    }
}

/// A trait that represents a statement that can be run against the database. Specifically for
/// selct statements.
#[async_trait::async_trait]
pub trait ReturnableSelect: Runnable
where
    Self: Parametric + Buildable,
{
    /// Runs the statement against the database and returns no result.
    async fn return_none(&self, db: Surreal<Db>) -> SurrealdbOrmResult<()> {
        self.run(db).await?;
        Ok(())
    }

    /// Runs the statement against the database and returns the first result.
    async fn return_first<T>(&self, db: Surreal<Db>) -> SurrealdbOrmResult<Option<T>>
    where
        T: Serialize + DeserializeOwned,
    {
        let response = self.run(db).await?;
        get_first::<T>(response)
    }

    /// Runs the statement against the database and returns the one result.
    async fn return_one<T>(&self, db: Surreal<Db>) -> SurrealdbOrmResult<Option<T>>
    where
        T: Serialize + DeserializeOwned,
    {
        let response = self.run(db).await?;
        get_one::<T>(response)
    }

    /// Runs the statement against the database and returns the one result with result unchecked.
    async fn return_one_unchecked<T>(&self, db: Surreal<Db>) -> T
    where
        T: Serialize + DeserializeOwned,
    {
        let response = self.run(db).await.unwrap();
        get_last::<T>(response).unwrap().unwrap()
    }

    /// Runs the statement against the database and returns the many results.
    async fn return_many<T>(&self, db: Surreal<Db>) -> SurrealdbOrmResult<Vec<T>>
    where
        T: Serialize + DeserializeOwned,
    {
        let response = self.run(db).await?;
        get_many::<T>(response)
    }
}

fn get_one<T>(mut response: surrealdb::Response) -> SurrealdbOrmResult<Option<T>>
where
    T: Serialize + DeserializeOwned,
{
    let mut value = response
        .take::<Vec<T>>(0)
        .map_err(SurrealdbOrmError::Deserialization)?;
    if value.len() > 1 {
        return Err(SurrealdbOrmError::TooManyItemsReturned(1.into()));
    }
    Ok(value.pop())
}

fn get_many<T>(mut response: surrealdb::Response) -> SurrealdbOrmResult<Vec<T>>
where
    T: Serialize + DeserializeOwned,
{
    let value = response
        .take::<Vec<T>>(0)
        .map_err(SurrealdbOrmError::Deserialization)?;

    Ok(value)
}

fn get_first<T>(mut response: surrealdb::Response) -> SurrealdbOrmResult<Option<T>>
where
    T: Serialize + DeserializeOwned,
{
    let mut value = response
        .take::<Vec<T>>(0)
        .map_err(SurrealdbOrmError::Deserialization)?;

    let value = if !value.is_empty() {
        Some(value.swap_remove(0))
    } else {
        None
    };

    Ok(value)
}

fn get_last<T>(mut response: surrealdb::Response) -> SurrealdbOrmResult<Option<T>>
where
    T: Serialize + DeserializeOwned,
{
    let mut value = response
        .take::<Vec<T>>(0)
        .map_err(SurrealdbOrmError::Deserialization)?;

    let value = if !value.is_empty() { value.pop() } else { None };

    Ok(value)
}
