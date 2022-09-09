use crate::{error_handling::ApiHttpStatus::*, middleware::get_session_expiry};
use async_graphql::{Context, ErrorExtensions, Result};
use chrono::{DateTime, Utc};
use log::warn;
use poem::session::Session;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum TypedSessionError {
    #[error("Failed to parse data")]
    ParsingFailure(#[from] serde_json::Error),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}

type TypedSessionResult<T> = Result<T>;

pub struct TypedSession(pub Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn from_ctx<'a>(ctx: &'a Context<'_>) -> Result<&'a Self> {
        let session = ctx.data::<Self>().map_err(|e| {
            warn!("{e:?}");
            InternalServerError("Something went wrong while getting session".into()).extend()
        });
        session
    }

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_user_id<T: Serialize + ?Sized>(&self, user_id: &T) {
        self.0.set(Self::USER_ID_KEY, user_id)
    }

    pub fn get_user_id<T>(&self) -> TypedSessionResult<T>
    where
        T: DeserializeOwned,
    {
        self.0
            .get::<T>(Self::USER_ID_KEY)
            .ok_or_else(|| Unauthorized("Not logged in. Please sign in.".into()).extend())
    }

    pub fn clear(&self) {
        self.0.clear()
    }

    pub fn purge(&self) {
        self.0.purge()
    }

    pub fn get_expiry() -> DateTime<Utc> {
        get_session_expiry()
    }
}
