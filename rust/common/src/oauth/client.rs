use std::fmt::Debug;
use typed_builder::TypedBuilder;
use url::Url;

use super::{
    account::{OauthProvider, UserAccount},
    cache_storage::CacheStorage,
    error::{OauthError, OauthResult},
    github::GithubConfig,
    google::GoogleConfig,
    oauth_config::{OauthConfigTrait, OauthProviderTrait},
    urls::{AuthUrlData, RedirectUrlReturned},
};

#[derive(Debug, TypedBuilder, Clone)]
pub struct OauthClient {
    #[builder(default, setter(strip_option))]
    github: Option<GithubConfig>,
    #[builder(default, setter(strip_option))]
    google: Option<GoogleConfig>,
}

impl OauthClient {
    pub async fn generate_auth_url_data<C>(
        &mut self,
        oauth_provider: OauthProvider,
        cache_storage: &mut C,
    ) -> OauthResult<AuthUrlData>
    where
        C: CacheStorage + Debug,
    {
        let auth_url_data = match oauth_provider {
            OauthProvider::Github => self
                .github
                .as_ref()
                .expect("no github config")
                .basic_config()
                .generate_auth_url(),
            OauthProvider::Google => self
                .google
                .as_ref()
                .expect("no google config")
                .basic_config()
                .generate_auth_url(),
        };

        auth_url_data.save(cache_storage).await?;
        Ok(auth_url_data)
    }

    pub async fn fetch_account<C>(
        &mut self,
        redirect_url: Url,
         cache_storage: &mut C,
    ) -> OauthResult<UserAccount>
    where
        C: CacheStorage + Debug,
    {
        let redirect_url_wrapped = RedirectUrlReturned::new(redirect_url.clone());

        let code = redirect_url_wrapped.get_authorization_code().ok_or(
            OauthError::AuthorizationCodeNotFoundInRedirectUrl(redirect_url.to_string()),
        )?;

        // make .verify give me back both the csrf token and the provider
        let csrf_token = redirect_url_wrapped.get_csrf_token().ok_or(
            OauthError::CsrfTokenNotFoundInRedirectUrl(redirect_url.to_string()),
        )?;

        let evidence = AuthUrlData::verify_csrf_token(csrf_token, cache_storage)
            .await?
            .evidence;

        let account_oauth = match evidence.provider {
            OauthProvider::Github => {
                self.github
                    .as_ref()
                    .expect("You must provide github credentials")
                    .fetch_oauth_account(code, evidence.pkce_code_verifier)
                    .await
            }
            OauthProvider::Google => {
                self.google
                    .as_ref()
                    .expect("You must provide google oauth credentials")
                    .fetch_oauth_account(code, evidence.pkce_code_verifier)
                    .await
            }
        }?;

        Ok(account_oauth)
    }
}
