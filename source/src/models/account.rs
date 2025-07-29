use dioxus::prelude::{ServerFnError, *};
use dioxus::logger::tracing::{info,warn,error};
use serde::{Serialize, Deserialize};
#[cfg(feature = "server")]
use crate::database::create_connection;
#[cfg(feature = "server")]
use sqlx::{FromRow, Row};

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
    let password_salt = std::env::var("DB_PWD_SALT")?;
    //TODO: Need to actually hash here but fine to just use password as
    //the hash for now to get this bit hooked up
    let password_hash = password;
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
                    Ok(Account {
                        account_id: Some(inserted_id),
                        username: "test".to_string(),
                        password_hash: "hashy_hash".to_string(),
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
    //TODO: Need to implement this
    Ok(Some(Account {
         account_id: None,
         username: "test".to_string(),
         password_hash: "".to_string(),
         role_id: Role::Admin as i32
    }))
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


//TODO: Need to fill these out to do stuff, just keeping here for now
#[cfg(test)]
mod test {
    use crate::models::account::save_new_account;
    use super::*;
    use async_std::task;

    #[test]
    fn save(){
        let username = "nico".to_string();
        let password = "password".to_string();
        let result = task::block_on(save_new_account(username, password, Role::Admin));
        assert!(result.is_ok(), "expected an account returned when there was none");
    }

    #[test]
    fn get_account_by_username_with_valid_account(){
        unimplemented!();
        let account_id = 1;
    }

    #[test]
    fn get_account_by_id_with_valid_account(){
        unimplemented!();
    }
}
