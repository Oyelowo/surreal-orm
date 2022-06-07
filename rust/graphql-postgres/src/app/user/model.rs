use async_graphql::*;
use sea_orm::{entity::prelude::*, QueryOrder};
use serde::{Deserialize, Serialize};
use sqlx::{
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
    FromRow,
};
// use chrono::{DateTime, Utc};
use validator::Validate;

use crate::{
    app::post::{self, Post, PostColumns, PostEntity},
    utils::{
        graphql,
        postgresdb::{get_pg_connection_from_ctx, get_pg_pool_from_ctx},
    },
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
    #[graphql(skip_input)]
    #[serde(skip_deserializing)] // Skip deserializing
    pub id: Uuid,

    #[sea_orm(default)]
    pub created_at: Option<DateTime<Utc>>,

    #[graphql(skip_input)]
    #[sea_orm(default)]
    pub updated_at: Option<DateTime<Utc>>,

    #[graphql(skip)]
    #[sea_orm(default)]
    pub deleted_at: Option<DateTime<Utc>>,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub username: String,

    #[validate(length(min = 1))]
    pub first_name: String,

    #[validate(length(min = 1))]
    pub last_name: String,

    #[validate(email)]
    pub email: String,

    #[validate(range(min = 18, max = 160))]
    pub age: i16,

    #[graphql(skip_input)]
    #[sea_orm(custom_type)]
    pub role: Role,

    pub disabled: Option<String>,

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
        let db = get_pg_connection_from_ctx(ctx)?;
        let id = format!("{}", self.id);
        let posts = PostEntity::find()
            .filter(PostColumns::UserId.contains(id.as_str()))
            .order_by_asc(PostColumns::CreatedAt)
            .all(db)
            .await?;

        Ok(posts)
    }

    async fn post_count(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<usize> {
        let post_count = self.posts(ctx).await?.len();
        Ok(post_count)
    }
}

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

impl Default for Role {
    fn default() -> Self {
        Self::User
    }
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
