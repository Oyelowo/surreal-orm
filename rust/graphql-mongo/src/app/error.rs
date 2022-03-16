use async_graphql::{ErrorExtensions, FieldError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("Could not find resource")]
    NotFound,

    #[error("Username or password is invalid")]
    InvalidCredentials,

    #[error("ServerError")]
    ServerError(String),

    #[error("No Extensions")]
    ErrorWithoutExtensions,
}

impl ErrorExtensions for ResolverError {
    // lets define our base extensions
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            ResolverError::NotFound => e.set("code", "NOT_FOUND"),
            ResolverError::InvalidCredentials => e.set("code", "INVALID_CREDENTIALS"),
            ResolverError::ServerError(reason) => e.set("reason", reason.to_string()),
            ResolverError::ErrorWithoutExtensions => {}
        })
    }
}
