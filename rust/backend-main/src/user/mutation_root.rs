use super::{User, UserInput};
use async_graphql::*;
use chrono::Utc;
use mongodb::Database;
use validator::Validate;
use wither::Model;

#[derive(Default)]
pub struct UserMutationRoot;

#[Object]
impl UserMutationRoot {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "user data")] user_input: UserInput,
    ) -> anyhow::Result<User> {
        // user_input.validate()?;
        let db = ctx.data_unchecked::<Database>();
        let mut user = User::builder()
            .created_at(Utc::now())
            .first_name(user_input.first_name)
            .last_name(user_input.last_name)
            .email(user_input.email)
            .age(user_input.age)
            .social_media(user_input.social_media)
            .build();
        // let mut user = User { ..user_input };
        user.validate()?;

        user.save(db, None).await?;
        Ok(user)
    }
}
