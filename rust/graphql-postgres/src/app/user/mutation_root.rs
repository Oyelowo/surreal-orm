use crate::utils::postgresdb::get_pg_connection_from_ctx;

use super::user;

use async_graphql::*;

use common::error_handling::ApiHttpStatus;

use log::error;
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
        #[graphql(desc = "user data")] user_input: user::Model,
    ) -> async_graphql::Result<user::Model> {
        user_input.validate()?;

        let db = get_pg_connection_from_ctx(ctx)?;
        let p = serde_json::to_value(user_input)?;
        let user = user::ActiveModel::from_json(p)?.insert(db).await?;

        Ok(user)
    }

    async fn update_user(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: Uuid,
        user_input: user::Model,
    ) -> async_graphql::Result<user::Model> {
        user_input.validate()?;
        let db = get_pg_connection_from_ctx(ctx)?;

        user_input.validate()?;

        // Extract user id from session or decoded token whichever way authentication is implemented
        // id = IdFromSession

        let updated_user = user::ActiveModel::from_json(serde_json::to_value(user_input)?)?;
        let user = user::Entity::find_by_id(id).one(db).await?;

        let user = user::ActiveModel {
            id: Set(user.unwrap().id),
            ..updated_user
        }
        .update(db)
        .await
        .map_err(|e| {
            error!("Problem updating user data. Error: {e}");
            ApiHttpStatus::InternalServerError(
                "Could not update your user data. Try again later".into(),
            )
            .extend()
        })?;

        Ok(user)
    }
}
