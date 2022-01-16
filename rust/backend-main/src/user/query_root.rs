use super::model::User;

use async_graphql::*;
use futures::stream::StreamExt;
use mongodb::{
    options::{FindOneOptions, ReadConcern},
    Database, bson::oid::ObjectId,
};
use wither::{bson::doc, prelude::Model};

#[derive(Default)]
pub struct UserQueryRoot;

#[Object]
impl UserQueryRoot {
    async fn user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the User")] id: String,
    ) -> Option<User> {
        let db = ctx.data_unchecked::<Database>();
        let find_one_options = FindOneOptions::builder()
            .read_concern(ReadConcern::majority())
            .build();

        let user = User::find_one(db, doc! {"id": id}, find_one_options)
            .await
            .expect("Unable to find user");
        user
    }

    async fn users(&self, ctx: &Context<'_>) -> Vec<User> {
        let db = ctx.data_unchecked::<Database>();
        let mut cursor = User::find(db, None, None).await.expect("fdfdf");

        let mut users = vec![];
        while let Some(user) = cursor.next().await {
            println!("User...{:?}", user);
            users.push(user.unwrap());
        }

        users
    }
}
