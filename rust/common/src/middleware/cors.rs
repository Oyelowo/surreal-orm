use poem::middleware::Cors;

/// https://javascript.info/fetch-crossorigin#cors-for-safe-requests
/// http://www.ruanyifeng.com/blog/2016/04/cors.html
pub fn get_cors() -> Cors {
    Cors::default()
        .allow_any_origin() // FIXME: // remove after testing.
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}
