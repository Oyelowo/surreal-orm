use chrono::{Duration, Utc};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

use super::utils::{
    AuthUrlData, OauthConfig, OauthProviderError, OauthProviderTrait, OauthUrl, TypedAuthUrl,
    TypedCsrfState, REDIRECT_URL,
};
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

#[derive(Debug, Deserialize)]
struct GithubEmail {
    email: String,
    primary: bool,
    verified: bool,
    visibility: Option<String>,
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
            // scopes: vec![GithubScopes::UserEmail, GithubScopes::PublicRepo]
            //     .iter()
            //     .map(|s| Scope::new(serde_json::to_string(s).unwrap()))
            //     .collect::<Vec<Scope>>(),
        };
        Self {
            basic_config,
            user_data_url: OauthUrl("https://api.github.com/user"),
            user_emails_url: OauthUrl("https://api.github.com/user/emails"),
        }
    }
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
        .set_redirect_uri(RedirectUrl::new(REDIRECT_URL.to_string()).expect("Invalid redirect URL"))
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

            let user_emails = self
                .user_data_url
                .get_resource::<Vec<GithubEmail>>(&token, None)
                .await;
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
