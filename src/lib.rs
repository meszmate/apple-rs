use serde::{Deserialize, Serialize};

pub mod signing;
pub mod error;

#[cfg(feature = "auth")]
pub mod auth;
#[cfg(feature = "auth")]
pub mod url;
#[cfg(feature = "auth")]
pub mod user;

#[cfg(feature = "cloudkit")]
pub mod cloudkit;

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: i32,
    pub id_token: String,
    pub refresh_token: String,
    pub token_type: String,
}
