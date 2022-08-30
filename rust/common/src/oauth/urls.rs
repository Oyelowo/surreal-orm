use derive_more::{From, Into};
use multimap::MultiMap;
use oauth2::{
    basic::BasicTokenType, http::HeaderMap, AuthorizationCode, CsrfToken, EmptyExtraTokenFields,
    PkceCodeVerifier, StandardTokenResponse, TokenResponse,
};
use std::fmt::Debug;

use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use super::{
    account::OauthProvider,
    cache_storage::CacheStorage,
    error::{OauthError, OauthResult},
};

pub(crate) fn get_redirect_url(base_url: &String) -> String {
    // let base_url = ApplicationConfigs::default().external_base_url;
    // Has to be defined in app router
    format!("{base_url}/api/oauth/callback")
}
/// Tokens stored in redis for returned url oauth verification
#[derive(Debug, Serialize, Deserialize)]
pub struct Evidence {
    pub csrf_token: CsrfToken,
    pub provider: OauthProvider,
    pub pkce_code_verifier: PkceCodeVerifier,
}

/// The url returned by the oauth provider with code and state(which should be the one we send)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectUrlReturned(Url);

impl RedirectUrlReturned {
    pub fn new(url: Url) -> Self {
        Self(url)
    }
    pub fn into_inner(self) -> Url {
        self.0
    }

    pub fn get_authorization_code(&self) -> Option<AuthorizationCode> {
        self.get_query_param_value("code")
            .map(AuthorizationCode::new)
    }

    pub fn get_csrf_token(&self) -> Option<CsrfToken> {
        self.get_query_param_value("state").map(CsrfToken::new)
    }

    fn get_query_param_value(&self, query_param: &str) -> Option<String> {
        let hash_query: MultiMap<_, _> = self.0.query_pairs().into_owned().collect();
        hash_query.get(query_param).map(String::from)
    }
}

/// authorization URL to which we'll redirect the user
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUrlData {
    pub authorize_url: RedirectUrlReturned,
    pub evidence: Evidence,
}

impl AuthUrlData {
    pub fn oauth_cache_key_prefix(csrf_token: CsrfToken) -> String {
        let oauth_csrf_state_key = "OAUTH_CSRF_STATE_KEY";
        format!("{oauth_csrf_state_key}_{}", csrf_token.secret().as_str())
    }

    pub async fn verify_csrf_token<C>(csrf_token: CsrfToken, storage: &mut C) -> OauthResult<Self>
    where
        C: CacheStorage,
    {
        let key = Self::oauth_cache_key_prefix(csrf_token);
        let auth_url_data = storage
            .get(key)
            .await
            .ok_or(OauthError::AuthUrlDataNotFoundInCache)?;

        Ok(serde_json::from_str::<Self>(auth_url_data.as_str())?)
    }

    pub async fn save<C>(&self, storage: &mut C) -> OauthResult<()>
    where
        C: CacheStorage,
    {
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
            .get(self.0.to_string())
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
