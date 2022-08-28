use std::fmt::Debug;
use typed_builder::TypedBuilder;
use url::Url;

use super::{
    cache_storage::CacheStorage,
    github::GithubConfig,
    google::GoogleConfig,
    utils::{
        AuthUrlData, OauthConfigTrait, OauthError, OauthProviderTrait, OauthResult,
        RedirectUrlReturned,
    },
    AccountOauth, OauthProvider,
};

#[derive(Debug, TypedBuilder)]
pub struct OauthClient<'a, C>
where
    C: CacheStorage + Debug,
{
    #[builder(default, setter(strip_option))]
    github: Option<&'a GithubConfig>,
    #[builder(default, setter(strip_option))]
    google: Option<&'a GoogleConfig>,
    cache_storage: &'a mut C,
}

// #[async_trait::async_trait]
impl<'a, C> OauthClient<'a, C>
where
    C: CacheStorage + Debug + 'a,
{
    pub async fn generate_auth_url_data(
        &mut self,
        oauth_provider: OauthProvider,
    ) -> OauthResult<AuthUrlData> {
        let auth_url_data = match oauth_provider {
            OauthProvider::Github => self
                .github
                .expect("no github config")
                .basic_config()
                .generate_auth_url(),
            OauthProvider::Google => self
                .google
                .expect("no google config")
                .basic_config()
                .generate_auth_url(),
        };

        auth_url_data.save(self.cache_storage).await?;
        Ok(auth_url_data)
    }

    pub async fn fetch_account(&self, redirect_url: Url) -> OauthResult<AccountOauth> {
        // let redirect_url = Url::parse(&format!("{base_url}{uri}")).map(RedirectUrlReturned)?;
        let redirect_url_wrapped = RedirectUrlReturned(redirect_url.clone());

        let code = redirect_url_wrapped.get_authorization_code().ok_or(
            OauthError::AuthorizationCodeNotFoundInRedirectUrl(redirect_url.to_string()),
        )?;

        // make .verify give me back both the csrf token and the provider
        let csrf_token = redirect_url_wrapped.get_csrf_token().ok_or(
            OauthError::CsrfTokenNotFoundInRedirectUrl(redirect_url.to_string()),
        )?;

        let evidence = AuthUrlData::verify_csrf_token(csrf_token, self.cache_storage)
            .await?
            .evidence;

        let account_oauth = match evidence.provider {
            OauthProvider::Github => {
                self.github
                    .expect("You must provide github credentials")
                    .fetch_oauth_account(code, evidence.pkce_code_verifier)
                    .await
            }
            OauthProvider::Google => {
                self.google
                    .expect("You must provide google oauth credentials")
                    .fetch_oauth_account(code, evidence.pkce_code_verifier)
                    .await
            }
        }?;

        Ok(account_oauth)
    }
}
