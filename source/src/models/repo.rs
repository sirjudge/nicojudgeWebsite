use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub description: Option<String>,
    pub pushed_at: String,
}
