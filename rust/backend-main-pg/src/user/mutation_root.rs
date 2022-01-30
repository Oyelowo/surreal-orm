use super::{CreateUserInput, InsertUser, Role, UpdateUserInput, User};
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
        #[graphql(desc = "user data")] user_input: CreateUserInput,
    ) -> anyhow::Result<User> {
        user_input.validate()?;
        let db = ctx.data_unchecked::<PgPool>();

        let new_user = InsertUser {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            username: user_input.username,
            first_name: user_input.first_name,
            last_name: user_input.last_name,
            email: user_input.email,
            role: Role::User,
            age: user_input.age,
            disabled: Some("nothing".into()), // age: user_input.age,
        };

        // This is necessary because ormx currently uses two transactions to enable insertion and selection
        // of latest inserted row for MySQL cos MySQL does not currently support returning from latest inserted
        // within a query like POSTGRES does. Thus, we need to require the connection for the pool for this second
        // selection even though we are not using MySQL. Until this issue is worked around...
        // It might be possible still achieve this within a transaction in MySQL tho.
        // Check the link for more info.
        // https://github.com/NyxCode/ormx/issues/22
        let connection = &mut *db.acquire().await?;

        let user = new_user.insert(connection).await?;

        Ok(user)
    }

    async fn update_user(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        user_input: UpdateUserInput,
    ) -> anyhow::Result<User> {
        // user_input.validate()?;
        let db = ctx.data_unchecked::<PgPool>();

        user_input.validate()?;

        // Extract user id from session or decoded token whichever way authentication is implemented
        // id = IdFromSession

        let mut user = User::by_id(db, &id).await?;

        user.set_last_login(db, Some(Utc::now())).await?;
        // user.email = "".into;
        user.patch(db, user_input).await?;

        log::info!("reload the user, in case it has been modified");
        user.reload(db).await?;

        Ok(user)
    }
}
