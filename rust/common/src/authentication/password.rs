use std::fmt::Debug;

use anyhow::Context;
use argon2::{password_hash::SaltString, PasswordHasher};
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordVerifier, Version};
use secrecy::{ExposeSecret, Secret};
use thiserror;
use tokio::task::JoinHandle;
use tracing;

pub struct PasswordPlain(Secret<String>);

impl PasswordPlain {
    pub fn new(pass: impl Into<String>) -> Self {
        Self(Secret::new(pass.into()))
    }

    pub fn to_bytes(&self) -> &[u8] {
        self.0.expose_secret().as_bytes()
    }
}

pub struct PasswordHashPHC(Secret<String>);
impl PasswordHashPHC {
    pub fn new(pass: impl Into<String>) -> Self {
        Self(Secret::new(pass.into()))
    }

    pub fn as_str(&self) -> &str {
        self.0.expose_secret().as_str()
    }
}

impl From<PasswordHashPHC> for String {
    fn from(password_phc: PasswordHashPHC) -> Self {
        password_phc.0.expose_secret().into()
    }
}

//  return a 500 for UnexpectedError, while AuthErrors should result into a 401.
#[derive(thiserror::Error, Debug)]
pub enum PasswordError {
    #[error("Authentication failed.")]
    InvalidPassword(#[source] anyhow::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}

pub async fn generate_password_hash(
    password: impl Into<String>,
) -> anyhow::Result<PasswordHashPHC> {
    let password: String = password.into();
    let password_hash =
        spawn_blocking_with_tracing(move || create_password_hash(PasswordPlain::new(password)))
            .await?
            .context("Failed to hash password")?;

    Ok(password_hash)
}

fn create_password_hash(password: PasswordPlain) -> anyhow::Result<PasswordHashPHC, PasswordError> {
    let salt = SaltString::generate(&mut rand::thread_rng());

    let params = Params::new(15000, 2, 1, None).context("Error building Argon2 parameters")?;

    let hasher = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let password_hash = hasher
        .hash_password(password.to_bytes(), &salt)
        .context("Failed to hash password")
        .map_err(PasswordError::UnexpectedError)?;

    Ok(PasswordHashPHC::new(password_hash.to_string()))
}

#[tracing::instrument(
    name = "Verify password hash",
    skip(plain_password, expected_password_hash)
)]
pub async fn validate_password(
    plain_password: PasswordPlain,
    expected_password_hash: PasswordHashPHC,
) -> anyhow::Result<bool, PasswordError> {
    // This executes before spawning the new thread
    spawn_blocking_with_tracing(move || {
        verify_password_hash(plain_password, expected_password_hash)
    })
    .await
    .context("Failed to spawn blocking task.")
    .map_err(PasswordError::UnexpectedError)??;

    Ok(true)
}

#[tracing::instrument(
    name = "Verify password hash",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    password_candidate: PasswordPlain,
    expected_password_hash: PasswordHashPHC,
) -> Result<(), PasswordError> {
    /*
    Password hash in PHC String Format
    # ${algorithm}${algorithm version}${$-separated algorithm parameters}${hash}${salt}
    $argon2id$v=19$m=15000,t=2,p=1$gZiV/M1gPc22ElAH/Jh1Hw$CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno
    */
    let expected_password_hash = PasswordHash::new(expected_password_hash.as_str())
        .context("Failed to parse hash in PHC string format.")
        .map_err(PasswordError::UnexpectedError)?;

    Argon2::default()
        .verify_password(password_candidate.to_bytes(), &expected_password_hash)
        .context("Invalid password.")
        .map_err(PasswordError::InvalidPassword)
}
