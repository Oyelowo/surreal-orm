use anyhow::Context as ErrorContext;
use async_graphql::{
    ComplexObject, Context, Enum, ErrorExtensions, FieldResult, Guard, InputObject, SimpleObject,
};
use chrono::{serde::ts_nanoseconds_option, DateTime, Utc};
use common::authentication::session_state::TypedSession;
use log::info;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;
use wither::{
    bson::{doc, oid::ObjectId},
    prelude::Model,
    WitherError,
};
// use bson::DateTime;

use crate::{
    app::{error::ResolverError, post::Post},
    configs::model_cursor_to_vec,
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

    // I intentionally not strip option here because I want it to be explicit that user is not specifying password
    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    #[graphql(skip_output)]
    pub password: Option<String>,

    // #[builder(default, setter(strip_option))]
    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub first_name: String,

    // #[builder(default, setter(strip_option))]
    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    pub last_name: String,

    // #[builder(default, setter(strip_option))]
    #[validate(email)]
    pub email: String,

    // #[builder(default, setter(strip_option))]
    #[graphql(skip_input)]
    pub email_verified_at: Option<DateTime<Utc>>,

    #[validate(range(min = 18, max = 160))]
    pub age: Option<u8>,

    #[serde(default)]
    pub social_media: Vec<String>,

    // #[serde(default)]
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
    async fn posts(&self, ctx: &Context<'_>) -> FieldResult<Vec<Post>> {
        // AuthGuard
        // let user = User::from_ctx(ctx)?.and_has_role(Role::Admin);
        let db = ctx.data_unchecked::<Database>();
        let cursor = Post::find(db, doc! {"posterId": self.id}, None).await?;
        Ok(model_cursor_to_vec(cursor).await?)
    }
    async fn post_count(&self, ctx: &Context<'_>) -> FieldResult<usize> {
        let post_count = self.posts(ctx).await?.len();
        Ok(post_count)
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
    async fn check(&self, ctx: &Context<'_>) -> FieldResult<()> {
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
    async fn check(&self, ctx: &Context<'_>) -> FieldResult<()> {
        let session = ctx.data::<TypedSession>()?;

        let maybe_user_id = session
            .get_user_object_id()
            .map_err(|_e| ResolverError::InvalidCredentials.extend())?;

        if maybe_user_id.is_some() {
            info!("Successfully authenticated: {:?}", maybe_user_id);
            Ok(())
        } else {
            Err(ResolverError::Forbidden.extend())
        }
    }
}

impl User {
    pub async fn get_current_user(ctx: &Context<'_>) -> FieldResult<User> {
        let session = ctx.data::<TypedSession>()?;
        let db = ctx.data::<Database>()?;

        let user_id = session
            .get_user_object_id()
            .map_err(|_| ResolverError::NotFound.extend())?
            .ok_or(ResolverError::NotFound.extend())?;

        let user = Self::find_by_id(db, &user_id).await;
        user
    }
    pub fn and_has_role(&self, scope: Role) -> FieldResult<&Self> {
        if !self.roles.contains(&scope) {
            return Err(ResolverError::Unauthorized.extend());
        }
        Ok(self)
    }

    // pub fn from_ctx<'a>(ctx: &'a Context) -> FieldResult<&'a Self> {
    //     ctx.data::<User>()
    //         .map_err(|_| ResolverError::NotFound.extend())
    // }

    //TODO: Better error handling
    pub async fn find_by_id(db: &Database, id: &ObjectId) -> FieldResult<Self> {
        User::find_one(&db, doc! { "_id": id }, None)
            .await?
            .context("Failed to find user")
            .map_err(|_e| ResolverError::NotFound.extend())
    }

    pub async fn find_by_username(db: &Database, username: impl Into<String>) -> Option<Self> {
        User::find_one(&db, doc! { "username": username.into() }, None)
            .await
            .expect("Failed to find user by username")
    }
    pub async fn find_by_account_oauth(
        db: &Database,
        provider: impl Into<String>,
        provider_account_id: impl Into<String>,
    ) -> Option<Self> {
        User::find_one(&db, doc! { "accounts": {"$elemMatch": {"provider": provider.into(), "providerAccountId": provider_account_id.into()}} }, None)
            .await
            .expect("Failed to find user by username")
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

#[derive(SimpleObject)]
pub struct SignOutMessage {
    pub message: String,
    pub user_id: ObjectId,
}

/*
datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
  shadowDatabaseUrl = env("SHADOW_DATABASE_URL") // Only needed when using a cloud provider that doesn't support the creation of new databases, like Heroku. Learn more: https://pris.ly/migrate-shadow
}

generator client {
  provider        = "prisma-client-js"
  previewFeatures = ["referentialActions"] // You won't need this in Prisma 3.X or higher.
}

model Account {
  id                 String  @id @default(cuid())
  userId             String
  type               String
  provider           String
  providerAccountId  String
  refresh_token      String?  @db.Text
  access_token       String?  @db.Text
  expires_at         Int?
  token_type         String?
  scope              String?
  id_token           String?  @db.Text
  session_state      String?

  user User @relation(fields: [userId], references: [id], onDelete: Cascade)

  @@unique([provider, providerAccountId])
}

model Session {
  id           String   @id @default(cuid())
  sessionToken String   @unique
  userId       String
  expires      DateTime
  user         User     @relation(fields: [userId], references: [id], onDelete: Cascade)
}

model User {
  id            String    @id @default(cuid())
  name          String?
  email         String?   @unique
  emailVerified DateTime?
  image         String?
  accounts      Account[]
  sessions      Session[]
}

model VerificationToken {
  identifier String
  token      String   @unique
  expires    DateTime

  @@unique([identifier, token])
}
*/
