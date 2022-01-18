use super::{Post, PostInput};
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
        #[graphql(desc = "user data")] book_input: PostInput,
    ) -> anyhow::Result<Post> {
        // book_input.validate()?;
        let db = ctx.data_unchecked::<Database>();
        let mut post = Post::builder()
            .poster_id(book_input.poster_id)
            .title(book_input.title)
            .content(book_input.content)
            .build();
        // let mut book = User { ..book_input };
        post.validate()?;

        post.save(db, None).await?;

        Ok(post)
    }
}
