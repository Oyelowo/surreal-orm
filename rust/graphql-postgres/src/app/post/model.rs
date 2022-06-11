use async_graphql::*;

use common::error_handling::ApiHttpStatus;
use sea_orm::{entity::prelude::*, DeleteMany, QueryOrder};
use serde::{Deserialize, Serialize};
use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};
use validator::Validate;

use crate::{app::user::user, utils::postgresdb::get_pg_connection_from_ctx};

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
#[graphql(complex, input_name = "PostInput", name = "Post")]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[graphql(skip_input)]
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)] // Skip deserializing
    pub id: Uuid,

    // FK -> poster
    pub user_id: Uuid,

    pub created_at: DateTime<Utc>,

    #[graphql(skip)]
    pub updated_at: Option<DateTime<Utc>>,

    #[graphql(skip)]
    pub deleted_at: Option<DateTime<Utc>>,

    pub title: String,

    pub content: String,
}

// UserColumn
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user::Entity",
        from = "Column::UserId",
        to = "user::Column::Id"
    )]
    User,
}

// impl RelationTrait for Relation {
//     fn def(&self) -> RelationDef {
//         match self {
//             Self::User => Entity::belongs_to(user::Entity)
//                 .from(Column::UserId)
//                 .to(user::Column::Id)
//                 .into(),
//         }
//     }
// }

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
impl Entity {
    pub fn find_by_id(id: Uuid) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_user_id(user_id: Uuid) -> Select<Entity> {
        Self::find()
            .filter(Column::UserId.eq(user_id))
            .order_by_asc(Column::CreatedAt)
    }

    pub fn find_by_username(username: &str) -> Select<Entity> {
        Self::find().filter(Column::Title.eq(username))
    }

    pub fn delete_by_id(id: Uuid) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}

pub type Post = Model;
pub type PostEntity = Entity;
pub type PostActiveModel = ActiveModel;

#[ComplexObject]
impl Post {
    async fn poster(&self, ctx: &Context<'_>) -> Result<user::Model> {
        // // TODO: Use dataloader to batch user
        let db = get_pg_connection_from_ctx(ctx)?;
        user::Entity::find_by_id(self.user_id)
            .one(db)
            .await?
            .ok_or_else(|| {
                ApiHttpStatus::NotFound("User not found. Try again later".into()).extend()
            })
    }
}
