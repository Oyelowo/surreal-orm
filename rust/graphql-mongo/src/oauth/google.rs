use chrono::{Duration, Utc};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeChallengeMethod, PkceCodeVerifier,
    RedirectUrl, RevocationUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

use super::utils::{
    AuthUrlData, CsrfState, OauthConfig, OauthError, OauthProviderTrait, OauthUrl,
    RedirectUrlReturned, REDIRECT_URL,
};
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
    name: String,
    given_name: String,
    family_name: String,
    picture: String,
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
        let k = CsrfToken::new_random;
        let basic_config = OauthConfig::builder()
            .client_id(ClientId::new(
                "855174209543-i23grd1ts6qbq568dfl43hla7hv9cn4u.apps.googleusercontent.com"
                    .to_string(),
            ))
            .client_secret(ClientSecret::new(
                "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".into(),
            ))
            .auth_url(
                AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                    .expect("Invalid authorization endpoint URL"),
            )
            .token_url(
                TokenUrl::new("https://www.googleapis.com/oauth2/v4/token".to_string())
                    .expect("Invalid token endpoint URL"),
            )
            .redirect_url(RedirectUrl::new(REDIRECT_URL.to_string()).expect("Invalid redirect URL"))
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
    fn client(self) -> BasicClient {
        let client = BasicClient::new(
            self.basic_config.client_id,
            Some(self.basic_config.client_secret),
            self.basic_config.auth_url,
            Some(self.basic_config.token_url),
        )
        .set_redirect_uri(self.basic_config.redirect_url);

        if let Some(url) = self.basic_config.revocation_url {
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
            .add_scopes(self.basic_config.clone().scopes)
            .set_pkce_challenge(pkce_code_challenge)
            .url();

        let csrf_state = CsrfState {
            csrf_token: csrf_token,
            provider: self.basic_config.provider,
            pkce_code_verifier: Some(pkce_code_verifier),
        };

        AuthUrlData {
            authorize_url: RedirectUrlReturned(authorize_url),
            csrf_state_data: csrf_state,
            // csrf_state: CsrfState(csrf_state),
        }
    }

    async fn fetch_oauth_account(
        &self,
        code: AuthorizationCode,
        pkce_code_verifier: Option<PkceCodeVerifier>,
    ) -> anyhow::Result<User, OauthError> {
        let token = self
            .clone()
            .client()
            .exchange_code(code)
            .set_pkce_verifier(pkce_code_verifier.expect("Must be provided for google"))
            .request_async(async_http_client)
            .await
            .map_err(|e| OauthError::TokenFetchFailed(e.to_string()))?;

        // let profile = OauthUrl("https://www.googleapis.com/auth/userinfo.profile")
        let profile = OauthUrl("https://www.googleapis.com/oauth2/v3/userinfo")
            .get_resource::<GoogleUserData>(&token, None)
            .await?;
        print!("Profile{:?}", profile);
        
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
        print!("scopes{:?}", scopes);
        
        let account = AccountOauth::builder()
            .id(profile.sub.to_string())
            .account_type("oauth".into())
            .provider(OauthProvider::Google)
            .provider_account_id(OauthProvider::Google)
            .access_token(token.access_token().secret().into())
            .refresh_token(token.refresh_token().map(|rf| rf.secret().into()))
            .expires_at(Some(expires_at))
            .token_type(Some(TokenType::Bearer))
            .scopes(scopes)
            .build();

        let autogenerated_id = uuid::Uuid::new_v4().to_string();
        let user = User::builder()
            .username(format!("{}-{}", profile.sub, autogenerated_id))
            .email(Some(profile.email))
            .roles(vec![Role::User])
            .accounts(vec![account])
            .age(None)
            .password(None)
            .build();
        println!("UUUUSER, {user:?}");
        Ok(user)
    }
}
