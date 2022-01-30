use async_graphql::*;

use ormx::{Insert, Table};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres};
use typed_builder::TypedBuilder;
use uuid::Uuid;
use validator::Validate;

use crate::user::User;

#[derive(Table, SimpleObject, Validate, Debug)]
#[serde(rename_all = "camelCase")]
//#[graphql(input_name = "PostInput")]
#[graphql(complex)]
#[ormx(table = "posts", id = id, insertable, deletable)]
pub struct Post {
    #[ormx(column = "id")]
    #[ormx(get_one)]
    pub id: Uuid,

    // FK -> poster
    #[ormx(get_many)]
    pub user_id: Uuid,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,

    #[ormx(default)]
    pub deleted_at: Option<DateTime<Utc>>,

    pub title: String,

    pub context: String, // FIXME: change back to content
}

#[ComplexObject]
impl Post {
    async fn poster(&self, ctx: &Context<'_>) -> anyhow::Result<User> {
        // TODO: Use dataloader to batch user
        let db = ctx.data_unchecked::<PgPool>();
        let poster = User::by_id(db, &self.poster_id).await;
        poster
    }
}

// pub type PostInput = Post;
#[derive(InputObject, Validate, Patch)]
#[ormx(table_name = "posts", table = Post, id = "id")]
pub struct UpdatePostInput {
    pub user_id: Uuid,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub title: String,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub context: String, // FIXME: change back to content
}

pub type CreatePostInput = CreatePostInput;
/*
fn validate_unique_postname(postname: &str) -> std::result::Result<(), ValidationError> {
    if postname == "xXxShad0wxXx" {
        // the value of the postname will automatically be added later
        return Err(ValidationError::new("terrible_postname"));
    }

    Ok(())
}
*/
