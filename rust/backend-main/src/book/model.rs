use async_graphql::*;

use mongodb::Database;
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;
use typed_builder::TypedBuilder;
use validator::Validate;
use wither::{
    bson::{doc, oid::ObjectId},
    prelude::Model,
};

use crate::user::User;

#[derive(
    Model, SimpleObject, Clone, Serialize, Deserialize, TypedBuilder, Validate, Debug,
)]
#[serde(rename_all = "camelCase")]
//#[graphql(input_name = "UserInput")]
#[graphql(complex)]
pub struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<ObjectId>,

    pub author_id: ObjectId,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub title: String,
}


#[ComplexObject]
impl Book {
    async fn author(&self, ctx: &Context<'_>) -> anyhow::Result<Option<User>> {
        // TODO: Use dataloader to batch user
        let db = ctx.data_unchecked::<Database>();
        let author = User::find_one(db, doc! {"_id": self.author_id}, None).await?;
        Ok(author)
    }
}


// pub type BookInput = Book;
#[derive(InputObject, TypedBuilder)]
pub struct BookInput {
    pub author_id: ObjectId,
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
