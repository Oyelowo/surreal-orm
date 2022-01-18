use async_graphql::*;

use futures::stream::StreamExt;
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
//#[graphql(input_name = "UserInput")]
#[graphql(complex)]
pub struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<ObjectId>,

    pub author_ids: Vec<ObjectId>,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub title: String,
}

#[ComplexObject]
impl Book {
    async fn authors(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<User>> {
        // TODO: Use dataloader to batch user
        let db = ctx.data_unchecked::<Database>();
        let mut cursor =
            User::find(db, doc! {"_id": { "$in": &self.author_ids}}, None).await?;

        let mut authors = vec![];
        while let Some(user) = cursor.next().await {
            authors.push(user?);
        }

        Ok(authors)
    }
}

// pub type BookInput = Book;
#[derive(InputObject, TypedBuilder)]
pub struct BookInput {
    pub author_ids: Vec<ObjectId>,
    pub title: String,
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
