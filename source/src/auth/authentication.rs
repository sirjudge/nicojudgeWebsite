#[cfg(feature = "server")]
use crate::database::create_connection;
use crate::models::{get_account_by_username, Account};
#[cfg(feature = "server")]
use password_hash::SaltString;
// #[cfg(feature = "server")]
// use rand::rngs::OsRng;
#[cfg(feature = "server")]
use argon2::{
    password_hash::Salt,
    Argon2,
    PasswordHasher,
    PasswordVerifier
};
use dioxus::{
    logger::tracing::{info, warn},
    prelude::*,
};
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use sqlx::{FromRow, Row};

// TODO: This is a placeholder for session validation logic.
// In a real application, you would check if the user is logged in.
#[server]
pub async fn validate_session() -> Result<bool, ServerFnError> {
    Ok(true)
}

// TODO: obviously this is not secure, I'll be coming back to this later
// and adding proper auth and session managment later
// but don't need to worry about that when I have no actual functionality
// TODO: Should also log bad login attempts as well
#[server]
pub async fn validate_login(username: String, password: String) -> Result<bool, ServerFnError> {
    let account = get_account_by_username(username);

    //TODO: Final step should be to remove this
    Ok(true)
}

#[server]
pub async fn hash_password(password: String) -> Result<String, ServerFnError> {
    if password.is_empty() {
        return Err(ServerFnError::new(
            "input password is empty when it should not be",
        ));
    }

    // Use salt from environment variable
    // TOO: This is bad?

    //TODO: RustCrypto suggests using this but it doesn't
    //specify where OsRng comes from . . . deal with it later and instead use env variable
    //for now
    // let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let salt = std::env::var("DB_PWD_SALT")
        .map_err(|_| ServerFnError::new("DB_PWD_SALT environment variable not set"))?;
    let salt_string = SaltString::new(&salt).unwrap();
    let pwd_hash = argon2.hash_password(password.as_bytes(), &salt_string).unwrap();
    // This is the hash we will store. Notice our salt string is included, as well as parameters:
    // version 0x13 (19), memory 19456KiB (19 MiB), 2 iterations (time), parallelism 1
    // let expected =
    //     "$argon2id$v=19$m=19456,t=2,p=1$YmFkIHNhbHQh$DqHGwv6NQV0VcaJi7jeF1E8IpfMXmXcpq4r2kKyqpXk";
    //   ^ hash ^ parameters            ^ salt       ^ combined hash

    // assert_eq!(expected, hash.to_string());

    //The verifier reads the salt and the parameters from the hash and verifies the result is equal
    Argon2::default()
        .verify_password(password.as_bytes(), &pwd_hash)
        .expect("invalid password");

    // Ok(hash.to_string())
    Ok("".to_string())
}
