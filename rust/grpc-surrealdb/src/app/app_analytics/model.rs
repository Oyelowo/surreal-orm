use my_macros::FieldsGetter;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;
use validator::Validate;

// rename to appuserevent
#[derive(Clone, Serialize, Deserialize, TypedBuilder, Validate, Debug, FieldsGetter)]
#[serde(rename_all = "camelCase")]
pub struct UserAppEvent {
    #[builder(default)]
    pub id: Option<Uuid>,

    // #[validate(length(min = 1), /*custom = "validate_is_id"*/)]
    pub user_id: Uuid,

    #[validate(length(min = 1))]
    pub page: String,

    #[validate(length(min = 1))]
    pub event_name: String,

    pub description: String,
}
