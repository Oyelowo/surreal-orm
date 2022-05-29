use super::{model::User, AuthGuard};

use anyhow::Context as ContextAnyhow;
use async_graphql::*;
use chrono::{DateTime, Utc};
use common::{error_handling::ApiHttpStatus, my_time};
use futures::stream::StreamExt;
use mongodb::{
    bson::oid::ObjectId,
    options::{FindOneOptions, FindOptions, ReadConcern},
    Database,
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
    ) -> FieldResult<User> {
        let db = ctx.data_unchecked::<Database>();
        let find_one_options = FindOneOptions::builder()
            .read_concern(ReadConcern::majority())
            .build();

        let user = User::find_one(db, doc! {"_id": id}, find_one_options)
            .await?
            .context("User not found")
            .map_err(|_| ApiHttpStatus::NotFound("User not found".into()).extend());

        user
    }

    #[graphql(guard = "AuthGuard")]
    async fn users(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<User>> {
        let db = ctx.data_unchecked::<Database>();
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

        // let users = User::find(db, None, None)
        //     .await?
        //     .map(|u| u.expect("coulnd not get user") )
        //     .collect::<Vec<User>>()
        //     .await;

        let find_option = FindOptions::builder().sort(doc! {"createdAt": -1}).build();

        let mut cursor = User::find(db, None, find_option).await?;

        let mut users = vec![];
        while let Some(user) = cursor.next().await {
            users.push(user?);
        }

        Ok(users)
    }

    #[graphql(guard = "AuthGuard")]
    async fn session(&self, ctx: &Context<'_>) -> FieldResult<Session> {
        let User {
            username, email, ..
        } = User::get_current_user(ctx).await?;

        Ok(Session {
            expires_at: my_time::get_session_expiry(),
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
