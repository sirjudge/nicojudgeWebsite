use dioxus::prelude::{ServerFnError, *};
use serde::{Serialize, Deserialize};
#[cfg(feature = "server")]
use crate::database::create_connection;
#[cfg(feature = "server")]
use sqlx::{FromRow, Row};
use crate::auth;
use dioxus::logger::tracing::{info, error};
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Role {
    Admin = 1,
    User = 2,
    Guest = 3
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Account {
    pub account_id: Option<i32>,
    pub username: String,
    pub password_hash: String,
    pub role_id: i32
}

#[server]
pub async fn save_new_account(username: String, password: String, role: Role) -> Result<Account, ServerFnError> {
    // Use random salt for better security
    let password_hash = auth::hash_password(password).await?;
    let role_id = role.clone() as i32;
    match create_connection().await {
        Ok(mut conn) => {
            let result = sqlx::query!(
                "INSERT INTO accounts (username,password_hash, role_id) VALUES (?1,?2, ?3)",
                username,
                password_hash,
                role_id
            )
            .execute(&mut conn)
            .await;

            match result {
                Ok(query_result) => {
                    let inserted_id = query_result.last_insert_rowid() as i32;
                    info!("Account inserted into db successfully!");
                    Ok(Account {
                        account_id: Some(inserted_id),
                        username,
                        password_hash,
                        role_id
                    })
                }
                Err(e) => {
                    Err(ServerFnError::new(format!(
                        "Error occurred during blog insert: {e}"
                    )))
                }
            }
        }
        Err(e) => {
            Err(ServerFnError::new(format!(
                "database connection error: {e}"
            )))
        }
    }
}

#[server]
pub async fn get_account_by_id(account_id: i32) -> Result<Option<Account>, ServerFnError>{
    match create_connection().await {
        Ok(mut conn) => {
            let result = sqlx::query_as::<_, Account>(
                "SELECT account_id, username, password_hash, role_id
                FROM accounts
                WHERE account_id = ?1",
            )
            .bind(account_id)
            .fetch_optional(&mut conn)
            .await;

            match result {
                Ok(account) => Ok(account),
                Err(e) => {
                    error!("Error loading account: {}", e);
                    Err(ServerFnError::new(format!("Error loading account: {}", e)))
                }
            }
        }
        Err(e) => {
            error!("Database connection error: {}", e);
            Err(ServerFnError::new(format!(
                "Database connection error: {}"
                , e
            )))
        }
    }
}

#[server]
pub async fn get_account_by_username(username: String) -> Result<Option<Account>, ServerFnError>{
    match create_connection().await {
        Ok(mut conn) => {
            let result = sqlx::query_as::<_, Account>(
                "SELECT account_id, username, password_hash, role_id
                FROM accounts
                WHERE username = ?1",
            )
            .bind(username)
            .fetch_optional(&mut conn)
            .await;

            match result {
                Ok(account) => {
                    if account.is_some() {
                        Ok(Some(account.expect("Account should be found but was not")))
                    } else {
                        Ok(None)
                    }
                }
                Err(e) => {
                    error!("Error loading blog post: {e}");
                    Err(ServerFnError::new(format!("Error loading blog post: {e}")))
                }
            }
        }
        Err(e) => {
            error!("Database connection error: {e}");
            Err(ServerFnError::new(format!(
                "Database connection error: {e}"
            )))
        }
    }
}
