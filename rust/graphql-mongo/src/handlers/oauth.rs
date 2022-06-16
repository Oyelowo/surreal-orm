use poem::error::{BadRequest, InternalServerError};
use poem::web::{Data, Redirect, Json};
use poem::{handler, http::Uri, web::Path};
use redis::RedisError;
use url::Url;

use crate::oauth::github::GithubConfig;
use crate::oauth::utils::{OauthError, OauthProviderTrait, TypedAuthUrl};
use common::configurations::redis::{RedisConfigError, RedisConfigs};

use crate::app::user::{OauthProvider, User};

#[derive(Debug, thiserror::Error)]
pub(crate) enum OauthHandlerError {
    #[error("The csrf code provided by the provider is invalid. Does not match the one sent. Potential spoofing")]
    OauthError(#[source] OauthError),

    #[error("Problem getting data. Try again laater")]
    RedisError(#[from] RedisError),

    #[error("Problem getting data. Try again laater")]
    RedisConfigError(#[from] RedisConfigError),

    #[error("Problem transforming data. Try again laater")]
    SerializationError(#[from] serde_json::Error),

    #[error("Problem transforming data. Try again laater")]
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
        .map_err(OauthHandlerError::RedisConfigError)
        .map_err(InternalServerError)
}

#[handler]
pub async fn oauth_login_initiator(
    Path(oauth_provider): Path<OauthProvider>,
    redis: Data<&RedisConfigs>,
) -> poem::Result<Redirect> {
    let mut con = get_redis_connection(redis).await?;

    let auth_url_data = match oauth_provider {
        OauthProvider::Github => GithubConfig::new().generate_auth_url(),
        OauthProvider::Google => todo!(),
    };

    // Send csrf state to redis
    auth_url_data
        .csrf_state
        .cache(oauth_provider, &mut con)
        .await
        .map_err(OauthHandlerError::OauthError)
        .map_err(InternalServerError)?;

    Ok(Redirect::moved_permanent(auth_url_data.authorize_url))
}

pub(crate) const OAUTH_LOGIN_AUTHENTICATION_ENDPOINT: &str = "/api/oauth/callback";

#[handler]
pub async fn oauth_login_authentication(
    uri: &Uri,
    rc: Data<&RedisConfigs>,
) -> poem::Result<Json<User>> {
    let mut con = get_redis_connection(rc).await?;

    let full_url = "http://localhost".to_string() + &uri.to_string();
    let redirect_url = Url::parse(&(full_url))
        .map_err(OauthHandlerError::ParseError)
        .map_err(InternalServerError)?;

    let redirect_url = TypedAuthUrl(redirect_url);
    let code = redirect_url.get_authorization_code();
    // make .verify give me back both the csrf token and the provider
    let provider = redirect_url
        .get_csrf_state()
        .verify(&mut con)
        .await
        .map_err(OauthHandlerError::OauthError)
        .map_err(BadRequest)?;

    let user = match provider {
        OauthProvider::Github => {
            let github_config = GithubConfig::new();

            github_config
                .fetch_oauth_account(code)
                .await
                .map_err(OauthHandlerError::OauthError)
                .map_err(BadRequest)
        }
        OauthProvider::Google => todo!(),
    };
    println!("USERRRR: {user:?}");
    //  Also, handle storing user session
    // poem::Response::builder().body(user).finish()
    // let mut r = poem::Response::default();

    Ok(Json(user.unwrap()))
    // Ok("efddfd".into())
}
