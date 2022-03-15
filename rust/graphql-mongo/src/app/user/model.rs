use async_graphql::*;
use chrono::{serde::ts_nanoseconds_option, DateTime, Utc};
use common::authentication::session_state::TypedSession;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;
use wither::{
    bson::{doc, oid::ObjectId},
    prelude::Model,
};
// use bson::DateTime;

use crate::{
    app::{post::Post, AppError},
    configs::model_cursor_to_vec,
};

#[derive(Model, SimpleObject, InputObject, Serialize, Deserialize, TypedBuilder, Validate)]
#[serde(rename_all = "camelCase")]
#[graphql(complex)]
#[graphql(input_name = "UserInput")]
#[model(index(keys = r#"doc!{"username": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[graphql(skip_input)]
    pub id: Option<ObjectId>,

    // Created_at should only be set once when creating the field, it should be ignored at other times
    #[serde(with = "ts_nanoseconds_option")] // not really necessary
    #[builder(default, setter(strip_option))]
    // make it possible do just do created_at(value) instead of created_at(Some(value)) at the call site
    #[graphql(skip_input)]
    // Skip only from input but available for output. Can be useful for sorting on the client side
    pub created_at: Option<DateTime<Utc>>,

    #[serde(with = "ts_nanoseconds_option")]
    #[builder(default=Some(Utc::now()), setter(strip_option))]
    #[graphql(skip)] // skip from noth input and output. Mainly for business logic stuff
    pub updated_at: Option<DateTime<Utc>>,

    #[serde(with = "ts_nanoseconds_option")]
    #[builder(default, setter(strip_option))]
    #[graphql(skip)] // skip from both input and output. Mainly for business logic stuff
    pub deleted_at: Option<DateTime<Utc>>,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub username: String,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    #[graphql(skip_output)]
    pub password: String,

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

    // #[serde(default)]
    #[graphql(skip_input)]
    pub roles: Vec<Role>,
}

#[ComplexObject]
impl User {
    async fn posts(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<Post>> {
        // AuthGuard
        // let user = User::from_ctx(ctx)?.and_has_role(Role::Admin);
        let db = ctx.data_unchecked::<Database>();
        let cursor = Post::find(db, doc! {"posterId": self.id}, None).await?;
        Ok(model_cursor_to_vec(cursor).await?)
    }
    async fn post_count(&self, ctx: &Context<'_>) -> anyhow::Result<usize> {
        let post_count = self.posts(ctx).await?.len();
        Ok(post_count)
    }
}

#[derive(InputObject, TypedBuilder)]
pub struct SignInInput {
    pub username: String,
    pub password: String,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
}

struct RoleGuard {
    role: Role,
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if ctx.data_opt::<Role>() == Some(&self.role) {
            Ok(())
        } else {
            Err("Forbidden".into())
        }
    }
}

impl RoleGuard {
    fn new(role: Role) -> Self {
        Self { role }
    }
}

pub struct AuthGuard;

#[async_trait::async_trait]
impl Guard for AuthGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let session = ctx
            .data::<TypedSession>()
            .expect("Failed to get actix session Object");

        let maybe_user_id = session.get_user_object_id().expect("failed1");
        if maybe_user_id.is_some() {
            Ok(())
        } else {
            Err("Forbidden".into())
        }
    }
}

impl User {
    pub fn and_has_role(&self, scope: Role) -> anyhow::Result<&Self, AppError> {
        if !self.roles.contains(&scope) {
            return Err(AppError::Forbidden(anyhow::anyhow!(
                "Does not have required permission"
            )));
        }
        Ok(self)
    }

    pub fn from_ctx<'a>(ctx: &'a Context) -> anyhow::Result<&'a Self, AppError> {
        let k = ctx
            .data::<User>()
            .map_err(|e| AppError::Forbidden(anyhow::anyhow!(e.message)));
        k
    }

    //TODO: Better error handling
    pub async fn find_by_id(db: &Database, id: &ObjectId) -> Option<Self> {
        User::find_one(&db, doc! { "_id": id }, None)
            .await
            .expect("Failed to find user by id")
    }

    pub async fn find_by_username(db: &Database, username: impl Into<String>) -> Option<Self> {
        User::find_one(&db, doc! { "username": username.into() }, None)
            .await
            .expect("Failed to find user by username")
    }
}

// No need to redefine. The simpleobject doubles as InputObject. This can still be used for other inputs if need be.
// // pub type UserInput = User;
// #[derive(InputObject, TypedBuilder)]
// pub struct UserInput {
//     pub last_name: String,
//     pub first_name: String,
//     pub email: String,
//     pub social_media: Vec<String>,
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
