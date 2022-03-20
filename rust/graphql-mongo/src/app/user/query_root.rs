use crate::app::error::ResolverError;

use super::{model::User, AuthGuard};

use anyhow::Context as ContextAnyhow;
use async_graphql::*;
use futures::stream::StreamExt;
use mongodb::{
    bson::oid::ObjectId,
    options::{FindOneOptions, FindOptions, ReadConcern},
    Database,
};
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
            .map_err(|e| ResolverError::NotFound.extend());

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
        // let user = User::from_ctx(ctx)?.and_has_role(Role::Admin);
        // let user = Self::from_ctx(ctx)?.and_has_role(Role::Admin);
        // let session = ctx
        //     .data::<TypedSession>()
        //     .map_err(|_| ResolverError::ServerError("Failed to get session".into()))?;
        // let uid = User::from_ctx(ctx);

        let User {
            username, email, ..
        } = User::get_current_user(ctx).await?;
        // let user = User::from_ctx(ctx)?;
        // let username = user.username.clone();
        // let email = user.email.clone();
        Ok(Session {
            expires_in: "uid".to_string(),
            user: SessionUser {
                name: username.into(),
                email: email.into(),
                image: "imageurl.com".into(),
            },
        })
    }
}

// #[serde(rename_all = "camelCase")]
// #[graphql(complex)]
// #[graphql(input_name = "UserInput")]
#[derive(SimpleObject, InputObject)]
struct Session {
    user: SessionUser,
    expires_in: String,
}
#[derive(SimpleObject, InputObject)]
struct SessionUser {
    name: String,
    email: String,
    image: String,
}

/*
{
    user?: {
        name?: string | null;
        email?: string | null;
        image?: string | null;
    };
    expires: ISODateString;
}
*/
