use crate::utils::postgresdb::{get_pg_connection_from_ctx, get_pg_pool_from_ctx};

use super::model::{Post, PostEntity};

use async_graphql::*;
use sea_orm::EntityTrait;
use sqlx::types::Uuid;

#[derive(Default)]
pub struct PostQueryRoot;

#[Object]
impl PostQueryRoot {
    async fn post(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Post> {
        let db = get_pg_connection_from_ctx(ctx)?;
        let post = PostEntity::find_by_id(id)
            .one(db)
            .await?
            .expect("User not found handle this with proepr error message using my Error enum");
        Ok(post)
    }

    async fn posts(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<Post>> {
        let db = get_pg_pool_from_ctx(ctx)?;
        let posts = ormx::conditional_query_as!(Post, "SELECT * FROM posts")
            .fetch_all(db)
            .await?;
        Ok(posts)
    }
}
