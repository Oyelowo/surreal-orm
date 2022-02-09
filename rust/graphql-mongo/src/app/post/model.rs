use async_graphql::*;

use mongodb::Database;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;
use wither::{
    bson::{doc, oid::ObjectId},
    prelude::Model,
};

use crate::app::user::User;

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

    // #[graphql(skip_input)] This will usually come from session/jwt token / oauth token but making it inputable in graphql for testing purposes in the meantime.
    pub poster_id: ObjectId,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub title: String,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub content: String,
}

#[ComplexObject]
impl Post {
    async fn poster(&self, ctx: &Context<'_>) -> anyhow::Result<Option<User>> {
        // TODO: Use dataloader to batch user
        let db = ctx.data_unchecked::<Database>();
        let poster = User::find_one(db, doc! {"_id": self.poster_id}, None).await?;
        Ok(poster)
    }
}

// pub type PostInput = Post;
// #[derive(InputObject, TypedBuilder)]
// pub struct PostInput {
//     pub poster_id: ObjectId,
//     pub title: String,
//     pub content: String,
// }

/*

fn validate_unique_postname(postname: &str) -> std::result::Result<(), ValidationError> {
    if postname == "xXxShad0wxXx" {
        // the value of the postname will automatically be added later
        return Err(ValidationError::new("terrible_postname"));
    }

    Ok(())
}
*/
