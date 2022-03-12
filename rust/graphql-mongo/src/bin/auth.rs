use anyhow::Context;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordVerifier, Version};
use common::authentication::password::{generate_password_hash, validate_password, PasswordPlain};
use secrecy::{ExposeSecret, Secret};
use thiserror;
use tracing;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let p = store_password(credentials.clone()).unwrap();
    let pass = generate_password_hash("Oyelowo").await?;

    // let pass = create_password_hash(PlainPassword::new("Oyelowo".into()))?;
    println!("HGRGHJG: {:?}", pass.as_str());

    validate_password(PasswordPlain::new("Oyelowo"), pass)
        .await
        .unwrap();

    Ok(())
}
