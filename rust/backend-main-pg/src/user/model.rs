use async_graphql::*;

use crate::post::Post;
use chrono::{
    serde::{ts_nanoseconds, ts_nanoseconds_option},
    DateTime, Utc,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres, types::Uuid};
use typed_builder::TypedBuilder;
use validator::Validate;

use ormx::{conditional_query_as, Delete, Insert, Patch, Table};

#[derive(SimpleObject, Table, Validate, Debug)]
#[serde(rename_all = "camelCase")]
#[graphql(complex)]
#[ormx(table = "users", id = id, insertable, deletable)]
pub struct User {
    #[ormx(column = "id")]
    #[ormx(get_one)]
    id: Uuid,

    created_at: DateTime<Utc>,

    updated_at: DateTime<Utc>,

    #[ormx(default)]
    deleted_at: Option<DateTime<Utc>>,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    first_name: String,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    last_name: String,

    // generate `User::by_email(&str) -> Result<Option<Self>>`
    #[ormx(get_optional(&str))]
    #[validate(email)]
    pub email: String,

    // #[validate(range(min = 18, max = 160))]
    // pub age: u8,

    #[ormx(custom_type)]
    role: Role,

    disabled: String,

    // #[serde(default)]
    // pub social_media: Vec<String>,

    // don't include this field into `InsertUser` since it has a default value
    // generate `User::set_last_login(Option<NaiveDateTime>) -> Result<()>`
    #[ormx(default, set)]
    last_login: Option<DateTime<Utc>>,
}

#[ComplexObject]
impl User {
    async fn posts(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<Post>> {
        let db = ctx.data_unchecked::<PgPool>();
        let posts = Post::by_user_id(db, &self.id).await?;
        Ok(posts)
    }

    async fn post_count(&self, ctx: &Context<'_>) -> anyhow::Result<usize> {
        let post_count = self.posts(ctx).await?.len();
        Ok(post_count)
    }
}

//impl InputObject for InsertUser {}
impl Validate for InsertUser {}

// #[derive(InputObject, TypedBuilder)]
// pub struct UserCreateInput {
//     pub last_name: String,
//     pub first_name: String,
//     pub email: String,
//     // pub social_media: Vec<String>,
//     pub age: u8,
// }

// Patches can be used to update multiple fields at once (in diesel, they're called "ChangeSets").
#[derive(Patch, InputObject, Validate, TypedBuilder)]
#[ormx(table_name = "users", table = User, id = "id")]
pub struct UpdateUser {
    first_name: String,
    last_name: String,
    disabled: Option<String>,

    #[graphql(skip)]
    #[ormx(custom_type)]
    role: Role,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "user_role")]
#[sqlx(rename_all = "lowercase")]
pub enum Role {
    User,
    Admin,
}

// pub type UserInput = User;

/*

fn validate_unique_username(username: &str) -> std::result::Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }

    Ok(())
}
*/
