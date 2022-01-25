use super::{InsertUser, Role, UpdateUser, User};
use async_graphql::*;
use chrono::Utc;
use ormx::{Insert, Table};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

#[derive(Default)]
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
            created_at: Utc::now(),
            updated_at: Utc::now(),
            first_name: user_input.first_name,
            last_name: user_input.last_name,
            email: user_input.email,
            age: user_input.age,
            role: Role::User,
        };

        new_user.validate()?;

        let connection = &mut *db.acquire().await?;

        let user = new_user.insert(connection).await?;

        Ok(user)
    }

    async fn update_user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "user data")] user_input: UpdateUser,
        #[graphql(desc = "user id to update")] id: Uuid,
    ) -> anyhow::Result<User> {
        // user_input.validate()?;
        let db = ctx.data_unchecked::<PgPool>();
        let mut updated_user = UpdateUser {
            first_name: user_input.first_name,
            last_name: user_input.last_name,
        };

        updated_user.validate()?;

        // Extract user id from session or decoded token whichever way authentication is implemented
        // id = IdFromSession

        let user = User::get_by_id(db, id).await?;

        // user.set_last_login(db, value)
        // user.email = "".into;
        user.patch(db, updated_user).await?;

        log::info!("reload the user, in case it has been modified");
        user.reload(&db).await?;

        Ok(user)
    }
}
