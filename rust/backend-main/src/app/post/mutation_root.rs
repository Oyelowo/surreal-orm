use super::model::{Post, PostInput};
use async_graphql::*;
use mongodb::Database;
use validator::Validate;
use wither::Model;

#[derive(Default)]
pub struct PostMutationRoot;

#[Object]
impl PostMutationRoot {
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "user data")] post_input: PostInput,
    ) -> anyhow::Result<Post> {
        // post_input.validate()?;
        let db = ctx.data_unchecked::<Database>();
        let mut post = Post::builder()
            .poster_id(post_input.poster_id)
            .title(post_input.title)
            .content(post_input.content)
            .build();
        // let mut post = User { ..post_input };
        post.validate()?;

        post.save(db, None).await?;

        Ok(post)
    }
}
