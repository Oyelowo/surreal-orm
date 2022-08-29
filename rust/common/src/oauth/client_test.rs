use super::{
    cache_storage::{CacheStorage, HashMapCache},
    client::OauthClient,
};
use crate::oauth::urls::AuthUrlData;
use crate::oauth::{account::OauthProvider, cache_storage::LruCache};
use crate::{
    configurations::oauth::{OauthGithubCredentials, OauthGoogleCredentials},
    oauth::{github::GithubConfig, google::GoogleConfig},
};

fn get_client<C>(cache_storage: C) -> OauthClient<C>
where
    C: CacheStorage + std::fmt::Debug,
{
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

    // let mut cache_storage = HashMapCache::new();
    // let (github, google) = get_client();
    let mut oauth_client = OauthClient::builder()
        .github(github)
        .google(google)
        .cache_storage(cache_storage)
        .build();
    oauth_client
}

#[tokio::test]
async fn generates_and_stores_and_get_right_auth_url_for_github_oauth() {
    // Act
    let mut oauth_client = get_client(HashMapCache::new());
    let auth_url_data = oauth_client
        .generate_auth_url_data(OauthProvider::Github)
        .await
        .unwrap();

    let prefixed_csrf_token =
        AuthUrlData::oauth_cache_key_prefix(auth_url_data.authorize_url.get_csrf_token().unwrap());

    // Assert
    assert!(oauth_client.get_cache_mut_ref().get(prefixed_csrf_token.to_string()).await.unwrap().clone().contains("https://github.com/login/oauth/authorize?response_type=code&client_id=89c19374f7e7b5b35164&state"));

    assert!(AuthUrlData::verify_csrf_token(
        auth_url_data.authorize_url.get_csrf_token().unwrap(),
        oauth_client.get_cache_mut_ref()
    )
    .await
    .is_ok());
}

#[tokio::test]
async fn generates_and_stores_and_get_right_auth_url_for_google_oauth() {
    let mut oauth_client = get_client(HashMapCache::new());
    // Act
    let auth_url_data = oauth_client
        .generate_auth_url_data(OauthProvider::Google)
        .await
        .unwrap();

    let prefixed_csrf_token =
        AuthUrlData::oauth_cache_key_prefix(auth_url_data.authorize_url.get_csrf_token().unwrap());

    // Assert
    assert!(oauth_client.get_cache_mut_ref().get(prefixed_csrf_token.to_string()).await.unwrap().clone().contains("https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id=855174209543-6m0f088e55d3mevhnr8bs0qjap8j6g0g.apps.googleusercontent.com&state"));

    assert!(AuthUrlData::verify_csrf_token(
        auth_url_data.authorize_url.get_csrf_token().unwrap(),
        oauth_client.get_cache_mut_ref()
    )
    .await
    .is_ok());
}

#[tokio::test]
async fn lru_generates_and_stores_and_get_right_auth_url_for_github_oauth() {
    let mut oauth_client = get_client(LruCache::new(2000));
    // Act
    let auth_url_data = oauth_client
        .generate_auth_url_data(OauthProvider::Github)
        .await
        .unwrap();

    let prefixed_csrf_token =
        AuthUrlData::oauth_cache_key_prefix(auth_url_data.authorize_url.get_csrf_token().unwrap());

    // Assert
    assert!(oauth_client.get_cache_mut_ref().get(prefixed_csrf_token.to_string()).await.unwrap().clone().contains("https://github.com/login/oauth/authorize?response_type=code&client_id=89c19374f7e7b5b35164&state"));

    assert!(AuthUrlData::verify_csrf_token(
        auth_url_data.authorize_url.get_csrf_token().unwrap(),
        oauth_client.get_cache_mut_ref()
    )
    .await
    .is_ok());
}

#[tokio::test]
async fn lru_generates_and_stores_and_get_right_auth_url_for_google_oauth() {
    let mut oauth_client = get_client(LruCache::new(10));

    // Act
    let auth_url_data = oauth_client
        .generate_auth_url_data(OauthProvider::Google)
        .await
        .unwrap();

    let prefixed_csrf_token =
        AuthUrlData::oauth_cache_key_prefix(auth_url_data.authorize_url.get_csrf_token().unwrap());

    // Assert
    assert!(oauth_client.get_cache_mut_ref().get(prefixed_csrf_token.to_string()).await.unwrap().clone().contains("https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id=855174209543-6m0f088e55d3mevhnr8bs0qjap8j6g0g.apps.googleusercontent.com&state"));

    assert!(AuthUrlData::verify_csrf_token(
        auth_url_data.authorize_url.get_csrf_token().unwrap(),
        oauth_client.get_cache_mut_ref()
    )
    .await
    .is_ok());
}

#[tokio::test]
async fn lru_empty_generates_and_stores_and_get_right_auth_url_for_google_oauth() {
    let mut oauth_client = get_client(LruCache::new(0));

    // Act
    let auth_url_data = oauth_client
        .generate_auth_url_data(OauthProvider::Google)
        .await
        .unwrap();

    let prefixed_csrf_token =
        AuthUrlData::oauth_cache_key_prefix(auth_url_data.authorize_url.get_csrf_token().unwrap());

    // Assert
    assert!(oauth_client
        .get_cache_mut_ref()
        .get(prefixed_csrf_token.to_string())
        .await
        .is_none());

    assert!(AuthUrlData::verify_csrf_token(
        auth_url_data.authorize_url.get_csrf_token().unwrap(),
        oauth_client.get_cache_mut_ref()
    )
    .await
    .is_err());
}
