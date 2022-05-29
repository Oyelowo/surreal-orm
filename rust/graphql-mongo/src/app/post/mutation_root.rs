use super::model::Post;
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
        #[graphql(desc = "user data")] mut post_input: Post,
    ) -> anyhow::Result<Post> {
        post_input.validate()?;
        let db = ctx.data_unchecked::<Database>();
        post_input.save(db, None).await?;

        Ok(post_input)
    }
}
