use async_graphql::*;

// use common::error_handling::ApiHttpStatus;
use my_macros::FieldsGetter;
use serde::{Deserialize, Serialize};
// use surrealdb::sql::Uuid;
use typed_builder::TypedBuilder;
use validator::Validate;

use crate::app::user::User;

#[derive(
    FieldsGetter,
    SimpleObject,
    InputObject,
    Clone,
    Serialize,
    Deserialize,
    TypedBuilder,
    Validate,
    Debug,
)]
#[serde(rename_all = "camelCase")]
#[graphql(input_name = "PostInput")]
#[graphql(complex)]
pub struct Post {
    #[builder(default)]
    #[graphql(skip_input)]
    pub id: Option<uuid::Uuid>,

    // This will usually come from session/jwt token / oauth token
    #[graphql(skip_input)]
    pub poster_id: uuid::Uuid,

    #[validate(length(min = 1))]
    pub title: String,

    #[validate(length(min = 1))]
    pub content: String,
}

#[ComplexObject]
impl Post {
    async fn poster(&self, ctx: &Context<'_>) -> Result<User> {
        // let db = get_db_from_ctx(ctx)?;
        let post_keys = Post::get_fields_serialized();
        todo!()
    }
}
