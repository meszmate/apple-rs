use crate::cloudkit::error::parse_cloudkit_error;
use crate::cloudkit::types::{DatabaseType, Environment};
use crate::error::AppleError;
use crate::signing::AppleKeyPair;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const CLOUDKIT_BASE_URL: &str = "https://api.apple-cloudkit.com";

pub struct CloudKitConfig {
    pub container: String,
    pub environment: Environment,
    pub key_pair: Arc<AppleKeyPair>,
}

pub struct CloudKitClient {
    config: CloudKitConfig,
    pub(crate) http_client: Client,
}

impl CloudKitClient {
    pub fn new(config: CloudKitConfig) -> Result<Self, AppleError> {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        Ok(CloudKitClient {
            config,
            http_client,
        })
    }

    /// Returns a reference to the client's configuration.
    pub fn config(&self) -> &CloudKitConfig {
        &self.config
    }

    pub(crate) fn build_url(&self, db: &DatabaseType, operation: &str) -> String {
        format!(
            "{}/database/1/{}/{}/{}/{}",
            CLOUDKIT_BASE_URL, self.config.container, self.config.environment, db, operation,
        )
    }

    pub(crate) fn build_base_url(&self, path: &str) -> String {
        format!(
            "{}/database/1/{}/{}/{}",
            CLOUDKIT_BASE_URL, self.config.container, self.config.environment, path,
        )
    }

    fn sign_request(&self, body: &str, subpath: &str) -> Result<Vec<(String, String)>, AppleError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AppleError::TimeError(e.to_string()))?;

        let date = chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
            .ok_or_else(|| AppleError::TimeError("Invalid timestamp".to_string()))?
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();

        let body_hash = {
            let mut hasher = Sha256::new();
            hasher.update(body.as_bytes());
            STANDARD.encode(hasher.finalize())
        };

        let message = format!("{}:{}:{}", date, body_hash, subpath);
        let signature = self.config.key_pair.sign(message.as_bytes());
        let signature_b64 = STANDARD.encode(&signature);

        Ok(vec![
            (
                "X-Apple-CloudKit-Request-KeyID".to_string(),
                self.config.key_pair.key_id().to_string(),
            ),
            ("X-Apple-CloudKit-Request-ISO8601Date".to_string(), date),
            (
                "X-Apple-CloudKit-Request-SignatureV1".to_string(),
                signature_b64,
            ),
        ])
    }

    pub(crate) fn extract_subpath(url: &str) -> &str {
        url.strip_prefix(CLOUDKIT_BASE_URL).unwrap_or(url)
    }

    pub(crate) async fn signed_post<Req: Serialize, Res: DeserializeOwned>(
        &self,
        url: &str,
        body: &Req,
    ) -> Result<Res, AppleError> {
        let body_str =
            serde_json::to_string(body).map_err(|e| AppleError::JsonError(e.to_string()))?;

        let subpath = Self::extract_subpath(url);
        let headers = self.sign_request(&body_str, subpath)?;

        let mut request = self
            .http_client
            .post(url)
            .header("Content-Type", "application/json");

        for (key, value) in &headers {
            request = request.header(key, value);
        }

        let res = request
            .body(body_str)
            .send()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        let status = res.status();
        let response_body = res
            .text()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        if !status.is_success() {
            return Err(parse_cloudkit_error(&response_body));
        }

        serde_json::from_str(&response_body).map_err(|e| AppleError::JsonError(e.to_string()))
    }

    #[allow(dead_code)]
    pub(crate) async fn signed_get<Res: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<Res, AppleError> {
        let subpath = Self::extract_subpath(url);
        let headers = self.sign_request("", subpath)?;

        let mut request = self.http_client.get(url);

        for (key, value) in &headers {
            request = request.header(key, value);
        }

        let res = request
            .send()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        let status = res.status();
        let response_body = res
            .text()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        if !status.is_success() {
            return Err(parse_cloudkit_error(&response_body));
        }

        serde_json::from_str(&response_body).map_err(|e| AppleError::JsonError(e.to_string()))
    }
}
