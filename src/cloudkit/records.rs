use crate::cloudkit::client::CloudKitClient;
use crate::cloudkit::query::Query;
use crate::cloudkit::types::*;
use crate::error::AppleError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
struct ModifyRecordsRequest {
    operations: Vec<RecordOperation>,
    #[serde(rename = "zoneID", skip_serializing_if = "Option::is_none")]
    zone_id: Option<ZoneID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    atomic: Option<bool>,
}

#[derive(Debug, Serialize)]
struct RecordOperation {
    #[serde(rename = "operationType")]
    operation_type: OperationType,
    record: RecordBody,
}

#[derive(Debug, Serialize)]
struct RecordBody {
    #[serde(rename = "recordName", skip_serializing_if = "Option::is_none")]
    record_name: Option<String>,
    #[serde(rename = "recordType", skip_serializing_if = "Option::is_none")]
    record_type: Option<String>,
    #[serde(rename = "recordChangeTag", skip_serializing_if = "Option::is_none")]
    record_change_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<HashMap<String, FieldValue>>,
}

#[derive(Debug, Deserialize)]
pub struct ModifyRecordsResponse {
    pub records: Vec<RecordResult>,
}

#[derive(Debug, Deserialize)]
pub struct RecordResult {
    #[serde(rename = "recordName")]
    pub record_name: Option<String>,
    #[serde(rename = "recordType")]
    pub record_type: Option<String>,
    #[serde(rename = "recordChangeTag")]
    pub record_change_tag: Option<String>,
    #[serde(default)]
    pub fields: HashMap<String, FieldValue>,
    #[serde(rename = "zoneID")]
    pub zone_id: Option<ZoneID>,
    pub created: Option<RecordTimestamp>,
    pub modified: Option<RecordTimestamp>,
    #[serde(rename = "serverErrorCode")]
    pub server_error_code: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
struct QueryRecordsRequest {
    query: Query,
    #[serde(rename = "zoneID", skip_serializing_if = "Option::is_none")]
    zone_id: Option<ZoneID>,
    #[serde(rename = "resultsLimit", skip_serializing_if = "Option::is_none")]
    results_limit: Option<u32>,
    #[serde(rename = "continuationMarker", skip_serializing_if = "Option::is_none")]
    continuation_marker: Option<String>,
    #[serde(rename = "desiredKeys", skip_serializing_if = "Option::is_none")]
    desired_keys: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct QueryResponse {
    pub records: Vec<RecordResult>,
    #[serde(rename = "continuationMarker")]
    pub continuation_marker: Option<String>,
}

#[derive(Debug, Serialize)]
struct LookupRecordsRequest {
    records: Vec<RecordLookup>,
    #[serde(rename = "zoneID", skip_serializing_if = "Option::is_none")]
    zone_id: Option<ZoneID>,
    #[serde(rename = "desiredKeys", skip_serializing_if = "Option::is_none")]
    desired_keys: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct RecordLookup {
    #[serde(rename = "recordName")]
    record_name: String,
}

impl CloudKitClient {
    pub async fn create_record(
        &self,
        db: &DatabaseType,
        record: Record,
    ) -> Result<Record, AppleError> {
        let url = self.build_url(db, "records/modify");
        let fields = if record.fields.is_empty() { None } else { Some(record.fields) };
        let request = ModifyRecordsRequest {
            operations: vec![RecordOperation {
                operation_type: OperationType::Create,
                record: RecordBody {
                    record_name: record.record_name,
                    record_type: Some(record.record_type),
                    record_change_tag: None,
                    fields,
                },
            }],
            zone_id: record.zone_id,
            atomic: None,
        };

        let response: ModifyRecordsResponse = self.signed_post(&url, &request).await?;
        let result = response.records.into_iter().next()
            .ok_or_else(|| AppleError::JsonError("Empty response from CloudKit".to_string()))?;

        if let Some(ref err_code) = result.server_error_code {
            return Err(AppleError::CloudKitError(crate::error::CloudKitErrorResponse {
                server_error_code: crate::error::CloudKitErrorCode::parse(err_code),
                reason: result.reason.unwrap_or_default(),
                uuid: None,
                retry_after: None,
            }));
        }

        Ok(Record {
            record_name: result.record_name,
            record_type: result.record_type.unwrap_or_default(),
            record_change_tag: result.record_change_tag,
            fields: result.fields,
            zone_id: result.zone_id,
            created: result.created,
            modified: result.modified,
        })
    }

    pub async fn update_record(
        &self,
        db: &DatabaseType,
        record: Record,
    ) -> Result<Record, AppleError> {
        let url = self.build_url(db, "records/modify");
        let fields = if record.fields.is_empty() { None } else { Some(record.fields) };
        let request = ModifyRecordsRequest {
            operations: vec![RecordOperation {
                operation_type: OperationType::Update,
                record: RecordBody {
                    record_name: record.record_name,
                    record_type: Some(record.record_type),
                    record_change_tag: record.record_change_tag,
                    fields,
                },
            }],
            zone_id: record.zone_id,
            atomic: None,
        };

        let response: ModifyRecordsResponse = self.signed_post(&url, &request).await?;
        let result = response.records.into_iter().next()
            .ok_or_else(|| AppleError::JsonError("Empty response from CloudKit".to_string()))?;

        if let Some(ref err_code) = result.server_error_code {
            return Err(AppleError::CloudKitError(crate::error::CloudKitErrorResponse {
                server_error_code: crate::error::CloudKitErrorCode::parse(err_code),
                reason: result.reason.unwrap_or_default(),
                uuid: None,
                retry_after: None,
            }));
        }

        Ok(Record {
            record_name: result.record_name,
            record_type: result.record_type.unwrap_or_default(),
            record_change_tag: result.record_change_tag,
            fields: result.fields,
            zone_id: result.zone_id,
            created: result.created,
            modified: result.modified,
        })
    }

    pub async fn delete_record(
        &self,
        db: &DatabaseType,
        record_name: &str,
        record_type: &str,
        zone_id: Option<ZoneID>,
    ) -> Result<(), AppleError> {
        let url = self.build_url(db, "records/modify");
        let request = ModifyRecordsRequest {
            operations: vec![RecordOperation {
                operation_type: OperationType::Delete,
                record: RecordBody {
                    record_name: Some(record_name.to_string()),
                    record_type: Some(record_type.to_string()),
                    record_change_tag: None,
                    fields: None,
                },
            }],
            zone_id,
            atomic: None,
        };

        let response: ModifyRecordsResponse = self.signed_post(&url, &request).await?;
        if let Some(result) = response.records.into_iter().next()
            && let Some(ref err_code) = result.server_error_code
        {
            return Err(AppleError::CloudKitError(crate::error::CloudKitErrorResponse {
                server_error_code: crate::error::CloudKitErrorCode::parse(err_code),
                reason: result.reason.unwrap_or_default(),
                uuid: None,
                retry_after: None,
            }));
        }

        Ok(())
    }

    pub async fn query_records(
        &self,
        db: &DatabaseType,
        query: Query,
        zone_id: Option<ZoneID>,
        results_limit: Option<u32>,
        continuation_marker: Option<String>,
        desired_keys: Option<Vec<String>>,
    ) -> Result<QueryResponse, AppleError> {
        let url = self.build_url(db, "records/query");
        let request = QueryRecordsRequest {
            query,
            zone_id,
            results_limit,
            continuation_marker,
            desired_keys,
        };

        self.signed_post(&url, &request).await
    }

    pub async fn lookup_records(
        &self,
        db: &DatabaseType,
        record_names: &[&str],
        zone_id: Option<ZoneID>,
        desired_keys: Option<Vec<String>>,
    ) -> Result<Vec<Record>, AppleError> {
        let url = self.build_url(db, "records/lookup");
        let request = LookupRecordsRequest {
            records: record_names.iter().map(|name| RecordLookup {
                record_name: name.to_string(),
            }).collect(),
            zone_id,
            desired_keys,
        };

        let response: ModifyRecordsResponse = self.signed_post(&url, &request).await?;
        Ok(response.records.into_iter().map(|r| Record {
            record_name: r.record_name,
            record_type: r.record_type.unwrap_or_default(),
            record_change_tag: r.record_change_tag,
            fields: r.fields,
            zone_id: r.zone_id,
            created: r.created,
            modified: r.modified,
        }).collect())
    }

    pub async fn modify_records(
        &self,
        db: &DatabaseType,
        operations: Vec<(OperationType, Record)>,
        zone_id: Option<ZoneID>,
        atomic: Option<bool>,
    ) -> Result<Vec<RecordResult>, AppleError> {
        let url = self.build_url(db, "records/modify");
        let ops: Vec<RecordOperation> = operations.into_iter().map(|(op_type, record)| {
            let fields = if record.fields.is_empty() { None } else { Some(record.fields) };
            RecordOperation {
                operation_type: op_type,
                record: RecordBody {
                    record_name: record.record_name,
                    record_type: Some(record.record_type),
                    record_change_tag: record.record_change_tag,
                    fields,
                },
            }
        }).collect();

        let request = ModifyRecordsRequest {
            operations: ops,
            zone_id,
            atomic,
        };

        let response: ModifyRecordsResponse = self.signed_post(&url, &request).await?;
        Ok(response.records)
    }
}
