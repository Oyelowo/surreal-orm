use async_graphql::{ErrorExtensions, FieldError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("Could not find resource")]
    NotFound,

    #[error("Username or password is invalid")]
    InvalidCredentials,

    #[error("Unauthorized: The client must authenticate itself to get the requested response")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Bad Request. Something went wrong")]
    BadRequest,

    #[error("ServerError")]
    ServerError(String),
    // #[error("No Extensions")]
    // ErrorWithoutExtensions,
}

impl ErrorExtensions for ResolverError {
    // lets define our base extensions
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            ResolverError::NotFound => e.set("code", "NOT_FOUND"),
            ResolverError::InvalidCredentials => e.set("code", "INVALID_CREDENTIALS"),
            ResolverError::Unauthorized => e.set("code", "UNAUTHORIZED"),
            ResolverError::Forbidden => e.set("code", "401"),
            ResolverError::BadRequest => e.set("code", "400: BAD_REQUEST"),
            ResolverError::ServerError(reason) => e.set("reason", reason.to_string()),
            // ResolverError::ErrorWithoutExtensions => {}
        })
    }
}
