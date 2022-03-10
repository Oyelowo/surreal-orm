use tracing;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let p = store_password(credentials.clone()).unwrap();
    let pass = spawn_blocking_with_tracing(move || {
        create_password_hash(PlainPassword::new("Oyelowo".into()))
    })
    .await?
    .context("Faailed to hash passwor")?;

    // let pass = create_password_hash(PlainPassword::new("Oyelowo".into()))?;
    println!("HGRGHJG: {:?}", pass.0.expose_secret());

    validate_credentials(PlainPassword::new("Oyelowo".into()), pass)
        .await
        .unwrap();

    Ok(())
}

pub struct PlainPassword(Secret<String>);
impl PlainPassword {
    fn new(pass: String) -> Self {
        Self(Secret::new(pass))
    }

    fn to_bytes(&self) -> &[u8] {
        self.0.expose_secret().as_bytes()
    }
}

// impl From<PlainPassword> for String {
//     fn from(p: PlainPassword) -> String {
//         let k = p.0.expose_secret().to_owned();
//         k
//     }
// }

pub struct PasswordHashPHC(Secret<String>);
impl PasswordHashPHC {
    fn new(pass: String) -> Self {
        Self(Secret::new(pass))
    }

    fn as_str(&self) -> &str {
        self.0.expose_secret().as_str()
    }
}

// impl From<PasswordHashPHC> for &str {
//     fn from(_: PasswordHashPHC) -> Self {
//         todo!()
//     }
// }
use anyhow::Context;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordVerifier, Version};
use secrecy::{ExposeSecret, Secret};
use thiserror;

//  return a 500 for UnexpectedError, while AuthErrors should result into a 401.
#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Authentication failed.")]
    InvalidCredentials(#[source] anyhow::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

use tokio::task::JoinHandle;

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}

// #[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
// #[tracing::instrument(
//     name = "Verify password hash",
//     skip(expected_password_hash, password_candidate)
// )]
async fn validate_credentials(
    plain_password: PlainPassword,
    expected_password_hash: PasswordHashPHC,
) -> anyhow::Result<bool, AuthError> {
    // This executes before spawning the new thread
    spawn_blocking_with_tracing(move || {
        verify_password_hash(plain_password, expected_password_hash)
    })
    .await
    .context("Failed to spawn blocking task.")
    .map_err(AuthError::UnexpectedError)??;

    println!("Yaay! Password successfully validated!!!!");
    // let password_hash = hasher.hash_password_into(credentials.password.expose_secret().as_bytes(), salt, out)
    Ok(true)
}

#[tracing::instrument(
    name = "Verify password hash",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    password_candidate: PlainPassword,
    expected_password_hash: PasswordHashPHC,
) -> Result<(), AuthError> {
    /*
    PHC String Format
    # ${algorithm}${algorithm version}${$-separated algorithm parameters}${hash}${salt}
    $argon2id$v=19$m=65536,t=2,p=1$gZiV/M1gPc22ElAH/Jh1Hw$CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno
    */
    let expected_password_hash = PasswordHash::new(expected_password_hash.as_str())
        .context("Failed to parse hash in PHC string format.")
        .map_err(AuthError::UnexpectedError)?;

    Argon2::default()
        .verify_password(password_candidate.to_bytes(), &expected_password_hash)
        .context("Invalid password.")
        .map_err(AuthError::InvalidCredentials)
}

use argon2::{password_hash::SaltString, PasswordHasher};

fn create_password_hash<'a>(password: PlainPassword) -> anyhow::Result<PasswordHashPHC, AuthError> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    // We don't care about the exact Argon2 parameters here
    // given that it's for testing purposes!
    // let password_hash = Argon2::default()
    //     .hash_password("pass".as_bytes(), &salt)
    //     .unwrap()
    //     .to_string();
    // println!("password_hash11111: {:?}", password_hash);

    let params = Params::new(15000, 2, 1, None).context("Error building Argon2 paramters")?;
    let hasher = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let password_hash = hasher
        .hash_password(password.0.expose_secret().as_bytes(), &salt)
        .context("Failed to hash password")
        .map_err(AuthError::UnexpectedError)?;

    println!("fdefdf: {:?}", password_hash);
    Ok(PasswordHashPHC::new(password_hash.to_string()))
}
