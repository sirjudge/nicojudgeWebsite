use dioxus::prelude::{ServerFnError, *};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Role {
    Admin = 1,
    User = 2,
    Guest = 3
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Account {
    pub username: String,
    pub password_hash: String,
    pub role: Role
}

#[server]
pub async fn save_new_account(username: String, password: String) -> Result<Account, ServerFnError> {
    let password_hash = std::env::var("DB_PWD_HASH")?;
    //TODO: Need to implement this
    Ok(Account {
         username: "test".to_string(),
         password_hash: "hashy_hash".to_string(),
         role: Role::Admin
    })
}

#[server]
pub async fn get_account(username: String) -> Result<Account, ServerFnError>{
    //TODO: Need to implement this
    Ok(Account {
         username: "test".to_string(),
         password_hash: "".to_string(),
         role: Role::Admin
    })
}
