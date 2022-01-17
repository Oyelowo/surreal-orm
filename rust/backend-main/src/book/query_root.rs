use super::model::Book;

use async_graphql::*;
use futures::stream::StreamExt;
use mongodb::{
    bson::oid::ObjectId,
    options::{FindOneOptions, ReadConcern},
    Database,
};
use wither::{bson::doc, prelude::Model};

#[derive(Default)]
pub struct BookQueryRoot;

#[Object]
impl BookQueryRoot {
    async fn book(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the Book")] id: ObjectId,
    ) -> anyhow::Result<Option<Book>> {
        let db = ctx.data_unchecked::<Database>();
        let find_one_options = FindOneOptions::builder()
            .read_concern(ReadConcern::majority())
            .build();

        let book = Book::find_one(db, doc! {"_id": id}, find_one_options).await?;

        Ok(book)
    }

    async fn books(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<Book>> {
        let db = ctx.data_unchecked::<Database>();
        let mut cursor = Book::find(db, None, None).await?;

        let mut books = vec![];
        while let Some(book) = cursor.next().await {
            books.push(book.unwrap());
        }

        Ok(books)
    }
}
