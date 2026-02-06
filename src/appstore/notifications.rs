use crate::error::AppleError;

use super::client::AppStoreServerClient;
use super::models::*;

impl AppStoreServerClient {
    pub async fn request_test_notification(
        &self,
    ) -> Result<SendTestNotificationResponse, AppleError> {
        self.jwt_post_empty_body("/inApps/v1/notifications/test")
            .await
    }

    pub async fn get_test_notification_status(
        &self,
        token: &str,
    ) -> Result<CheckTestNotificationResponse, AppleError> {
        let path = format!("/inApps/v1/notifications/test/{}", token);
        self.jwt_get(&path).await
    }

    pub async fn get_notification_history(
        &self,
        request: &NotificationHistoryRequest,
        pagination_token: Option<&str>,
    ) -> Result<NotificationHistoryResponse, AppleError> {
        let path = match pagination_token {
            Some(token) => format!("/inApps/v1/notifications/history?paginationToken={}", token),
            None => "/inApps/v1/notifications/history".to_string(),
        };
        self.jwt_post(&path, request).await
    }
}
