use common::configurations::redis::{RedisConfigError, RedisConfigs};
use oauth2::basic::BasicClient;
use poem::middleware::AddData;
use poem::web::{Data, Redirect};
use poem::{get, handler, http::Uri, listener::TcpListener, web::Path, Route, Server};
use poem::{EndpointExt, IntoResponse};

// Alternatively, this can be `oauth2::curl::http_client` or a custom client.
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use poem_openapi::payload::{PlainText, Response};
use redis::{Connection, RedisError};
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
// use redis::aio::Connection;
use std::env;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
// use tokio::net::TcpListener;
use crate::oauth::github::GithubConfig;
use crate::oauth::utils::{OauthError, OauthProviderTrait, TypedAuthUrl};
use url::Url;

use crate::app::user::OauthProvider;

#[handler]
pub async fn oauth_login_initiator(
    Path(oauth_provider): Path<OauthProvider>,
    rc: Data<&RedisConfigs>,
) -> Redirect {
    let mut con = rc.clone().get_client().unwrap().get_connection().unwrap();
    println!("XXXXXX : {oauth_provider:?}");
    let auth_url_data = match oauth_provider {
        OauthProvider::Github => GithubConfig::new().generate_auth_url(),
        OauthProvider::Google => todo!(),
    };

    // Send csrf state to redis
    auth_url_data
        .csrf_state
        .cache(oauth_provider, &mut con)
        .unwrap();

    println!("ewertyrewWRTYREW : {:?}", auth_url_data.authorize_url);
    Redirect::moved_permanent(auth_url_data.authorize_url)
}

pub(crate) const OAUTH_LOGIN_AUTHENTICATION_ENDPOINT: &str = "/api/oauth/callback";

#[derive(Debug, thiserror::Error)]
pub(crate) enum OauthHandlerError {
    #[error("The csrf code provided by the provider is invalid. Does not match the one sent. Potential spoofing")]
    OauthError(#[from] OauthError),

    #[error(transparent)]
    RedisError(#[from] RedisError),

    #[error(transparent)]
    RedisConfigError(#[from] RedisConfigError),

    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
}

impl poem::error::ResponseError for OauthHandlerError {
    fn status(&self) -> poem::http::StatusCode {
        match self {
            Self::OauthError(_) => StatusCode::BAD_REQUEST,
            Self::RedisError(_) | Self::SerializationError(_) | Self::RedisConfigError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn as_response(&self) -> poem::Response
    where
        Self: std::error::Error + Send + Sync + 'static,
    {
        let error_message = match self {
            Self::OauthError(_) => "invalid token",
            _ => "Server error. Please try again",
        };
    
        log::error!("{error_message}");
        poem::Response::builder().status(self.status())
        .body(error_message.to_string())
        // poem::Response::builder().status(self.status())
        // .body(error_message.to_string())
    }
}

#[handler]
pub async fn oauth_login_authentication(
    uri: &Uri,
    rc: Data<&RedisConfigs>,
) -> poem::Result<poem::Response> {
    let mut con = rc
        .clone()
        .get_client()
        .map_err(OauthHandlerError::RedisConfigError)?
        .get_connection()
        .map_err(OauthHandlerError::RedisError)?;

    let redirect_url = Url::parse(&("http://localhost".to_string() + &uri.to_string())).unwrap();
    let redirect_url = TypedAuthUrl(redirect_url);
    let code = redirect_url.get_authorization_code();
    // make .verify give me back both the csrf token and the provider
    let provider = redirect_url
        .get_csrf_state()
        .verify(&mut con)
        .map_err(OauthHandlerError::OauthError)?;

    let user = match provider {
        OauthProvider::Github => {
            let github_config = GithubConfig::new();

            // All these are the profile fetch should probably also be part of github config(OauthProvider) trait
            github_config.fetch_oauth_account(code).await.unwrap()
            //  {
            //                 Ok(u)=>u,
            //                 Err(e)=>eprintln!("WERYRT: {e:?}");
        }
        OauthProvider::Google => todo!(),
    };
    println!("USERRRR: {user:?}");
    //  Also, handle storing user session
    // poem::Response::builder().body(user).finish()
    // let mut r = poem::Response::default();
    

    Ok("efddfd".into())
}
