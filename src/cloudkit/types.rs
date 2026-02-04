use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Environment {
    Development,
    Production,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Production => write!(f, "production"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DatabaseType {
    Public,
    Private,
    Shared,
}

impl fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseType::Public => write!(f, "public"),
            DatabaseType::Private => write!(f, "private"),
            DatabaseType::Shared => write!(f, "shared"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneID {
    #[serde(rename = "zoneName")]
    pub zone_name: String,
    #[serde(rename = "ownerRecordName", skip_serializing_if = "Option::is_none")]
    pub owner_record_name: Option<String>,
}

impl ZoneID {
    pub fn new(zone_name: &str) -> Self {
        ZoneID {
            zone_name: zone_name.to_string(),
            owner_record_name: None,
        }
    }

    pub fn default_zone() -> Self {
        ZoneID {
            zone_name: "_defaultZone".to_string(),
            owner_record_name: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    #[serde(rename = "zoneID")]
    pub zone_id: ZoneID,
    #[serde(rename = "syncToken", skip_serializing_if = "Option::is_none")]
    pub sync_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atomic: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    #[serde(rename = "recordName", skip_serializing_if = "Option::is_none")]
    pub record_name: Option<String>,
    #[serde(rename = "recordType")]
    pub record_type: String,
    #[serde(rename = "recordChangeTag", skip_serializing_if = "Option::is_none")]
    pub record_change_tag: Option<String>,
    #[serde(default)]
    pub fields: HashMap<String, FieldValue>,
    #[serde(rename = "zoneID", skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<ZoneID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RecordTimestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<RecordTimestamp>,
}

impl Record {
    pub fn new(record_type: &str) -> Self {
        Record {
            record_name: None,
            record_type: record_type.to_string(),
            record_change_tag: None,
            fields: HashMap::new(),
            zone_id: None,
            created: None,
            modified: None,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.record_name = Some(name.to_string());
        self
    }

    pub fn with_zone(mut self, zone_id: ZoneID) -> Self {
        self.zone_id = Some(zone_id);
        self
    }

    pub fn with_field(mut self, name: &str, value: FieldValue) -> Self {
        self.fields.insert(name.to_string(), value);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum FieldValue {
    #[serde(rename = "STRING")]
    String(String),
    #[serde(rename = "INT64")]
    Int64(i64),
    #[serde(rename = "DOUBLE")]
    Double(f64),
    #[serde(rename = "TIMESTAMP")]
    Timestamp(i64),
    #[serde(rename = "REFERENCE")]
    Reference(ReferenceValue),
    #[serde(rename = "ASSET")]
    Asset(AssetValue),
    #[serde(rename = "LOCATION")]
    Location(LocationValue),
    #[serde(rename = "BYTES")]
    Bytes(String),
    #[serde(rename = "STRING_LIST")]
    StringList(Vec<String>),
    #[serde(rename = "INT64_LIST")]
    Int64List(Vec<i64>),
    #[serde(rename = "DOUBLE_LIST")]
    DoubleList(Vec<f64>),
    #[serde(rename = "TIMESTAMP_LIST")]
    TimestampList(Vec<i64>),
    #[serde(rename = "REFERENCE_LIST")]
    ReferenceList(Vec<ReferenceValue>),
    #[serde(rename = "LOCATION_LIST")]
    LocationList(Vec<LocationValue>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceValue {
    #[serde(rename = "recordName")]
    pub record_name: String,
    #[serde(rename = "zoneID", skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<ZoneID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ReferenceAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReferenceAction {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "DELETE_SELF")]
    DeleteSelf,
    #[serde(rename = "VALIDATE")]
    Validate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetValue {
    #[serde(rename = "fileChecksum", skip_serializing_if = "Option::is_none")]
    pub file_checksum: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(rename = "downloadURL", skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationValue {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f64>,
    #[serde(rename = "horizontalAccuracy", skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    #[serde(rename = "verticalAccuracy", skip_serializing_if = "Option::is_none")]
    pub vertical_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordTimestamp {
    pub timestamp: i64,
    #[serde(rename = "userRecordName", skip_serializing_if = "Option::is_none")]
    pub user_record_name: Option<String>,
    #[serde(rename = "deviceID", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "forceUpdate")]
    ForceUpdate,
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "forceReplace")]
    ForceReplace,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "forceDelete")]
    ForceDelete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "subscriptionID", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "subscriptionType")]
    pub subscription_type: SubscriptionType,
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<super::query::Query>,
    #[serde(rename = "firesOn", skip_serializing_if = "Option::is_none")]
    pub fires_on: Option<Vec<FiresOn>>,
    #[serde(rename = "firesOnRecordCreation", skip_serializing_if = "Option::is_none")]
    pub fires_on_record_creation: Option<bool>,
    #[serde(rename = "firesOnRecordUpdate", skip_serializing_if = "Option::is_none")]
    pub fires_on_record_update: Option<bool>,
    #[serde(rename = "firesOnRecordDeletion", skip_serializing_if = "Option::is_none")]
    pub fires_on_record_deletion: Option<bool>,
    #[serde(rename = "notificationInfo", skip_serializing_if = "Option::is_none")]
    pub notification_info: Option<NotificationInfo>,
    #[serde(rename = "zoneID", skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<ZoneID>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionType {
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "zone")]
    Zone,
    #[serde(rename = "database")]
    Database,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FiresOn {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "delete")]
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationInfo {
    #[serde(rename = "alertBody", skip_serializing_if = "Option::is_none")]
    pub alert_body: Option<String>,
    #[serde(rename = "alertLocalizationKey", skip_serializing_if = "Option::is_none")]
    pub alert_localization_key: Option<String>,
    #[serde(rename = "alertLocalizationArgs", skip_serializing_if = "Option::is_none")]
    pub alert_localization_args: Option<Vec<String>>,
    #[serde(rename = "alertActionLocalizationKey", skip_serializing_if = "Option::is_none")]
    pub alert_action_localization_key: Option<String>,
    #[serde(rename = "alertLaunchImage", skip_serializing_if = "Option::is_none")]
    pub alert_launch_image: Option<String>,
    #[serde(rename = "soundName", skip_serializing_if = "Option::is_none")]
    pub sound_name: Option<String>,
    #[serde(rename = "shouldBadge", skip_serializing_if = "Option::is_none")]
    pub should_badge: Option<bool>,
    #[serde(rename = "shouldSendContentAvailable", skip_serializing_if = "Option::is_none")]
    pub should_send_content_available: Option<bool>,
    #[serde(rename = "shouldSendMutableContent", skip_serializing_if = "Option::is_none")]
    pub should_send_mutable_content: Option<bool>,
}
