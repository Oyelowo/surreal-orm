use actix_redis::SameSite;
use actix_web::cookie::{
    time::{macros::offset, Date, Duration, OffsetDateTime},
    Cookie,
};

fn main() {
    let datetime = Date::MAX.midnight();

    let cookie = Cookie::build("user_id", "1234555")
        .domain("www.oyelowo.com")
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        // .permanent()
        .expires(OffsetDateTime::now_utc().checked_add(Duration::days(365)))
        .secure(true)
        .finish();

    println!("coookie");
    println!("{:#}", cookie);
    println!("coookie");
}
