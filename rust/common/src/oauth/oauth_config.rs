use std::fmt::Debug;

// use common::configurations::{application::ApplicationConfigs, redis::RedisConfigError};
use derive_more::{From, Into};
use multimap::MultiMap;
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

use super::{
    account::OauthProvider,
    cache_storage::CacheStorage,
    error::{OauthError, OauthResult},
    urls::{AuthUrlData, Evidence, RedirectUrlReturned},
};

#[derive(Debug, TypedBuilder, Clone)]
pub struct OauthConfig {
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
    type OauthResponse: DeserializeOwned;

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
            authorize_url: RedirectUrlReturned::new(authorize_url),
        }
    }
}
