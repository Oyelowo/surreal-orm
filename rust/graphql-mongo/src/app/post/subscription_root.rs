use async_graphql::*;
use common::error_handling::ApiHttpStatus;
use futures_util::Stream;

use crate::utils::token::Token;

#[derive(Default)]
pub struct PostSubscriptionRoot;

#[Subscription]
impl PostSubscriptionRoot {
    async fn values(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = i32>> {
        if ctx.data::<Token>()?.0 != "123456" {
            return Err(ApiHttpStatus::Forbidden("Forbidden".into()).extend());
        }
        Ok(futures_util::stream::once(async move { 10 }))
    }
}
