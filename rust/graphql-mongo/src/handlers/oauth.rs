use std::fmt::Display;

use mongodb::Database;
use poem::error::{BadRequest, InternalServerError};
use poem::web::{Data, Json};
use poem::{error::Result, handler, http::Uri, web::Path};
use poem::{IntoResponse, Response};
use redis::RedisError;
use reqwest::{header, StatusCode};
use url::Url;
use wither::Model;

use crate::oauth::github::GithubConfig;
use crate::oauth::utils::{OauthError, OauthProviderTrait, RedirectUrlReturned};
use common::configurations::redis::{RedisConfigError, RedisConfigs};

use crate::app::user::{OauthProvider, User};

#[derive(Debug, thiserror::Error)]
pub(crate) enum HandlerError {
    #[error("Server error. Please try again")]
    StorageError(#[source] OauthError),

    #[error("Server error. Unable to retrieve code. Please try again")]
    MalformedState(#[source] OauthError),

    #[error("The state token provided is invalid.")]
    InvalidState(#[source] OauthError),

    #[error("Problem fetching account")]
    FetchAccountFailed(#[source] OauthError),

    #[error("Server error. Try again laater")]
    RedisError(#[from] RedisError),

    #[error("Server error. Try again laater")]
    RedisConfigError(#[from] RedisConfigError),

    #[error("Malformed data. Try again laater")]
    SerializationError(#[from] serde_json::Error),

    #[error("Malformed url. Try again laater")]
    ParseError(#[from] url::ParseError),
}

async fn get_redis_connection(
    redis: Data<&RedisConfigs>,
) -> Result<redis::aio::Connection, poem::Error> {
    redis
        .clone()
        .get_async_connection()
        .await
        // First transform message to client message. So we dont expose server error to client
        .map_err(HandlerError::RedisConfigError)
        .map_err(InternalServerError)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Redirect {
    status: StatusCode,
    uri: String,
}

impl Redirect {
    /// A simple `302` redirect to a different location.
    pub fn found(uri: impl Display) -> Self {
        Self {
            status: StatusCode::FOUND,
            uri: uri.to_string(),
        }
    }
}

impl IntoResponse for Redirect {
    fn into_response(self) -> Response {
        self.status
            .with_header(header::LOCATION, self.uri)
            .into_response()
    }
}

#[handler]
pub async fn oauth_login_initiator(
    Path(oauth_provider): Path<OauthProvider>,
    redis: Data<&RedisConfigs>,
) -> Result<Redirect> {
    let mut connection = get_redis_connection(redis).await?;

    let auth_url_data = match oauth_provider {
        OauthProvider::Github => GithubConfig::new().generate_auth_url(),
        OauthProvider::Google => todo!(),
    };

    auth_url_data
        .csrf_state
        .cache(oauth_provider, &mut connection)
        .await
        .map_err(HandlerError::StorageError)
        .map_err(InternalServerError)?;

    Ok(Redirect::found(auth_url_data.authorize_url))
}

// TODO: Handle failure redirect cases. Pack all the logic into a function and redirect if error returned.
#[handler]
pub async fn oauth_login_authentication(
    uri: &Uri,
    db: Data<&Database>,
    redis: Data<&RedisConfigs>,
) -> Result<Redirect> {
    let mut connection = get_redis_connection(redis).await?;

    let redirect_url = Url::parse(&format!("http://localhost:{uri}"))
        .map_err(HandlerError::ParseError)
        .map_err(InternalServerError)?;

    let redirect_url = RedirectUrlReturned(redirect_url);
    let code = redirect_url.get_authorization_code().map_err(BadRequest)?;
    // make .verify give me back both the csrf token and the provider
    let provider = redirect_url
        .get_csrf_state()
        .map_err(HandlerError::MalformedState)
        .map_err(BadRequest)?
        .verify(&mut connection)
        .await
        .map_err(HandlerError::InvalidState)
        .map_err(BadRequest)?;

    let user = match provider {
        OauthProvider::Github => {
            let github_config = GithubConfig::new();

            github_config
                .fetch_oauth_account(code)
                .await
                .map_err(HandlerError::FetchAccountFailed)
                .map_err(BadRequest)
        }
        OauthProvider::Google => todo!(),
    };
    user?
        .find_or_create_for_oauth(&db)
        .await
        .map_err(BadRequest)?;

    //  Also, handle storing user session
    // Ok(Json(user?))
    Ok(Redirect::found("http://localhost:8000"))
}
