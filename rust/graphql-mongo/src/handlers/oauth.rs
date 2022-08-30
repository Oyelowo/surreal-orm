use std::fmt::Display;

use anyhow::Context;
use bson::oid::ObjectId;
use common::authentication::TypedSession;

use common::configurations::application::ApplicationConfigs;
use common::oauth::cache_storage::RedisCache;
use mongodb::Database;
use poem::error as poem_error;
use poem::session::Session;
use poem::web::Data;
use poem::{error::Result, handler, http::Uri, web::Path};
use poem::{IntoResponse, Response};
use redis::RedisError;
use reqwest::{header, StatusCode};
use url::Url;

use crate::app::user::{OauthProvider, User};
use common::oauth::client::OauthClient;

/// These are created to map internal error message that we
/// only want to expose as logs for debugging to messages we
/// would want to show to the client/frontend.
/// Otherwise, we could have mapped directly. We could also use poem's
/// custom error but that feels a little verbose/overkill
#[derive(Debug, thiserror::Error)]
pub(crate) enum HandlerError {
    #[error(transparent)]
    OauthError(#[from] common::oauth::error::OauthError),

    #[error("Problem retrieving account")]
    GetAccountFailed,
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
pub async fn start_authentication(
    Path(oauth_provider): Path<OauthProvider>,
    session: &Session,
    oauth_client: Data<&OauthClient<RedisCache>>,
) -> Result<RedirectCustom> {
    let session = TypedSession(session.to_owned());
    let env = ApplicationConfigs::default();

    // If session still valid, redirect to home page
    if let Ok(_s) = session.get_user_id::<ObjectId>() {
        session.renew();
        return Ok(RedirectCustom::found(env.external_base_url));
    };

    let auth_url_data = oauth_client
        .to_owned()
        .generate_auth_url_data(oauth_provider.into())
        .await
        .map_err(poem_error::BadGateway)?;

    Ok(RedirectCustom::found(
        auth_url_data.authorize_url.into_inner(),
    ))
}

#[handler]
pub async fn complete_authentication(
    uri: &Uri,
    session: &Session,
    db: Data<&Database>,
    oauth_client: Data<&OauthClient<RedisCache>>,
) -> Result<RedirectCustom> {
    let base_url = ApplicationConfigs::default().external_base_url;

    let user = authenticate_user(uri, session, db, oauth_client).await;

    match user {
        // Redirect to the frontend app which is served at base.
        Ok(_u) => Ok(RedirectCustom::found(base_url)),
        Err(e) => Ok(RedirectCustom::found(format!("{base_url}/login?error={e}"))),
    }
}

async fn authenticate_user(
    uri: &Uri,
    session: &Session,
    db: Data<&Database>,
    oauth_client: Data<&OauthClient<RedisCache>>,
) -> Result<User> {
    let base_url = ApplicationConfigs::default().external_base_url;
    let redirect_url = Url::parse(&format!("{base_url}{uri}")).context("Problem parsing URL")?;

    let account_oauth = oauth_client
        .clone()
        .fetch_account(redirect_url)
        .await
        .context("Problem fetching Oauth account")?;

    let user = User::find_or_create_for_oauth(&db, account_oauth.into())
        .await
        .context("User not found nor created. Try again later")?;

    let session = TypedSession(session.to_owned());
    session.insert_user_id(&user.id);
    Ok(user)
}
