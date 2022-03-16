use anyhow::{Context, Ok};
use common::authentication::{
    self,
    password::{generate_password_hash, PasswordHashPHC, PasswordPlain},
    session_state::TypedSession,
};

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
    ) -> anyhow::Result<User> {
        user.validate()?;
        let db = ctx.data_unchecked::<Database>();
        let password_hash = generate_password_hash(user.password).await?;

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

        user.save(db, None).await?;

        Ok(user)
    }

    // TODO: Improve all errors using error extension
    async fn sign_in(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(desc = "sign in credentials")] sign_in_credentials: SignInCredentials,
    ) -> anyhow::Result<User> {
        // let user = User::from_ctx(ctx)?.and_has_role(Role::Admin);
        // let user = Self::from_ctx(ctx)?.and_has_role(Role::Admin);
        let db = ctx.data_unchecked::<Database>();
        let session = ctx
            .data::<TypedSession>()
            .expect("Failed to get actix session Object");

        let maybe_user_id = session.get_user_object_id().expect("failed1");
        let k = match maybe_user_id {
            Some(ref user_id) => {
                let user = User::find_by_id(db, user_id).await;
                session.renew();
                user
            }
            None => {
                let user = User::find_by_username(db, sign_in_credentials.username)
                    .await
                    .context("User not found")?;
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
                    Some(user)
                } else {
                    None
                }
            }
        };

        let p = k.expect("trttrtrt");
        Ok(p)
    }

    async fn sign_out(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> anyhow::Result<SignOutMessage, anyhow::Error> {
        // let user = User::from_ctx(ctx)?.and_has_role(Role::Admin);
        // let user = Self::from_ctx(ctx)?.and_has_role(Role::Admin);
        // let db = ctx.data_unchecked::<Database>();
        let session = ctx
            .data::<TypedSession>()
            .expect("Failed to get actix session Object");

        let maybe_user = session
            .get_user_object_id()
            .expect("Failed to get user session. Improve error handling later");

        match maybe_user {
            Some(user_id) => {
                session.clear();
                Ok(SignOutMessage {
                    message: "successfully signed out".into(),
                    user_id,
                })
            }
            None => Err(anyhow::anyhow!("Already logged out")),
        }
    }

    async fn get_session(&self, ctx: &async_graphql::Context<'_>) -> anyhow::Result<Something> {
        // let user = User::from_ctx(ctx)?.and_has_role(Role::Admin);
        // let user = Self::from_ctx(ctx)?.and_has_role(Role::Admin);
        let session = ctx.data::<TypedSession>().expect("run it");
        let uid = session
            .get_user_object_id()
            .expect("Failed to get user_id session")
            .expect("Failed to get user_id session value");
        Ok(Something {
            name: "good guy".into(),
            user_id: uid.to_string(),
        })
    }
}

// #[serde(rename_all = "camelCase")]
// #[graphql(complex)]
// #[graphql(input_name = "UserInput")]
#[derive(SimpleObject, InputObject, Serialize, Deserialize, TypedBuilder)]
struct Something {
    user_id: String,
    name: String,
}
