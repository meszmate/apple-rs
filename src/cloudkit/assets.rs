use crate::cloudkit::client::CloudKitClient;
use crate::cloudkit::types::DatabaseType;
use crate::error::AppleError;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize)]
struct AssetUploadRequest {
    tokens: Vec<AssetUploadToken>,
    #[serde(rename = "zoneID", skip_serializing_if = "Option::is_none")]
    zone_id: Option<crate::cloudkit::types::ZoneID>,
}

#[derive(Debug, Serialize)]
struct AssetUploadToken {
    #[serde(rename = "recordName")]
    record_name: String,
    #[serde(rename = "recordType")]
    record_type: String,
    #[serde(rename = "fieldName")]
    field_name: String,
}

#[derive(Debug, Deserialize)]
pub struct AssetUploadResponse {
    pub tokens: Vec<AssetTokenInfo>,
}

#[derive(Debug, Deserialize)]
pub struct AssetTokenInfo {
    #[serde(rename = "recordName")]
    pub record_name: Option<String>,
    #[serde(rename = "fieldName")]
    pub field_name: Option<String>,
    pub url: Option<String>,
}

impl CloudKitClient {
    pub async fn request_asset_upload(
        &self,
        db: &DatabaseType,
        record_name: &str,
        record_type: &str,
        field_name: &str,
        zone_id: Option<crate::cloudkit::types::ZoneID>,
    ) -> Result<AssetUploadResponse, AppleError> {
        let url = self.build_url(db, "assets/upload");
        let request = AssetUploadRequest {
            tokens: vec![AssetUploadToken {
                record_name: record_name.to_string(),
                record_type: record_type.to_string(),
                field_name: field_name.to_string(),
            }],
            zone_id,
        };

        self.signed_post(&url, &request).await
    }

    pub async fn upload_asset(
        &self,
        upload_url: &str,
        data: &[u8],
    ) -> Result<AssetUploadResult, AppleError> {
        let checksum = {
            let mut hasher = Sha256::new();
            hasher.update(data);
            STANDARD.encode(hasher.finalize())
        };

        let res = self.http_client
            .post(upload_url)
            .header("Content-Type", "application/octet-stream")
            .body(data.to_vec())
            .send()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        let status = res.status();
        let response_body = res.text().await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        if !status.is_success() {
            return Err(AppleError::HttpError(format!(
                "Asset upload failed with status {}: {}",
                status, response_body
            )));
        }

        let result: AssetUploadResult = serde_json::from_str(&response_body)
            .map_err(|e| AppleError::JsonError(e.to_string()))?;

        Ok(AssetUploadResult {
            file_checksum: result.file_checksum.or(Some(checksum)),
            size: result.size.or(Some(data.len() as u64)),
            receipt: result.receipt,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct AssetUploadResult {
    #[serde(rename = "fileChecksum")]
    pub file_checksum: Option<String>,
    pub size: Option<u64>,
    pub receipt: Option<String>,
}
