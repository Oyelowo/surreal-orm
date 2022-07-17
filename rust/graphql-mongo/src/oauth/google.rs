use chrono::{Duration, Utc};
use common::configurations::oauth::OauthGoogleConfigs;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, PkceCodeVerifier, RedirectUrl,
    RevocationUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

use super::utils::{get_redirect_url, OauthConfig, OauthProviderTrait, OauthResult, OauthUrl};
use crate::app::user::{AccountOauth, OauthProvider, Role, TokenType, User};

#[derive(Debug, Deserialize, Serialize)]
enum GoogleScopes {
    #[serde(rename = "public_repo")]
    PublicRepo,
    #[serde(rename = "user:email")]
    UserEmail,
}

#[derive(Debug, Deserialize, Serialize)]
struct GoogleUserData {
    sub: String,
    name: Option<String>,
    given_name: Option<String>,
    family_name: Option<String>,
    picture: Option<String>,
    email: String,
    email_verified: bool,
    locale: String,
}

#[derive(Debug, Clone)]
pub(crate) struct GoogleConfig {
    basic_config: OauthConfig,
}

impl GoogleConfig {
    pub fn new() -> Self {
        let env = OauthGoogleConfigs::default();

        let basic_config = OauthConfig::builder()
            .client_id(ClientId::new(env.client_id))
            .client_secret(ClientSecret::new(env.client_secret))
            .auth_url(
                AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                    .expect("Invalid authorization endpoint URL"),
            )
            .token_url(
                TokenUrl::new("https://www.googleapis.com/oauth2/v4/token".to_string())
                    .expect("Invalid token endpoint URL"),
            )
            .redirect_url(RedirectUrl::new(get_redirect_url()).expect("Invalid redirect URL"))
            .scopes(vec![
                Scope::new("profile".into()),
                Scope::new("email".into()),
                // Scope::new("openid".into()),
                // Scope::new("https://www.googleapis.com/auth/plus.me".into()),
                // Scope::new("https://www.googleapis.com/auth/calendar".into()),
            ])
            .provider(OauthProvider::Google)
            .revocation_url(
                RevocationUrl::new("https://oauth2.googleapis.com/revoke".into()).expect("msg"),
            )
            .build();
        Self { basic_config }
    }
}

#[async_trait::async_trait]
impl OauthProviderTrait for GoogleConfig {
    fn basic_config(&self) -> OauthConfig {
        self.basic_config.to_owned()
    }

    async fn fetch_oauth_account(
        &self,
        code: AuthorizationCode,
        pkce_code_verifier: PkceCodeVerifier,
    ) -> OauthResult<AccountOauth> {
        let token = self.exchange_token(code, pkce_code_verifier).await?;

        let profile = OauthUrl("https://www.googleapis.com/oauth2/v3/userinfo")
            .fetch_resource::<GoogleUserData>(&token, None)
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

        let account = AccountOauth::builder()
            .id(profile.sub.to_string())
            .provider(OauthProvider::Google)
            .provider_account_id(OauthProvider::Google)
            .access_token(token.access_token().secret().into())
            .refresh_token(token.refresh_token().map(|rf| rf.secret().into()))
            .expires_at(Some(expires_at))
            .token_type(Some(TokenType::Bearer))
            .scopes(scopes)
            .email(Some(profile.email))
            .email_verified(profile.email_verified)
            .display_name(profile.name.or(profile.given_name).or(profile.family_name))
            .build();

        Ok(account)
    }
}
