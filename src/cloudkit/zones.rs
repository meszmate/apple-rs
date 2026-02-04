use crate::cloudkit::client::CloudKitClient;
use crate::cloudkit::types::*;
use crate::error::AppleError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct ModifyZonesRequest {
    operations: Vec<ZoneOperation>,
}

#[derive(Debug, Serialize)]
struct ZoneOperation {
    #[serde(rename = "operationType")]
    operation_type: String,
    zone: ZoneBody,
}

#[derive(Debug, Serialize)]
struct ZoneBody {
    #[serde(rename = "zoneID")]
    zone_id: ZoneID,
    #[serde(skip_serializing_if = "Option::is_none")]
    atomic: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ModifyZonesResponse {
    pub zones: Vec<Zone>,
}

#[derive(Debug, Deserialize)]
pub struct ListZonesResponse {
    pub zones: Vec<Zone>,
}

impl CloudKitClient {
    pub async fn create_zone(
        &self,
        db: &DatabaseType,
        zone_id: ZoneID,
        atomic: Option<bool>,
    ) -> Result<Zone, AppleError> {
        let url = self.build_url(db, "zones/modify");
        let request = ModifyZonesRequest {
            operations: vec![ZoneOperation {
                operation_type: "create".to_string(),
                zone: ZoneBody { zone_id, atomic },
            }],
        };

        let response: ModifyZonesResponse = self.signed_post(&url, &request).await?;
        response.zones.into_iter().next()
            .ok_or_else(|| AppleError::JsonError("Empty zone response".to_string()))
    }

    pub async fn delete_zone(
        &self,
        db: &DatabaseType,
        zone_id: ZoneID,
    ) -> Result<(), AppleError> {
        let url = self.build_url(db, "zones/modify");
        let request = ModifyZonesRequest {
            operations: vec![ZoneOperation {
                operation_type: "delete".to_string(),
                zone: ZoneBody { zone_id, atomic: None },
            }],
        };

        let _response: ModifyZonesResponse = self.signed_post(&url, &request).await?;
        Ok(())
    }

    pub async fn list_zones(
        &self,
        db: &DatabaseType,
    ) -> Result<Vec<Zone>, AppleError> {
        let url = self.build_url(db, "zones/list");

        #[derive(Serialize)]
        struct EmptyBody {}

        let response: ListZonesResponse = self.signed_post(&url, &EmptyBody {}).await?;
        Ok(response.zones)
    }
}
