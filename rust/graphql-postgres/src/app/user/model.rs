use async_graphql::*;
use ormx::{Patch, Table};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
    FromRow,
};
use validator::Validate;

use crate::{
    app::post::{self, Post},
    utils::postgresdb::get_pg_pool_from_ctx,
};

#[derive(
    Clone,
    DeriveEntityModel,
    SimpleObject,
    InputObject,
    Serialize,
    Deserialize,
    FromRow,
    Validate,
    Debug,
)]
#[graphql(complex, input_name = "UserInput")]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    // #[serde(skip_deserializing)] // Skip deserializing
    pub id: Uuid,

    #[sea_orm(default)]
    pub created_at: DateTime<Utc>,

    #[sea_orm(default)]
    #[graphql(skip)]
    pub updated_at: Option<DateTime<Utc>>,

    #[sea_orm(default)]
    #[graphql(skip)]
    pub deleted_at: Option<DateTime<Utc>>,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub username: String,

    #[validate(length(min = 1))]
    pub first_name: String,

    #[validate(length(min = 1))]
    pub last_name: String,

    // generate `User::by_email(&str) -> Result<Option<Self>>`
    #[sea_orm(get_optional(&str))]
    #[validate(email)]
    pub email: String,

    #[validate(range(min = 18, max = 160))]
    pub age: i16,

    #[sea_orm(custom_type)]
    pub role: Role,

    pub disabled: Option<String>,

    // #[serde(default)]
    // pub social_media: Vec<String>,

    // don't include this field into `InsertUser` since it has a default value
    // generate `User::set_last_login(Option<NaiveDateTime>) -> Result<()>`
    #[sea_orm(default, set)]
    pub last_login: Option<DateTime<Utc>>,
}
// use super::super::post::Entity

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    // #[sea_orm(has_many = "super::super::post::Entity")]
    // Post,
}

impl ActiveModelBehavior for ActiveModel {}

pub type User = Model;
pub type UserEntity = Entity;
pub type UserActiveModel = ActiveModel;

#[ComplexObject]
impl User {
    async fn posts(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<Post>> {
        let db = get_pg_pool_from_ctx(ctx)?;
        // let posts = Post::by_user_id(db, &self.id).await?;
        // Ok(posts)
        todo!()
    }

    async fn post_count(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<usize> {
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
// #[derive(Patch, InputObject, Validate)]
// #[ormx(table_name = "users", table = User, id = "id")]
// pub struct CreateUserInput {
//     #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
//     pub username: String,

//     #[validate(length(min = 1))]
//     pub first_name: String,

//     #[validate(length(min = 1))]
//     pub last_name: String,

//     #[validate(email)]
//     pub email: String,

//     pub disabled: Option<String>,

//     #[validate(range(min = 18, max = 160))]
//     pub age: i16,

//     // #[graphql(skip)]
//     #[ormx(custom_type)]
//     pub role: Role,
// }

// pub type UpdateUserInput = CreateUserInput;

#[derive(
    Debug,
    sqlx::Type,
    DeriveActiveEnum,
    EnumIter,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Enum,
)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Role {
    #[sea_orm(string_value = "user")]
    User,

    #[sea_orm(string_value = "admin")]
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
