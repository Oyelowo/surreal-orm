use std::fmt::Display;

use anyhow::Context;
use bson::oid::ObjectId;
use common::authentication::TypedSession;
use envy::Error;
use mongodb::Database;
use poem::error::{BadRequest, InternalServerError};
use poem::session::Session;
use poem::web::{Data, Json};
use poem::{error::Result, handler, http::Uri, web::Path};
use poem::{IntoResponse, Response};
use redis::RedisError;
use reqwest::{header, StatusCode};
use url::Url;
use wither::Model;

use crate::oauth::github::GithubConfig;
use crate::oauth::google::GoogleConfig;
use crate::oauth::utils::{
    CsrfState, OauthConfigTrait, OauthError, OauthProviderTrait, RedirectUrlReturned,
};
use common::configurations::redis::RedisConfigError;

use crate::app::user::{OauthProvider, User};

/// These are created to map internral error message that we
/// only want to expose as logs for debugging to messages we
/// would want to show to the client/frontend.
/// Otherwise, we could have mapped directly. We could also use poem's
/// custom error but that feels a little verbose/overkill
#[derive(Debug, thiserror::Error)]
pub(crate) enum HandlerError {
    #[error("Server error. Please try again")]
    StorageError(#[source] OauthError),

    #[error("Server error. Unable to retrieve code. Please try again")]
    MalformedState(#[source] OauthError),

    #[error("The state token provided is invalid.")]
    InvalidState(#[source] OauthError),

    #[error("The auth code provided is invalid.")]
    InvalidAuthCode(#[source] OauthError),

    #[error("Problem fetching account")]
    FetchAccountFailed(#[source] OauthError),

    #[error("Server error. Failed to retrieve data. Please, try again laater")]
    RedisError(#[source] RedisError),

    #[error("Server error. Try again laater")]
    RedisConfigError(#[from] RedisConfigError),

    #[error("Malformed data. Try again laater")]
    SerializationError(#[from] serde_json::Error),

    #[error("Malformed url. Try again laater")]
    ParseError(#[from] url::ParseError),

    #[error("Something went wrong")]
    UnknownError(#[source] anyhow::Error),
}

async fn get_redis_connection(
    // redis: Data<&RedisConfigs>,
    redis: Data<&redis::Client>,
) -> Result<redis::aio::Connection, poem::Error> {
    redis
        .get_async_connection()
        .await
        // First transform message to client message. So we dont expose server error to client
        .map_err(HandlerError::RedisError)
        .map_err(InternalServerError)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RedirectCustom {
    status: StatusCode,
    uri: String,
}

impl RedirectCustom {
    /// A simple `302` redirect to a different location.
    pub fn found(uri: impl Display) -> Self {
        Self {
            status: StatusCode::FOUND,
            uri: uri.to_string(),
        }
    }
}

impl IntoResponse for RedirectCustom {
    fn into_response(self) -> Response {
        self.status
            .with_header(header::LOCATION, self.uri)
            .into_response()
    }
}

#[handler]
pub async fn oauth_login_initiator(
    Path(oauth_provider): Path<OauthProvider>,
    session: &Session,
    redis: Data<&redis::Client>,
) -> Result<RedirectCustom> {
    let mut connection = get_redis_connection(redis).await?;
    let session = TypedSession(session.to_owned());
    if let Ok(s) = session.get_user_id::<ObjectId>() {
        session.renew();
        return Ok(RedirectCustom::found("http://localhost:8000"));
    };

    let auth_url_data = match oauth_provider {
        OauthProvider::Github => GithubConfig::new().basic_config().generate_auth_url(),
        OauthProvider::Google => GoogleConfig::new().basic_config().generate_auth_url(),
    };

    auth_url_data
        .csrf_state
        .cache(&mut connection)
        .await
        .map_err(HandlerError::StorageError)
        .map_err(InternalServerError)?;

    Ok(RedirectCustom::found(
        auth_url_data.authorize_url.into_inner(),
    ))
}

#[handler]
pub async fn oauth_login_authentication(
    uri: &Uri,
    session: &Session,
    db: Data<&Database>,
    redis: Data<&redis::Client>,
) -> Result<RedirectCustom> {
    let user = authenticate_user(uri, redis).await;
    match user {
        Ok(user) => {
            let session = TypedSession(session.to_owned());
            let user = user.find_or_create_for_oauth(&db)
                // User::find_or_create_for_oauth(&db)
                .await
                .map_err(HandlerError::UnknownError).expect("ererre");
                // ?
                // .map_err(BadRequest)?;

            //  Also, handle storing user session
            // Ok(Json(user?))
            // Ok(Redirect::found("http://localhost:8000"))
            session.insert_user_id(&user.id);
            // session.renew();
            Ok(RedirectCustom::found("http://localhost:8000"))
        }
        Err(e) => Ok(RedirectCustom::found("http://localhost:8000/login")),
    }

    // user?
}

async fn authenticate_user(uri: &Uri, redis: Data<&redis::Client>) -> Result<User> {
    let mut connection = get_redis_connection(redis).await?;

    let redirect_url = Url::parse(&format!("http://localhost:{uri}"))
        .map(RedirectUrlReturned)
        .map_err(HandlerError::ParseError)
        .map_err(InternalServerError)?;

    let code = redirect_url
        .authorization_code()
        .map_err(HandlerError::InvalidAuthCode)
        .map_err(BadRequest)?;

    // make .verify give me back both the csrf token and the provider
    let csrf_token = redirect_url
        .csrf_token()
        .map_err(HandlerError::MalformedState)
        .map_err(BadRequest)?;

    let csrf_state = CsrfState::verify_csrf_token(csrf_token, &mut connection)
        .await
        .map_err(HandlerError::InvalidState)
        .map_err(BadRequest)?;

    let user = match csrf_state.provider {
        OauthProvider::Github => {
            GithubConfig::new()
                .fetch_oauth_account(code, csrf_state.pkce_code_verifier)
                .await
        }
        OauthProvider::Google => {
            GoogleConfig::new()
                .fetch_oauth_account(code, csrf_state.pkce_code_verifier)
                .await
        }
    };

    let user = user
        .map_err(HandlerError::FetchAccountFailed)
        .map_err(BadRequest);

    user
}
