use crate::{
    app::{post::Post, user::UserEntity},
    utils::postgresdb::{get_pg_connection_from_ctx, get_pg_pool_from_ctx},
};

use super::{Role, User, UserActiveModel};
// use super::{CreateUserInput, InsertUser, Role, UpdateUserInput, User};
use async_graphql::*;
use chrono::Utc;
use common::error_handling::ApiHttpStatus;
use ormx::{Insert, Table};

use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use uuid::Uuid;
use validator::Validate;

#[derive(Default)]
pub struct UserMutationRoot;

#[Object]
impl UserMutationRoot {
    async fn create_user(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(desc = "user data")] user_input: User,
    ) -> async_graphql::Result<User> {
        user_input.validate()?;

        let db = get_pg_connection_from_ctx(ctx)?;
        let p = serde_json::to_value(user_input)?;
        let user = UserActiveModel::from_json(p)?.insert(db).await?;

        Ok(user)
    }

    async fn update_user(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: Uuid,
        user_input: User,
    ) -> async_graphql::Result<User> {
        user_input.validate()?;
        let db = get_pg_connection_from_ctx(ctx)?;

        user_input.validate()?;

        // Extract user id from session or decoded token whichever way authentication is implemented
        // id = IdFromSession

        let updated_user = UserActiveModel::from_json(serde_json::to_value(user_input)?)?;
        let user = UserEntity::find_by_id(id).one(db).await?;

        let user = UserActiveModel {
            id: Set(user.unwrap().id),
            ..updated_user
        }
        .update(db)
        .await
        .map_err(|e| {
            ApiHttpStatus::InternalServerError(
                "Could not update your user data. Try again later".into(),
            )
            .extend()
        })?;

        Ok(user)
    }
}
