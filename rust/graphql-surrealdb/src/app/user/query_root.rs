use super::{guards::AuthGuard, model::User, UserBy};

use async_graphql::*;
use chrono::{DateTime, Utc};
use common::{authentication::TypedSession, error_handling::ApiHttpStatus};

use futures_util::TryStreamExt;
use log::warn;
use my_macros::FieldsGetter;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct UserQueryRoot;

#[Object]
impl UserQueryRoot {
    async fn me(&self, ctx: &Context<'_>) -> Result<User> {
        User::get_current_user(ctx)
            .await
            .map_err(|_e| ApiHttpStatus::NotFound("User not found".into()).extend())
    }

    async fn user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the User")] id: uuid::Uuid,
    ) -> Result<User> {
        todo!()
    }

    pub async fn get_user(&self, ctx: &Context<'_>, user_by: UserBy) -> Result<User> {
        todo!()
    }

    #[graphql(guard = "AuthGuard")]
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        todo!()
    }

    async fn session(&self, ctx: &Context<'_>) -> Result<Session> {
        let user_id = TypedSession::from_ctx(ctx)?.get_user_id()?;
        log::info!("Successfully retrieved session for user: {user_id:?}");

        Ok(Session {
            expires_at: TypedSession::get_expiry(),
            user_id,
        })
    }
}

#[derive(SimpleObject, InputObject, Serialize, Deserialize)]
struct Session {
    user_id: uuid::Uuid,
    expires_at: DateTime<Utc>,
}
