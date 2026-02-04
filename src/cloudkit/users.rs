use crate::cloudkit::client::CloudKitClient;
use crate::error::AppleError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct CloudKitUser {
    #[serde(rename = "userRecordName")]
    pub user_record_name: Option<String>,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[serde(rename = "emailAddress")]
    pub email_address: Option<String>,
    #[serde(rename = "isDiscoverable")]
    pub is_discoverable: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct CurrentUserResponse {
    #[serde(rename = "userRecordName")]
    user_record_name: Option<String>,
    #[serde(rename = "firstName")]
    first_name: Option<String>,
    #[serde(rename = "lastName")]
    last_name: Option<String>,
    #[serde(rename = "emailAddress")]
    email_address: Option<String>,
    #[serde(rename = "isDiscoverable")]
    is_discoverable: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct DiscoverUsersResponse {
    users: Vec<CloudKitUser>,
}

#[derive(Debug, Serialize)]
struct LookupUsersRequest {
    #[serde(rename = "emailAddresses")]
    email_addresses: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct LookupUsersResponse {
    users: Vec<CloudKitUser>,
}

impl CloudKitClient {
    pub async fn get_current_user(&self) -> Result<CloudKitUser, AppleError> {
        let url = self.build_base_url("users/caller");

        #[derive(Serialize)]
        struct EmptyBody {}

        let response: CurrentUserResponse = self.signed_post(&url, &EmptyBody {}).await?;

        Ok(CloudKitUser {
            user_record_name: response.user_record_name,
            first_name: response.first_name,
            last_name: response.last_name,
            email_address: response.email_address,
            is_discoverable: response.is_discoverable,
        })
    }

    pub async fn discover_users(&self) -> Result<Vec<CloudKitUser>, AppleError> {
        let url = self.build_base_url("users/discover");

        #[derive(Serialize)]
        struct EmptyBody {}

        let response: DiscoverUsersResponse = self.signed_post(&url, &EmptyBody {}).await?;
        Ok(response.users)
    }

    pub async fn lookup_users_by_email(
        &self,
        email_addresses: &[&str],
    ) -> Result<Vec<CloudKitUser>, AppleError> {
        let url = self.build_base_url("users/lookup/email");
        let request = LookupUsersRequest {
            email_addresses: email_addresses.iter().map(|s| s.to_string()).collect(),
        };

        let response: LookupUsersResponse = self.signed_post(&url, &request).await?;
        Ok(response.users)
    }
}
