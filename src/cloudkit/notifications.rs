use crate::error::AppleError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum QueryNotificationReason {
    RecordCreated,
    RecordUpdated,
    RecordDeleted,
}

impl TryFrom<i32> for QueryNotificationReason {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(QueryNotificationReason::RecordCreated),
            2 => Ok(QueryNotificationReason::RecordUpdated),
            3 => Ok(QueryNotificationReason::RecordDeleted),
            _ => Err(format!("Unknown QueryNotificationReason: {}", value)),
        }
    }
}

impl Serialize for QueryNotificationReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let value = match self {
            QueryNotificationReason::RecordCreated => 1,
            QueryNotificationReason::RecordUpdated => 2,
            QueryNotificationReason::RecordDeleted => 3,
        };
        serializer.serialize_i32(value)
    }
}

impl<'de> Deserialize<'de> for QueryNotificationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        QueryNotificationReason::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseScope {
    Public,
    Private,
    Shared,
}

impl TryFrom<i32> for DatabaseScope {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DatabaseScope::Public),
            2 => Ok(DatabaseScope::Private),
            3 => Ok(DatabaseScope::Shared),
            _ => Err(format!("Unknown DatabaseScope: {}", value)),
        }
    }
}

impl Serialize for DatabaseScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let value = match self {
            DatabaseScope::Public => 1,
            DatabaseScope::Private => 2,
            DatabaseScope::Shared => 3,
        };
        serializer.serialize_i32(value)
    }
}

impl<'de> Deserialize<'de> for DatabaseScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        DatabaseScope::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APNsCloudKitPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aps: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ck: Option<CKNotificationPayload>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CKNotificationPayload {
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(rename = "nid", skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<String>,
    #[serde(rename = "p", skip_serializing_if = "Option::is_none")]
    pub is_pruned: Option<bool>,
    // Query notification fields
    #[serde(rename = "rid", skip_serializing_if = "Option::is_none")]
    pub record_id: Option<serde_json::Value>,
    #[serde(rename = "zid", skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<serde_json::Value>,
    #[serde(rename = "zoid", skip_serializing_if = "Option::is_none")]
    pub zone_owner: Option<String>,
    #[serde(rename = "fo", skip_serializing_if = "Option::is_none")]
    pub reason: Option<i32>,
    #[serde(rename = "af", skip_serializing_if = "Option::is_none")]
    pub record_fields: Option<serde_json::Value>,
    #[serde(rename = "rt", skip_serializing_if = "Option::is_none")]
    pub record_type: Option<String>,
    #[serde(rename = "dbs", skip_serializing_if = "Option::is_none")]
    pub database_scope: Option<i32>,
}

#[derive(Debug, Clone)]
pub enum CKNotification {
    Query(CKQueryNotification),
    RecordZone(CKRecordZoneNotification),
    Database(CKDatabaseNotification),
}

#[derive(Debug, Clone)]
pub struct CKQueryNotification {
    pub container_id: Option<String>,
    pub notification_id: Option<String>,
    pub is_pruned: Option<bool>,
    pub record_id: Option<serde_json::Value>,
    pub zone_id: Option<serde_json::Value>,
    pub zone_owner: Option<String>,
    pub reason: Option<QueryNotificationReason>,
    pub record_fields: Option<serde_json::Value>,
    pub record_type: Option<String>,
    pub database_scope: Option<DatabaseScope>,
}

#[derive(Debug, Clone)]
pub struct CKRecordZoneNotification {
    pub container_id: Option<String>,
    pub notification_id: Option<String>,
    pub is_pruned: Option<bool>,
    pub zone_id: Option<serde_json::Value>,
    pub zone_owner: Option<String>,
    pub database_scope: Option<DatabaseScope>,
}

#[derive(Debug, Clone)]
pub struct CKDatabaseNotification {
    pub container_id: Option<String>,
    pub notification_id: Option<String>,
    pub is_pruned: Option<bool>,
    pub database_scope: Option<DatabaseScope>,
}

pub fn parse_notification(json: &str) -> Result<CKNotification, AppleError> {
    let payload: APNsCloudKitPayload =
        serde_json::from_str(json).map_err(|e| AppleError::JsonError(e.to_string()))?;

    let ck = payload
        .ck
        .ok_or_else(|| AppleError::JsonError("Missing 'ck' field in payload".to_string()))?;

    let db_scope = ck
        .database_scope
        .and_then(|v| DatabaseScope::try_from(v).ok());

    // Determine notification type based on fields present
    if ck.record_id.is_some() || ck.record_type.is_some() || ck.reason.is_some() {
        // Query notification
        let reason = ck
            .reason
            .and_then(|v| QueryNotificationReason::try_from(v).ok());

        Ok(CKNotification::Query(CKQueryNotification {
            container_id: ck.container_id,
            notification_id: ck.notification_id,
            is_pruned: ck.is_pruned,
            record_id: ck.record_id,
            zone_id: ck.zone_id,
            zone_owner: ck.zone_owner,
            reason,
            record_fields: ck.record_fields,
            record_type: ck.record_type,
            database_scope: db_scope,
        }))
    } else if ck.zone_id.is_some() {
        // Record zone notification
        Ok(CKNotification::RecordZone(CKRecordZoneNotification {
            container_id: ck.container_id,
            notification_id: ck.notification_id,
            is_pruned: ck.is_pruned,
            zone_id: ck.zone_id,
            zone_owner: ck.zone_owner,
            database_scope: db_scope,
        }))
    } else {
        // Database notification
        Ok(CKNotification::Database(CKDatabaseNotification {
            container_id: ck.container_id,
            notification_id: ck.notification_id,
            is_pruned: ck.is_pruned,
            database_scope: db_scope,
        }))
    }
}

impl super::client::CloudKitClient {
    pub async fn poll_notifications(
        &self,
        webcourier_url: &str,
    ) -> Result<Vec<CKNotification>, AppleError> {
        let long_poll_client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        let res = long_poll_client
            .get(webcourier_url)
            .send()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        let status = res.status();
        let body = res
            .text()
            .await
            .map_err(|e| AppleError::HttpError(e.to_string()))?;

        if !status.is_success() {
            return Err(AppleError::HttpError(format!(
                "WebCourier polling failed with status: {}",
                status
            )));
        }

        // Parse the response - the webcourier returns a JSON object
        let response: serde_json::Value =
            serde_json::from_str(&body).map_err(|e| AppleError::JsonError(e.to_string()))?;

        let mut notifications = Vec::new();

        if let Some(items) = response.get("notifications").and_then(|v| v.as_array()) {
            for item in items {
                let item_str = serde_json::to_string(item)
                    .map_err(|e| AppleError::JsonError(e.to_string()))?;
                if let Ok(notification) = parse_notification(&item_str) {
                    notifications.push(notification);
                }
            }
        }

        Ok(notifications)
    }
}
