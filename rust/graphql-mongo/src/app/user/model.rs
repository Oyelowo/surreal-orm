use async_graphql::*;
use chrono::{serde::ts_nanoseconds_option, DateTime, Utc};
use common::{authentication::TypedSession, error_handling::ApiHttpStatus};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;
use wither::{
    bson::{doc, oid::ObjectId},
    prelude::Model,
    WitherError,
};

use crate::{
    app::post::Post,
    utils::mongodb::{get_db_from_ctx, model_cursor_to_vec, MONGO_ID_KEY},
};

#[derive(
    Model, SimpleObject, InputObject, Serialize, Deserialize, TypedBuilder, Validate, Debug,
)]
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
    // make it possible do just do created_at(value) instead of created_at(Some(value)) at the call site
    // Skip only from input but available for output. Can be useful for sorting on the client side
    #[serde(with = "ts_nanoseconds_option")] // not really necessary
    #[builder(default, setter(strip_option))]
    #[graphql(skip_input)]
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

    // I intentionally not strip option here because I want
    // it to be explicit that user is not specifying password e.g when using Oauth login
    #[validate(length(min = 1))]
    #[graphql(skip_output)]
    pub password: Option<String>,

    #[validate(length(min = 1))]
    pub first_name: String,

    #[validate(length(min = 1))]
    pub last_name: String,

    #[validate(email)]
    pub email: String,

    #[graphql(skip_input)]
    pub email_verified_at: Option<DateTime<Utc>>,

    #[validate(range(min = 18, max = 160))]
    pub age: Option<u8>,

    #[serde(default)]
    pub social_media: Vec<String>,

    #[graphql(skip_input)]
    pub roles: Vec<Role>,

    #[graphql(skip_input)]
    pub accounts: Vec<AccountOauth>,
}

#[derive(InputObject, SimpleObject, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[graphql(input_name = "AccountOauthInput")]
pub struct AccountOauth {
    #[graphql(skip_input)]
    pub id: String,
    #[graphql(skip_input)]
    pub user_id: String,
    pub account_type: String,
    pub provider: String,
    pub provider_account_id: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub token_type: Option<String>, // Should probably be changed to an enum. i.e oauth | anything else?
    pub scope: Option<String>,
    pub id_token: Option<String>,
    pub session_state: Option<String>,
    pub profile: ProfileOauth,
}

#[derive(InputObject, SimpleObject, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[graphql(input_name = "ProfileOauthInput")]
pub struct ProfileOauth {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub email_verified: bool,
}

/*
EXAMPLE ON HOW TO QUERY MONGO_DB:
https://docs.mongodb.com/manual/tutorial/query-documents/
{ status: "D" }
SELECT * FROM inventory WHERE status = "D"

{ status: { $in: [ "A", "D" ] } }
SELECT * FROM inventory WHERE status in ("A", "D")

{ status: "A", qty: { $lt: 30 } }
SELECT * FROM inventory WHERE status = "A" AND qty < 30


{ $or: [ { status: "A" }, { qty: { $lt: 30 } } ] }
SELECT * FROM inventory WHERE status = "A" AND qty < 30



Specify AND as well as OR Conditions
In the following example, the compound query document selects all documents in the collection where the status equals "A" and either qty is less than ($lt) 30 or item starts with the character p:
{ status: "A", $or: [ { qty: { $lt: 30 } }, { item: /^p/ } ] }
SELECT * FROM inventory WHERE status = "A" AND ( qty < 30 OR item LIKE "p%")
*/

#[ComplexObject]
impl User {
    #[graphql(guard = "RoleGuard::new(Role::Admin).or(AuthGuard)")]
    async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        // let user = User::from_ctx(ctx)?.and_has_role(Role::Admin);
        let db = ctx.data_unchecked::<Database>();
        let cursor = Post::find(db, doc! {"posterId": self.id}, None).await?;
        model_cursor_to_vec(cursor)
            .await
            .map_err(|_| ApiHttpStatus::NotFound("Post not found".into()).extend())
    }

    async fn post_count(&self, ctx: &Context<'_>) -> Result<usize> {
        self.posts(ctx).await.map(|p| p.len()).map_err(|_| {
            ApiHttpStatus::UnprocessableEntity("Problem occured while getting posts".into())
                .extend()
        })
    }
}

#[derive(InputObject, TypedBuilder)]
pub struct SignInCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
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
            Err(ApiHttpStatus::Unauthorized(
                "You are not authourized to carry out that request.".into(),
            )
            .extend())
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
        TypedSession::from_ctx(ctx)?
            .get_user_id::<ObjectId>()
            .map(|_| ())
    }
}

impl User {
    pub async fn get_current_user(ctx: &Context<'_>) -> Result<User> {
        let db = get_db_from_ctx(ctx)?;
        let user_id = TypedSession::from_ctx(ctx)?.get_user_id::<ObjectId>()?;

        let user = Self::find_by_id(db, &user_id).await;
        user
    }

    pub async fn find_by_id(db: &Database, id: &ObjectId) -> Result<Self> {
        User::find_one(db, doc! { MONGO_ID_KEY: id }, None)
            .await?
            .ok_or_else(|| ApiHttpStatus::NotFound("User not found".into()).extend())
    }

    pub async fn find_by_username(db: &Database, username: impl Into<String>) -> Result<Self> {
        User::find_one(db, doc! { "username": username.into() }, None)
            .await?
            .ok_or_else(|| ApiHttpStatus::NotFound("User not found".into()).extend())
    }

    pub async fn _find_by_account_oauth(
        db: &Database,
        provider: impl Into<String>,
        provider_account_id: impl Into<String>,
    ) -> Result<Self> {
        User::find_one(db, doc! { "accounts": {"$elemMatch": {"provider": provider.into(), "providerAccountId": provider_account_id.into()}} }, None)
        .await?
        .ok_or_else(||ApiHttpStatus::NotFound("User not found".into()).extend())
    }

    pub async fn find_or_replace_account_oauth(
        mut self,
        db: &Database,
        provider: impl Into<String>,
        provider_account_id: impl Into<String>,
    ) -> Result<Self, WitherError> {
        let filter = doc! { "accounts": {"$elemMatch": {"provider": provider.into(), "providerAccountId": provider_account_id.into()}}};
        User::save(&mut self, db, Some(filter)).await?;

        Ok(self)
    }
}

#[derive(SimpleObject)]
pub struct SignOutMessage {
    pub message: String,
    pub user_id: ObjectId,
}
