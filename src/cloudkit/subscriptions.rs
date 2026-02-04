use crate::cloudkit::client::CloudKitClient;
use crate::cloudkit::types::*;
use crate::error::AppleError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct ModifySubscriptionsRequest {
    operations: Vec<SubscriptionOperation>,
}

#[derive(Debug, Serialize)]
struct SubscriptionOperation {
    #[serde(rename = "operationType")]
    operation_type: String,
    subscription: Subscription,
}

#[derive(Debug, Deserialize)]
pub struct ModifySubscriptionsResponse {
    pub subscriptions: Vec<Subscription>,
}

#[derive(Debug, Deserialize)]
pub struct ListSubscriptionsResponse {
    pub subscriptions: Vec<Subscription>,
}

impl CloudKitClient {
    pub async fn create_subscription(
        &self,
        db: &DatabaseType,
        subscription: Subscription,
    ) -> Result<Subscription, AppleError> {
        let url = self.build_url(db, "subscriptions/modify");
        let request = ModifySubscriptionsRequest {
            operations: vec![SubscriptionOperation {
                operation_type: "create".to_string(),
                subscription,
            }],
        };

        let response: ModifySubscriptionsResponse = self.signed_post(&url, &request).await?;
        response.subscriptions.into_iter().next()
            .ok_or_else(|| AppleError::JsonError("Empty subscription response".to_string()))
    }

    pub async fn delete_subscription(
        &self,
        db: &DatabaseType,
        subscription_id: &str,
        subscription_type: SubscriptionType,
    ) -> Result<(), AppleError> {
        let url = self.build_url(db, "subscriptions/modify");
        let request = ModifySubscriptionsRequest {
            operations: vec![SubscriptionOperation {
                operation_type: "delete".to_string(),
                subscription: Subscription {
                    subscription_id: Some(subscription_id.to_string()),
                    subscription_type,
                    query: None,
                    fires_on: None,
                    fires_on_record_creation: None,
                    fires_on_record_update: None,
                    fires_on_record_deletion: None,
                    notification_info: None,
                    zone_id: None,
                },
            }],
        };

        let _response: ModifySubscriptionsResponse = self.signed_post(&url, &request).await?;
        Ok(())
    }

    pub async fn list_subscriptions(
        &self,
        db: &DatabaseType,
    ) -> Result<Vec<Subscription>, AppleError> {
        let url = self.build_url(db, "subscriptions/list");

        #[derive(Serialize)]
        struct EmptyBody {}

        let response: ListSubscriptionsResponse = self.signed_post(&url, &EmptyBody {}).await?;
        Ok(response.subscriptions)
    }
}
