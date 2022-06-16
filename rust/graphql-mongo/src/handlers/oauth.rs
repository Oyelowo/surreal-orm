use anyhow::Context;
use common::configurations::redis::{RedisConfigError, RedisConfigs};
use log::logger;
use oauth2::basic::BasicClient;
use poem::error::{BadRequest, InternalServerError};
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
use serde::{Deserialize, Serialize};
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
) -> poem::Result<Redirect> {
    let mut con = rc
        .clone()
        .get_async_connection()
        .await
        // First transform message to client message. So we dont expose server error to client
        .map_err(OauthHandlerError::RedisConfigError)
        .map_err(InternalServerError)?;

    println!("XXXXXX : {oauth_provider:?}");
    let auth_url_data = match oauth_provider {
        OauthProvider::Github => GithubConfig::new().generate_auth_url(),
        OauthProvider::Google => todo!(),
    };

    // Send csrf state to redis
    auth_url_data
        .csrf_state
        .cache(oauth_provider, &mut con)
        .await
        .unwrap();

    println!("ewertyrewWRTYREW : {:?}", auth_url_data.authorize_url);
    Ok(Redirect::moved_permanent(auth_url_data.authorize_url))
}

pub(crate) const OAUTH_LOGIN_AUTHENTICATION_ENDPOINT: &str = "/api/oauth/callback";

#[derive(Debug, thiserror::Error)]
pub(crate) enum OauthHandlerError {
    #[error("The csrf code provided by the provider is invalid. Does not match the one sent. Potential spoofing")]
    OauthError(#[source] OauthError),

    #[error("Problem getting data. Try again laater")]
    RedisError(#[from] RedisError),

    // #[error(transparent)]
    #[error("Problem getting data. Try again laater")]
    RedisConfigError(#[from] RedisConfigError),

    // #[error(transparent)]
    #[error("Problem transforming data. Try again laater")]
    SerializationError(#[from] serde_json::Error),
}

#[handler]
pub async fn oauth_login_authentication(
    uri: &Uri,
    rc: Data<&RedisConfigs>,
) -> poem::Result<poem::Response> {
    let mut con = rc
        .clone()
        .get_async_connection()
        .await
        // First transform message to client message. So we dont expose server error to client
        .map_err(OauthHandlerError::RedisConfigError)
        .map_err(InternalServerError)?;

    let redirect_url = Url::parse(&("http://localhost".to_string() + &uri.to_string())).unwrap();
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
