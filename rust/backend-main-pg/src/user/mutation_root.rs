use super::{InsertUser, Role, UpdateUser, User};
use async_graphql::*;
use chrono::Utc;
use ormx::{Insert, Table};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

pub struct UserMutationRoot;

#[Object]
impl UserMutationRoot {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        // #[graphql(desc = "user data")] user_input: UserInput,
        #[graphql(desc = "user data")] user_input: InsertUser,
    ) -> anyhow::Result<User> {
        // user_input.validate()?;
        let db = ctx.data_unchecked::<PgPool>();

        let mut new_user = InsertUser {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            first_name: user_input.first_name,
            last_name: user_input.last_name,
            email: user_input.email,
            role: Role::User,
            disabled: "nothing".into()
            // age: user_input.age,
        };

        new_user.validate()?;

        let connection = &mut *db.acquire().await?;

        let user = new_user.insert(db
        ).await?;

        Ok(user)
    }

    async fn update_user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "user data to update")] user_input: &UpdateUser,
        #[graphql(desc = "user id")] id: &Uuid,
    ) -> anyhow::Result<User> {
        // user_input.validate()?;
        let db = ctx.data_unchecked::<PgPool>();
        let mut updated_user = UpdateUser {
            role: Role::User,
            ..user_input
        };

        updated_user.validate()?;

        // Extract user id from session or decoded token whichever way authentication is implemented
        // id = IdFromSession

        let user = User::by_id(db, id).await?;

        user.set_last_login(db, Utc::now());
        // user.email = "".into;
        user.patch(db, updated_user).await?;

        log::info!("reload the user, in case it has been modified");
        user.reload(&db).await?;

        Ok(user)
    }
}
