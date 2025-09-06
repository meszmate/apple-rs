use crate::error::AppleError;
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RealUserStatus {
    Unsupported = 0,
    Unknown = 1,
    LikelyReal = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleUser {
    pub issuer: Option<String>,
    pub audience: Option<String>,
    pub subject: Option<String>,
    pub issued_at: Option<i64>,
    pub expiry: Option<i64>,
    pub nonce: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub is_private_email: bool,
    pub real_user_status: RealUserStatus,
    pub auth_time: Option<i64>,
    pub nonce_supported: Option<bool>,
    pub transfer_sub: Option<String>,
    pub org_id: Option<String>,
}

pub fn get_user_info_from_id_token(id_token: &str) -> Result<AppleUser, AppleError> {
    let token = decode::<Claims>(
        id_token,
        &jsonwebtoken::DecodingKey::from_secret(&[]), // No validation for simplicity
        &Validation::new(jsonwebtoken::Algorithm::ES256),
    )
        .map_err(|e| AppleError::JwtError(e.to_string()))?;

    let claims = token.claims;
    let mut user = AppleUser {
        issuer: claims.iss,
        audience: claims.aud,
        subject: claims.sub,
        nonce: claims.nonce,
        issued_at: claims.iat,
        expiry: claims.exp,
        email: claims.email,
        email_verified: parse_bool(&claims.email_verified),
        is_private_email: parse_bool(&claims.is_private_email),
        real_user_status: match claims.real_user_status.unwrap_or(0.0) as i32 {
            2 => RealUserStatus::LikelyReal,
            1 => RealUserStatus::Unknown,
            _ => RealUserStatus::Unsupported,
        },
        auth_time: claims.auth_time,
        nonce_supported: claims.nonce_supported,
        transfer_sub: claims.transfer_sub,
        org_id: claims.org_id,
    };

    if let Some(iat) = user.issued_at {
        user.issued_at = Some(iat);
    }
    if let Some(exp) = user.expiry {
        user.expiry = Some(exp);
    }

    Ok(user)
}

fn parse_bool(value: &Option<serde_json::Value>) -> bool {
    match value {
        Some(serde_json::Value::Bool(b)) => *b,
        Some(serde_json::Value::String(s)) => s == "true",
        Some(serde_json::Value::Number(n)) => n.as_f64().map_or(false, |n| n != 0.0),
        _ => false,
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: Option<String>,
    aud: Option<String>,
    sub: Option<String>,
    iat: Option<i64>,
    exp: Option<i64>,
    nonce: Option<String>,
    email: Option<String>,
    email_verified: Option<serde_json::Value>,
    is_private_email: Option<serde_json::Value>,
    real_user_status: Option<f64>,
    auth_time: Option<i64>,
    nonce_supported: Option<bool>,
    transfer_sub: Option<String>,
    org_id: Option<String>,
}