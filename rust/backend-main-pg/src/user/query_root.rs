use super::model::{User};

use async_graphql::*;
use sqlx::{query_as, PgPool};
use uuid::Uuid;

#[derive(Default)]
pub struct UserQueryRoot;

#[Object]
impl UserQueryRoot {
    async fn user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the User")] id: Uuid,
    ) -> anyhow::Result<User> {
        let db = ctx.data_unchecked::<PgPool>();

        let user = User::by_id(db, &id).await?;

        Ok(user)
    }

    async fn users(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<User>> {
        let db = ctx.data_unchecked::<PgPool>();
        let users = query_as::<_, User>("SELECT * FROM users").fetch_all(db).await?;

       Ok(users)
    }
}
