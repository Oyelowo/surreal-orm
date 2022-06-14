use common::configurations::redis::RedisConfigs;
use oauth2::basic::BasicClient;
use poem::middleware::AddData;
use poem::web::{Data, Redirect};
use poem::EndpointExt;
use poem::{get, handler, http::Uri, listener::TcpListener, web::Path, Route, Server};

// Alternatively, this can be `oauth2::curl::http_client` or a custom client.
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use poem_openapi::payload::PlainText;
use redis::Connection;
// use redis::aio::Connection;
use std::env;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
// use tokio::net::TcpListener;
use crate::oauth::github::{GithubConfig, OauthProviderTrait, TypedAuthUrl, TypedCsrfState};
use url::Url;

use crate::app::user::OauthProvider;

#[handler]
pub async fn oauth_login(Path(provider): Path<OauthProvider>, rc: Data<&RedisConfigs>) -> Redirect {
    let mut con = rc.clone().get_client().unwrap().get_connection().unwrap();

    let auth_url_data = match provider {
        OauthProvider::Github => GithubConfig::new().generate_auth_url(),
        OauthProvider::Google => todo!(),
    };

    // Send csrf state to redis
    auth_url_data.csrf_state.cache(provider, &mut con).unwrap();

    Redirect::moved_permanent(auth_url_data.authorize_url)
}

#[handler]
async fn oauth_redirect_url(uri: &Uri, rc: Data<&RedisConfigs>) -> String {
    let redirect_url = Url::parse(&("http://localhost".to_string() + &uri.to_string())).unwrap();
    let redirect_url = TypedAuthUrl(redirect_url);
    let code = redirect_url.get_authorization_code();

    let mut con = rc.clone().get_client().unwrap().get_connection().unwrap();
    // make .verify give me back both the csrf token and the provider
    let provider = redirect_url.get_csrf_state().verify(&mut con).expect("er");

    let token_res = match provider {
        OauthProvider::Github => {
            let github_config = GithubConfig::new();
            println!("my state: {provider:?}");

            // All these are the profile fetch should probably also be part of github config(OauthProvider) trait
            let token_res = github_config
                .client()
                .exchange_code(code)
                .request_async(async_http_client)
                .await;
            token_res
        }
        OauthProvider::Google => todo!(),
    };

    if let Ok(token) = token_res {
        // NB: Github returns a single comma-separated "scope" parameter instead of multiple
        // space-separated scopes. Github-specific clients can parse this scope into
        // multiple scopes by splitting at the commas. Note that it's not safe for the
        // library to do this by default because RFC 6749 allows scopes to contain commas.
        println!("TOKENNNN{:?}", token);
        println!("Accesssss{:?}", token.access_token().secret().as_str());
        let url = "https://api.github.com/user";
        let body = reqwest::Client::new()
                    .get(url)
                                // .header("accept", "application/vnd.github.v3+json")
            .header("user-agent","Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.4577.82 Safari/537.36")
                    .header("Authorization", format!("Bearer {}",token.access_token().secret().as_str()))
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

        print!("FGREWRTBODY:{body}");

        let scopes = if let Some(scopes_vec) = token.scopes() {
            scopes_vec
                .iter()
                .map(|comma_separated| comma_separated.split(','))
                .flatten()
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        println!("Github returned the following scopes:\n{:?}\n", scopes);
    }
    "erer".into()
}

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     let (client, authorize_url, csrf_state) = fun_name();

//     let auth_data = AuthData {
//         authorize_url,
//         csrf_state,
//         client,
//     };

//     let app = Route::new()
//         .at("/api/auth/signin", get(oauth))
//         // .at("/hello/:name", get(hello))
//         .at("/", get(handle_oauth_redirect_url))
//         .with(AddData::new(auth_data));
//     Server::new(TcpListener::bind("127.0.0.1:8080"))
//         .run(app)
//         .await
// }

#[derive(Debug, Clone)]
pub struct AuthData {
    client: BasicClient,
    authorize_url: Url,
    csrf_state: CsrfToken,
}

fn authorize_user() -> AuthData {
    let github_client_id = ClientId::new("57d332c258954615aac7".to_string());
    let github_client_secret = ClientSecret::new("e41a1fb86af01532fe640a2d79ad6608c3774261".into());
    // let github_client_id = ClientId::new(
    //     env::var("GITHUB_CLIENT_ID").expect("Missing the GITHUB_CLIENT_ID environment variable."),
    // );
    // let github_client_secret = ClientSecret::new(
    //     env::var("GITHUB_CLIENT_SECRET")
    //         .expect("Missing the GITHUB_CLIENT_SECRET environment variable."),
    // );
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        .expect("Invalid token endpoint URL");
    // Set up the config for the Github OAuth2 process.
    let client = BasicClient::new(
        github_client_id,
        Some(github_client_secret),
        auth_url,
        Some(token_url),
    )
    // This example will be running its own server at localhost:8080.
    // See below for the server implementation.
    .set_redirect_uri(
        RedirectUrl::new("http://localhost:8080/api/oauth/redirect_uri".to_string())
            .expect("Invalid redirect URL"),
    );
    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the user's public repos and email.
        // .add_scope(Scope::new("public_repo".to_string()))
        .add_scopes(["read:user", "user:email"].map(|s| Scope::new(s.into())))
        // .add_scope(Scope::new("read:user".to_string()))
        // .add_scope(Scope::new("user:email".to_string()))
        .url();
    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );
    AuthData {
        client,
        authorize_url,
        csrf_state,
    }
}
