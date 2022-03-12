use async_graphql::{Context, Result, Subscription};
use futures_util::Stream;

use crate::configs::Token;

#[derive(Default)]
pub struct UserSubscriptionRoot;

#[Subscription]
impl UserSubscriptionRoot {
    async fn values(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = i32>> {
        if ctx.data::<Token>()?.0 != "123456" {
            return Err("Forbidden".into());
        }
        Ok(futures_util::stream::once(async move { 10 }))
    }
}
