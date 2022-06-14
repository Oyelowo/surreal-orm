use std::fmt;

use anyhow::Context;
use bson::DateTime;
use chrono::{Duration, Utc};

use common::sum;
use derive_more::{From, Into};
use oauth2::{
    basic::{BasicClient, BasicTokenType},
    http::HeaderMap,
    reqwest::async_http_client,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    RedirectUrl, Scope, StandardTokenResponse, TokenResponse, TokenUrl,
};
use redis::{Commands, FromRedisValue, RedisError};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use crate::app::user::{AccountOauth, OauthProvider, Role, TokenType, User};

#[derive(Debug, Deserialize, Serialize)]
enum GithubScopes {
    #[serde(rename = "public_repo")]
    PublicRepo,
    #[serde(rename = "user:email")]
    UserEmail,
}

#[derive(Debug, Deserialize, Serialize)]
struct GithubUserData {
    login: String,
    id: String,
    node_id: String,
    #[serde(rename = "type")]
    account_type: String,
    name: String,
    email: Option<Option<String>>,
    avatar_url: Option<String>,
    gravatar_id: Option<String>,
    url: Option<String>,
    html_url: Option<String>,
    followers_url: Option<String>,
    following_url: Option<String>,
    gists_url: Option<String>,
    starred_url: Option<String>,
    subscriptions_url: Option<String>,
    organizations_url: Option<String>,
    repos_url: Option<String>,
    events_url: Option<String>,
    received_events_url: Option<String>,
    site_admin: Option<String>,
    company: Option<String>,
    blog: Option<String>,
    location: Option<String>,
    hireable: Option<String>,
    bio: Option<String>,
    twitter_username: Option<String>,
    public_repos: Option<String>,
    public_gists: Option<String>,
    followers: Option<String>,
    following: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

enum ProviderType {
    Credentials,
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
    ) -> anyhow::Result<User, OauthProviderError>;

    // fn exchange_code_for_token(self) {
    //        let token_res = client
    //     .exchange_code(code)
    //     .request_async(async_http_client)
    //     .await;
    // }
}

const REDIRECT_URL: &str = "http://localhost:8080";
// use derive_more::{Add, Display, From, Into};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedAuthUrl(pub Url);

// impl IntoInner for TypedAuthUrl {

// }

impl fmt::Display for TypedAuthUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// impl Into<Url> for TypedAuthUrl {
//     // fn from(url: Url) -> Self {
//     //     Self(url)
//     // }

//     fn into(self) -> Url {
//         self.0
//     }
// }

impl TypedAuthUrl {
    pub fn get_authorization_code(&self) -> AuthorizationCode {
        let value = self.get_query_param_value("code");
        AuthorizationCode::new(value.into_owned())
    }

    pub(crate) fn get_csrf_state(&self) -> TypedCsrfState {
        let value = self.get_query_param_value("state");
        TypedCsrfState(CsrfToken::new(value.into_owned()))
    }

    fn get_query_param_value(&self, query_param: &str) -> std::borrow::Cow<str> {
        let state_pair = self
            .0
            .query_pairs()
            .find(|pair| {
                let &(ref key, _) = pair;
                key == query_param
            })
            .expect("Not found. TODO: Handle error properly later");
        let (_, value) = state_pair;
        value
    }
}

#[derive(Debug, Clone)]
struct OauthConfig {
    client_id: ClientId,
    client_secret: ClientSecret,
    auth_url: AuthUrl,
    token_url: TokenUrl,
    scopes: Vec<Scope>,
}

#[derive(Debug, Clone)]
pub(crate) struct GithubConfig {
    basic_config: OauthConfig,
    user_data_url: OauthUrl,
    user_emails_url: OauthUrl,
}

impl GithubConfig {
    pub fn new() -> Self {
        let basic_config = OauthConfig {
            // Get first two from environment variable
            client_id: ClientId::new("57d332c258954615aac7".to_string()),
            client_secret: ClientSecret::new("e41a1fb86af01532fe640a2d79ad6608c3774261".into()),
            auth_url: AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                .expect("Invalid authorization endpoint URL"),
            token_url: TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                .expect("Invalid token endpoint URL"),
            scopes: vec![
                Scope::new("public_repo".into()),
                Scope::new("read:user".into()),
                Scope::new("user:email".into()),
            ],
        };
        Self {
            basic_config,
            user_data_url: OauthUrl("https://api.github.com/user"),
            user_emails_url: OauthUrl("https://api.github.com/user/emails"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum CsrfStateError {
    #[error("The csrf code provided by the provider is invalid. Does not match the one sent. Potential spoofing")]
    InvalidCsrfToken,

    #[error(transparent)]
    RedisError(#[from] RedisError),

    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TypedCsrfState(CsrfToken);
pub(crate) struct TypedCsrfStateData {
    csrf_token: CsrfToken,
    provider: OauthProvider,
}

impl TypedCsrfState {
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
    ) -> anyhow::Result<(), CsrfStateError> {
        let m = serde_json::to_string(&provider)?;
        let _: () = con.set(self.redis_key(), m)?;
        Ok(())
    }

    pub(crate) fn verify(
        &self,
        con: &mut redis::Connection,
    ) -> Result<OauthProvider, CsrfStateError> {
        let csrf_state: String = con.get(self.redis_key())?;
        serde_json::from_str::<OauthProvider>(csrf_state.as_str())
            .map_err(|_e| CsrfStateError::InvalidCsrfToken)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AuthUrlData {
    pub(crate) authorize_url: TypedAuthUrl,
    pub(crate) csrf_state: TypedCsrfState,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum OauthProviderError {
    #[error("Failed to fetch user profile")]
    FailedToFetchUserProfile,
    // FailedToFetchUserProfile(#[from] anyhow::Error),
}

#[derive(Debug, From, Into, Clone)]
pub(crate) struct OauthUrl(&'static str);

#[async_trait::async_trait]
impl OauthUrlTrait for OauthUrl {
    async fn get_resource<T: DeserializeOwned>(
        &self,
        token: &StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        headers: Option<HeaderMap>,
    ) -> T {
        let headers = headers.unwrap_or_default();
        // TODO: parse to serde as json
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
        let k = serde_json::from_str::<T>(remote_data.as_str()).unwrap();
        k
    }
}

// impl From<String> for OauthUrl {
//     fn from(url: Url) -> Self {
//         Self(url)
//     }
// }

#[async_trait::async_trait]
trait OauthUrlTrait {
    async fn get_resource<T: DeserializeOwned>(
        &self,
        token: &StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        headers: Option<HeaderMap>,
    ) -> T;
}

#[async_trait::async_trait]
impl OauthProviderTrait for GithubConfig {
    fn client(self) -> BasicClient {
        BasicClient::new(
            self.basic_config.client_id,
            Some(self.basic_config.client_secret),
            self.basic_config.auth_url,
            Some(self.basic_config.token_url),
        )
        // This example will be running its own server at localhost:8080.
        // See below for the server implementation.
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"),
        )
    }

    /// Generate the authorization URL to which we'll redirect the user.
    fn generate_auth_url(&self) -> AuthUrlData {
        // let c = self;
        let (authorize_url, csrf_state) = self
            .clone()
            .client()
            .authorize_url(CsrfToken::new_random)
            // This example is requesting access to the user's public repos and email.
            // .add_scope(Scope::new("public_repo".to_string()))
            // .add_scope(Scope::new("read:user".to_string()))
            // .add_scope(Scope::new("user:email".to_string()))
            .add_scopes(self.basic_config.clone().scopes)
            .url();
        AuthUrlData {
            authorize_url: TypedAuthUrl(authorize_url),
            csrf_state: TypedCsrfState(csrf_state),
        }
    }

    async fn fetch_oauth_account(
        &self,
        code: AuthorizationCode,
    ) -> anyhow::Result<User, OauthProviderError> {
        let token_res = self
            .clone()
            .client()
            .exchange_code(code)
            .request_async(async_http_client)
            .await;

        if let Ok(token) = token_res {
            let profile = self
                .user_data_url
                .get_resource::<GithubUserData>(&token, None)
                .await;
            print!("FGREWRTBODY:{profile:?}");

            #[derive(Debug, Deserialize)]
            struct GithubEmail {
                email: String,
                primary: bool,
                verified: bool,
                visibility: Option<String>,
            }

            let user_emails = self.user_data_url.get_resource::<Vec<GithubEmail>>(&token, None).await;
            print!("FGREWRTBODY:{user_emails:?}");

            // Get the primary email or any first
            let primary_email = user_emails
                .iter()
                .filter(|r| r.primary)
                .next()
                .or_else(|| user_emails.first());

            // TODO: First search the database if the user exists, if exists, just update, else create
            // User::find({profile: profile.id, provider: provider});
            let expiration = token.expires_in().unwrap_or(std::time::Duration::new(0, 0));
            let expiration = Duration::from_std(expiration).unwrap_or(Duration::seconds(0));
            let expires_at = Utc::now() + expiration;
            let scopes = self
                .basic_config
                .scopes
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            let account = AccountOauth::builder()
                .id(profile.id.clone())
                .account_type(profile.account_type)
                .provider(OauthProvider::Github)
                .provider_account_id(OauthProvider::Github)
                .access_token(token.access_token().secret().into())
                .refresh_token(token.refresh_token().map(|rf| rf.secret().into()))
                .expires_at(Some(expires_at))
                .token_type(Some(TokenType::Bearer))
                .scopes(scopes)
                .build();

            let autogenerated_id = uuid::Uuid::new_v4().to_string();
            let p = primary_email.map(|p| p.email.to_string());
            let user = User::builder()
                .username(format!("{}-{autogenerated_id}", profile.name))
                .email(p.or(profile.email.unwrap_or_default()))
                .roles(vec![Role::User])
                .accounts(vec![account])
                .age(None)
                .password(None)
                .build();

            // TODO: Search user from db by github id and provider. If present, upsert other attributes, otherwise create
            return Ok(user);
        }

        return Err(OauthProviderError::FailedToFetchUserProfile);
    }
}
