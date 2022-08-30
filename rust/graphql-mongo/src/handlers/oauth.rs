use std::fmt::Display;

use bson::oid::ObjectId;
use common::authentication::TypedSession;

use common::configurations::application::ApplicationConfigs;
use mongodb::Database;
use poem::error::{BadRequest, InternalServerError};
use poem::session::Session;
use poem::web::Data;
use poem::{error::Result, handler, http::Uri, web::Path};
use poem::{IntoResponse, Response};
use redis::RedisError;
use reqwest::{header, StatusCode};
use url::Url;

use crate::oauth::cache_storage as cg;
use crate::oauth::github::GithubConfig;
use crate::oauth::google::GoogleConfig;
use crate::oauth::utils::{
    AuthUrlData, OauthConfigTrait, OauthError, OauthProviderTrait, RedirectUrlReturned,
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

    // #[error("The state token provided is invalid.")]
    // InvalidState(#[source] OauthError),
    #[error("The auth code provided is invalid.")]
    InvalidAuthCode(#[source] OauthError),

    #[error("Problem fetching account")]
    FetchAccountFailed(#[source] OauthError),

    #[error("Problem retrieving account")]
    GetAccountFailed,

    #[error("Server error. Failed to retrieve data. Please, try again laater")]
    RedisError(#[source] RedisError),

    #[error("Server error. Try again laater")]
    RedisConfigError(#[from] RedisConfigError),

    #[error("Malformed data. Try again laater")]
    SerializationError(#[from] serde_json::Error),

    #[error("Malformed url. Try again laater")]
    ParseError(#[from] url::ParseError),
}

pub async fn get_redis_connection(
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
    // let mut connection = get_redis_connection(redis).await?;
    let session = TypedSession(session.to_owned());
    let env = ApplicationConfigs::default();

    if let Ok(_s) = session.get_user_id::<ObjectId>() {
        session.renew();
        return Ok(RedirectCustom::found(env.external_base_url));
    };

    let auth_url_data = match oauth_provider {
        OauthProvider::Github => GithubConfig::new().basic_config().generate_auth_url(),
        OauthProvider::Google => GoogleConfig::new().basic_config().generate_auth_url(),
    };

    let cache = cg::RedisCache(redis.clone());

    auth_url_data
        .save(cache)
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
    let user = authenticate_user(uri, redis, session, db).await;
    let base_url = ApplicationConfigs::default().external_base_url;
    match user {
        // Redirect to the frontend app which is served at base.
        Ok(_u) => Ok(RedirectCustom::found(base_url)),
        Err(e) => Ok(RedirectCustom::found(format!("{base_url}/login?error={e}"))),
    }
}

async fn authenticate_user(
    uri: &Uri,
    redis: Data<&redis::Client>,
    session: &Session,
    db: Data<&Database>,
) -> Result<User> {
    // let mut connection = get_redis_connection(redis.clone()).await?;
    let base_url = ApplicationConfigs::default().external_base_url;

    let redirect_url = Url::parse(&format!("{base_url}{uri}"))
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

    let cache = cg::RedisCache(redis.clone());
    let evidence = AuthUrlData::verify_csrf_token(csrf_token, cache)
        .await
        .unwrap()
        .evidence;

    let account_oauth = match evidence.provider {
        OauthProvider::Github => {
            GithubConfig::new()
                .fetch_oauth_account(code, evidence.pkce_code_verifier)
                .await
        }
        OauthProvider::Google => {
            GoogleConfig::new()
                .fetch_oauth_account(code, evidence.pkce_code_verifier)
                .await
        }
    }
    .map_err(HandlerError::FetchAccountFailed)
    .map_err(BadRequest)?;

    let user = User::find_or_create_for_oauth(&db, account_oauth)
        .await
        .map_err(|_e| HandlerError::GetAccountFailed)
        .map_err(BadRequest)?;

    let session = TypedSession(session.to_owned());
    session.insert_user_id(&user.id);
    Ok(user)
}
