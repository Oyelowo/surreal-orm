use crate::configs::Shared;

use super::{Role, User};
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
        ctx: &Context<'_>,
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
            .build();

        user.save(db, None).await?;

        Ok(user)
    }

    async fn signin(&self, ctx: &Context<'_>) -> anyhow::Result<Something> {
        // let user = User::from_ctx(ctx)?.and_has_role(Role::Admin);
        // let user = Self::from_ctx(ctx)?.and_has_role(Role::Admin);
        let session = ctx
            .data::<Shared<actix_session::Session>>()
            .expect("Failed to get actix session Object");

        session.insert("user_id", "id1234").expect("poor insertion");
        let uid = session
            .get::<String>("user_id")
            .expect("Failed to get user_id session")
            .expect("Failed to get user_id session value");
        // session.get::<UserId>(Self);
        // let db = ctx.data_unchecked::<Database>();
        // let cursor = Post::find(db, doc! {"posterId": self.id}, None).await?;
        // Ok(model_cursor_to_vec(cursor).await?)
        Ok(Something {
            name: "good guy".into(),
            user_id: uid,
        })
    }
    async fn get_session(&self, ctx: &Context<'_>) -> anyhow::Result<Something> {
        // let user = User::from_ctx(ctx)?.and_has_role(Role::Admin);
        // let user = Self::from_ctx(ctx)?.and_has_role(Role::Admin);
        let session = ctx
            .data::<Shared<actix_session::Session>>()
            .expect("run it");
        let uid = session
            .get::<String>("user_id")
            .expect("Failed to get user_id session")
            .expect("Failed to get user_id session value");
        // session.get::<UserId>(Self);
        // let db = ctx.data_unchecked::<Database>();
        // let cursor = Post::find(db, doc! {"posterId": self.id}, None).await?;
        // Ok(model_cursor_to_vec(cursor).await?)
        Ok(Something {
            name: "good guy".into(),
            user_id: uid,
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
