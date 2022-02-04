use async_graphql::*;
use ormx::{Patch, Table};
use serde::{Deserialize, Serialize};
use sqlx::{
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
    FromRow, PgPool,
};
use validator::Validate;

use crate::app::post::Post;

#[derive(SimpleObject, Serialize, Deserialize, Table, FromRow, Validate, Debug)]
#[graphql(complex)]
#[ormx(table = "users", id = id, insertable, deletable)]
pub struct User {
    #[ormx(column = "id", get_one, default)]
    pub id: Uuid,

    #[ormx(default)]
    pub created_at: DateTime<Utc>,

    #[ormx(default)]
    #[graphql(skip)]
    pub updated_at: Option<DateTime<Utc>>,

    #[ormx(default)]
    #[graphql(skip)]
    pub deleted_at: Option<DateTime<Utc>>,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub username: String,

    #[validate(length(min = 1))]
    pub first_name: String,

    #[validate(length(min = 1))]
    pub last_name: String,

    // generate `User::by_email(&str) -> Result<Option<Self>>`
    #[ormx(get_optional(&str))]
    #[validate(email)]
    pub email: String,

    #[validate(range(min = 18, max = 160))]
    pub age: i16,

    #[ormx(custom_type)]
    pub role: Role,

    pub disabled: Option<String>,

    // #[serde(default)]
    // pub social_media: Vec<String>,

    // don't include this field into `InsertUser` since it has a default value
    // generate `User::set_last_login(Option<NaiveDateTime>) -> Result<()>`
    #[ormx(default, set)]
    pub last_login: Option<DateTime<Utc>>,
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

// #[derive(InputObject, TypedBuilder)]
// pub struct UserCreateInput {
//     pub last_name: String,
//     pub first_name: String,
//     pub email: String,
//     // pub social_media: Vec<String>,
//     pub age: u8,
// }

// Patches can be used to update multiple fields at once (in diesel, they're called "ChangeSets").
#[derive(Patch, InputObject, Validate)]
#[ormx(table_name = "users", table = User, id = "id")]
pub struct CreateUserInput {
    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub username: String,

    #[validate(length(min = 1))]
    pub first_name: String,

    #[validate(length(min = 1))]
    pub last_name: String,

    #[validate(email)]
    pub email: String,

    pub disabled: Option<String>,

    #[validate(range(min = 18, max = 160))]
    pub age: i16,

    // #[graphql(skip)]
    #[ormx(custom_type)]
    pub role: Role,
}

pub type UpdateUserInput = CreateUserInput;

#[derive(Debug, sqlx::Type, Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
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
