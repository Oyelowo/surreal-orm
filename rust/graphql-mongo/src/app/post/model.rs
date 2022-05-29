use async_graphql::*;

use mongodb::Database;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;
use wither::{
    bson::{doc, oid::ObjectId},
    prelude::Model,
};

use crate::{app::user::User, configs::MONGO_ID_KEY};

#[derive(
    Model, SimpleObject, InputObject, Clone, Serialize, Deserialize, TypedBuilder, Validate, Debug,
)]
#[serde(rename_all = "camelCase")]
#[graphql(input_name = "PostInput")]
#[graphql(complex)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[graphql(skip_input)]
    pub id: Option<ObjectId>,

    // This will usually come from session/jwt token / oauth token
    #[graphql(skip_input)]
    pub poster_id: ObjectId,

    #[validate(length(min = 1))]
    pub title: String,

    #[validate(length(min = 1))]
    pub content: String,
}

#[ComplexObject]
impl Post {
    async fn poster(&self, ctx: &Context<'_>) -> anyhow::Result<Option<User>> {
        // TODO: Use dataloader to batch user
        let db = ctx.data_unchecked::<Database>();

        let poster = User::find_one(db, doc! {MONGO_ID_KEY: self.poster_id}, None).await?;
        Ok(poster)
    }
}
