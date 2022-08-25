// use common::configurations::{application::ApplicationConfigs, redis::RedisConfigError};
use derive_more::{From, Into};
use oauth2::{
    basic::{BasicClient, BasicTokenType},
    http::HeaderMap,
    reqwest::async_http_client,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RevocationUrl, Scope, StandardTokenResponse,
    TokenResponse, TokenUrl,
};
use redis::RedisError;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::Url;

use super::{cache_storage::CacheStorage, OauthProvider};
// use crate::app::user::{AccountOauth, OauthProvider};

pub(crate) fn get_redirect_url(base_url: String) -> String {
    // let base_url = ApplicationConfigs::default().external_base_url;
    // Has to be defined in app router
    format!("{base_url}/api/oauth/callback")
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum OauthError {
    #[error("Failed to fetch token. Error: {0}")]
    TokenFetchFailed(String),

    #[error("Failed to fetch resource. Error: {0}")]
    ResourceFetchFailed(String),

    #[error("Failed to get query param from URL: {0}")]
    GetUrlQueryParamFailed(String),

    // #[error("Failed to fetch data. Please try again")]
    #[error(transparent)]
    RedisError(#[from] RedisError),

    // #[error(transparent)]
    // RedisConfigError(#[from] RedisConfigError),

    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
}

/// Tokens stored in redis for returned url oauth verification
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Evidence {
    pub(crate) csrf_token: CsrfToken,
    pub(crate) provider: OauthProvider,
    pub(crate) pkce_code_verifier: PkceCodeVerifier,
}

pub(crate) type OauthResult<T> = Result<T, OauthError>;
/// The url returned by the oauth provider with code and state(which should be the one we send)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RedirectUrlReturned(pub(crate) Url);

impl RedirectUrlReturned {
    pub(crate) fn into_inner(self) -> Url {
        self.0
    }

    pub fn authorization_code(&self) -> OauthResult<AuthorizationCode> {
        let value = self.get_query_param_value("code")?;
        Ok(AuthorizationCode::new(value.into_owned()))
    }

    pub(crate) fn csrf_token(&self) -> OauthResult<CsrfToken> {
        let value = self.get_query_param_value("state")?;
        Ok(CsrfToken::new(value.into_owned()))
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

/// authorization URL to which we'll redirect the user
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AuthUrlData {
    pub(crate) authorize_url: RedirectUrlReturned,
    pub(crate) evidence: Evidence,
}

impl AuthUrlData {
    const OAUTH_CSRF_STATE_KEY: &'static str = "OAUTH_CSRF_STATE_KEY";

    fn oauth_cache_key_prefix(csrf_token: CsrfToken) -> String {
        format!(
            "{}{:?}",
            Self::OAUTH_CSRF_STATE_KEY,
            csrf_token.secret().as_str()
        )
    }

    pub(crate) async fn verify_csrf_token(
        csrf_token: CsrfToken,
        storage: impl CacheStorage,
    ) -> Option<Self> {
        let key = Self::oauth_cache_key_prefix(csrf_token);
        let auth_url_data = storage.get(key).await.unwrap();

        let auth_url_data = serde_json::from_str::<Self>(&auth_url_data.as_str()).unwrap();

        Some(auth_url_data)
    }

    pub(crate) async fn save(&self, storage: impl CacheStorage) -> OauthResult<()> {
        let key = Self::oauth_cache_key_prefix(self.evidence.csrf_token.clone());
        let csrf_state_data_string = serde_json::to_string(&self)?;
        storage.set(key, csrf_state_data_string).await;
        Ok(())
    }
}

#[derive(Debug, From, Into, Clone)]
pub(crate) struct OauthUrl(pub(crate) &'static str);

impl OauthUrl {
    pub async fn fetch_resource<T: DeserializeOwned>(
        &self,
        token: &StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        headers: Option<HeaderMap>,
    ) -> OauthResult<T> {
        let headers = headers.unwrap_or_default();
        let remote_data = reqwest::Client::new()
            .get(self.0)
            .header(ACCEPT, "application/vnd.github.v3+json")
            .header(USER_AGENT, "oyelowo")
            .headers(headers)
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

#[derive(Debug, TypedBuilder, Clone)]
pub(crate) struct OauthConfig {
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub auth_url: AuthUrl,
    pub token_url: TokenUrl,
    pub redirect_url: RedirectUrl,

    #[builder(default, setter(strip_option))]
    pub revocation_url: Option<RevocationUrl>,
    pub scopes: Vec<Scope>,
    pub provider: OauthProvider, // pub csrf_token: CsrfToken,
}

// Nice to have: Account linking. User has to be logged in to link another account.
// Linking Accounts to Users happen automatically, only when they have the same e-mail address, and the user is currently signed in. Check the FAQ for more information why this is a requirement.
#[async_trait::async_trait]
pub(crate) trait OauthProviderTrait {
    type OauthResponse : DeserializeOwned;

    fn basic_config(&self) -> OauthConfig;

    async fn exchange_token(
        &self,
        code: AuthorizationCode,
        pkce_code_verifier: PkceCodeVerifier,
    ) -> OauthResult<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
        let token = self
            .basic_config()
            .client()
            .exchange_code(code)
            .set_pkce_verifier(pkce_code_verifier)
            .request_async(async_http_client)
            .await
            .map_err(|e| OauthError::TokenFetchFailed(e.to_string()))?;
        Ok(token)
    }

    async fn fetch_oauth_account(
        &self,
        code: AuthorizationCode,
        pkce_code_verifier: PkceCodeVerifier,
    ) -> OauthResult<Self::OauthResponse>;
}

#[async_trait::async_trait]
pub(crate) trait OauthConfigTrait {
    fn client(self) -> BasicClient;

    fn generate_auth_url(&self) -> AuthUrlData;
}

#[async_trait::async_trait]
impl OauthConfigTrait for OauthConfig {
    fn client(self) -> BasicClient {
        let client = BasicClient::new(
            self.client_id,
            Some(self.client_secret),
            self.auth_url,
            Some(self.token_url),
        )
        .set_redirect_uri(self.redirect_url);

        if let Some(url) = self.revocation_url {
            return client.set_revocation_uri(url);
        }
        client
    }

    /// Generate the authorization URL to which we'll redirect the user.
    fn generate_auth_url(&self) -> AuthUrlData {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        let (authorize_url, csrf_token) = self
            .clone()
            .client()
            .authorize_url(CsrfToken::new_random)
            .add_scopes(self.clone().scopes)
            .set_pkce_challenge(pkce_code_challenge)
            .url();

        let evidence = Evidence {
            csrf_token,
            pkce_code_verifier,
            provider: self.provider,
        };

        AuthUrlData {
            evidence,
            authorize_url: RedirectUrlReturned(authorize_url),
        }
    }
}
