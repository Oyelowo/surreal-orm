use async_graphql::*;
use common::error_handling::ApiHttpStatus;

use log::warn;
use mongodb::Database;

pub fn get_db_from_ctx<'a>(ctx: &'a Context<'_>) -> Result<&'a Database> {
    ctx.data::<Database>().map_err(|e| {
        warn!("{e:?}");
        ApiHttpStatus::InternalServerError(
            "Something went wrong while fetching your data. Please try again later".into(),
        )
        .extend()
    })
}
