use crate::configs::{get_db_from_ctx, model_cursor_to_vec, MONGO_ID_KEY};

use super::{model::User, AuthGuard};

use async_graphql::*;
use chrono::{DateTime, Utc};
use common::{authentication::TypedSession, error_handling::ApiHttpStatus};

use mongodb::{
    bson::oid::ObjectId,
    options::{FindOneOptions, FindOptions, ReadConcern},
};
use serde::{Deserialize, Serialize};
use wither::{bson::doc, prelude::Model};

#[derive(Default)]
pub struct UserQueryRoot;

#[Object]
impl UserQueryRoot {
    async fn user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the User")] id: ObjectId,
    ) -> Result<User> {
        let db = get_db_from_ctx(ctx)?;
        let find_one_options = FindOneOptions::builder()
            .read_concern(ReadConcern::majority())
            .build();

        let user = User::find_one(db, doc! {MONGO_ID_KEY: id}, find_one_options)
            .await?
            .ok_or_else(|| ApiHttpStatus::NotFound("User not found".into()).extend());

        user
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

        let find_option = FindOptions::builder().sort(doc! {"createdAt": -1}).build();

        let cursor = User::find(db, None, find_option).await?;
        model_cursor_to_vec(cursor).await
    }

    #[graphql(guard = "AuthGuard")]
    async fn session(&self, ctx: &Context<'_>) -> Result<Session> {
        let User {
            username, email, ..
        } = User::get_current_user(ctx).await?;

        Ok(Session {
            expires_at: TypedSession::get_expiry(),
            user: SessionUser {
                name: username,
                email,
                image: "imageurl.com".into(),
            },
        })
    }
}

#[derive(SimpleObject, InputObject, Serialize, Deserialize)]
struct Session {
    user: SessionUser,
    expires_at: DateTime<Utc>,
}
#[derive(SimpleObject, InputObject, Serialize, Deserialize)]
struct SessionUser {
    name: String,
    email: String,
    image: String,
}
