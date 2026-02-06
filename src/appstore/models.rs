use serde::{Deserialize, Serialize};

use super::types::*;

// Transaction History

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionHistoryRequest {
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i64>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i64>,
    #[serde(rename = "productId", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<Vec<String>>,
    #[serde(rename = "productType", skip_serializing_if = "Option::is_none")]
    pub product_type: Option<Vec<TransactionHistoryProductType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Order>,
    #[serde(
        rename = "subscriptionGroupIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_group_identifier: Option<Vec<String>>,
    #[serde(rename = "inAppOwnershipType", skip_serializing_if = "Option::is_none")]
    pub in_app_ownership_type: Option<InAppOwnershipType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryResponse {
    #[serde(rename = "signedTransactions")]
    pub signed_transactions: Vec<String>,
    pub revision: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "appAppleId", skip_serializing_if = "Option::is_none")]
    pub app_apple_id: Option<i64>,
    pub environment: AppStoreEnvironment,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}

// Subscription Status

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub environment: AppStoreEnvironment,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "appAppleId", skip_serializing_if = "Option::is_none")]
    pub app_apple_id: Option<i64>,
    pub data: Vec<SubscriptionGroupIdentifierItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionGroupIdentifierItem {
    #[serde(rename = "subscriptionGroupIdentifier")]
    pub subscription_group_identifier: String,
    #[serde(rename = "lastTransactions")]
    pub last_transactions: Vec<LastTransactionsItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastTransactionsItem {
    pub status: SubscriptionStatus,
    #[serde(rename = "originalTransactionId")]
    pub original_transaction_id: String,
    #[serde(rename = "signedTransactionInfo")]
    pub signed_transaction_info: String,
    #[serde(rename = "signedRenewalInfo")]
    pub signed_renewal_info: String,
}

// Consumption

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumptionRequest {
    #[serde(rename = "accountTenure")]
    pub account_tenure: AccountTenure,
    #[serde(rename = "appAccountToken")]
    pub app_account_token: String,
    #[serde(rename = "consumptionStatus")]
    pub consumption_status: ConsumptionStatus,
    #[serde(rename = "customerConsented")]
    pub customer_consented: bool,
    #[serde(rename = "deliveryStatus", skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<DeliveryStatus>,
    #[serde(
        rename = "lifetimeDollarsPurchased",
        skip_serializing_if = "Option::is_none"
    )]
    pub lifetime_dollars_purchased: Option<LifetimeDollarsPurchased>,
    #[serde(
        rename = "lifetimeDollarsRefunded",
        skip_serializing_if = "Option::is_none"
    )]
    pub lifetime_dollars_refunded: Option<LifetimeDollarsRefunded>,
    pub platform: Platform,
    #[serde(rename = "playTime", skip_serializing_if = "Option::is_none")]
    pub play_time: Option<PlayTime>,
    #[serde(rename = "refundPreference", skip_serializing_if = "Option::is_none")]
    pub refund_preference: Option<RefundPreference>,
    #[serde(
        rename = "sampleContentProvided",
        skip_serializing_if = "Option::is_none"
    )]
    pub sample_content_provided: Option<bool>,
    #[serde(rename = "userStatus")]
    pub user_status: UserStatus,
}

// Extend Renewal Date

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendRenewalDateRequest {
    #[serde(rename = "extendByDays")]
    pub extend_by_days: i32,
    #[serde(rename = "extendReasonCode")]
    pub extend_reason_code: ExtendReasonCode,
    #[serde(rename = "requestIdentifier")]
    pub request_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendRenewalDateResponse {
    #[serde(rename = "effectiveDate")]
    pub effective_date: i64,
    #[serde(rename = "originalTransactionId")]
    pub original_transaction_id: String,
    #[serde(rename = "webOrderLineItemId")]
    pub web_order_line_item_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassExtendRenewalDateRequest {
    #[serde(rename = "extendByDays")]
    pub extend_by_days: i32,
    #[serde(rename = "extendReasonCode")]
    pub extend_reason_code: ExtendReasonCode,
    #[serde(rename = "requestIdentifier")]
    pub request_identifier: String,
    #[serde(
        rename = "storefrontCountryCodes",
        skip_serializing_if = "Option::is_none"
    )]
    pub storefront_country_codes: Option<Vec<String>>,
    #[serde(rename = "productId")]
    pub product_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassExtendRenewalDateResponse {
    #[serde(rename = "requestIdentifier")]
    pub request_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassExtendRenewalDateStatusResponse {
    #[serde(rename = "requestIdentifier")]
    pub request_identifier: String,
    pub complete: bool,
    #[serde(rename = "completeDate", skip_serializing_if = "Option::is_none")]
    pub complete_date: Option<i64>,
    #[serde(rename = "succeededCount", skip_serializing_if = "Option::is_none")]
    pub succeeded_count: Option<i64>,
    #[serde(rename = "failedCount", skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i64>,
}

// Order Lookup

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLookupResponse {
    pub status: OrderLookupStatus,
    #[serde(rename = "signedTransactions")]
    pub signed_transactions: Vec<String>,
}

// Refund History

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundHistoryResponse {
    #[serde(rename = "signedTransactions")]
    pub signed_transactions: Vec<String>,
    pub revision: String,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}

// Transaction Info

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfoResponse {
    #[serde(rename = "signedTransactionInfo")]
    pub signed_transaction_info: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppTransactionInfoResponse {
    #[serde(rename = "signedTransactionInfo")]
    pub signed_transaction_info: String,
}

// Notification History

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationHistoryRequest {
    #[serde(rename = "startDate")]
    pub start_date: i64,
    #[serde(rename = "endDate")]
    pub end_date: i64,
    #[serde(rename = "notificationType", skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<NotificationTypeV2>,
    #[serde(
        rename = "notificationSubtype",
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_subtype: Option<Subtype>,
    #[serde(
        rename = "onlyFailures",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub only_failures: Option<bool>,
    #[serde(
        rename = "transactionId",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationHistoryResponse {
    #[serde(rename = "notificationHistory")]
    pub notification_history: Vec<NotificationHistoryResponseItem>,
    #[serde(rename = "paginationToken", skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationHistoryResponseItem {
    #[serde(rename = "signedPayload")]
    pub signed_payload: String,
    #[serde(rename = "sendAttempts")]
    pub send_attempts: Vec<SendAttemptItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendAttemptItem {
    #[serde(rename = "attemptDate")]
    pub attempt_date: i64,
    #[serde(rename = "sendAttemptResult")]
    pub send_attempt_result: SendAttemptResult,
}

// Test Notification

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTestNotificationResponse {
    #[serde(rename = "testNotificationToken")]
    pub test_notification_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckTestNotificationResponse {
    #[serde(rename = "signedPayload")]
    pub signed_payload: String,
    #[serde(rename = "sendAttempts")]
    pub send_attempts: Vec<SendAttemptItem>,
}

// App Account Token

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAppAccountTokenRequest {
    #[serde(rename = "appAccountToken")]
    pub app_account_token: String,
}

// Retention Messaging

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadMessageRequest {
    pub title: String,
    pub body: String,
    #[serde(rename = "imageIdentifier", skip_serializing_if = "Option::is_none")]
    pub image_identifier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageListResponse {
    pub images: Vec<ImageListItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageListItem {
    #[serde(rename = "imageIdentifier")]
    pub image_identifier: String,
    pub state: ImageState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageListResponse {
    pub messages: Vec<MessageListItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageListItem {
    #[serde(rename = "messageIdentifier")]
    pub message_identifier: String,
    pub state: MessageState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultMessageRequest {
    pub title: String,
    pub body: String,
}
