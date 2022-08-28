use redis::RedisError;

#[derive(Debug, thiserror::Error)]
pub enum OauthError {
    #[error("Failed to fetch token. Error: {0}")]
    TokenFetchFailed(String),

    #[error("Failed to fetch resource. Error: {0}")]
    ResourceFetchFailed(String),

    #[error("Authorization code not found in redirect URL: {0}")]
    AuthorizationCodeNotFoundInRedirectUrl(String),

    #[error("Csrf Token not found in redirect Url: {0}")]
    CsrfTokenNotFoundInRedirectUrl(String),

    #[error("Auth url data not found in cache")]
    AuthUrlDataNotFoundInCache,

    #[error(transparent)]
    RedisError(#[from] RedisError),

    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
}

pub(crate) type OauthResult<T> = Result<T, OauthError>;
