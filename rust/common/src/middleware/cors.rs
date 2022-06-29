use poem::{
    http::{header, Method},
    middleware::Cors,
};

use crate::configurations::application::Environment;

/// https://javascript.info/fetch-crossorigin#cors-for-safe-requests
/// http://www.ruanyifeng.com/blog/2016/04/cors.html
pub fn get_cors(environment: Environment) -> Cors {
    Cors::default()
        .allow_origins_fn(move |_origin| matches!(environment, Environment::Local))
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ])
        .allow_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .allow_credentials(true)
        .max_age(3600)
}
