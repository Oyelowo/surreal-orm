use super::model::User;

use async_graphql::*;
use futures::stream::StreamExt;
use mongodb::{
    bson::oid::ObjectId,
    options::{FindOneOptions, ReadConcern},
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

    async fn users(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<User>> {
        let db = ctx.data_unchecked::<Database>();
        let mut cursor = User::find(db, None, None).await?;

        let mut users = vec![];
        while let Some(user) = cursor.next().await {
            users.push(user?);
        }

        Ok(users)
    }
}
