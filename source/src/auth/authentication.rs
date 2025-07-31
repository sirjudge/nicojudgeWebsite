#[cfg(feature = "server")]
use crate::database::create_connection;
use crate::models::{get_account_by_username, Account};
#[cfg(feature = "server")]
use password_hash::SaltString;
// #[cfg(feature = "server")]
// use rand::rngs::OsRng;
#[cfg(feature = "server")]
use argon2::{password_hash::Salt, Argon2, PasswordHasher, PasswordVerifier};
use dioxus::{
    logger::tracing::{error, info, warn},
    prelude::*,
};
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use sqlx::{FromRow, Row};

// TODO: This is a placeholder for session validation logic.
// In a real application, you would check if the user is logged in.
#[server]
pub async fn validate_session() -> Result<bool, ServerFnError> {
    warn!("This feature is not yet implemented and is returning true every time");
    Ok(true)
}

// TODO: obviously this is not secure, I'll be coming back to this later
// and adding proper auth and session managment later
// but don't need to worry about that when I have no actual functionality
// TODO: Should also log bad login attempts as well
#[server]
pub async fn validate_login(username: String, password: String) -> Result<bool, ServerFnError> {
    match get_account_by_username(username)
        .await
        .expect("error retrieving account for login validation")
    {
        Some(account) => {
            let password_hash = hash_password(password)
                .await
                .expect("Error hashing password for login validation");
            if password_hash == account.password_hash {
                info!("Passwords match yippee");
                Ok(true)
            } else {
                error!(
                    "Passwords don't match. new hash:{} Saved hash:{}",
                    password_hash, account.password_hash
                );
                Ok(false)
            }
        }
        None => {
            //TODO: Should handle this a bit better, fine to keep as
            //error message for now
            error!("account not found in lookup to validate");
            Ok(false)
        }
    }
}

#[server]
pub async fn hash_password(password: String) -> Result<String, ServerFnError> {
    if password.is_empty() {
        return Err(ServerFnError::new(
            "input password is empty when it should not be",
        ));
    }
    //TODO: RustCrypto suggests using this but it doesn't
    //specify where OsRng comes from . . . deal with it later and instead use env variable
    //for now
    // let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let salt = std::env::var("DB_PWD_SALT")
        .map_err(|_| ServerFnError::new("DB_PWD_SALT environment variable not set"))?;
    let salt_string = SaltString::new(&salt).unwrap();
    let pwd_hash = argon2
        .hash_password(password.as_bytes(), &salt_string)
        .unwrap();
    info!(
        "Successfully hashed password:{} into hash:{}",
        password, pwd_hash
    );
    Argon2::default()
        .verify_password(password.as_bytes(), &pwd_hash)
        .expect("invalid password");

    // Ok(hash.to_string())
    Ok("".to_string())
}
