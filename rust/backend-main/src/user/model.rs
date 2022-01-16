use async_graphql::*;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;
use wither::{
    bson::{doc, oid::ObjectId},
    prelude::Model,
};

#[derive(
    Model, SimpleObject, InputObject, Clone, Serialize, Deserialize, TypedBuilder, Validate, Debug,
)]
#[serde(rename_all = "camelCase")]
#[graphql(input_name = "UserInput")]
#[model(index(keys = r#"doc!{"email": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<ObjectId>,

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

pub type UserInput = User;
// #[derive(InputObject, TypedBuilder)]
// pub struct UserInput {
//     pub last_name: String,
//     pub first_name: String,
//     pub email: String,
//     pub age: u8,
// }

/*

fn validate_unique_username(username: &str) -> std::result::Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }

    Ok(())
}
*/
