use poem::{
    http::{header, Method},
    middleware::Cors,
};

use crate::configurations::application::Environment;

/// https://javascript.info/fetch-crossorigin#cors-for-safe-requests
/// http://www.ruanyifeng.com/blog/2016/04/cors.html
pub fn get_cors(environment: Environment) -> Cors {
    Cors::default()
        .allow_origins_fn(move |origin| {
            matches!(environment, Environment::Local) && origin.contains("local")
        })
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .allow_credentials(true)
        // .allow_origin("origin") FIXME: // readd after testing. Default is any
        .max_age(3600)
}
