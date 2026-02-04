use crate::cloudkit::client::CloudKitClient;
use crate::cloudkit::records::RecordResult;
use crate::cloudkit::types::*;
use crate::error::AppleError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct FetchZoneChangesRequest {
    #[serde(rename = "zoneID")]
    zone_id: ZoneID,
    #[serde(rename = "syncToken", skip_serializing_if = "Option::is_none")]
    sync_token: Option<String>,
    #[serde(rename = "resultsLimit", skip_serializing_if = "Option::is_none")]
    results_limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ZoneChangesResponse {
    pub records: Vec<RecordResult>,
    #[serde(rename = "syncToken")]
    pub sync_token: Option<String>,
    #[serde(rename = "moreComing")]
    pub more_coming: Option<bool>,
}

#[derive(Debug, Serialize)]
struct FetchDatabaseChangesRequest {
    #[serde(rename = "syncToken", skip_serializing_if = "Option::is_none")]
    sync_token: Option<String>,
    #[serde(rename = "resultsLimit", skip_serializing_if = "Option::is_none")]
    results_limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseChangesResponse {
    pub zones: Vec<ZoneChangeInfo>,
    #[serde(rename = "syncToken")]
    pub sync_token: Option<String>,
    #[serde(rename = "moreComing")]
    pub more_coming: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ZoneChangeInfo {
    #[serde(rename = "zoneID")]
    pub zone_id: ZoneID,
}

impl CloudKitClient {
    pub async fn fetch_zone_changes(
        &self,
        db: &DatabaseType,
        zone_id: ZoneID,
        sync_token: Option<String>,
        results_limit: Option<u32>,
    ) -> Result<ZoneChangesResponse, AppleError> {
        let url = self.build_url(db, "changes/zone");
        let request = FetchZoneChangesRequest {
            zone_id,
            sync_token,
            results_limit,
        };

        self.signed_post(&url, &request).await
    }

    pub async fn fetch_database_changes(
        &self,
        db: &DatabaseType,
        sync_token: Option<String>,
        results_limit: Option<u32>,
    ) -> Result<DatabaseChangesResponse, AppleError> {
        let url = self.build_url(db, "changes/database");
        let request = FetchDatabaseChangesRequest {
            sync_token,
            results_limit,
        };

        self.signed_post(&url, &request).await
    }
}
