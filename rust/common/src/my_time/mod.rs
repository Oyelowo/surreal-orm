use chrono::{DateTime, Duration, Utc};

const EXPIRES_IN_DAY: u8 = 180;

pub fn get_session_duration() -> Duration {
    Duration::days(EXPIRES_IN_DAY.into())
}

pub fn get_session_expiry() -> DateTime<Utc> {
    Utc::now() + Duration::days(EXPIRES_IN_DAY.into())
}
