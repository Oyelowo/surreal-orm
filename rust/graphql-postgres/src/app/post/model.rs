use async_graphql::*;

use common::error_handling::ApiHttpStatus;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};
use validator::Validate;

use crate::{
    app::user::{User, UserEntity},
    utils::postgresdb::get_pg_connection_from_ctx,
};

#[derive(
    Clone,
    PartialEq,
    DeriveEntityModel,
    SimpleObject,
    Validate,
    Debug,
    InputObject,
    Serialize,
    Deserialize,
)]
#[graphql(complex, input_name = "PostInput")]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[graphql(skip_input)]
    #[serde(skip_deserializing)] // Skip deserializing
    pub id: Uuid,

    // FK -> poster
    #[sea_orm(get_many)]
    pub user_id: Uuid,

    #[sea_orm(default)]
    pub created_at: DateTime<Utc>,

    #[sea_orm(default)]
    #[graphql(skip)]
    pub updated_at: Option<DateTime<Utc>>,

    #[sea_orm(default, set)]
    #[graphql(skip)]
    pub deleted_at: Option<DateTime<Utc>>,

    pub title: String,

    pub content: String,
}

// UserColumn
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::super::user::Entity",
        from = "Column::UserId",
        to = "super::super::user::Column::Id"
    )]
    User,
}

impl Related<super::super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub type Post = Model;
pub type PostColumns = Column;
pub type PostEntity = Entity;
pub type PostActiveModel = ActiveModel;

#[ComplexObject]
impl Post {
    async fn poster(&self, ctx: &Context<'_>) -> Result<User> {
        // // TODO: Use dataloader to batch user
        let db = get_pg_connection_from_ctx(ctx)?;
        UserEntity::find_by_id(self.user_id)
            .one(db)
            .await?
            .ok_or_else(|| {
                ApiHttpStatus::NotFound("User not found. Try again later".into()).extend()
            })
    }
}
