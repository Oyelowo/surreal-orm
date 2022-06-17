use common::configurations::redis::RedisConfigError;
use derive_more::{From, Into};
use oauth2::{
    basic::{BasicClient, BasicTokenType},
    http::HeaderMap,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeVerifier, RedirectUrl, RevocationUrl, Scope, StandardTokenResponse, TokenResponse,
    TokenUrl, PkceCodeChallenge,
};
use redis::{AsyncCommands, RedisError};
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{default, fmt};
use typed_builder::TypedBuilder;
use url::Url;

use crate::app::user::{OauthProvider, User};

pub(crate) enum ProviderType {
    Credentials,
}

pub(crate) const REDIRECT_URL: &str = "http://localhost:8000/api/oauth/callback";

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

    #[error(transparent)]
    RedisConfigError(#[from] RedisConfigError),

    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub(crate) struct CsrfState(pub(crate) CsrfToken);

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CsrfState {
    pub(crate) csrf_token: CsrfToken,
    pub(crate) provider: OauthProvider,
    pub(crate) pkce_code_verifier: Option<PkceCodeVerifier>,
}

impl CsrfState {
    const CSRF_STATE_REDIS_KEY: &'static str = "CSRF_STATE_REDIS_KEY";

    fn redis_key(csrf_token: CsrfToken) -> String {
        format!(
            "{}{:?}",
            Self::CSRF_STATE_REDIS_KEY,
            csrf_token.secret().as_str()
        )
    }

    pub(crate) async fn verify_csrf_token(
        csrf_token: CsrfToken,
        connection: &mut redis::aio::Connection,
    ) -> OauthResult<Self> {
        let ref key = Self::redis_key(csrf_token);

        let csrf_state: String = connection.get(key).await?;

        Ok(serde_json::from_str::<Self>(csrf_state.as_str())?)
    }

    pub(crate) async fn cache(self, connection: &mut redis::aio::Connection) -> OauthResult<Self> {
        let ref key = Self::redis_key(self.csrf_token.clone());
        let csrf_state_data_string = serde_json::to_string(&self)?;

        connection.set(key, csrf_state_data_string).await?;
        connection.expire::<_, u16>(key, 600).await?;
        Ok(self)
    }
}

/// The url returned by the oauth provider with code and state(which should be the one we send)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RedirectUrlReturned(pub(crate) Url);

impl RedirectUrlReturned {
    pub(crate) fn into_inner(self) -> Url {
        self.0
    }
}

pub(crate) type OauthResult<T> = Result<T, OauthError>;
impl RedirectUrlReturned {
    pub fn get_authorization_code(&self) -> OauthResult<AuthorizationCode> {
        let value = self.get_query_param_value("code")?;
        Ok(AuthorizationCode::new(value.into_owned()))
    }

    pub(crate) fn get_csrf_token(&self) -> OauthResult<CsrfToken> {
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

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AuthUrlData {
    pub(crate) authorize_url: RedirectUrlReturned,
    pub(crate) csrf_state_data: CsrfState,
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

            print!("ERRTEWREWR:::::{remote_data:#}");
        Ok(serde_json::from_str::<T>(remote_data.as_str()).expect("errr"))
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

// TODO: Account linking
// Linking Accounts to Users happen automatically, only when they have the same e-mail address, and the user is currently signed in. Check the FAQ for more information why this is a requirement.

#[async_trait::async_trait]
pub(crate) trait OauthProviderTrait {
    // type Confr;

    // fn client(self) -> BasicClient;

    // /// Generate the authorization URL to which we'll redirect the user.
    // fn generate_auth_url(&self) -> AuthUrlData;
    fn basic_config(self)-> OauthConfig;

    async fn fetch_oauth_account(
        self,
        code: AuthorizationCode,
        pkce_code_verifier: Option<PkceCodeVerifier>,
    ) -> OauthResult<User>;
}


#[async_trait::async_trait]
pub(crate) trait OauthConfigTrait {
    // type Confr;

    fn client(self) -> BasicClient;

    /// Generate the authorization URL to which we'll redirect the user.
    fn generate_auth_url(&self) -> AuthUrlData;

    // async fn fetch_oauth_account(
    //     &self,
    //     code: AuthorizationCode,
    //     pkce_code_verifier: Option<PkceCodeVerifier>,
    // ) -> OauthResult<User>;
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
        // let csrf_token = CsrfToken::new_random();
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        let (authorize_url, csrf_token) = self
            .clone()
            .client()
            .authorize_url(CsrfToken::new_random)
            .add_scopes(self.clone().scopes)
            .set_pkce_challenge(pkce_code_challenge)
            .url();

        let csrf_state = CsrfState {
            csrf_token: csrf_token,
            provider: self.provider,
            pkce_code_verifier: Some(pkce_code_verifier),
        };

        AuthUrlData {
            authorize_url: RedirectUrlReturned(authorize_url),
            csrf_state_data: csrf_state,
            // csrf_state: CsrfState(csrf_state),
        }
    }
}