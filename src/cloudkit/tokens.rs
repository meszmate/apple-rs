use crate::cloudkit::client::CloudKitClient;
use crate::error::AppleError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct CreateTokenRequest {
    #[serde(rename = "apnsEnvironment", skip_serializing_if = "Option::is_none")]
    apns_environment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenCreateResponse {
    #[serde(rename = "webcourierURL")]
    pub webcourier_url: Option<String>,
    #[serde(rename = "apnsToken")]
    pub apns_token: Option<String>,
}

#[derive(Debug, Serialize)]
struct RegisterTokenRequest {
    #[serde(rename = "apnsToken")]
    apns_token: String,
    #[serde(rename = "apnsEnvironment", skip_serializing_if = "Option::is_none")]
    apns_environment: Option<String>,
}

impl CloudKitClient {
    pub async fn create_token(
        &self,
        apns_environment: Option<&str>,
    ) -> Result<TokenCreateResponse, AppleError> {
        let url = self.build_base_url("tokens/create");
        let request = CreateTokenRequest {
            apns_environment: apns_environment.map(|s| s.to_string()),
        };

        self.signed_post(&url, &request).await
    }

    pub async fn register_token(
        &self,
        apns_token: &str,
        apns_environment: Option<&str>,
    ) -> Result<(), AppleError> {
        let url = self.build_base_url("tokens/register");
        let request = RegisterTokenRequest {
            apns_token: apns_token.to_string(),
            apns_environment: apns_environment.map(|s| s.to_string()),
        };

        #[derive(Deserialize)]
        struct EmptyResponse {}

        let _: EmptyResponse = self.signed_post(&url, &request).await?;
        Ok(())
    }
}
