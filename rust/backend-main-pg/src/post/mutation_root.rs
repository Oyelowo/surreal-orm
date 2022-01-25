use super::{InsertPost, Post, PostInput};
use async_graphql::*;
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
        #[graphql(desc = "user data")] post_input: PostInput,
    ) -> anyhow::Result<Post> {
        // post_input.validate()?;
        let db = ctx.data_unchecked::<PgPool>();
        let mut post = Post::builder()
            .poster_id(post_input.poster_id)
            .title(post_input.title)
            .content(post_input.content)
            .build();
        post.validate()?;
        let mut post = InsertPost { ..post }
            .insert(&mut *db.acquire().await?)
            .await?;
        Ok(post)
    }

    async fn update_post(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the post")] &id: Uuid,
        #[graphql(desc = "post input")] post_input: PostInput,
    ) -> anyhow::Result<Post> {
        // post_input.validate()?;
        let db = ctx.data_unchecked::<PgPool>();
        
        let post = Post::get_by_id(db, id).await?;
        
        // Update multiple attributes of a post a time
        
    //      log::info!("update a single field");
    //      new.set_last_login(&db, Some(Utc::now().naive_utc()))
    //          .await?;
     
    //      log::info!("update all fields at once");
    //      new.email = "asdf".to_owned();
    //      new.update(&db).await?;
    //  ;

        log::info!("apply a patch to the post");
        post.patch(
            db,
            PostInput {
                title: post_input.title,
                content: post_input.content,
                poster_id: post_input.poster_id,
            },
        )
        .await?;

        post.reload(db).await?;

   
        Ok(post)
    }
}
/*
     let
        let post = InsertPost {
            id: None,
             poster_id: post_input.poster_id,
             title: post_input.title,
             content: post_input.content,
        };
        let p = Post {..post};

        // let mut post = User { ..post_input };
        post.validate()?;

        post.save(db, None).await?;
*/
