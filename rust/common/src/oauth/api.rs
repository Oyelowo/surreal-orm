use oauth2::http::Uri;
use typed_builder::TypedBuilder;
use url::Url;

use super::{
    cache_storage::CacheStorage,
    github::GithubConfig,
    google::GoogleConfig,
    utils::{AuthUrlData, OauthError, OauthProviderTrait, OauthResult, RedirectUrlReturned},
    AccountOauth, OauthProvider,
};

#[derive(Debug, TypedBuilder)]
pub struct Provider {
    #[builder(default, setter(strip_option))]
    github: Option<GithubConfig>,
    #[builder(default, setter(strip_option))]
    google: Option<GoogleConfig>,
}

#[derive(Debug, TypedBuilder)]
pub struct Config<T: CacheStorage> {
    base_url: String,
    uri: Uri,
    provider_configs: Provider,
    cache_storage: T,
}

pub async fn fetch_account<T>(config: Config<T>) -> OauthResult<AccountOauth>
where
    T: CacheStorage,
{
    let Config {
        base_url,
        uri,
        cache_storage: cache,
        provider_configs,
    } = config;
    let redirect_url = Url::parse(&format!("{base_url}{uri}")).map(RedirectUrlReturned)?;

    let code = redirect_url.authorization_code().ok_or(
        OauthError::AuthorizationCodeNotFoundInRedirectUrl(uri.to_string()),
    )?;

    // make .verify give me back both the csrf token and the provider
    let csrf_token = redirect_url
        .csrf_token()
        .ok_or(OauthError::CsrfTokenNotFoundInRedirectUrl(uri.to_string()))?;

    // let cache = cg::RedisCache(redis.clone());
    let evidence = AuthUrlData::verify_csrf_token(csrf_token, cache)
        .await
        .unwrap()
        .evidence;

    let account_oauth = match evidence.provider {
        OauthProvider::Github => {
            provider_configs
                .github
                .expect("You must provide github credentials")
                .fetch_oauth_account(code, evidence.pkce_code_verifier)
                .await
        }
        OauthProvider::Google => {
            provider_configs
                .google
                .expect("You must provide google oauth credentials")
                .fetch_oauth_account(code, evidence.pkce_code_verifier)
                .await
        }
    }?;

    Ok(account_oauth)
}
