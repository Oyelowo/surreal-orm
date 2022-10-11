use crate::utils::tidb::{get_pg_connection_from_ctx, get_tidb_pool_from_ctx};

use super::user;

use async_graphql::*;
use sqlx::query_as;
use uuid::Uuid;

#[derive(Default)]
pub struct UserQueryRoot;

#[Object]
impl UserQueryRoot {
    async fn user(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(desc = "id of the User")] id: Uuid,
    ) -> async_graphql::Result<user::Model> {
        let db = get_pg_connection_from_ctx(ctx)?;

        let user = user::Entity::find_by_id(id).one(db).await?.expect("msg");

        Ok(user)
    }

    async fn users(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<Vec<user::Model>> {
        let db = get_tidb_pool_from_ctx(ctx)?;
        // let users = query_as::<_, User>("SELECT * FROM users").fetch_all(db).await?;
        let users = query_as!(
            user::Model,
            r#"SELECT first_name, id, created_at, updated_at, deleted_at, username, last_name, email, age, disabled, last_login, role as "role: _" FROM users"#
        )
        .fetch_all(db)
        .await?;

        Ok(users)
    }
}
