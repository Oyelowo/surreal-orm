use async_graphql::*;

use ormx::Insert;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres};
use typed_builder::TypedBuilder;
use uuid::Uuid;
use validator::Validate;

use crate::user::User;

trait Collection {
    fn find_one() -> Post;
    fn create() -> Post;
}

#[derive(
    ormx::Table,SimpleObject, Clone, Serialize, Deserialize, TypedBuilder, Validate, Debug,
)]
#[serde(rename_all = "camelCase")]
//#[graphql(input_name = "PostInput")]
#[graphql(complex)]
#[ormx(table = "posts", id = id, insertable, patchable, deletable)]
pub struct Post {
    #[ormx(column = "id")]
    #[ormx(get_one)]
    pub id: Uuid,

    // FK
    pub poster_id: Uuid,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub title: String,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub content: String,
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
#[derive(InputObject, TypedBuilder, ormx::Patch)]
#[ormx(table_name = "posts", table = Post, id = "id")]
pub struct PostInput {
    pub poster_id: Uuid,
    pub title: String,
    pub content: String,
}

/*
fn validate_unique_postname(postname: &str) -> std::result::Result<(), ValidationError> {
    if postname == "xXxShad0wxXx" {
        // the value of the postname will automatically be added later
        return Err(ValidationError::new("terrible_postname"));
    }

    Ok(())
}
*/
