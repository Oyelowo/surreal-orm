use multimap::MultiMap;
use serde_json::json;
use std::collections::HashMap;
use surf::{Client, StatusCode};
use url::Url;
use wiremock::matchers::{
    any, body_json, body_partial_json, body_string, method, path, path_regex, PathExactMatcher,
};
use wiremock::{Mock, MockServer, ResponseTemplate};

use oauth2::http::Uri;
use pretty_assertions::{assert_eq, assert_str_eq};

use super::{
    api::{Config, Provider},
    cache_storage::{CacheStorage, HashMapCache, RedisCache},
};
use crate::{
    configurations::oauth::{OauthCredentials, OauthGithubCredentials, OauthGoogleCredentials},
    oauth::{
        github::{GithubConfig, OauthGithubSettings},
        google::{GoogleConfig, OauthGoogleSettings},
    },
};

#[tokio::test]
async fn hello_reqwest() {
    // Arrange
    // Arrange
    let expected_body = json!({ "a": 1, "c": { "e": 2 } });
    let body = json!({ "a": 1, "b": 2, "c": { "d": 1, "e": 2 } });

    let mock_server = MockServer::start().await;
    let response = ResponseTemplate::new(200);
    // let mock = Mock::given(method("GET"))
    let mock = Mock::given(any())
        // .and(path_regex("/[a-z]+[0-9]+/v2"))
        // .and(path_regex("/[a-z]+[0-9]+/v2"))
        // .and(path_regex("/[a-z]+2/v2"))
        // .and(path("/o/oauth2/v2/auth"))
        .and(PathExactMatcher::new("https://api.github.com/user"))
        // .and(PathExactMatcher::new("o/oauth2/v2/auth"))
        // .and(PathExactMatcher::new("google"))
        // .and(path("accounts.google.com/o/oauth2/v2/auth"))
        .and(body_partial_json(expected_body))
        .respond_with(response);
    mock_server.register(mock).await;

    // ////////
    // let giCrede = OauthCredentials::builder().google(google).
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
    let github = GithubConfig::new(oauth_github_settings, mock_server.uri());
    let google = GoogleConfig::new(oauth_goole_settings);

    let providers = Provider::builder().github(github).google(google).build();
    // let cache = HashMap::new();
    let mut cache_storage = HashMapCache::new();

    // Act
    // let response = surf::get(mock_server.uri()).body(body).await.unwrap();
    // let response = surf::get(format!("{}/o/oauth2/v2/auth", mock_server.uri()))
    // let response = surf::get(format!("https://api.github.com/user"))
    //     .body(body)
    //     .await
    //     .unwrap();

    let conf = Config::builder().provider_configs(providers).build();
    conf.generate_auth_url_data(super::OauthProvider::Github).save(&mut cache_storage).await.unwrap();

    // Assert
    assert_eq!(cache_storage.0, HashMap::new());
    // providers
    // assert_eq!(mock_server.address().to_string(), mock_server.uri());
    // assert_eq!(response.status(), StatusCode::Ok);
    // assert_eq!(response.status(), StatusCode::Ok);
}
