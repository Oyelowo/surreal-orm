use async_graphql::*;

use mongodb::Database;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;
use wither::{
    bson::{doc, oid::ObjectId},
    prelude::Model,
};

use crate::user::User;

#[derive(Model, SimpleObject, Clone, Serialize, Deserialize, TypedBuilder, Validate, Debug)]
#[serde(rename_all = "camelCase")]
//#[graphql(input_name = "PostInput")]
#[graphql(complex)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<ObjectId>,

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

// pub type PostInput = Book;
#[derive(InputObject, TypedBuilder)]
pub struct PostInput {
    pub poster_id: ObjectId,
    pub title: String,
    pub content: String,
}

/*

fn validate_unique_bookname(bookname: &str) -> std::result::Result<(), ValidationError> {
    if bookname == "xXxShad0wxXx" {
        // the value of the bookname will automatically be added later
        return Err(ValidationError::new("terrible_bookname"));
    }

    Ok(())
}
*/
