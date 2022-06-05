use my_macros::KeyNamesGetter;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;
use validator::Validate;
use wither::{
    bson::{doc, oid::ObjectId},
    prelude::Model,
};

// rename to appuserevent
#[derive(Model, Clone, Serialize, Deserialize, TypedBuilder, Validate, Debug, KeyNamesGetter)]
#[serde(rename_all = "camelCase")]
pub struct UserAppEvent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<ObjectId>,

    // #[validate(length(min = 1), /*custom = "validate_is_id"*/)]
    pub user_id: Uuid,

    #[validate(length(min = 1))]
    pub page: String,

    #[validate(length(min = 1))]
    pub event_name: String,

    pub description: String,
}
