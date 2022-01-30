use super::model::Post;

use async_graphql::*;
use sqlx::{query_as, types::Uuid, PgPool};

#[derive(Default)]
pub struct PostQueryRoot;

#[Object]
impl PostQueryRoot {
    async fn post(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the Post")] id: &Uuid,
    ) -> anyhow::Result<Option<Post>> {
        let db = ctx.data_unchecked::<PgPool>();
        let post = Post::by_id(db, id).await?;
        Ok(post)
    }

    async fn posts(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<Post>> {
        let db = ctx.data_unchecked::<PgPool>();
        let posts = ormx::conditional_query_as!(Post, "SELECT * FROM posts")
            .fetch_all(db)
            .await?;
        Ok(posts)
    }
}
