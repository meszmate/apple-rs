use crate::error::AppleError;

use super::client::AppStoreServerClient;
use super::models::*;

impl AppStoreServerClient {
    pub async fn send_consumption_data(
        &self,
        transaction_id: &str,
        request: &ConsumptionRequest,
    ) -> Result<(), AppleError> {
        let path = format!("/inApps/v1/transactions/consumption/{}", transaction_id);
        self.jwt_put_empty_response(&path, request).await
    }

    pub async fn get_refund_history(
        &self,
        transaction_id: &str,
        revision: Option<&str>,
    ) -> Result<RefundHistoryResponse, AppleError> {
        let path = match revision {
            Some(rev) => format!(
                "/inApps/v2/refund/lookup/{}?revision={}",
                transaction_id, rev
            ),
            None => format!("/inApps/v2/refund/lookup/{}", transaction_id),
        };
        self.jwt_get(&path).await
    }

    pub async fn set_app_account_token(
        &self,
        original_transaction_id: &str,
        request: &UpdateAppAccountTokenRequest,
    ) -> Result<(), AppleError> {
        let path = format!(
            "/inApps/v1/transactions/{}/appAccountToken",
            original_transaction_id
        );
        self.jwt_put_empty_response(&path, request).await
    }
}
