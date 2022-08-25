use chrono::{Duration, Utc};
use common::configurations::oauth::OauthGoogleConfigs;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, PkceCodeVerifier, RedirectUrl,
    RevocationUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

use super::utils::{get_redirect_url, OauthConfig, OauthProviderTrait, OauthResult, OauthUrl};
use crate::app::user::{AccountOauth, OauthProvider, TokenType};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oauth::utils::{OauthConfigTrait, RedirectUrlReturned};
    use multimap::MultiMap;
    use std::env;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        // env::set_var("OAUTH_GOOGLE_CLIENT_ID", "1234");
        // env::set_var("OAUTH_GOOGLE_CLIENT_SECRET", "string_secret");
        // env::set_var("APP_PORT", "5000");
        // env::set_var("APP_PORT", "5000");

        env::set_var("APP_ENVIRONMENT", "local");
        env::set_var("APP_HOST", "random_APP_HOST");
        env::set_var("APP_PORT", "5000");
        env::set_var("APP_EXTERNAL_BASE_URL", "http://oyelowo.test");
        env::set_var("OAUTH_GITHUB_CLIENT_ID", "random_OAUTH_GITHUB_CLIENT_ID");
        env::set_var(
            "OAUTH_GITHUB_CLIENT_SECRET",
            "random_OAUTH_GITHUB_CLIENT_SECRET",
        );
        env::set_var("OAUTH_GOOGLE_CLIENT_ID", "random_OAUTH_GOOGLE_CLIENT_ID");
        env::set_var(
            "OAUTH_GOOGLE_CLIENT_SECRET",
            "random_OAUTH_GOOGLE_CLIENT_SECRET",
        );
        env::set_var("MONGODB_NAME", "random_MONGODB_NAME");
        env::set_var("MONGODB_USERNAME", "random_MONGODB_USERNAME");
        env::set_var("MONGODB_PASSWORD", "random_MONGODB_PASSWORD");
        env::set_var("MONGODB_ROOT_USERNAME", "random_MONGODB_ROOT_USERNAME");
        env::set_var("MONGODB_ROOT_PASSWORD", "random_MONGODB_ROOT_PASSWORD");
        env::set_var("MONGODB_HOST", "random_MONGODB_HOST");
        env::set_var("MONGODB_SERVICE_NAME", "random_MONGODB_SERVICE_NAME");
        env::set_var("MONGODB_STORAGE_CLASS", "random_MONGODB_STORAGE_CLASS");
        env::set_var("MONGODB_PORT", "27017");
        env::set_var("REDIS_USERNAME", "random_REDIS_USERNAME");
        env::set_var("REDIS_PASSWORD", "random_REDIS_PASSWORD");
        env::set_var("REDIS_HOST", "random_REDIS_HOST");
        env::set_var("REDIS_SERVICE_NAME", "random_REDIS_SERVICE_NAME");
        env::set_var(
            "REDIS_SERVICE_NAME_MASTER",
            "random_REDIS_SERVICE_NAME_MASTER",
        );
        env::set_var("REDIS_PORT", "6379");

        let google_config = GoogleConfig::new().basic_config();
        let auth_url_dataa = google_config.clone().generate_auth_url();

        let auth_url = auth_url_dataa.authorize_url.clone().0;
        let hash_query: MultiMap<_, _> = auth_url_dataa
            .authorize_url
            .into_inner()
            .query_pairs()
            .into_owned()
            .collect();

        assert_eq!(auth_url.as_str().len(), 302);

        let state = hash_query.get("state").unwrap();
        let code_challenge = hash_query.get("code_challenge").unwrap();
        assert_eq!(state.len(), 22);
        assert_eq!(code_challenge.len(), 43);
        assert_eq!(
            auth_url.as_str(),
            format!(
                "https://accounts.google.com/o/oauth2/v2/auth?\
            response_type=code&client_id=random_OAUTH_GOOGLE_CLIENT_ID&\
            state={state}&code_challenge={code_challenge}&code_challenge_method=S256&\
            redirect_uri=http%3A%2F%2Foyelowo.test%2Fapi%2Foauth%2Fcallback&scope=profile+email"
            )
        );
    }
}
