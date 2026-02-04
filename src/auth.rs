use crate::TokenResponse;
use crate::error::*;
use crate::signing::AppleKeyPair;
use jsonwebtoken::{EncodingKey, Header, encode};
use p256::pkcs8::EncodePrivateKey;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const VALIDATION_ENDPOINT: &str = "https://appleid.apple.com/auth/token";
const APPLE_AUDIENCE: &str = "https://appleid.apple.com";

#[derive(Serialize, Deserialize)]
struct AppleErrorResponseBody {
    error: String,
}

pub trait AppleAuth {
    fn validate_code(
        &self,
        code: &str,
    ) -> impl std::future::Future<Output = Result<TokenResponse, AppleError>> + Send;
    fn validate_code_with_redirect_uri(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> impl std::future::Future<Output = Result<TokenResponse, AppleError>> + Send;
    fn validate_refresh_token(
        &self,
        refresh_token: &str,
    ) -> impl std::future::Future<Output = Result<TokenResponse, AppleError>> + Send;
}

pub struct AppleAuthImpl {
    app_id: String,
    team_id: String,
    key_pair: Arc<AppleKeyPair>,
    http_client: Client,
}

impl AppleAuthImpl {
    pub fn new(
        app_id: &str,
        team_id: &str,
        key_id: &str,
        key_path: &str,
    ) -> Result<Self, AppleError> {
        let key_pair = AppleKeyPair::from_file(key_id, key_path)?;
        Ok(AppleAuthImpl {
            app_id: app_id.to_string(),
            team_id: team_id.to_string(),
            key_pair,
            http_client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .map_err(|e| AppleError::HttpError(e.to_string()))?,
        })
    }

    pub fn new_b64(
        app_id: &str,
        team_id: &str,
        key_id: &str,
        b64: &str,
    ) -> Result<Self, AppleError> {
        let key_pair = AppleKeyPair::from_base64(key_id, b64)?;
        Ok(AppleAuthImpl {
            app_id: app_id.to_string(),
            team_id: team_id.to_string(),
            key_pair,
            http_client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .map_err(|e| AppleError::HttpError(e.to_string()))?,
        })
    }

    pub fn from_key_pair(
        app_id: &str,
        team_id: &str,
        key_pair: Arc<AppleKeyPair>,
    ) -> Result<Self, AppleError> {
        Ok(AppleAuthImpl {
            app_id: app_id.to_string(),
            team_id: team_id.to_string(),
            key_pair,
            http_client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .map_err(|e| AppleError::HttpError(e.to_string()))?,
        })
    }

    pub fn key_pair(&self) -> &Arc<AppleKeyPair> {
        &self.key_pair
    }

    fn client_secret(&self) -> Result<String, AppleError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AppleError::TimeError(e.to_string()))?
            .as_secs();

        let claims = Claims {
            iss: self.team_id.clone(),
            sub: self.app_id.clone(),
            aud: APPLE_AUDIENCE.to_string(),
            iat: now as i64,
            exp: now as i64 + 15776999, // ~6 months
        };

        let mut header = Header::new(jsonwebtoken::Algorithm::ES256);
        header.kid = Some(self.key_pair.key_id().to_string());

        let der = self
            .key_pair
            .signing_key()
            .to_pkcs8_der()
            .map_err(|e: p256::pkcs8::Error| AppleError::KeyParseError(e.to_string()))?;

        let token = encode(&header, &claims, &EncodingKey::from_ec_der(der.as_bytes()))
            .map_err(|e| AppleError::JwtError(e.to_string()))?;

        Ok(token)
    }

    async fn validate_request(
        &self,
        form_query: Vec<(&str, &str)>,
    ) -> Result<TokenResponse, AppleError> {
        let res = self
            .http_client
            .post(VALIDATION_ENDPOINT)
            .form(&form_query)
            .send()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        if res.status() != reqwest::StatusCode::OK {
            let error_response: AppleErrorResponseBody = res
                .json()
                .await
                .map_err(|e| AppleError::JsonError(e.to_string()))?;
            return match error_response.error.as_str() {
                "invalid_scope" => Err(ERROR_RESPONSE_INVALID_SCOPE),
                "unsupported_grant_type" => Err(ERROR_RESPONSE_UNSUPPORTED_GRANT_TYPE),
                "unauthorized_client" => Err(ERROR_RESPONSE_UNAUTHORIZED_CLIENT),
                "invalid_grant" => Err(ERROR_RESPONSE_INVALID_GRANT),
                "invalid_client" => Err(ERROR_RESPONSE_INVALID_CLIENT),
                "invalid_request" => Err(ERROR_RESPONSE_INVALID_REQUEST),
                _ => Err(AppleError::UnrecognizedError(error_response.error)),
            };
        }

        let token_response = res
            .json()
            .await
            .map_err(|e| AppleError::JsonError(e.to_string()))?;
        Ok(token_response)
    }
}

impl AppleAuth for AppleAuthImpl {
    async fn validate_code(&self, code: &str) -> Result<TokenResponse, AppleError> {
        let client_secret = self.client_secret()?;
        let form_query = vec![
            ("client_id", self.app_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", code),
            ("grant_type", "authorization_code"),
        ];
        self.validate_request(form_query).await
    }

    async fn validate_code_with_redirect_uri(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<TokenResponse, AppleError> {
        let client_secret = self.client_secret()?;
        let form_query = vec![
            ("client_id", self.app_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", code),
            ("grant_type", "authorization_code"),
            ("redirect_uri", redirect_uri),
        ];
        self.validate_request(form_query).await
    }

    async fn validate_refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<TokenResponse, AppleError> {
        let client_secret = self.client_secret()?;
        let form_query = vec![
            ("client_id", self.app_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
        ];
        self.validate_request(form_query).await
    }
}

#[derive(Serialize)]
struct Claims {
    iss: String,
    sub: String,
    aud: String,
    iat: i64,
    exp: i64,
}
