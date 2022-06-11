use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany};
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
    utils::postgresdb::get_pg_connection_from_ctx,
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
#[graphql(complex, input_name = "UserInput", name = "User")]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[graphql(skip_input)]
    #[serde(skip_deserializing)] // Skip deserializing
    pub id: Uuid,

    pub created_at: DateTime<Utc>,

    #[graphql(skip_input)]
    pub updated_at: Option<DateTime<Utc>>,

    #[graphql(skip)]
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
    pub age: i16, // Should be u8 but pleasing sqlx for now till i update my db model

    #[graphql(skip_input)]
    #[sea_orm(custom_type)]
    pub role: Option<Role>,

    pub disabled: Option<String>,

    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "post::Entity")]
    Post,
}

impl Related<post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id(id: Uuid) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_username(username: &str) -> Select<Entity> {
        Self::find().filter(Column::Username.eq(username))
    }

    pub fn delete_by_id(id: Uuid) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}

#[ComplexObject]
impl Model {
    async fn posts(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<Post>> {
        let db = get_pg_connection_from_ctx(ctx)?;
        let posts = post::Entity::find_by_user_id(self.id).all(db).await?;
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
