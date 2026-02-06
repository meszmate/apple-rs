use serde::{Deserialize, Serialize};

use super::types::NotificationTypeV1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerNotificationV1 {
    #[serde(rename = "notification_type")]
    pub notification_type: NotificationTypeV1,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(rename = "auto_renew_adam_id", skip_serializing_if = "Option::is_none")]
    pub auto_renew_adam_id: Option<String>,
    #[serde(
        rename = "auto_renew_product_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_renew_product_id: Option<String>,
    #[serde(rename = "auto_renew_status", skip_serializing_if = "Option::is_none")]
    pub auto_renew_status: Option<String>,
    #[serde(
        rename = "auto_renew_status_change_date",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_renew_status_change_date: Option<String>,
    #[serde(
        rename = "auto_renew_status_change_date_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_renew_status_change_date_ms: Option<String>,
    #[serde(
        rename = "auto_renew_status_change_date_pst",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_renew_status_change_date_pst: Option<String>,
    #[serde(rename = "bid", skip_serializing_if = "Option::is_none")]
    pub bid: Option<String>,
    #[serde(rename = "bvrs", skip_serializing_if = "Option::is_none")]
    pub bvrs: Option<String>,
    #[serde(rename = "cancellation_date", skip_serializing_if = "Option::is_none")]
    pub cancellation_date: Option<String>,
    #[serde(
        rename = "cancellation_date_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancellation_date_ms: Option<String>,
    #[serde(
        rename = "cancellation_date_pst",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancellation_date_pst: Option<String>,
    #[serde(rename = "expiration_intent", skip_serializing_if = "Option::is_none")]
    pub expiration_intent: Option<String>,
    #[serde(rename = "unified_receipt", skip_serializing_if = "Option::is_none")]
    pub unified_receipt: Option<UnifiedReceipt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedReceipt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(rename = "latest_receipt", skip_serializing_if = "Option::is_none")]
    pub latest_receipt: Option<String>,
    #[serde(
        rename = "latest_receipt_info",
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_receipt_info: Option<Vec<LatestReceiptInfo>>,
    #[serde(
        rename = "pending_renewal_info",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_renewal_info: Option<Vec<PendingRenewalInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestReceiptInfo {
    #[serde(rename = "cancellation_date", skip_serializing_if = "Option::is_none")]
    pub cancellation_date: Option<String>,
    #[serde(
        rename = "cancellation_date_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancellation_date_ms: Option<String>,
    #[serde(
        rename = "cancellation_date_pst",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancellation_date_pst: Option<String>,
    #[serde(
        rename = "cancellation_reason",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancellation_reason: Option<String>,
    #[serde(rename = "expires_date", skip_serializing_if = "Option::is_none")]
    pub expires_date: Option<String>,
    #[serde(rename = "expires_date_ms", skip_serializing_if = "Option::is_none")]
    pub expires_date_ms: Option<String>,
    #[serde(rename = "expires_date_pst", skip_serializing_if = "Option::is_none")]
    pub expires_date_pst: Option<String>,
    #[serde(
        rename = "in_app_ownership_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub in_app_ownership_type: Option<String>,
    #[serde(
        rename = "is_in_intro_offer_period",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_in_intro_offer_period: Option<String>,
    #[serde(rename = "is_trial_period", skip_serializing_if = "Option::is_none")]
    pub is_trial_period: Option<String>,
    #[serde(rename = "is_upgraded", skip_serializing_if = "Option::is_none")]
    pub is_upgraded: Option<String>,
    #[serde(
        rename = "offer_code_ref_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub offer_code_ref_name: Option<String>,
    #[serde(
        rename = "original_purchase_date",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_purchase_date: Option<String>,
    #[serde(
        rename = "original_purchase_date_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_purchase_date_ms: Option<String>,
    #[serde(
        rename = "original_purchase_date_pst",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_purchase_date_pst: Option<String>,
    #[serde(
        rename = "original_transaction_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_transaction_id: Option<String>,
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(
        rename = "promotional_offer_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub promotional_offer_id: Option<String>,
    #[serde(rename = "purchase_date", skip_serializing_if = "Option::is_none")]
    pub purchase_date: Option<String>,
    #[serde(rename = "purchase_date_ms", skip_serializing_if = "Option::is_none")]
    pub purchase_date_ms: Option<String>,
    #[serde(rename = "purchase_date_pst", skip_serializing_if = "Option::is_none")]
    pub purchase_date_pst: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(
        rename = "subscription_group_identifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_group_identifier: Option<String>,
    #[serde(rename = "transaction_id", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(
        rename = "web_order_line_item_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub web_order_line_item_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingRenewalInfo {
    #[serde(
        rename = "auto_renew_product_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_renew_product_id: Option<String>,
    #[serde(rename = "auto_renew_status", skip_serializing_if = "Option::is_none")]
    pub auto_renew_status: Option<String>,
    #[serde(rename = "expiration_intent", skip_serializing_if = "Option::is_none")]
    pub expiration_intent: Option<String>,
    #[serde(
        rename = "grace_period_expires_date",
        skip_serializing_if = "Option::is_none"
    )]
    pub grace_period_expires_date: Option<String>,
    #[serde(
        rename = "grace_period_expires_date_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub grace_period_expires_date_ms: Option<String>,
    #[serde(
        rename = "grace_period_expires_date_pst",
        skip_serializing_if = "Option::is_none"
    )]
    pub grace_period_expires_date_pst: Option<String>,
    #[serde(
        rename = "is_in_billing_retry_period",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_in_billing_retry_period: Option<String>,
    #[serde(
        rename = "offer_code_ref_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub offer_code_ref_name: Option<String>,
    #[serde(
        rename = "original_transaction_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_transaction_id: Option<String>,
    #[serde(
        rename = "price_consent_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub price_consent_status: Option<String>,
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(
        rename = "promotional_offer_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub promotional_offer_id: Option<String>,
}
