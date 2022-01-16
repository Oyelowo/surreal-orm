// use super::query::User;
// use super::User;
// use super::query_user::User;
use async_graphql::*;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::{Validate, ValidationError};
use wither::{
    bson::{doc, oid::ObjectId},
    mongodb::Client,
    prelude::Model,
    Result,
};




#[derive(Debug, Serialize, Deserialize, TypedBuilder, Validate, Model)]
#[serde(rename_all = "camelCase")]
#[derive(SimpleObject,InputObject ,Clone)]
// #[model(index(keys=r#"doc!{"email": 1}"#, options=r#"doc!{"unique": true}"#))]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[graphql(skip)]
    pub id:  Option<ObjectId>,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    first_name: String,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    last_name: String,

    // #[builder(default, setter(strip_option))]
    #[validate(email)]
    email: String,

    #[validate(range(min = 18, max = 160))]
    #[builder(default = 20)]
    age: u8,

    // #[graphql(skip)]
    // pub family_count: i32,
}

