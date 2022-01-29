use async_graphql::*;

use crate::post::Post;
use chrono::{
    serde::{ts_nanoseconds, ts_nanoseconds_option},
    DateTime, Utc,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres};
use typed_builder::TypedBuilder;
use validator::Validate;

// use bson::DateTime;
use ormx::{conditional_query_as, Insert, Patch, Table};


#[derive(SimpleObject, Table, TypedBuilder, Validate, Serialize, Deserialize, Debug)]
// #[derive(InputObject)]
#[serde(rename_all = "camelCase")]
// #[graphql(input_name = "UserInput")]
#[graphql(complex)]
#[ormx(table = "users", id = id, insertable)]
pub struct User {
    #[ormx(column = "id")]
    #[ormx(get_one = get_by_id)]
    #[ormx(default)]
    #[builder(default)]
    pub id: Option<uuid::Uuid>,

    // #[builder(default)]
    pub created_at: DateTime<Utc>,

    // #[builder(default=Utc::now())]
    pub updated_at: DateTime<Utc>,

    #[builder(default)]
    #[ormx(default)]
    pub deleted_at: Option<DateTime<Utc>>,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub first_name: String,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub last_name: String,

    // #[builder(default, setter(strip_option))]
    // generate `User::by_email(&str) -> Result<Option<Self>>`
    #[ormx(get_optional(&str))]
    #[validate(email)]
    pub email: String,

    #[validate(range(min = 18, max = 160))]
    pub age: u8,

    #[ormx(custom_type)]
    role: Role,

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
        // let posts = ormx::conditional_query_as!(
        let k = Post::by_poster_id()
        let posts = ormx::conditional_query_as!(
            Post,
            r#"SELECT *"#
            "FROM posts"
        )
        .fetch_all(db)
        .await?;

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
#[derive(Patch, InputObject, Validate)]
#[ormx(table_name = "users", table = User, id = "id")]
pub struct UpdateUser {
    first_name: String,
    last_name: String,
    // disabled: Option<String>,
    // #[ormx(custom_type)]
    // role: Role,
}

#[derive(Debug, Copy, Clone, sqlx::Type)]
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
