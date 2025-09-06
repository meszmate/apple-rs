use serde::{Deserialize, Serialize};

pub mod auth;
pub mod error;
pub mod url;
pub mod user;

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: i32,
    pub id_token: String,
    pub refresh_token: String,
    pub token_type: String,
}