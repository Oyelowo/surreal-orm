use async_graphql::*;
use chrono::{serde::ts_nanoseconds_option, DateTime, Utc};
use common::{authentication::TypedSession, error_handling::ApiHttpStatus};
use futures_util::TryStreamExt;
use mongo_helpers::{as_bson, ops};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{FindOneOptions, ReadConcern},
    Database,
};
use my_macros::FieldsGetter;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;
use wither::Model;

use super::guards::{AuthGuard, RoleGuard};
use crate::{app::post::Post, utils::mongodb::get_db_from_ctx};

#[derive(
    Model,
    SimpleObject,
    InputObject,
    Serialize,
    Deserialize,
    TypedBuilder,
    Validate,
    Debug,
    FieldsGetter,
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

    // use bson::serde_helpers::uuid_as_binary;
    // #[serde(with = "uuid_as_binary")]
    // uuid: Uuid,

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
    #[builder(default)]
    pub first_name: Option<String>,

    #[validate(length(min = 1))]
    #[builder(default)]
    pub last_name: Option<String>,

    #[validate(length(min = 1))]
    #[builder(default, setter(strip_option))]
    pub city: Option<String>,

    #[validate(email)]
    #[builder(default)]
    pub email: Option<String>,

    #[graphql(skip_input)]
    #[builder(default)]
    pub email_verified: bool,

    #[validate(range(min = 18, max = 160))]
    pub age: Option<u8>,

    #[serde(default)]
    #[builder(default)]
    pub social_media: Vec<String>,

    #[graphql(skip_input)]
    pub roles: Vec<Role>,

    #[graphql(skip_input)]
    pub accounts: Vec<AccountOauth>,
}

#[derive(
    InputObject,
    SimpleObject,
    TypedBuilder,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    FieldsGetter,
    Validate,
)]
#[serde(rename_all = "camelCase")]
#[graphql(input_name = "AccountOauthInput")]
pub struct AccountOauth {
    /// unique identifier for the oauth provider. Don't use name of user because that could be changed
    #[graphql(skip_input)]
    pub id: String,

    pub display_name: Option<String>,

    #[validate(email)]
    pub email: Option<String>,
    pub email_verified: bool,

    pub provider: OauthProvider,
    pub provider_account_id: OauthProvider,
    pub access_token: String,
    pub refresh_token: Option<String>,

    /// access token expiration timestamp, represented as the number of seconds since the epoch (January 1, 1970 00:00:00 UTC).
    pub expires_at: Option<DateTime<Utc>>,
    pub token_type: Option<TokenType>, // Should probably be changed to an enum. i.e oauth | anything else?
    pub scopes: Vec<String>,

    #[builder(default)]
    pub id_token: Option<String>,
    /* NOTE
    In case of an OAuth 1.0 provider (like Twitter), you will have to look for oauth_token and oauth_token_secret string fields. GitHub also has an extra refresh_token_expires_in integer field. You have to make sure that your database schema includes these fields.

    A single User can have multiple Accounts, but each Account can only have one User.
                 */
    #[builder(default, setter(strip_option))]
    oauth_token: Option<String>,
    #[builder(default, setter(strip_option))]
    oauth_token_secret: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Enum, PartialEq, Eq, Copy, Hash)]
#[serde(rename_all = "lowercase")]
pub enum OauthProvider {
    Github,
    Google,
}

#[derive(Debug, Deserialize, Serialize, Clone, Enum, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Bearer,
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
    #[graphql(guard = "RoleGuard::new(Role::User).or(AuthGuard)")]
    async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        // let user = User::from_ctx(ctx)?.and_has_role(Role::Admin);
        let db = get_db_from_ctx(ctx)?;
        let post_fields = Post::get_fields_serialized();

        Post::collection(db)
            .find(doc! {post_fields.posterId: self.id}, None)
            .await?
            .try_collect()
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

#[derive(OneofObject)]
pub enum UserBy {
    UserId(ObjectId),
    Username(String),
    Address(Address),
    // #[validate(email)]
    Email(String),
}
#[derive(InputObject, Validate)]
pub struct Address {
    street: String,
    house_number: String,
    city: String,
    zip: String,
}

impl User {
    pub async fn get_current_user(ctx: &Context<'_>) -> Result<User> {
        let db = get_db_from_ctx(ctx)?;
        let user_id = TypedSession::from_ctx(ctx)?.get_user_id::<ObjectId>()?;

        let user = Self::find_by_id(db, &user_id).await;
        user
    }

    pub async fn get_user(db: &Database, user_by: UserBy) -> Result<Self> {
        let uf = User::get_fields_serialized();
        let search_doc = match user_by {
            UserBy::UserId(id) => doc! { uf._id: id },
            UserBy::Username(user_name) => {
                let doc = doc! { uf.username: user_name };
                doc
            }
            // Temporary
            UserBy::Address(address) => doc! { uf.city: address.city },
            UserBy::Email(email) => doc! { uf.email: email },
        };
        // todo!()
        // User::find_one(db, search_doc, None)
        //     .await?
        //     .ok_or_else(|| ApiHttpStatus::NotFound("User not found".into()).extend())
        User::collection(db)
            .find_one(search_doc, None)
            .await?
            .ok_or_else(|| ApiHttpStatus::NotFound("User not found".into()).extend())
    }
    // pub async fn _search_users(db: &Database, user_by: Vec<UserBy>) -> Result<Vec<Self>> {
    //     todo!()
    // }
    pub async fn find_by_id(db: &Database, id: &ObjectId) -> Result<Self> {
        let uk = User::get_fields_serialized();
        let find_one_options = FindOneOptions::builder()
            .read_concern(ReadConcern::majority())
            .build();

        User::collection(db)
            .find_one(doc! {uk._id: id}, find_one_options)
            .await?
            .ok_or_else(|| ApiHttpStatus::NotFound("User not found".into()).extend())
    }

    pub async fn find_or_create_for_oauth(
        db: &Database,
        account_oauth: AccountOauth,
    ) -> anyhow::Result<Self> {
        let user_fields = User::get_fields_serialized();
        let AccountOauth { id, provider, .. } = account_oauth.clone();

        let autogenerated_id = uuid::Uuid::new_v4().to_string();
        let mut user = User::builder()
            .username(format!("{}-{}", id, autogenerated_id))
            .roles(vec![Role::User])
            .accounts(vec![account_oauth])
            .age(None)
            .password(None)
            .build();

        let acc_fields = AccountOauth::get_fields_serialized();

        let check_for_existing_account = doc! {
            user_fields.accounts:
            {
                ops::ElemMatch : {
                    acc_fields.id: id,
                    acc_fields.provider: as_bson(&provider)
                }
            }
        };

        user.save(db, Some(check_for_existing_account)).await?;

        Ok(user)
    }

    pub async fn find_by_username(db: &Database, username: impl Into<String>) -> Result<Self> {
        let uk = User::get_fields_serialized();
        User::collection(db)
            .find_one(doc! { uk.username: username.into() }, None)
            .await?
            .ok_or_else(|| ApiHttpStatus::NotFound("User not found".into()).extend())
    }
}

#[derive(SimpleObject)]
pub struct SignOutMessage {
    pub message: String,
    pub user_id: ObjectId,
}
