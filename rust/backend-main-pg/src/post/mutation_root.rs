use crate::post::CreatePostInput;

use super::{InsertPost, Post, CreatePostInput, UpdatePostInput};
use async_graphql::*;
use chrono::Utc;
use ormx::{Insert, Table};
use sqlx::{Acquire, PgPool};
use uuid::Uuid;
use validator::Validate;

#[derive(Default)]
pub struct PostMutationRoot;

#[Object]
impl PostMutationRoot {
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of user")] user_id: Uuid,
        #[graphql(desc = "post data")] post_input: CreatePostInput,
    ) -> anyhow::Result<Post> {
        let db = ctx.data_unchecked::<PgPool>();
        // NOTE: Normally, user id will be retrieved from session or jwt or oauth.
        // but hard code as a parameter for now.
        post_input.validate()?;

        let mut new_post = InsertPost { 
            ..post_input,
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            user_id,
         }
            .insert(&mut *db.acquire().await?)
            .await?;
        Ok(new_post)
    }

    async fn update_post(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the post")] &id: Uuid,
        #[graphql(desc = "post input")] post_input: UpdatePostInput,
    ) -> anyhow::Result<Post> {
        post_input.validate()?;
        let db = ctx.data_unchecked::<PgPool>();
        // TODO: validate using async-graphql guard that the updater is the authenticated user i.e post.user_id === session/jwt.user_id
        let post = Post::by_id(db, id).await?;
        log::info!("update a single field");
        post.updated_at = Utc::now();
        post.title = post_input.title;
        post.context = post_input.context;
        
        // log::info!("apply a patch to the post");
        // post.patch(
        //     db,
        //     UpdatePostInput {
        //         title: post_input.title,
        //         context: post_input.context,
        //         user_id: post.user_id,
                
        //     },
        // )
        // .await?;

        post.reload(db).await?;

   
        Ok(post)
    }
}
