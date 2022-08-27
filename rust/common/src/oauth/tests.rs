use super::{
    api::{Config, Provider},
    cache_storage::{CacheStorage, HashMapCache, RedisCache},
};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use oauth2::http::Uri;

    use crate::{
        configurations::oauth::{OauthCredentials, OauthGithubCredentials, OauthGoogleCredentials},
        oauth::{
            github::{GithubConfig, OauthGithubSettings},
            google::{GoogleConfig, OauthGoogleSettings},
        },
    };

    use super::*;

    #[test]
    fn oauth_api_tests() {
        // let giCrede = OauthCredentials::builder().google(google).
        /*
             GITHUB_CLIENT_ID: '89c19374f7e7b5b35164',
        GITHUB_CLIENT_SECRET: '129488cc92e2d2f91e3a5a024086396c48c65339',
        GOOGLE_CLIENT_ID: '855174209543-6m0f088e55d3mevhnr8bs0qjap8j6g0g.apps.googleusercontent.com',
        GOOGLE_CLIENT_SECRET: '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',

        */
        let github_credentials = OauthGithubCredentials::builder()
            .client_id(String::from("89c19374f7e7b5b35164"))
            .client_secret(String::from("129488cc92e2d2f91e3a5a024086396c48c65339"))
            .build();

        let google_credentials = OauthGoogleCredentials::builder()
            .client_id(String::from(
                "855174209543-6m0f088e55d3mevhnr8bs0qjap8j6g0g.apps.googleusercontent.com",
            ))
            .client_secret(String::from("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"))
            .build();

        let oauth_credentials = OauthCredentials::builder()
            .google(google_credentials.clone())
            .github(github_credentials.clone())
            .build();
        let oauth_github_settings = OauthGithubSettings::builder()
            .base_url("https://oyelowo.test".to_string())
            .credentials(github_credentials)
            .build();
        let oauth_goole_settings = OauthGoogleSettings::builder()
            .base_url("https://oyelowo.test".to_string())
            .credentials(google_credentials)
            .build();
        let github = GithubConfig::new(oauth_github_settings);
        let google = GoogleConfig::new(oauth_goole_settings);

        let providers = Provider::builder().github(github).google(google).build();
        // let cache = HashMap::new();
        let cache_storage = HashMapCache::new();
        let conf = Config::builder()
            .base_url("base_url".to_string())
            .uri(Uri::from_static("src"))
            .provider_configs(providers)
            .cache_storage(cache_storage)
            .build();
            
        let p = conf.fetch_account();

        assert_eq!(4, 4);
    }
}


