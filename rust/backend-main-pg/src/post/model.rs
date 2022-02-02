use async_graphql::*;

use ormx::{self, Patch, Table};
use sqlx::{
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
    PgPool,
};
use validator::Validate;

use crate::user::User;

#[derive(Table, SimpleObject, Validate, Debug)]
#[graphql(complex)]
#[ormx(table = "posts", id = id, insertable, deletable)]
pub struct Post {
    #[ormx(column = "id", get_one, default)]
    pub id: Uuid,

    // FK -> poster
    #[ormx(get_many)]
    pub user_id: Uuid,

    #[ormx(default)]
    pub created_at: DateTime<Utc>,

    #[ormx(default)]
    #[graphql(skip)]
    pub updated_at: Option<DateTime<Utc>>,

    #[ormx(default, set)]
    #[graphql(skip)]
    pub deleted_at: Option<DateTime<Utc>>,

    pub title: String,

    pub content: String,
}

#[ComplexObject]
impl Post {
    async fn poster(&self, ctx: &Context<'_>) -> anyhow::Result<User> {
        // TODO: Use dataloader to batch user
        let db = ctx.data_unchecked::<PgPool>();
        let poster = User::by_id(db, &self.user_id).await?;
        Ok(poster)
    }
}

// pub type PostInput = Post;
#[derive(InputObject, Validate, Patch)]
#[ormx(table_name = "posts", table = Post, id = "id")]
pub struct CreatePostInput {
    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub title: String,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub content: String,
}

pub type UpdatePostInput = CreatePostInput;
/*
fn validate_unique_postname(postname: &str) -> std::result::Result<(), ValidationError> {
    if postname == "xXxShad0wxXx" {
        // the value of the postname will automatically be added later
        return Err(ValidationError::new("terrible_postname"));
    }

    Ok(())
}
*/
