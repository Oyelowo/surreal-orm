use actix_web::cookie::time::Duration as DurationCookie;
use chrono::{DateTime, Duration as DurationChrono, Utc};

const EXPIRES_IN_DAY: u8 = 180;

pub fn get_session_duration() -> DurationCookie {
    DurationCookie::days(EXPIRES_IN_DAY.into())
}

pub fn get_session_expiry() -> DateTime<Utc> {
    Utc::now() + DurationChrono::days(EXPIRES_IN_DAY.into())
}
