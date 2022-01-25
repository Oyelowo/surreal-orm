use crate::configs::model_cursor_to_vec;

use super::model::Post;

use async_graphql::*;
use mongodb::{
    bson::oid::ObjectId,
    options::{FindOneOptions, ReadConcern},
    Database,
};
use sqlx::PgPool;
use wither::{bson::doc, prelude::Model};

#[derive(Default)]
pub struct PostQueryRoot;

#[Object]
impl PostQueryRoot {
    async fn post(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the Post")] id: ObjectId,
    ) -> anyhow::Result<Option<Post>> {
        let db = ctx.data_unchecked::<PgPool>();
        let find_one_options = FindOneOptions::builder()
            .read_concern(ReadConcern::majority())
            .build();

        let post = Post::find_one(db, doc! {"_id": id}, find_one_options).await?;

        Ok(post)
    }

    async fn posts(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<Post>> {
        let db = ctx.data_unchecked::<PgPool>();
        let cursor = Post::find(db, None, None).await?;
        Ok(model_cursor_to_vec(cursor).await?)
    }
}
