use super::model::User;

use async_graphql::*;
use futures::stream::StreamExt;
use mongodb::{
    options::{FindOneOptions, FindOptions, ReadConcern},
    Database,
};
use wither::{
    bson::{doc, oid::ObjectId},
    mongodb::Client,
    prelude::Model,
    ModelCursor, Result,
};

#[derive(Default)]
pub struct UserQueryRoot;

#[Object]
impl UserQueryRoot {
    async fn user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the droid")] id: i32,
    ) -> Option<User> {
        let db = ctx.data_unchecked::<Database>();
        let k = User::find_one(db, None, None).await.expect("fdfdf");
        k
    }

    async fn users(&self, ctx: &Context<'_>) -> Vec<User> {
        let db = ctx.data_unchecked::<Database>();
        User::sync(db).await.unwrap();
        // let mut cursor = User::find(db, None, None).await.expect("fdfdf");
        let mut cursor = User::find(db, None, None).await.expect("fdfdf");

        let mut users = vec![];
        for user in cursor.next().await {
            users.push(user.expect("not working users"));
        }
        users
        //     let mut users = vec![];
        //     while let Some(user) = cursor.next().await {
        //         println!("User...{:?}", user);
        //         users.push(user.unwrap());
        //     }

        //     users
    }
}
