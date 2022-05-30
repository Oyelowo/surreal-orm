use async_graphql::*;
use common::error_handling::ApiHttpStatus;
use futures::stream::StreamExt;
use log::warn;
use mongodb::Database;
use wither::{Model, ModelCursor};

pub async fn model_cursor_to_vec<T: Model>(mut cursor: ModelCursor<T>) -> Result<Vec<T>> {
    // https://doc.rust-lang.org/rust-by-example/error/iter_result.html
    // This gets all the errors out. So, will still not throw
    // if one of the items fail but will gather those failures
    // and log them as warning. This has the added advantage of returning the items(vev) directly
    // rather than a result.
    //  The alternative would be to pass all or fail  which would then return a result instead.
    // let mut errors = vec![];
    // let data = cursor
    //     .next()
    //     .await
    //     .into_iter()
    //     .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
    //     .collect::<Vec<_>>();

    // log::error!("Something went wrong while collecting iterators into vectors: {errors:?}");
    // data
    // The potential alternative which would pass all or fail all
    cursor
        .next()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| {
            warn!("{e:?}");
            ApiHttpStatus::InternalServerError("Try again later.".into()).extend()
        })
}

pub fn get_db_from_ctx<'a>(ctx: &'a Context<'_>) -> Result<&'a Database> {
    let db = ctx.data::<Database>().map_err(|e| {
        warn!("{e:?}");
        ApiHttpStatus::InternalServerError(
            "Something went wrong while fetching your data. Please try again later".into(),
        )
        .extend()
    });
    return db;
}
