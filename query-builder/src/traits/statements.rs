use super::{Buildable, Parametric};
use crate::{
    AllGetter, Field, Model, Projections, Queryable, ReturnType, SurrealOrmError, SurrealOrmResult,
    ValueLike,
};
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{Connection, Surreal};

// Create, Update, Relate, Delete
// [ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]

/// A trait that represents a statement that can be run against the database.
#[async_trait::async_trait]
pub trait Runnable
where
    Self: Queryable,
{
    /// Runs the statement against the database.
    async fn run(
        &self,
        db: Surreal<impl surrealdb::Connection>,
    ) -> SurrealOrmResult<surrealdb::Response> {
        let query_builder_error = self.get_errors();

        if !query_builder_error.is_empty() {
            return Err(SurrealOrmError::QueryBuilder(
                query_builder_error.join(". \n"),
            ));
        }

        let query = self.build();
        let query = db.query(query);
        let query = self.get_bindings().iter().fold(query, |acc, val| {
            acc.bind((val.get_param(), val.get_value()))
        });

        Ok(query.await.map_err(SurrealOrmError::QueryRun)?)
    }

    /// Runs the statement against the database and returns the deserialized result.
    async fn get_data<T>(
        &self,
        db: Surreal<impl surrealdb::Connection>,
    ) -> SurrealOrmResult<Option<T>>
    where
        T: Sized + Serialize + DeserializeOwned,
    {
        let mut response = self.run(db).await?;

        Ok(response
            .take::<Option<T>>(0)
            .map_err(SurrealOrmError::Deserialization)?)
    }
}

impl<Q> Runnable for Q where Q: Queryable {}

/// A trait that represents a statement that can be run against the database and return a single
#[async_trait::async_trait]
pub trait ReturnableStandard<T>
where
    Self: Parametric + Buildable + Sized + Send + Sync + Runnable,
    T: Serialize + DeserializeOwned + Model,
{
    // /// Runs the statement against the database and returns the first result before the change.
    // async fn return_first_before(self, db: Surreal<Db>) -> SurrealOrmResult<Option<T>> {
    //     let query = self.set_return_type(ReturnType::Before);
    //     query.return_first(db).await
    // }
    //
    // /// Runs the statement against the database and returns the first result after the change.
    // async fn return_first_after(self, db: Surreal<Db>) -> SurrealOrmResult<Option<T>> {
    //     let query = self.set_return_type(ReturnType::After);
    //     query.return_first(db).await
    // }
    //
    // /// Runs the statement against the database and returns the first result of the change.
    // async fn return_first_diff(self, db: Surreal<Db>) -> SurrealOrmResult<Option<T>> {
    //     let query = self.set_return_type(ReturnType::Diff);
    //     query.return_first(db).await
    // }

    /// Runs the statement against the database and returns the first result of the change with the
    /// specified projections or list of fields.
    async fn return_first_projections<P>(
        self,
        db: Surreal<impl Connection>,
        projections: impl Send + Into<Projections>,
    ) -> SurrealOrmResult<Option<P>>
    where
        P: Serialize + DeserializeOwned,
    {
        let mut query = self;
        let projections: Projections = projections.into();
        query = query.set_return_type(ReturnType::Projections(projections));

        let response = query.run(db).await?;
        get_first::<P>(response)
    }

    /// Runs the statement against the database and returns the one result of the change with the
    /// specified projections or list of fields.
    async fn return_one_projections<P>(
        self,
        db: Surreal<impl Connection>,
        projections: impl Send + Into<Projections>,
    ) -> SurrealOrmResult<Option<P>>
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
        db: Surreal<impl Connection>,
        projections: impl Send + Into<Projections>,
    ) -> SurrealOrmResult<Vec<P>>
    where
        P: Serialize + DeserializeOwned,
    {
        let mut query = self;
        let projections: Projections = projections.into();
        query = query.set_return_type(ReturnType::Projections(projections));

        let response = query.run(db).await?;
        get_many::<P>(response)
    }

    #[doc(hidden)]
    fn validate_fields_to_fetch(linked_fields_to_fetch: &[Field]) -> SurrealOrmResult<Vec<String>> {
        let result = linked_fields_to_fetch
            .iter()
            .map(|n| {
                let n: ValueLike = n.into();
                n.build()
            })
            .filter(|n| !T::get_linked_fields().iter().any(|m| m.build() == *n))
            // .map(|&n| {
            //     let n: ValueLike = n.into();
            //     n.build()
            // })
            .collect::<Vec<_>>();

        if !result.is_empty() {
            return Err(SurrealOrmError::FieldsUnfetchableNotARecordLink(
                result.join(", "),
            ));
        }
        Ok(result)
    }

    /// Sets the return type to projections and fetches all records links.
    /// Defaults values to null for referenced records that do not exist.
    fn load_links(
        self,
        linked_fields_to_fetch: Vec<impl Into<ValueLike>>,
    ) -> SurrealOrmResult<Self> {
        let linked_fields_to_fetch = linked_fields_to_fetch
            .into_iter()
            .map(|field| {
                let value_like: ValueLike = field.into();
                Field::new(value_like.build()).with_bindings(value_like.get_bindings())
            })
            .collect::<Vec<_>>();
        Self::validate_fields_to_fetch(&linked_fields_to_fetch)?;

        let projections = ReturnType::Projections(
            vec![Field::new("*")]
                .into_iter()
                .chain(
                    linked_fields_to_fetch
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
        );

        Ok(self.set_return_type(projections))
    }

    /// load links values and filter out null references.
    // fn load_links_non_null(self, linked_fields_to_fetch: Vec<Field>) -> SurrealOrmResult<Self> {
    //     Self::validate_fields_to_fetch(&linked_fields_to_fetch)?;
    //     let projections = ReturnType::Projections(
    //         vec![Field::new("*")]
    //             .into_iter()
    //             .chain(
    //                 linked_fields_to_fetch
    //                     .into_iter()
    //                     // Fetch only where the link is not null.
    //                     .map(|field| {
    //                         // format!("{}[WHERE type::thing(id) IS NOT NONE].*", field).into()
    //                         format!("{}[WHERE id].*", field).into()
    //                     })
    //                     .collect::<Vec<_>>(),
    //             )
    //             .collect::<Vec<_>>()
    //             .into(),
    //     );
    //
    //     Ok(self.set_return_type(projections))
    // }
    //
    /// Sets the return type to projections and fetches the all record links values.
    /// For link_one, link_self, it returns null if the link is null or the reference
    /// does not exist. For link_many, it returns None for items that are null or the reference
    /// do not exist.
    fn load_all_links(self) -> SurrealOrmResult<Self> {
        self.load_links(T::get_linked_fields())
    }

    // /// Sets the return type to projections and fetches the all record links values.
    // /// For link_one, link_self, it returns null if the link is null or the reference
    // /// does not exist. For link_many, it returns filters out links that are null or that
    // /// point to reference that does not yet exist
    // fn load_all_links_non_null(self) -> SurrealOrmResult<Self> {
    //     self.load_links_non_null(T::get_linked_fields())
    // }

    /// Sets the return type to projections and fetches the all record links values
    /// for link_many fields including the null record links.
    fn load_link_manys(self) -> SurrealOrmResult<Self> {
        self.load_links(T::get_link_many_fields())
    }

    // /// Sets the return type to projections and fetches the all record links values
    // /// for link_many fields excluding the null record links.
    // fn load_link_manys_non_null(self) -> SurrealOrmResult<Self> {
    //     self.load_links_non_null(T::get_link_many_fields())
    // }

    /// Sets the return type to projections and fetches the all record links values
    /// for link_one fields. Defaults to null if the reference does not exist.
    fn load_link_ones(self) -> SurrealOrmResult<Self> {
        self.load_links(T::get_link_one_fields())
    }

    /// Sets the return type to projections and fetches the all record links values
    /// for link_self fields. Defaults to null if the reference does not exist.
    fn load_link_selfs(self) -> SurrealOrmResult<Self> {
        self.load_links(T::get_link_self_fields())
    }

    /// Runs the statement against the database and returns the one result.
    async fn return_one(&self, db: Surreal<impl Connection>) -> SurrealOrmResult<Option<T>> {
        let response = self.run(db).await?;
        get_one::<T>(response)
    }

    /// Runs the statement against the database and returns the one result.
    /// Returns an error if the result is not exactly one.
    /// It does best effort to make sure all fields are selected
    /// even if you select subset, it fills up the rest to make
    /// sure you get the full record and can be properly deserialized.
    async fn get_one(self, db: Surreal<impl Connection>) -> SurrealOrmResult<T> {
        let response = self.run(db).await?;
        let returned_type = self.get_return_type();
        let all = vec![ValueLike::from(Field::new("*"))];
        let selected_fields = match returned_type {
            ReturnType::Projections(projections) => all
                .into_iter()
                .chain(
                    projections
                        .iter()
                        .filter(|p| p.build() != "*")
                        .cloned()
                        .collect::<Vec<_>>(),
                )
                .collect::<Vec<_>>(),
            _ => all,
        };

        let query = self.set_return_type(ReturnType::Projections(Projections(selected_fields)));

        get_one::<T>(response)
            .map(|r| r.ok_or_else(|| SurrealOrmError::RecordNotFound(query.build())))?
    }

    /// Runs the statement against the database and returns the many results.
    async fn return_many(&self, db: Surreal<impl Connection>) -> SurrealOrmResult<Vec<T>> {
        let response = self.run(db).await?;
        get_many::<T>(response)
    }

    /// Runs the statement against the database and returns no result.
    async fn return_none(&self, db: Surreal<impl Connection>) -> SurrealOrmResult<()> {
        self.run(db).await?;
        Ok(())
    }

    /// Runs the statement against the database and returns the first result.
    async fn return_first(&self, db: Surreal<impl Connection>) -> SurrealOrmResult<Option<T>> {
        let response = self.run(db).await?;
        get_first::<T>(response)
    }

    /// Runs the statement against the database and returns the many results before the change.
    async fn return_many_before(self, db: Surreal<impl Connection>) -> SurrealOrmResult<Vec<T>> {
        let query = self.set_return_type(ReturnType::Before);
        query.return_many(db).await
    }

    /// Internal method to set the surrealdb return type of the statement.
    fn set_return_type(self, return_type: ReturnType) -> Self;

    /// Internal method to get the surrealdb return type of the statement.
    fn get_return_type(&self) -> ReturnType;
}

/// A trait that represents a statement that can be run against the database
#[async_trait::async_trait]
pub trait ReturnableDefault<T>
where
    Self: Parametric + Buildable + Runnable,
    T: Serialize + DeserializeOwned,
{
    /// Runs the statement against the database and returns the one result with custom specified
    /// return type.
    async fn return_one_explicit<V>(
        &self,
        db: Surreal<impl Connection>,
    ) -> SurrealOrmResult<Option<V>>
    where
        V: Serialize + DeserializeOwned,
    {
        let response = self.run(db).await?;
        get_one::<V>(response)
    }

    /// Runs the statement against the database and returns the many results with custom
    /// specified.
    async fn return_many_explicit<V>(
        &self,
        db: Surreal<impl Connection>,
    ) -> SurrealOrmResult<Vec<V>>
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
    async fn return_none(&self, db: Surreal<impl Connection>) -> SurrealOrmResult<()> {
        self.run(db).await?;
        Ok(())
    }

    /// Runs the statement against the database and returns the first result.
    async fn return_first<T>(&self, db: Surreal<impl Connection>) -> SurrealOrmResult<Option<T>>
    where
        T: Serialize + DeserializeOwned,
    {
        let response = self.run(db).await?;
        get_first::<T>(response)
    }

    /// Runs the statement against the database and returns the one result.
    async fn return_one<T>(
        &self,
        db: Surreal<impl surrealdb::Connection>,
    ) -> SurrealOrmResult<Option<T>>
    where
        T: Serialize + DeserializeOwned,
    {
        let response = self.run(db).await?;
        get_one::<T>(response)
    }

    /// Runs the statement against the database and returns the one result with result unchecked.
    async fn return_one_unchecked<T>(&self, db: Surreal<impl Connection>) -> T
    where
        T: Serialize + DeserializeOwned,
    {
        let response = self.run(db).await.unwrap();
        get_last::<T>(response).unwrap().unwrap()
    }

    /// Runs the statement against the database and returns the many results.
    async fn return_many<T>(&self, db: Surreal<impl Connection>) -> SurrealOrmResult<Vec<T>>
    where
        T: Serialize + DeserializeOwned,
    {
        let response = self.run(db).await?;
        get_many::<T>(response)
    }
}

fn get_one<T>(mut response: surrealdb::Response) -> SurrealOrmResult<Option<T>>
where
    T: Serialize + DeserializeOwned,
{
    let mut value = response
        .take::<Vec<T>>(0)
        .map_err(SurrealOrmError::Deserialization)?;
    if value.len() > 1 {
        return Err(SurrealOrmError::TooManyItemsReturned(1.into()));
    }
    Ok(value.pop())
}

fn get_many<T>(mut response: surrealdb::Response) -> SurrealOrmResult<Vec<T>>
where
    T: Serialize + DeserializeOwned,
{
    let value = response
        .take::<Vec<T>>(0)
        .map_err(SurrealOrmError::Deserialization)?;

    Ok(value)
}

fn get_first<T>(mut response: surrealdb::Response) -> SurrealOrmResult<Option<T>>
where
    T: Serialize + DeserializeOwned,
{
    let mut value = response
        .take::<Vec<T>>(0)
        .map_err(SurrealOrmError::Deserialization)?;

    let value = if !value.is_empty() {
        Some(value.swap_remove(0))
    } else {
        None
    };

    Ok(value)
}

fn get_last<T>(mut response: surrealdb::Response) -> SurrealOrmResult<Option<T>>
where
    T: Serialize + DeserializeOwned,
{
    let mut value = response
        .take::<Vec<T>>(0)
        .map_err(SurrealOrmError::Deserialization)?;

    let value = if !value.is_empty() { value.pop() } else { None };

    Ok(value)
}
