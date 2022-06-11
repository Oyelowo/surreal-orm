use async_graphql::ErrorExtensions;
use common::error_handling::ApiHttpStatus;
use derive_more::From;
use log::warn;
use poem::http::HeaderMap;

#[derive(From, PartialEq)]
pub struct Token(pub String);

impl Token {
    pub fn from_ctx<'a>(ctx: &'a async_graphql::Context<'_>) -> async_graphql::Result<&'a Self> {
        return ctx.data::<Self>().map_err(|e| {
            warn!("{e:?}");
            ApiHttpStatus::InternalServerError("Something went wrong while getting session".into())
                .extend()
        });
    }

    pub fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
        // This should probably include some validations
        // of the token and its expiry date and maybe refreshing the token or something
        headers
            .get("Token")
            .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
    }
}
