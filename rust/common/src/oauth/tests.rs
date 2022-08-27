use super::{
    api::{Config, Provider},
    cache_storage::{CacheStorage, HashMapCache, RedisCache},
};

#[cfg(test)]
mod tests {
    use async_std;
    use httpmock::prelude::*;
    use std::collections::HashMap;

    use oauth2::http::Uri;
    use pretty_assertions::{assert_str_eq, assert_eq};

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

        // let oauth_credentials = OauthCredentials::builder()
        //     .google(google_credentials.clone())
        //     .github(github_credentials.clone())
        //     .build();
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
        let mut cache_storage = HashMapCache::new();
        // {

        // }

        async_std::task::block_on(async {
            // use httpmock::{Mock, MockServer};
            // use std::time::{Duration, SystemTime};
            // use tokio_test::block_on;
            let _ = env_logger::try_init;
            // Arrange
            let server = MockServer::start_async().await;
            // let mock = server
            //     .mock_async(|when, then| {
            //         when.path_contains("oyelowo");
            //         then.status(200);
            //     })
            //     .await;

            let conf = Config::builder()
                .base_url("base_url".to_string())
                .uri(Uri::from_static("/oauth/callback"))
                .provider_configs(providers)
                .cache_storage(&mut cache_storage)
                .build();
            let p = conf
                .initiate_oauth(crate::oauth::OauthProvider::Google)
                .await
                .unwrap();
            // let p = conf.fetch_account().await.unwrap();

            // let k = o.0.insert("key".to_string(), "query".to_string());
            // mock.assert_async().await;
            assert_eq!(4, 4);
            // let x = cache_storage.clone().0;
            let s = HashMap::from([
                ("1".to_string(), "2".to_string()),
                ("3".to_string(), "4".to_string()),
            ]);
            assert_eq!(cache_storage.0, s);
        });
    }
}

/*
async_std::task::block_on(async {
    use std::time::{SystemTime, Duration};
    use httpmock::{MockServer, Mock};
    use tokio_test::block_on;
    let _ = env_logger::try_init();

    // Arrange
    let server = MockServer::start_async().await;

    let mock = Mock::new()
        .return_status(200)
        .create_on_async(&server)
        .await;

    // Act
    let response = isahc::get_async(server.url("/delay")).await.unwrap();

    // Assert
    mock.assert_async().await;
});
*/
