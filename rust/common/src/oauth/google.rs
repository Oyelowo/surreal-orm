use chrono::{Duration, Utc};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, PkceCodeVerifier, RedirectUrl,
    RevocationUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

use crate::configurations::oauth::OauthGoogleCredentials;

use super::{
    utils::{get_redirect_url, OauthConfig, OauthProviderTrait, OauthResult, OauthUrl},
    AccountOauth, OauthProvider, TokenType,
};

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
pub struct GoogleConfig {
    basic_config: OauthConfig,
}

impl GoogleConfig {
    pub fn new(base_url: &String, credentials: OauthGoogleCredentials) -> Self {
        let basic_config = OauthConfig::builder()
            .client_id(ClientId::new(credentials.client_id))
            .client_secret(ClientSecret::new(credentials.client_secret))
            .auth_url(
                AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                    .expect("Invalid authorization endpoint URL"),
            )
            .token_url(
                TokenUrl::new("https://www.googleapis.com/oauth2/v4/token".to_string())
                    .expect("Invalid token endpoint URL"),
            )
            .redirect_url(
                RedirectUrl::new(get_redirect_url(base_url)).expect("Invalid redirect URL"),
            )
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
    type OauthResponse = AccountOauth;

    fn basic_config(&self) -> OauthConfig {
        self.basic_config.to_owned()
    }

    /// Fetch oauth account. You can override this method
    ///
    /// # Errors:
    /// Exhange token problem
    ///
    /// Problem fetching user's oauth data
    ///
    /// This function will return an error if.
    ///
    /// Oauth verification fails during token/authorization code exchange with the provider
    ///
    /// User data fetching fails
    async fn fetch_oauth_account(
        &self,
        code: AuthorizationCode,
        pkce_code_verifier: PkceCodeVerifier,
    ) -> OauthResult<Self::OauthResponse> {
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
    use crate::oauth::utils::OauthConfigTrait;
    use multimap::MultiMap;

    #[test]
    fn it_should_properly_generate_auth_url() {
        let client_id = String::from("oyelowo_1234");
        let client_secret = String::from("secret_xxx");
        let credentials = OauthGoogleCredentials {
            client_id: client_id.clone(),
            client_secret: client_secret.clone(),
        };

        const HOST_NAME: &str = "oyelowo.test";
        let base_url = format!("http://{HOST_NAME}");

        let google_config = GoogleConfig::new(&base_url, credentials).basic_config();
        let auth_url_data = google_config.clone().generate_auth_url();

        let auth_url = auth_url_data.authorize_url.into_inner().clone();
        let hash_query: MultiMap<_, _> = auth_url.query_pairs().into_owned().collect();

        assert_eq!(auth_url.as_str().len(), 285);

        let state = hash_query.get("state").unwrap();
        let code_challenge = hash_query.get("code_challenge").unwrap();
        assert_eq!(state, auth_url_data.evidence.csrf_token.secret().as_str());
        assert_eq!(OauthProvider::Google, auth_url_data.evidence.provider);
        assert_eq!(state.len(), 22);
        assert_eq!(code_challenge.len(), 43);
        assert_eq!(
            auth_url.as_str(),
            format!(
                "https://accounts.google.com/o/oauth2/v2/auth?\
            response_type=code&client_id={client_id}&\
            state={state}&code_challenge={code_challenge}&code_challenge_method=S256&\
            redirect_uri=http%3A%2F%2F{HOST_NAME}%2Fapi%2Foauth%2Fcallback&scope=profile+email"
            )
        );
    }
}
