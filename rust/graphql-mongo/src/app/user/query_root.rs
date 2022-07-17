use crate::utils::mongodb::get_db_from_ctx;

use super::{guards::AuthGuard, model::User, UserBy};

use async_graphql::*;
use chrono::{DateTime, Utc};
use common::{authentication::TypedSession, error_handling::ApiHttpStatus};

use futures_util::TryStreamExt;
use log::warn;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{FindOneOptions, FindOptions, ReadConcern},
};
use my_macros::FieldsGetter;
use serde::{Deserialize, Serialize};
use wither::Model;

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
        #[graphql(desc = "id of the User")] id: ObjectId,
    ) -> Result<User> {
        let db = get_db_from_ctx(ctx)?;
        let find_one_options = FindOneOptions::builder()
            .read_concern(ReadConcern::majority())
            .build();
        let user_key = User::get_fields_serialized();

        let user = User::collection(db)
            .find_one(doc! {user_key._id: id}, find_one_options)
            .await?
            .ok_or_else(|| ApiHttpStatus::NotFound("User not found".into()).extend());

        user
    }

    pub async fn get_user(&self, ctx: &Context<'_>, user_by: UserBy) -> Result<User> {
        let db = get_db_from_ctx(ctx)?;

        User::get_user(db, user_by).await
    }

    #[graphql(guard = "AuthGuard")]
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let db = get_db_from_ctx(ctx)?;
        // let pipeline = vec![
        //     //    doc! {
        //     //       // filter on movie title:
        //     //       "$match": {
        //     //          "title": "A Star Is Born"
        //     //       }
        //     //    },
        //     doc! {
        //        // sort by year, ascending:
        //        "$sort": {
        //           "createdAt": -1
        //        }
        //     },
        // ];
        // let mut cursor = User::collection(db).aggregate(pipeline, None).await?;

        let user_keys = User::get_fields_serialized();
        let find_option = FindOptions::builder()
            .sort(doc! {user_keys.createdAt: -1})
            .build();

        User::collection(db)
            .find(None, find_option)
            .await?
            .try_collect()
            .await
            .map_err(|e| {
                // We don't want to expose our server internals to the end user.
                warn!("{e:?}");
                ApiHttpStatus::BadRequest("Could not fetch users. Try again later".into()).extend()
            })
    }

    async fn session(&self, ctx: &Context<'_>) -> Result<Session> {
        let user_id = TypedSession::from_ctx(ctx)?.get_user_id()?;
        log::info!("Successfully retrieved session for user: {user_id}");

        Ok(Session {
            expires_at: TypedSession::get_expiry(),
            user_id,
        })
    }
}

#[derive(SimpleObject, InputObject, Serialize, Deserialize)]
struct Session {
    user_id: ObjectId,
    expires_at: DateTime<Utc>,
}
