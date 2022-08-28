use multimap::MultiMap;
use serde_json::json;
use std::collections::HashMap;
use std::fmt::Debug;
use surf::{Client, StatusCode};
use url::Url;
use wiremock::matchers::{
    any, body_json, body_partial_json, body_string, method, path, path_regex, PathExactMatcher,
};
use wiremock::{Mock, MockServer, ResponseTemplate};

use oauth2::http::Uri;
use pretty_assertions::{assert_eq, assert_str_eq};

use super::{
    client::OauthClient,
    cache_storage::{CacheStorage, HashMapCache, RedisCache},
};
use crate::oauth::utils::AuthUrlData;
use crate::{
    configurations::oauth::{OauthCredentials, OauthGithubCredentials, OauthGoogleCredentials},
    oauth::{
        github::GithubConfig,
        google::{GoogleConfig, OauthGoogleSettings},
    },
};

fn prepare_client() -> (GithubConfig, GoogleConfig) {
    let base_url = String::from("https://oyelowo.test");
    let github_creds = OauthGithubCredentials::builder()
        .client_id(String::from("89c19374f7e7b5b35164"))
        .client_secret(String::from("129488cc92e2d2f91e3a5a024086396c48c65339"))
        .build();

    let google_creds = OauthGoogleCredentials {
        client_id: "855174209543-6m0f088e55d3mevhnr8bs0qjap8j6g0g.apps.googleusercontent.com"
            .to_string(),
        client_secret: "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".to_string(),
    };

    let github = GithubConfig::new(&base_url, github_creds);
    let google = GoogleConfig::new(&base_url, google_creds);

    (github, google)
}

#[tokio::test]
async fn generates_and_stores_and_get_right_auth_url_for_github_oauth() {
    let mut cache_storage = HashMapCache::new();
    let (github, google) = prepare_client();
    let mut oauth_client = OauthClient::builder()
        .github(&github)
        .google(&google)
        .cache_storage(&mut cache_storage)
        .build();

    // Act
    let auth_url_data = oauth_client
        .generate_auth_url_data(super::OauthProvider::Github)
        .await
        .unwrap();

    let prefixed_csrf_token =
        AuthUrlData::oauth_cache_key_prefix(auth_url_data.authorize_url.get_csrf_token().unwrap());

    // Assert
    assert!(cache_storage.get(prefixed_csrf_token.to_string()).await.unwrap().clone().contains("https://github.com/login/oauth/authorize?response_type=code&client_id=89c19374f7e7b5b35164&state"));

    assert!(AuthUrlData::verify_csrf_token(
        auth_url_data.authorize_url.get_csrf_token().unwrap(),
        &mut cache_storage
    )
    .await
    .is_ok());
}

#[tokio::test]
async fn generates_and_stores_and_get_right_auth_url_for_google_oauth() {
    let mut cache_storage = HashMapCache::new();
    let (github, google) = prepare_client();
    let mut oauth_client = OauthClient::builder()
        .github(&github)
        .google(&google)
        .cache_storage(&mut cache_storage)
        .build();

    // Act
    let auth_url_data = oauth_client
        .generate_auth_url_data(super::OauthProvider::Google)
        .await
        .unwrap();

    let prefixed_csrf_token =
        AuthUrlData::oauth_cache_key_prefix(auth_url_data.authorize_url.get_csrf_token().unwrap());

    // Assert
    assert!(cache_storage.get(prefixed_csrf_token.to_string()).await.unwrap().clone().contains("https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id=855174209543-6m0f088e55d3mevhnr8bs0qjap8j6g0g.apps.googleusercontent.com&state"));

    assert!(AuthUrlData::verify_csrf_token(
        auth_url_data.authorize_url.get_csrf_token().unwrap(),
        &mut cache_storage
    )
    .await
    .is_ok());
}
