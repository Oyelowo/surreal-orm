fn main() -> anyhow::Result<()> {
    let credentials = Credentials {
        password: Secret::new("12345".into()),
        username: "oyelowo".into(),
    };

    // let p = store_password(credentials.clone()).unwrap();
    validate_credentials(credentials.clone()).unwrap();
    Ok(())
}

use anyhow::Context;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordVerifier, Version};
use secrecy::{ExposeSecret, Secret};
use thiserror;

#[derive(Clone)]
struct Credentials {
    username: String,
    password: Secret<String>,
}

#[derive(thiserror::Error, Debug)]
enum PasswordError {
    #[error("Authentication failed.")]
    AuthError,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

fn validate_credentials(credentials: Credentials) -> anyhow::Result<(), PasswordError> {
    /*
    PHC String Format
    # ${algorithm}${algorithm version}${$-separated algorithm parameters}${hash}${salt}
    $argon2id$v=19$m=65536,t=2,p=1$gZiV/M1gPc22ElAH/Jh1Hw$CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno
    */
    let expected_password_hash = "$argon2id$v=19$m=15000,t=2,p=1$1H39qaqJ+MP5M/yetv8SMg$9H1CT8EZZkMm1zVKd9t4PFUUZ5OyfOZE2fat88Y0rbc";
    let expected_password_hash = PasswordHash::new(&expected_password_hash)
        .context("Failed to parse hash in PHC string format.")
        .map_err(PasswordError::UnexpectedError)?;

    println!(
        "Expected password hash: {:?}",
        expected_password_hash.to_string()
    );

    Argon2::default()
        .verify_password(
            credentials.password.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password")?;
    // .map_err( PasswordError::AuthError)?;

    println!("Yaay! Password successfully validated!!!!");
    // let password_hash = hasher.hash_password_into(credentials.password.expose_secret().as_bytes(), salt, out)
    Ok(())
}
use argon2::{password_hash::SaltString, PasswordHasher};
use rand::Rng;
fn store_password(credentials: Credentials) -> anyhow::Result<String, PasswordError> {
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
        .hash_password(credentials.password.expose_secret().as_bytes(), &salt)
        .context("Failed to hash password")
        .map_err(PasswordError::UnexpectedError)?;
    let password_hash_str = password_hash.to_string();

    println!("password_hash: {:?}", password_hash);
    println!("password_hash_str: {:?}", password_hash_str);
    Ok(password_hash_str)
}
