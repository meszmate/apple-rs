use serde::{Deserialize, Serialize};

use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseBodyV2 {
    #[serde(rename = "signedPayload")]
    pub signed_payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseBodyV2DecodedPayload {
    #[serde(rename = "notificationType")]
    pub notification_type: NotificationTypeV2,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<Subtype>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<NotificationData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<NotificationSummary>,
    #[serde(
        rename = "externalPurchaseToken",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_purchase_token: Option<ExternalPurchaseToken>,
    pub version: String,
    #[serde(rename = "signedDate")]
    pub signed_date: i64,
    #[serde(rename = "notificationUUID")]
    pub notification_uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationData {
    pub environment: AppStoreEnvironment,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "appAppleId", skip_serializing_if = "Option::is_none")]
    pub app_apple_id: Option<i64>,
    #[serde(rename = "bundleVersion", skip_serializing_if = "Option::is_none")]
    pub bundle_version: Option<String>,
    #[serde(
        rename = "signedTransactionInfo",
        skip_serializing_if = "Option::is_none"
    )]
    pub signed_transaction_info: Option<String>,
    #[serde(rename = "signedRenewalInfo", skip_serializing_if = "Option::is_none")]
    pub signed_renewal_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubscriptionStatus>,
    #[serde(
        rename = "consumptionRequestReason",
        skip_serializing_if = "Option::is_none"
    )]
    pub consumption_request_reason: Option<ConsumptionRequestReason>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSummary {
    pub environment: AppStoreEnvironment,
    #[serde(rename = "requestIdentifier")]
    pub request_identifier: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(
        rename = "storefrontCountryCodes",
        skip_serializing_if = "Option::is_none"
    )]
    pub storefront_country_codes: Option<Vec<String>>,
    #[serde(rename = "succeededCount")]
    pub succeeded_count: i64,
    #[serde(rename = "failedCount")]
    pub failed_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalPurchaseToken {
    #[serde(rename = "externalPurchaseId")]
    pub external_purchase_id: String,
    #[serde(rename = "tokenCreationDate")]
    pub token_creation_date: i64,
    #[serde(rename = "appAppleId", skip_serializing_if = "Option::is_none")]
    pub app_apple_id: Option<i64>,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
}
