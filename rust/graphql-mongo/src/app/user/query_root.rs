use super::{model::User, AuthGuard};

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
    ) -> anyhow::Result<Option<User>> {
        let db = ctx.data_unchecked::<Database>();
        let find_one_options = FindOneOptions::builder()
            .read_concern(ReadConcern::majority())
            .build();

        let user = User::find_one(db, doc! {"_id": id}, find_one_options).await?;

        Ok(user)
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
}
