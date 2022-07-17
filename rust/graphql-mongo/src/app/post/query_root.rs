use crate::utils::mongodb::get_db_from_ctx;

use super::model::Post;

use async_graphql::*;
use common::error_handling::ApiHttpStatus;
use futures_util::TryStreamExt;
use log::warn;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{FindOneOptions, ReadConcern},
};
use my_macros::FieldsGetter;
use wither::prelude::Model;

#[derive(Default)]
pub struct PostQueryRoot;

#[Object]
impl PostQueryRoot {
    async fn post(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the Post")] id: ObjectId,
    ) -> Result<Post> {
        let db = get_db_from_ctx(ctx)?;
        let find_one_options = FindOneOptions::builder()
            .read_concern(ReadConcern::majority())
            .build();

        // TODO: Move to model
        let post_keys = Post::get_fields_serialized();
        Post::collection(db)
            .find_one(doc! {post_keys._id: id}, find_one_options)
            .await?
            // Lazily evaluate the error:
            // Note: Always use _or_else variant of any helper function cos
            // eagerly evaluating can yield unintended consequences.
            // Readmore here  https://stackoverflow.com/questions/45547293/why-should-i-prefer-optionok-or-else-instead-of-optionok-or#:~:text=The%20only%20differences%20I%20know,Some%20data%20in%20the%20Option%20.
            .ok_or_else(|| ApiHttpStatus::NotFound("Post not found.".into()).extend())
    }

    async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let db = get_db_from_ctx(ctx)?;
        Post::collection(db)
            .find(None, None)
            .await?
            .try_collect()
            .await
            .map_err(|e| {
                // We don't want to expose our server internals to the end user.
                warn!("{e:?}");
                ApiHttpStatus::BadRequest("Could not fetch posts. Try again later".into()).extend()
            })
    }
}
