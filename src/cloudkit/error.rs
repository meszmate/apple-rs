use crate::error::{AppleError, CloudKitErrorCode, CloudKitErrorResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct RawCloudKitErrorWrapper {
    #[serde(rename = "serverErrorCode")]
    pub server_error_code: Option<String>,
    pub reason: Option<String>,
    pub uuid: Option<String>,
    #[serde(rename = "retryAfter")]
    pub retry_after: Option<u64>,
}

pub(crate) fn parse_cloudkit_error(body: &str) -> AppleError {
    if let Ok(wrapper) = serde_json::from_str::<RawCloudKitErrorWrapper>(body)
        && let Some(code) = wrapper.server_error_code
    {
        return AppleError::CloudKitError(CloudKitErrorResponse {
            server_error_code: CloudKitErrorCode::parse(&code),
            reason: wrapper.reason.unwrap_or_default(),
            uuid: wrapper.uuid,
            retry_after: wrapper.retry_after,
        });
    }
    AppleError::JsonError(format!("Failed to parse CloudKit error response: {}", body))
}
