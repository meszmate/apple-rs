use crate::error::AppleError;
use crate::signing::AppleKeyPair;
use jsonwebtoken::{EncodingKey, Header, encode};
use p256::pkcs8::EncodePrivateKey;
use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::error::parse_appstore_error;
use super::types::AppStoreEnvironment;

const PRODUCTION_BASE_URL: &str = "https://api.storekit.itunes.apple.com";
const SANDBOX_BASE_URL: &str = "https://api.storekit-sandbox.itunes.apple.com";

pub struct AppStoreConfig {
    pub issuer_id: String,
    pub bundle_id: String,
    pub key_pair: Arc<AppleKeyPair>,
    pub environment: AppStoreEnvironment,
}

pub struct AppStoreServerClient {
    config: AppStoreConfig,
    http_client: Client,
}

#[derive(Serialize)]
struct AppStoreClaims {
    iss: String,
    iat: i64,
    exp: i64,
    aud: String,
    bid: String,
}

impl AppStoreServerClient {
    pub fn new(config: AppStoreConfig) -> Result<Self, AppleError> {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        Ok(AppStoreServerClient {
            config,
            http_client,
        })
    }

    pub fn config(&self) -> &AppStoreConfig {
        &self.config
    }

    fn base_url(&self) -> &str {
        match self.config.environment {
            AppStoreEnvironment::Production => PRODUCTION_BASE_URL,
            _ => SANDBOX_BASE_URL,
        }
    }

    fn generate_token(&self) -> Result<String, AppleError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AppleError::TimeError(e.to_string()))?
            .as_secs();

        let claims = AppStoreClaims {
            iss: self.config.issuer_id.clone(),
            iat: now as i64,
            exp: now as i64 + 3600, // 1 hour max
            aud: "appstoreconnect-v1".to_string(),
            bid: self.config.bundle_id.clone(),
        };

        let mut header = Header::new(jsonwebtoken::Algorithm::ES256);
        header.kid = Some(self.config.key_pair.key_id().to_string());

        let der = self
            .config
            .key_pair
            .signing_key()
            .to_pkcs8_der()
            .map_err(|e: p256::pkcs8::Error| AppleError::KeyParseError(e.to_string()))?;

        let token = encode(&header, &claims, &EncodingKey::from_ec_der(der.as_bytes()))
            .map_err(|e| AppleError::JwtError(e.to_string()))?;

        Ok(token)
    }

    pub(crate) async fn jwt_get<Res: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<Res, AppleError> {
        let token = self.generate_token()?;
        let url = format!("{}{}", self.base_url(), path);

        let res = self
            .http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        let status = res.status();
        let body = res
            .text()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        if !status.is_success() {
            return Err(parse_appstore_error(&body));
        }

        serde_json::from_str(&body).map_err(|e| AppleError::JsonError(e.to_string()))
    }

    pub(crate) async fn jwt_post<Req: Serialize, Res: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<Res, AppleError> {
        let token = self.generate_token()?;
        let url = format!("{}{}", self.base_url(), path);

        let res = self
            .http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(body)
            .send()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        let status = res.status();
        let body = res
            .text()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        if !status.is_success() {
            return Err(parse_appstore_error(&body));
        }

        serde_json::from_str(&body).map_err(|e| AppleError::JsonError(e.to_string()))
    }

    #[allow(dead_code)]
    pub(crate) async fn jwt_post_empty_response<Req: Serialize>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<(), AppleError> {
        let token = self.generate_token()?;
        let url = format!("{}{}", self.base_url(), path);

        let res = self
            .http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(body)
            .send()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        let status = res.status();
        if !status.is_success() {
            let body = res
                .text()
                .await
                .map_err(|e| AppleError::HttpError(e.to_string()))?;
            return Err(parse_appstore_error(&body));
        }

        Ok(())
    }

    pub(crate) async fn jwt_put<Req: Serialize, Res: DeserializeOwned>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<Res, AppleError> {
        let token = self.generate_token()?;
        let url = format!("{}{}", self.base_url(), path);

        let res = self
            .http_client
            .put(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(body)
            .send()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        let status = res.status();
        let body = res
            .text()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        if !status.is_success() {
            return Err(parse_appstore_error(&body));
        }

        serde_json::from_str(&body).map_err(|e| AppleError::JsonError(e.to_string()))
    }

    pub(crate) async fn jwt_put_empty_response<Req: Serialize>(
        &self,
        path: &str,
        body: &Req,
    ) -> Result<(), AppleError> {
        let token = self.generate_token()?;
        let url = format!("{}{}", self.base_url(), path);

        let res = self
            .http_client
            .put(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(body)
            .send()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        let status = res.status();
        if !status.is_success() {
            let body = res
                .text()
                .await
                .map_err(|e| AppleError::HttpError(e.to_string()))?;
            return Err(parse_appstore_error(&body));
        }

        Ok(())
    }

    pub(crate) async fn jwt_post_empty_body<Res: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<Res, AppleError> {
        let token = self.generate_token()?;
        let url = format!("{}{}", self.base_url(), path);

        let res = self
            .http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        let status = res.status();
        let body = res
            .text()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        if !status.is_success() {
            return Err(parse_appstore_error(&body));
        }

        serde_json::from_str(&body).map_err(|e| AppleError::JsonError(e.to_string()))
    }

    pub(crate) async fn jwt_put_bytes<Res: DeserializeOwned>(
        &self,
        path: &str,
        data: Vec<u8>,
    ) -> Result<Res, AppleError> {
        let token = self.generate_token()?;
        let url = format!("{}{}", self.base_url(), path);

        let res = self
            .http_client
            .put(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/octet-stream")
            .body(data)
            .send()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        let status = res.status();
        let body = res
            .text()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        if !status.is_success() {
            return Err(parse_appstore_error(&body));
        }

        serde_json::from_str(&body).map_err(|e| AppleError::JsonError(e.to_string()))
    }

    pub(crate) async fn jwt_delete(&self, path: &str) -> Result<(), AppleError> {
        let token = self.generate_token()?;
        let url = format!("{}{}", self.base_url(), path);

        let res = self
            .http_client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        let status = res.status();
        if !status.is_success() {
            let body = res
                .text()
                .await
                .map_err(|e| AppleError::HttpError(e.to_string()))?;
            return Err(parse_appstore_error(&body));
        }

        Ok(())
    }
}
