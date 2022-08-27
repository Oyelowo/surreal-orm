use oauth2::http::Uri;
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

#[derive(Debug, TypedBuilder, Clone)]
pub struct Provider {
    #[builder(default, setter(strip_option))]
    github: Option<GithubConfig>,
    #[builder(default, setter(strip_option))]
    google: Option<GoogleConfig>,
}

#[derive(Debug, TypedBuilder)]
pub struct Config {
    base_url: String,
    uri: Uri,
    provider_configs: Provider,
    // cache_storage: &'a mut Cache,
}

// #[async_trait::async_trait]
impl Config {
    // pub async fn fetch_account<T>(config: Config<T>) -> OauthResult<AccountOauth>
    // where
    // T: CacheStorage,
    pub async fn fetch_account(
        &self,
        cache_storage: impl CacheStorage,
    ) -> OauthResult<AccountOauth> {
        let Config {
            base_url,
            uri,
            // cache_storage: cache,
            provider_configs,
            ..
        } = self;
        let redirect_url = Url::parse(&format!("{base_url}{uri}")).map(RedirectUrlReturned)?;

        let code = redirect_url.authorization_code().ok_or(
            OauthError::AuthorizationCodeNotFoundInRedirectUrl(uri.to_string()),
        )?;

        // make .verify give me back both the csrf token and the provider
        let csrf_token = redirect_url
            .csrf_token()
            .ok_or(OauthError::CsrfTokenNotFoundInRedirectUrl(uri.to_string()))?;

        // let cache = cg::RedisCache(redis.clone());
        let evidence = AuthUrlData::verify_csrf_token(csrf_token, cache_storage)
            .await
            .unwrap()
            .evidence;

        let account_oauth = match evidence.provider {
            OauthProvider::Github => {
                provider_configs
                    .github
                    .as_ref()
                    .expect("You must provide github credentials")
                    .fetch_oauth_account(code, evidence.pkce_code_verifier)
                    .await
            }
            OauthProvider::Google => {
                provider_configs
                    .google
                    .as_ref()
                    .expect("You must provide google oauth credentials")
                    .fetch_oauth_account(code, evidence.pkce_code_verifier)
                    .await
            }
        }?;

        Ok(account_oauth)
    }

    pub async fn initiate_oauth(
        self,
        oauth_provider: OauthProvider,
        mut cache_storage: impl CacheStorage,
    ) -> OauthResult<AuthUrlData> {
        // self.provider_configs.github.unwrap().basic_config().generate_auth_url()
        let Provider { github, google } = self.provider_configs.clone();
        let auth_url_data = match oauth_provider {
            OauthProvider::Github => github
                .expect("no github config")
                .basic_config()
                .generate_auth_url(),
            OauthProvider::Google => google
                .expect("no google config")
                .basic_config()
                .generate_auth_url(),
        };

        // let cache = cg::RedisCache(redis.clone());
        // let p = self.cache_storage;
        auth_url_data.save(&mut cache_storage).await?;
        Ok(auth_url_data)
    }
}

/*
   let auth_url_data = match oauth_provider {
       OauthProvider::Github => GithubConfig::new().basic_config().generate_auth_url(),
       OauthProvider::Google => GoogleConfig::new().basic_config().generate_auth_url(),
   };

   let cache = cg::RedisCache(redis.clone());

   auth_url_data
       .save(cache)
*/

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn internal() {
//         assert_eq!(4, 2);
//     }
// }
