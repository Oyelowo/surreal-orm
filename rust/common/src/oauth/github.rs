use chrono::{Duration, Utc};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, PkceCodeVerifier, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

use crate::configurations::oauth::OauthGithubCredentials;

use super::{
    utils::{get_redirect_url, OauthConfig, OauthProviderTrait, OauthResult, OauthUrl},
    AccountOauth, OauthProvider, TokenType,
};

#[derive(Debug, Deserialize, Serialize)]
struct GithubUserData {
    id: u32,
    login: String,
    name: Option<String>,
    email: Option<String>,
    avatar_url: Option<String>,
    gravatar_id: Option<String>,
    url: Option<String>,
    location: Option<String>,
    // Many other irrelevant fields discarded
}

#[derive(Debug, Deserialize)]
struct GithubEmail {
    email: String,
    primary: bool,
    verified: bool,
    // visibility: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GithubConfig {
    pub basic_config: OauthConfig,
}

impl GithubConfig {
    /// Creates a new [`GithubConfig`].
    /// Takes in the settings
    pub fn new(base_url: &String, credentials: OauthGithubCredentials) -> Self {
        // let env = OauthGithubCredentials::default();
        let basic_config = OauthConfig {
            client_id: ClientId::new(credentials.client_id),
            client_secret: ClientSecret::new(credentials.client_secret),
            auth_url: AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                .expect("Invalid authorization endpoint URL"),
            token_url: TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                .expect("Invalid token endpoint URL"),
            redirect_url: RedirectUrl::new(get_redirect_url(base_url))
                .expect("Invalid redirect URL"),
            scopes: vec![
                Scope::new("public_repo".into()),
                Scope::new("read:user".into()),
                Scope::new("user:email".into()),
            ],
            provider: OauthProvider::Github,
            revocation_url: None,
        };
        Self { basic_config }
    }
}

#[async_trait::async_trait]
impl OauthProviderTrait for GithubConfig {
    type OauthResponse = AccountOauth;

    fn basic_config(&self) -> OauthConfig {
        self.basic_config.to_owned()
    }

    async fn fetch_oauth_account(
        &self,
        code: AuthorizationCode,
        pkce_code_verifier: PkceCodeVerifier,
    ) -> OauthResult<Self::OauthResponse> {
        let token = self.exchange_token(code, pkce_code_verifier).await?;

        let profile = OauthUrl("https://api.github.com/user")
            .fetch_resource::<GithubUserData>(&token, None)
            .await?;

        let user_emails = OauthUrl("https://api.github.com/user/emails")
            .fetch_resource::<Vec<GithubEmail>>(&token, None)
            .await?;

        let expiration = token.expires_in().unwrap_or(std::time::Duration::new(0, 0));
        let expiration = Duration::from_std(expiration).unwrap_or_else(|_| Duration::seconds(0));
        let expires_at = Utc::now() + expiration;
        let scopes = self
            .basic_config()
            .scopes
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        // Get the primary email or any first
        let primary_email = user_emails
            .iter()
            .find(|r| r.primary)
            .or_else(|| user_emails.first());

        let email = primary_email.map(|p| p.email.to_string());
        let account = AccountOauth::builder()
            .id(profile.id.to_string())
            .display_name(Some(profile.login.clone()))
            .provider(OauthProvider::Github)
            .provider_account_id(OauthProvider::Github)
            .access_token(token.access_token().secret().into())
            .refresh_token(token.refresh_token().map(|rf| rf.secret().into()))
            .expires_at(Some(expires_at))
            .token_type(Some(TokenType::Bearer))
            .scopes(scopes)
            .email(email.or(profile.email))
            .email_verified(primary_email.map(|p| p.verified).unwrap_or(false))
            .build();

        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oauth::utils::OauthConfigTrait;
    use multimap::MultiMap;
    #[cfg(test)]
    use pretty_assertions::{assert_eq, assert_str_eq};

    #[test]
    fn it_should_properly_generate_auth_url() {
        let client_id = String::from("oyelowo_1234");
        let client_secret = String::from("secret_xxx");
        let credentials = OauthGithubCredentials {
            client_id: client_id.clone(),
            client_secret: client_secret.clone(),
        };

        const HOST_NAME: &str = "oyelowo.test";
        let base_url = format!("http://{HOST_NAME}");

        let google_config = GithubConfig::new(&base_url, credentials).basic_config();
        let auth_url_data = google_config.clone().generate_auth_url();

        let auth_url = auth_url_data.authorize_url.into_inner();
        let hash_query: MultiMap<_, _> = auth_url.query_pairs().into_owned().collect();

        let state = hash_query.get("state").unwrap();
        let code_challenge = hash_query.get("code_challenge").unwrap();
        assert_eq!(auth_url.as_str().len(), 304);
        assert_eq!(state, auth_url_data.evidence.csrf_token.secret().as_str());
        assert_eq!(OauthProvider::Github, auth_url_data.evidence.provider);
        assert_eq!(state.len(), 22);
        assert_eq!(code_challenge.len(), 43);
        assert_str_eq!(
            auth_url.as_str(),
            format!(
                "https://github.com/login/oauth/authorize?\
            response_type=code&client_id={client_id}&\
            state={state}&code_challenge={code_challenge}&code_challenge_method=S256&\
            redirect_uri=http%3A%2F%2F{HOST_NAME}%2Fapi%2Foauth%2Fcallback&scope=public_repo+read%3Auser+user%3Aemail"
            )
        );
    }
}
