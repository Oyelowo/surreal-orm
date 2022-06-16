use std::fmt;

use anyhow::Context;
use common::configurations::redis::RedisConfigError;
use derive_more::{From, Into};
use oauth2::{
    basic::{BasicClient, BasicTokenType},
    http::HeaderMap,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields, Scope,
    StandardTokenResponse, TokenResponse, TokenUrl,
};
use redis::{Commands, RedisError};
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
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

    #[error(transparent)]
    RedisError(#[from] RedisError),

    #[error(transparent)]
    RedisConfigError(#[from] RedisConfigError),

    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CsrfStateWrapper(pub(crate) CsrfToken);

pub(crate) struct TypedCsrfStateData {
    csrf_token: CsrfToken,
    provider: OauthProvider,
    // pkce_verifier: Option<String>,
}

impl CsrfStateWrapper {
    const CSRF_STATE_REDIS_KEY: &'static str = "CSRF_STATE_REDIS_KEY";

    fn redis_key(&self) -> String {
        format!(
            "{}{:?}",
            Self::CSRF_STATE_REDIS_KEY,
            self.0.secret().as_str()
        )
    }

    pub(crate) fn cache(
        &self,
        provider: OauthProvider,
        con: &mut redis::Connection,
    ) -> anyhow::Result<(), OauthError> {
        let provider = serde_json::to_string(&provider)?;
        println!("RTYUTREWQ: {provider:?}");
        let _: () = con.set(self.redis_key(), provider)?;
        // It seems github is storing a session for the auth. So, keep this for much
        // Longer
        // con.expire::<_, u16>(self.redis_key(), 600)?;
        Ok(())
    }

    pub(crate) fn verify(
        &self,
        con: &mut redis::Connection,
    ) -> anyhow::Result<OauthProvider, OauthError> {
        let csrf_state: String = con.get(self.redis_key())?;
        // Todo: The strategy of delete after use is probably better but leave using TTL at line 72 for now.
        // con.del::<_, String>(self.redis_key())?;
        Ok(serde_json::from_str::<OauthProvider>(csrf_state.as_str())?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TypedAuthUrl(pub Url);

impl fmt::Display for TypedAuthUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TypedAuthUrl {
    pub fn get_authorization_code(&self) -> AuthorizationCode {
        let value = self.get_query_param_value("code");
        AuthorizationCode::new(value.into_owned())
    }

    pub(crate) fn get_csrf_state(&self) -> CsrfStateWrapper {
        let value = self.get_query_param_value("state");
        CsrfStateWrapper(CsrfToken::new(value.into_owned()))
    }

    fn get_query_param_value(&self, query_param: &str) -> std::borrow::Cow<str> {
        let state_pair = self
            .0
            .query_pairs()
            .find(|pair| {
                let &(ref key, _) = pair;
                key == query_param
            })
            .expect(
                format!(
                    "Not found. TODO: Handle error properly later, param: {query_param}. url:{}",
                    self.0
                )
                .as_str(),
            );
        let (_, value) = state_pair;
        value
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AuthUrlData {
    pub(crate) authorize_url: TypedAuthUrl,
    pub(crate) csrf_state: CsrfStateWrapper,
}

#[derive(Debug, From, Into, Clone)]
pub(crate) struct OauthUrl(pub(crate) &'static str);

impl OauthUrl {
    pub async fn get_resource<T: DeserializeOwned>(
        &self,
        token: &StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        headers: Option<HeaderMap>,
    ) -> T {
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
                    // TODO: Handler error properly
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

        serde_json::from_str::<T>(remote_data.as_str()).expect("erer")
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OauthConfig {
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub auth_url: AuthUrl,
    pub token_url: TokenUrl,
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
    ) -> anyhow::Result<User, OauthError>;
}
