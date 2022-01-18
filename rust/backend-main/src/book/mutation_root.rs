use super::{Book, BookInput};
use async_graphql::*;
use mongodb::Database;
use validator::Validate;
use wither::Model;

#[derive(Default)]
pub struct BookMutationRoot;

#[Object]
impl BookMutationRoot {
    async fn add_book(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "user data")] book_input: BookInput,
    ) -> anyhow::Result<Book> {
        // book_input.validate()?;
        let db = ctx.data_unchecked::<Database>();
        let mut book = Book::builder()
            .author_ids(book_input.author_ids)
            .title(book_input.title)
            .build();
        // let mut book = User { ..book_input };
        book.validate()?;

        book.save(db, None).await?;

        Ok(book)
    }
}
