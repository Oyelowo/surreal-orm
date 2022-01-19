use async_graphql::*;

use chrono::{
    serde::{ts_nanoseconds, ts_nanoseconds_option},
    DateTime, Utc,
};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;
use wither::{
    bson::{doc, oid::ObjectId, Bson},
    prelude::Model,
};
// use bson::DateTime;
use crate::{configs::model_cursor_to_vec, post::Post};

#[derive(Model, SimpleObject, Serialize, Deserialize, TypedBuilder, Validate, Debug)]
// #[derive(InputObject)]
#[serde(rename_all = "camelCase")]
// #[graphql(input_name = "UserInput")]
#[graphql(complex)]
#[model(index(keys = r#"doc!{"email": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<ObjectId>,

    #[serde(with = "ts_nanoseconds_option")] // not really necessary
    #[builder(default, setter(strip_option))]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(with = "ts_nanoseconds")]
    #[builder(default=Utc::now())]
    pub updated_at: DateTime<Utc>,

    #[serde(with = "ts_nanoseconds_option")]
    #[builder(default)]
    pub deleted_at: Option<DateTime<Utc>>,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub first_name: String,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub last_name: String,

    // #[builder(default, setter(strip_option))]
    #[validate(email)]
    pub email: String,

    #[validate(range(min = 18, max = 160))]
    pub age: u8,

    #[serde(default)]
    pub social_media: Vec<String>,
}

#[ComplexObject]
impl User {
    async fn posts(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<Post>> {
        let db = ctx.data_unchecked::<Database>();
        let cursor = Post::find(db, doc! {"posterId": self.id}, None).await?;
        Ok(model_cursor_to_vec(cursor).await?)
    }
}

// pub type UserInput = User;
#[derive(InputObject, TypedBuilder)]
pub struct UserInput {
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub social_media: Vec<String>,
    pub age: u8,
}

/*

fn validate_unique_username(username: &str) -> std::result::Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }

    Ok(())
}
*/
