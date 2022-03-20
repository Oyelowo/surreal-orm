use anyhow::Context as ContextAnyhow;
use common::authentication::{
    self,
    password::{generate_password_hash, PasswordHashPHC, PasswordPlain},
    session_state::TypedSession,
};

use crate::app::error::ResolverError;

use super::{Role, SignInCredentials, SignOutMessage, User};
use async_graphql::*;
use chrono::Utc;

use mongodb::Database;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;
use wither::Model;

#[derive(Default)]
pub struct UserMutationRoot;

#[Object]
impl UserMutationRoot {
    async fn create_user(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(desc = "user data")] user_input: User,
    ) -> anyhow::Result<User> {
        user_input.validate()?;
        let db = ctx.data_unchecked::<Database>();

        let mut user = User::builder()
            .created_at(Utc::now())
            .username(user_input.username)
            .first_name(user_input.first_name)
            .last_name(user_input.last_name)
            .email(user_input.email)
            .age(user_input.age)
            .social_media(user_input.social_media)
            .roles(vec![Role::User])
            .password(user_input.password)
            .build();

        user.save(db, None).await?;

        Ok(user)
    }

    /// Creates a new user but doesn't log in the user
    /// Currently like this because of future developments
    async fn sign_up(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(desc = "Sign Up credentials")] user: User,
    ) -> FieldResult<User> {
        user.validate()?;
        let db = ctx.data_unchecked::<Database>();
        let password_hash = generate_password_hash(user.password)
            .await
            .map_err(|_| ResolverError::ServerError("Something went wrong".into()))?;

        let mut user = User::builder()
            .created_at(Utc::now())
            .username(user.username)
            .first_name(user.first_name)
            .last_name(user.last_name)
            .email(user.email)
            .age(user.age)
            .social_media(user.social_media)
            .roles(vec![Role::User])
            .password(password_hash.into())
            .build();

        user.save(db, None)
            .await
            .map_err(|_| ResolverError::BadRequest.extend())?;
        Ok(user)
    }

    // TODO: Improve all errors using error extension
    async fn sign_in(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(desc = "sign in credentials")] sign_in_credentials: SignInCredentials,
    ) -> FieldResult<User> {
        // let user = User::from_ctx(ctx)?.and_has_role(Role::Admin);
        // let user = Self::from_ctx(ctx)?.and_has_role(Role::Admin);
        let db = ctx.data_unchecked::<Database>();
        let session = ctx.data::<TypedSession>()?;

        let maybe_user_id = session
            .get_user_object_id()
            .map_err(|_| ResolverError::NotFound.extend())?;
        let k = match maybe_user_id {
            Some(ref user_id) => {
                let user = User::find_by_id(db, user_id).await;
                session.renew();
                user
            }
            None => {
                let user = User::find_by_username(db, sign_in_credentials.username)
                    .await
                    .context("Failed to find user")?;
                let plain_password = PasswordPlain::new(sign_in_credentials.password);
                let hashed_password = PasswordHashPHC::new(&user.password);

                let password_verified =
                    authentication::password::validate_password(plain_password, hashed_password)
                        .await?;

                if password_verified {
                    // let k = user.id?;
                    let id = user.id.expect("no");
                    session.insert_user_object_id(&id).expect("Failed");
                    // session.insert_user_role(user.roles).expect("Failed");
                    Ok(user)
                } else {
                    Err(ResolverError::Unauthorized.extend())
                }
            }
        };

        let p = k.expect("trttrtrt");
        Ok(p)
    }

    async fn sign_out(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<SignOutMessage> {
        // let user = User::from_ctx(ctx)?.and_has_role(Role::Admin);
        // let user = Self::from_ctx(ctx)?.and_has_role(Role::Admin);
        // let db = ctx.data_unchecked::<Database>();
        let session = ctx.data::<TypedSession>()?;

        let maybe_user = session
            .get_user_object_id()
            .map_err(|_| ResolverError::NotFound.extend())?;

        match maybe_user {
            Some(user_id) => {
                session.clear();
                Ok(SignOutMessage {
                    message: "successfully signed out".into(),
                    user_id,
                })
            }
            None => Err(ResolverError::BadRequest)
                .extend_err(|_, e| e.set("reason", "Already Logged out")),
        }
    }
}
