use crate::error::AppleError;

use super::client::AppStoreServerClient;
use super::models::*;

impl AppStoreServerClient {
    pub async fn get_all_subscription_statuses(
        &self,
        transaction_id: &str,
    ) -> Result<StatusResponse, AppleError> {
        let path = format!("/inApps/v1/subscriptions/{}", transaction_id);
        self.jwt_get(&path).await
    }

    pub async fn extend_renewal_date(
        &self,
        original_transaction_id: &str,
        request: &ExtendRenewalDateRequest,
    ) -> Result<ExtendRenewalDateResponse, AppleError> {
        let path = format!(
            "/inApps/v1/subscriptions/extend/{}",
            original_transaction_id
        );
        self.jwt_put(&path, request).await
    }

    pub async fn mass_extend_renewal_date(
        &self,
        request: &MassExtendRenewalDateRequest,
    ) -> Result<MassExtendRenewalDateResponse, AppleError> {
        self.jwt_post("/inApps/v1/subscriptions/extend/mass", request)
            .await
    }

    pub async fn get_mass_extension_status(
        &self,
        product_id: &str,
        request_identifier: &str,
    ) -> Result<MassExtendRenewalDateStatusResponse, AppleError> {
        let path = format!(
            "/inApps/v1/subscriptions/extend/mass/{}/{}",
            product_id, request_identifier
        );
        self.jwt_get(&path).await
    }
}
