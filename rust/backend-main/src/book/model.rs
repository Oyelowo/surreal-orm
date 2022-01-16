#![warn(unused_imports)]
use std::fmt::Display;

use actix_web::{guard, web, App, HttpServer};

mod configs;

use anyhow::Context;
use configs::{index, index_playground, Configs, GraphQlApp};

use futures::stream::StreamExt;
use mongodb::options::{FindOneOptions, FindOptions, ReadConcern};
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
// #[model(index(keys=r#"doc!{"email": 1}"#, options=r#"doc!{"unique": true}"#))]
struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<ObjectId>,

    #[validate(length(min = 1), custom = "validate_unique_username")]
    first_name: String,
    title: String,
    author: String,

    // #[builder(default, setter(strip_option))]
    #[validate(email)]
    email: String,

    #[validate(range(min = 18, max = 50))]
    #[builder(default = 20)]
    age: u32,
}