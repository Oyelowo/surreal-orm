use async_graphql::{Context, ErrorExtensions};
use common::error_handling::ApiHttpStatus;
use log::warn;
use sea_orm::DatabaseConnection;
use sqlx::MySqlPool;

pub fn get_tidb_pool_from_ctx<'a>(ctx: &'a Context<'_>) -> async_graphql::Result<&'a MySqlPool> {
    ctx.data::<MySqlPool>().map_err(|e| {
        warn!("{e:?}");
        ApiHttpStatus::InternalServerError(
            "Something went wrong while fetching your data. Please try again later".into(),
        )
        .extend()
    })
}
pub fn get_tidb_connection_from_ctx<'a>(
    ctx: &'a Context<'_>,
) -> async_graphql::Result<&'a DatabaseConnection> {
    ctx.data::<DatabaseConnection>().map_err(|e| {
        warn!("{e:?}");
        ApiHttpStatus::InternalServerError(
            "Something went wrong while fetching your data. Please try again later".into(),
        )
        .extend()
    })
}
