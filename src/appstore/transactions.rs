use crate::error::AppleError;

use super::client::AppStoreServerClient;
use super::models::*;

impl AppStoreServerClient {
    pub async fn get_transaction_history(
        &self,
        transaction_id: &str,
        revision: Option<&str>,
        request: Option<&TransactionHistoryRequest>,
    ) -> Result<HistoryResponse, AppleError> {
        let mut path = format!("/inApps/v2/history/{}", transaction_id);
        let mut params = Vec::new();
        if let Some(rev) = revision {
            params.push(format!("revision={}", rev));
        }
        if let Some(req) = request {
            if let Some(ref start_date) = req.start_date {
                params.push(format!("startDate={}", start_date));
            }
            if let Some(ref end_date) = req.end_date {
                params.push(format!("endDate={}", end_date));
            }
            if let Some(ref product_ids) = req.product_id {
                for pid in product_ids {
                    params.push(format!("productId={}", pid));
                }
            }
            if let Some(ref product_types) = req.product_type {
                for pt in product_types {
                    let pt_str = serde_json::to_string(pt)
                        .unwrap_or_default()
                        .trim_matches('"')
                        .to_string();
                    params.push(format!("productType={}", pt_str));
                }
            }
            if let Some(ref sort) = req.sort {
                let sort_str = serde_json::to_string(sort)
                    .unwrap_or_default()
                    .trim_matches('"')
                    .to_string();
                params.push(format!("sort={}", sort_str));
            }
            if let Some(ref sub_groups) = req.subscription_group_identifier {
                for sg in sub_groups {
                    params.push(format!("subscriptionGroupIdentifier={}", sg));
                }
            }
            if let Some(ref ownership) = req.in_app_ownership_type {
                let ownership_str = serde_json::to_string(ownership)
                    .unwrap_or_default()
                    .trim_matches('"')
                    .to_string();
                params.push(format!("inAppOwnershipType={}", ownership_str));
            }
            if let Some(revoked) = req.revoked {
                params.push(format!("revoked={}", revoked));
            }
        }
        if !params.is_empty() {
            path = format!("{}?{}", path, params.join("&"));
        }
        self.jwt_get(&path).await
    }

    pub async fn get_transaction_history_v1(
        &self,
        transaction_id: &str,
        revision: Option<&str>,
    ) -> Result<HistoryResponse, AppleError> {
        let path = match revision {
            Some(rev) => format!("/inApps/v1/history/{}?revision={}", transaction_id, rev),
            None => format!("/inApps/v1/history/{}", transaction_id),
        };
        self.jwt_get(&path).await
    }

    pub async fn get_transaction_info(
        &self,
        transaction_id: &str,
    ) -> Result<TransactionInfoResponse, AppleError> {
        let path = format!("/inApps/v1/transactions/{}", transaction_id);
        self.jwt_get(&path).await
    }

    pub async fn look_up_order_id(
        &self,
        order_id: &str,
    ) -> Result<OrderLookupResponse, AppleError> {
        let path = format!("/inApps/v1/lookup/{}", order_id);
        self.jwt_get(&path).await
    }

    pub async fn get_app_transaction(
        &self,
        transaction_id: &str,
    ) -> Result<AppTransactionInfoResponse, AppleError> {
        let path = format!("/inApps/v1/transactions/appTransactions/{}", transaction_id);
        self.jwt_get(&path).await
    }
}
