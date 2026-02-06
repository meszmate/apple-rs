use crate::error::AppleError;

use super::client::AppStoreServerClient;
use super::models::*;

impl AppStoreServerClient {
    pub async fn upload_image(
        &self,
        image_identifier: &str,
        data: Vec<u8>,
    ) -> Result<ImageListItem, AppleError> {
        let path = format!("/inApps/v1/messaging/image/{}", image_identifier);
        self.jwt_put_bytes(&path, data).await
    }

    pub async fn delete_image(&self, image_identifier: &str) -> Result<(), AppleError> {
        let path = format!("/inApps/v1/messaging/image/{}", image_identifier);
        self.jwt_delete(&path).await
    }

    pub async fn get_image_list(&self) -> Result<ImageListResponse, AppleError> {
        self.jwt_get("/inApps/v1/messaging/image/list").await
    }

    pub async fn upload_message(
        &self,
        message_identifier: &str,
        body: &UploadMessageRequest,
    ) -> Result<MessageListItem, AppleError> {
        let path = format!("/inApps/v1/messaging/message/{}", message_identifier);
        self.jwt_put(&path, body).await
    }

    pub async fn delete_message(&self, message_identifier: &str) -> Result<(), AppleError> {
        let path = format!("/inApps/v1/messaging/message/{}", message_identifier);
        self.jwt_delete(&path).await
    }

    pub async fn get_message_list(&self) -> Result<MessageListResponse, AppleError> {
        self.jwt_get("/inApps/v1/messaging/message/list").await
    }

    pub async fn configure_default_message(
        &self,
        product_id: &str,
        locale: &str,
        body: &DefaultMessageRequest,
    ) -> Result<(), AppleError> {
        let path = format!("/inApps/v1/messaging/default/{}/{}", product_id, locale);
        self.jwt_put_empty_response(&path, body).await
    }

    pub async fn delete_default_message(
        &self,
        product_id: &str,
        locale: &str,
    ) -> Result<(), AppleError> {
        let path = format!("/inApps/v1/messaging/default/{}/{}", product_id, locale);
        self.jwt_delete(&path).await
    }
}
