use common::configurations::redis::RedisConfigError;
use derive_more::{From, Into};
use oauth2::{
    basic::{BasicClient, BasicTokenType},
    http::HeaderMap,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    RedirectUrl, Scope, StandardTokenResponse, TokenResponse, TokenUrl,
};
use redis::{AsyncCommands, RedisError};
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;
use url::Url;

use crate::app::user::{OauthProvider, User};

pub(crate) enum ProviderType {
    Credentials,
}

pub(crate) const REDIRECT_URL: &str = "http://localhost:8000/api/oauth/callback";

#[derive(Debug, thiserror::Error)]
pub(crate) enum OauthError {
    #[error("The csrf code provided by the provider is invalid. Does not match the one sent. Potential spoofing")]
    InvalidCsrfToken,

    #[error("Failed to fetch token. Error: {0}")]
    TokenFetchFailed(String),

    #[error("Failed to fetch resource. Error: {0}")]
    ResourceFetchFailed(String),

    #[error("Failed to get query param from URL: {0}")]
    GetUrlQueryParamFailed(String),

    // #[error("Failed to fetch data. Please try again")]
    #[error(transparent)]
    RedisError(#[from] RedisError),

    #[error(transparent)]
    RedisConfigError(#[from] RedisConfigError),

    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CsrfState(pub(crate) CsrfToken);

pub(crate) struct CsrfStateData {
    csrf_token: CsrfToken,
    provider: OauthProvider,
    // pkce_verifier: Option<String>,
}

impl CsrfState {
    const CSRF_STATE_REDIS_KEY: &'static str = "CSRF_STATE_REDIS_KEY";

    fn redis_key(&self) -> String {
        format!(
            "{}{:?}",
            Self::CSRF_STATE_REDIS_KEY,
            self.0.secret().as_str()
        )
    }

    pub(crate) async fn cache(
        &self,
        provider: OauthProvider,
        connection: &mut redis::aio::Connection,
    ) -> OauthResult<()> {
        let provider = serde_json::to_string(&provider)?;
        connection.set(self.redis_key(), provider).await?;
        connection.expire::<_, u16>(self.redis_key(), 600).await?;
        Ok(())
    }

    pub(crate) async fn verify(
        &self,
        connection: &mut redis::aio::Connection,
    ) -> OauthResult<OauthProvider> {
        let csrf_state: String = connection.get(self.redis_key()).await.map_err(|e| {
            log::error!("Problem getting redis connection. Error:{e:?}");
            e
        })?;
        connection.del::<_, String>(self.redis_key()).await?;
        Ok(serde_json::from_str::<OauthProvider>(csrf_state.as_str())?)
    }
}

/// The url returned by the oauth provider with code and state(which should be the one we send)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RedirectUrlReturned(pub Url);

impl fmt::Display for RedirectUrlReturned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

const CODE: &str = "code";
const STATE: &str = "state";

pub(crate) type OauthResult<T> = Result<T, OauthError>;
impl RedirectUrlReturned {
    pub fn get_authorization_code(&self) -> OauthResult<AuthorizationCode> {
        let value = self.get_query_param_value(CODE)?;
        Ok(AuthorizationCode::new(value.into_owned()))
    }

    pub(crate) fn get_csrf_state(&self) -> OauthResult<CsrfState> {
        let value = self.get_query_param_value(STATE)?;
        Ok(CsrfState(CsrfToken::new(value.into_owned())))
    }

    fn get_query_param_value(&self, query_param: &str) -> OauthResult<std::borrow::Cow<str>> {
        let (_, value) = self
            .0
            .query_pairs()
            .find(|&(ref key, _)| key == query_param)
            .ok_or_else(|| OauthError::GetUrlQueryParamFailed(query_param.into()))?;
        Ok(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AuthUrlData {
    pub(crate) authorize_url: RedirectUrlReturned,
    pub(crate) csrf_state: CsrfState,
}

#[derive(Debug, From, Into, Clone)]
pub(crate) struct OauthUrl(pub(crate) &'static str);

impl OauthUrl {
    pub async fn get_resource<T: DeserializeOwned>(
        &self,
        token: &StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        headers: Option<HeaderMap>,
    ) -> OauthResult<T> {
        let headers = headers.unwrap_or_default();
        let remote_data = reqwest::Client::new()
            .get(self.0)
            .header(ACCEPT, "application/vnd.github.v3+json")
            .header(USER_AGENT,"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.4577.82 Safari/537.36")
            .headers(headers)
            // .header(AUTHORIZATION, format!("Bearer {}",token.access_token().secret().as_str()))
            .bearer_auth(token.access_token().secret().as_str())
            .send()
            .await
            .map_err(|_| OauthError::ResourceFetchFailed(self.0.to_string()))?
            .text()
            .await
            .map_err(|_| OauthError::ResourceFetchFailed(self.0.to_string()))?;

        Ok(serde_json::from_str::<T>(remote_data.as_str())?)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OauthConfig {
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub auth_url: AuthUrl,
    pub token_url: TokenUrl,
    pub redirect_url: RedirectUrl,
    pub scopes: Vec<Scope>,
}

// TODO: Account linking
// Linking Accounts to Users happen automatically, only when they have the same e-mail address, and the user is currently signed in. Check the FAQ for more information why this is a requirement.

#[async_trait::async_trait]
pub(crate) trait OauthProviderTrait {
    // type Confr;

    fn client(self) -> BasicClient;

    /// Generate the authorization URL to which we'll redirect the user.
    fn generate_auth_url(&self) -> AuthUrlData;

    async fn fetch_oauth_account(
        &self,
        code: AuthorizationCode,
    ) -> OauthResult<User>;
}
