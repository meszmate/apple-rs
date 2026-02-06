# apple-rs

A Rust library for Apple Sign-In authentication, CloudKit Web Services, and the App Store Server API.

## Features

- **Apple Sign-In** — Validate authorization codes, refresh tokens, generate authorization URLs, and parse user info from JWT ID tokens.
- **CloudKit Web Services** — Full CRUD for records, zones, subscriptions, change tracking, user discovery, asset uploads, APNs token management, and push notification parsing.
- **App Store Server API** — Transaction history, subscription management, consumption reporting, refund lookup, server notification handling (V1 & V2), JWS signed data verification with X.509 chain validation, and retention messaging.
- **Shared key management** — A single ECDSA P-256 key pair can be shared across Sign-In, CloudKit, and App Store.
- **Async/await** — All network operations are async.

## Cargo Features

| Feature    | Default | Description                                           |
|------------|---------|-------------------------------------------------------|
| `auth`     | Yes     | Apple Sign-In authentication                          |
| `cloudkit` | Yes     | CloudKit Web Services (adds `sha2`, `chrono`)         |
| `appstore` | No      | App Store Server API (adds `chrono`, `x509-cert`)     |

```toml
[dependencies]
apple = "0.2.0"

# Or pick features:
# apple = { version = "0.2.0", default-features = false, features = ["auth"] }
# apple = { version = "0.2.0", default-features = false, features = ["cloudkit"] }
# apple = { version = "0.2.0", features = ["appstore"] }
```

## Apple Sign-In

### Initializing the Auth Client

```rust
use apple::auth::{AppleAuth, AppleAuthImpl};

// From a .p8 file
let auth = AppleAuthImpl::new(
    "your-app-id",
    "your-team-id",
    "your-key-id",
    "path/to/AuthKey.p8",
)?;

// Or from a base64-encoded key
let auth = AppleAuthImpl::new_b64(
    "your-app-id",
    "your-team-id",
    "your-key-id",
    "base64-encoded-key",
)?;
```

### Validating an Authorization Code

```rust
let token_response = auth.validate_code("authorization-code").await?;
println!("Access token: {}", token_response.access_token);
```

### Validating with a Redirect URI

```rust
let token_response = auth.validate_code_with_redirect_uri(
    "authorization-code",
    "https://your-redirect-uri.com",
).await?;
```

### Refreshing a Token

```rust
let token_response = auth.validate_refresh_token("refresh-token").await?;
```

### Generating an Authorization URL

```rust
use apple::url::{AuthorizeURLConfig, ResponseMode, ResponseType};

let config = AuthorizeURLConfig {
    client_id: "your-client-id".to_string(),
    redirect_uri: "https://your-redirect-uri.com".to_string(),
    state: Some("state".to_string()),
    scope: Some(vec!["email".to_string(), "name".to_string()]),
    nonce: Some("nonce".to_string()),
    response_mode: Some(ResponseMode::FormPost),
    response_type: Some(ResponseType::CodeId),
};

let url = apple::url::authorize_url(config);
```

### Parsing User Info from ID Token

```rust
use apple::user::get_user_info_from_id_token;

let user = get_user_info_from_id_token("id-token")?;
println!("Email: {:?}", user.email);
```

## CloudKit Web Services

### Setup

```rust
use apple::signing::AppleKeyPair;
use apple::cloudkit::{CloudKitClient, CloudKitConfig, Environment};

let key_pair = AppleKeyPair::from_file("your-key-id", "path/to/AuthKey.p8")?;

let client = CloudKitClient::new(CloudKitConfig {
    container: "iCloud.com.company.app".to_string(),
    environment: Environment::Development,
    key_pair,
})?;
```

### Record CRUD

```rust
use apple::cloudkit::{Record, FieldValue, DatabaseType};

// Create
let record = Record::new("MyRecordType")
    .with_name("my-record-1")
    .with_field("title", FieldValue::String("Hello".to_string()))
    .with_field("count", FieldValue::Int64(42));

let created = client.create_record(&DatabaseType::Public, record).await?;

// Update
let mut updated = created;
updated.fields.insert("title".to_string(), FieldValue::String("Updated".to_string()));
let updated = client.update_record(&DatabaseType::Public, updated).await?;

// Delete
client.delete_record(&DatabaseType::Public, "my-record-1", "MyRecordType", None).await?;

// Lookup
let records = client.lookup_records(&DatabaseType::Public, &["id-1", "id-2"], None, None).await?;
```

### Querying with QueryBuilder

```rust
use apple::cloudkit::{QueryBuilder, Comparator, FieldValue, DatabaseType};

let query = QueryBuilder::new("MyRecordType")
    .filter("status", Comparator::Equals, FieldValue::String("active".to_string()))
    .sort("createdAt", false)
    .build();

let response = client.query_records(&DatabaseType::Public, query, None, Some(20), None, None).await?;
for record in &response.records {
    println!("{:?}", record.record_name);
}
```

### Zone Management

```rust
use apple::cloudkit::{ZoneID, DatabaseType};

let zone = client.create_zone(&DatabaseType::Private, ZoneID::new("MyZone"), None).await?;
let zones = client.list_zones(&DatabaseType::Private).await?;
client.delete_zone(&DatabaseType::Private, ZoneID::new("MyZone")).await?;
```

### Subscriptions & Push Notifications

```rust
use apple::cloudkit::{Subscription, SubscriptionType, NotificationInfo, DatabaseType, ZoneID};

let subscription = Subscription {
    subscription_id: Some("my-sub".to_string()),
    subscription_type: SubscriptionType::Zone,
    query: None,
    fires_on: None,
    fires_on_record_creation: Some(true),
    fires_on_record_update: Some(true),
    fires_on_record_deletion: Some(true),
    notification_info: Some(NotificationInfo {
        alert_body: Some("Record changed".to_string()),
        alert_localization_key: None,
        alert_localization_args: None,
        alert_action_localization_key: None,
        alert_launch_image: None,
        sound_name: None,
        should_badge: Some(true),
        should_send_content_available: Some(true),
        should_send_mutable_content: None,
        collapse_id_key: None,
        desired_keys: None,
        category: None,
        title_localization_key: None,
        title_localization_args: None,
        subtitle_localization_key: None,
        subtitle_localization_args: None,
    }),
    zone_id: Some(ZoneID::new("MyZone")),
};

let sub = client.create_subscription(&DatabaseType::Private, subscription).await?;
let subs = client.list_subscriptions(&DatabaseType::Private).await?;
client.delete_subscription(&DatabaseType::Private, "my-sub", SubscriptionType::Zone).await?;
```

### CloudKit Push Notification Parsing

Parse incoming APNs payloads containing CloudKit notification data:

```rust
use apple::cloudkit::notifications::{parse_notification, CKNotification};

let apns_json = r#"{
    "aps": { "content-available": 1 },
    "ck": {
        "cid": "iCloud.com.company.app",
        "nid": "notification-uuid",
        "rid": { "recordName": "record-1", "zoneID": { "zoneName": "MyZone" } },
        "rt": "MyRecordType",
        "fo": 1,
        "dbs": 2
    }
}"#;

let notification = parse_notification(apns_json)?;
match notification {
    CKNotification::Query(query) => {
        println!("Record type: {:?}", query.record_type);
        println!("Reason: {:?}", query.reason); // RecordCreated, RecordUpdated, RecordDeleted
    }
    CKNotification::RecordZone(zone) => {
        println!("Zone changed: {:?}", zone.zone_id);
    }
    CKNotification::Database(db) => {
        println!("Database scope: {:?}", db.database_scope);
    }
}
```

### WebCourier Long-Polling

Poll for CloudKit notifications using Apple's webcourier service:

```rust
let notifications = client.poll_notifications("https://api.apple-cloudkit.com/...webcourier-url...")
    .await?;

for notification in notifications {
    println!("{:?}", notification);
}
```

### Change Tracking

```rust
use apple::cloudkit::{ZoneID, DatabaseType};

// Zone changes
let changes = client.fetch_zone_changes(
    &DatabaseType::Private,
    ZoneID::new("MyZone"),
    None, // sync_token (None for first fetch)
    Some(100),
).await?;

// Use changes.sync_token for subsequent fetches
// changes.more_coming indicates if there are more changes

// Database changes
let db_changes = client.fetch_database_changes(
    &DatabaseType::Private,
    None,
    None,
).await?;
```

### User Discovery

```rust
let current_user = client.get_current_user().await?;
let users = client.discover_users().await?;
let found = client.lookup_users_by_email(&["user@example.com"]).await?;
```

### Asset Uploads

```rust
use apple::cloudkit::DatabaseType;

// Step 1: Request upload URL
let upload = client.request_asset_upload(
    &DatabaseType::Private,
    "record-name",
    "RecordType",
    "assetField",
    None,
).await?;

// Step 2: Upload data to the returned URL
if let Some(token) = upload.tokens.first() {
    if let Some(ref url) = token.url {
        let result = client.upload_asset(url, b"file-contents").await?;
    }
}
```

## App Store Server API

### Setup

```rust
use std::sync::Arc;
use apple::signing::AppleKeyPair;
use apple::appstore::{AppStoreServerClient, AppStoreConfig, AppStoreEnvironment};

let key_pair = AppleKeyPair::from_file("your-key-id", "path/to/SubscriptionKey.p8")?;

let client = AppStoreServerClient::new(AppStoreConfig {
    issuer_id: "your-issuer-id".to_string(),
    bundle_id: "com.company.app".to_string(),
    key_pair,
    environment: AppStoreEnvironment::Production,
})?;
```

### Transaction History

```rust
use apple::appstore::TransactionHistoryRequest;

// Get full transaction history
let history = client.get_transaction_history("transaction-id", None, None).await?;
for signed_tx in &history.signed_transactions {
    println!("Signed transaction: {}", signed_tx);
}

// Paginate with revision
if history.has_more {
    let next = client.get_transaction_history(
        "transaction-id",
        Some(&history.revision),
        None,
    ).await?;
}

// With filters
let request = TransactionHistoryRequest {
    sort: Some(apple::appstore::Order::DESCENDING),
    product_type: Some(vec![apple::appstore::TransactionHistoryProductType::AUTO_RENEWABLE]),
    ..Default::default()
};
let filtered = client.get_transaction_history("transaction-id", None, Some(&request)).await?;
```

### Transaction Info

```rust
let info = client.get_transaction_info("transaction-id").await?;
println!("Signed info: {}", info.signed_transaction_info);
```

### Order Lookup

```rust
let order = client.look_up_order_id("order-id").await?;
println!("Status: {:?}", order.status); // Valid or Invalid
```

### Subscription Status

```rust
let status = client.get_all_subscription_statuses("transaction-id").await?;
for group in &status.data {
    println!("Group: {}", group.subscription_group_identifier);
    for item in &group.last_transactions {
        println!("  Status: {:?}", item.status);
    }
}
```

### Extend Subscription Renewal Date

```rust
use apple::appstore::{ExtendRenewalDateRequest, ExtendReasonCode};

let request = ExtendRenewalDateRequest {
    extend_by_days: 30,
    extend_reason_code: ExtendReasonCode::CustomerSatisfaction,
    request_identifier: "unique-request-id".to_string(),
};

let response = client.extend_renewal_date("original-transaction-id", &request).await?;
println!("New effective date: {}", response.effective_date);
```

### Mass Extend Renewal Dates

```rust
use apple::appstore::{MassExtendRenewalDateRequest, ExtendReasonCode};

let request = MassExtendRenewalDateRequest {
    extend_by_days: 7,
    extend_reason_code: ExtendReasonCode::ServiceIssueOrOutage,
    request_identifier: "mass-extend-id".to_string(),
    storefront_country_codes: None,
    product_id: "com.company.app.subscription".to_string(),
};

let response = client.mass_extend_renewal_date(&request).await?;

// Check status later
let status = client.get_mass_extension_status(
    "com.company.app.subscription",
    &response.request_identifier,
).await?;
println!("Complete: {}", status.complete);
```

### Consumption Data

```rust
use apple::appstore::*;

let request = ConsumptionRequest {
    account_tenure: AccountTenure::ThirtyToNinetyDays,
    app_account_token: "user-token".to_string(),
    consumption_status: ConsumptionStatus::NotConsumed,
    customer_consented: true,
    delivery_status: Some(DeliveryStatus::DeliveredAndWorking),
    lifetime_dollars_purchased: Some(LifetimeDollarsPurchased::OneToFortyNine),
    lifetime_dollars_refunded: Some(LifetimeDollarsRefunded::Zero),
    platform: Platform::Apple,
    play_time: Some(PlayTime::OneToSixHours),
    refund_preference: Some(RefundPreference::DECLINE),
    sample_content_provided: Some(false),
    user_status: UserStatus::Active,
};

client.send_consumption_data("transaction-id", &request).await?;
```

### Refund History

```rust
let refunds = client.get_refund_history("transaction-id", None).await?;
for signed_tx in &refunds.signed_transactions {
    println!("Refunded: {}", signed_tx);
}

// Paginate
if refunds.has_more {
    let next = client.get_refund_history("transaction-id", Some(&refunds.revision)).await?;
}
```

### Test Notifications

```rust
// Request a test notification
let response = client.request_test_notification().await?;

// Check the status
let status = client.get_test_notification_status(&response.test_notification_token).await?;
println!("Payload: {}", status.signed_payload);
```

### Notification History

```rust
use apple::appstore::{NotificationHistoryRequest, NotificationTypeV2};

let request = NotificationHistoryRequest {
    start_date: 1700000000000,
    end_date: 1700100000000,
    notification_type: Some(NotificationTypeV2::DID_RENEW),
    notification_subtype: None,
    only_failures: Some(false),
    transaction_id: None,
};

let history = client.get_notification_history(&request, None).await?;
for item in &history.notification_history {
    println!("Payload: {}", item.signed_payload);
}
```

### Retention Messaging

```rust
use apple::appstore::{UploadMessageRequest, DefaultMessageRequest};

// Upload an image for retention messaging
let image_data = std::fs::read("banner.png").unwrap();
let image = client.upload_image("my-image-id", image_data).await?;

// List images
let images = client.get_image_list().await?;

// Upload a message
let message = UploadMessageRequest {
    title: "We miss you!".to_string(),
    body: "Come back and check out new features.".to_string(),
    image_identifier: Some("my-image-id".to_string()),
};
let msg = client.upload_message("my-message-id", &message).await?;

// List messages
let messages = client.get_message_list().await?;

// Configure a default message for a product
let default_msg = DefaultMessageRequest {
    title: "Your subscription".to_string(),
    body: "Renew to keep access.".to_string(),
};
client.configure_default_message("com.company.app.sub", "en-US", &default_msg).await?;

// Clean up
client.delete_message("my-message-id").await?;
client.delete_image("my-image-id").await?;
client.delete_default_message("com.company.app.sub", "en-US").await?;
```

### JWS Signed Data Verification

Verify and decode signed transaction data, renewal info, and server notifications using X.509 certificate chain validation:

```rust
use apple::appstore::{SignedDataVerifier, AppStoreEnvironment};

// Load Apple Root CA certificate
let root_cert = std::fs::read("AppleRootCA-G3.cer").unwrap();

let verifier = SignedDataVerifier::new(
    vec![root_cert],
    "com.company.app",
    AppStoreEnvironment::Production,
    Some(123456789), // your app's Apple ID
);

// Verify and decode a signed transaction
let transaction = verifier.verify_and_decode_transaction("eyJ...")?;
println!("Product: {}", transaction.product_id);
println!("Purchase date: {}", transaction.purchase_date);
println!("Expires: {:?}", transaction.expires_date);

// Verify and decode renewal info
let renewal = verifier.verify_and_decode_renewal_info("eyJ...")?;
println!("Auto-renew status: {:?}", renewal.auto_renew_status);
println!("Auto-renew product: {}", renewal.auto_renew_product_id);

// Verify and decode a server notification
let notification = verifier.verify_and_decode_notification("eyJ...")?;
println!("Type: {:?}", notification.notification_type);
println!("Subtype: {:?}", notification.subtype);

// Verify and decode an app transaction
let app_tx = verifier.verify_and_decode_app_transaction("eyJ...")?;
println!("Bundle: {}", app_tx.bundle_id);
```

### Handling Server Notifications V2

Parse webhook payloads from Apple's App Store Server Notifications V2:

```rust
use apple::appstore::{ResponseBodyV2, NotificationTypeV2, Subtype};

// In your webhook handler, deserialize the request body
let body: ResponseBodyV2 = serde_json::from_str(&request_body)?;

// Then verify and decode the signed payload
let decoded = verifier.verify_and_decode_notification(&body.signed_payload)?;

match decoded.notification_type {
    NotificationTypeV2::SUBSCRIBED => {
        println!("New subscription!");
    }
    NotificationTypeV2::DID_RENEW => {
        println!("Subscription renewed");
    }
    NotificationTypeV2::EXPIRED => {
        if decoded.subtype == Some(Subtype::VOLUNTARY) {
            println!("User cancelled");
        }
    }
    NotificationTypeV2::REFUND => {
        println!("Refund processed");
    }
    _ => {}
}

// Access the notification data
if let Some(data) = &decoded.data {
    println!("Bundle: {}", data.bundle_id);
    if let Some(ref signed_tx) = data.signed_transaction_info {
        let transaction = verifier.verify_and_decode_transaction(signed_tx)?;
        println!("Product: {}", transaction.product_id);
    }
}
```

### Handling Server Notifications V1 (Deprecated)

```rust
use apple::appstore::ServerNotificationV1;

let notification: ServerNotificationV1 = serde_json::from_str(&request_body)?;
println!("Type: {:?}", notification.notification_type);

if let Some(receipt) = &notification.unified_receipt {
    if let Some(info) = &receipt.latest_receipt_info {
        for item in info {
            println!("Product: {:?}", item.product_id);
        }
    }
}
```

## Shared Key Management

You can share a single key pair between Apple Sign-In, CloudKit, and the App Store:

```rust
use apple::signing::AppleKeyPair;
use apple::auth::AppleAuthImpl;
use apple::cloudkit::{CloudKitClient, CloudKitConfig, Environment};
use apple::appstore::{AppStoreServerClient, AppStoreConfig, AppStoreEnvironment};

let key_pair = AppleKeyPair::from_file("key-id", "AuthKey.p8")?;

// Use with Sign-In
let auth = AppleAuthImpl::from_key_pair("app-id", "team-id", key_pair.clone())?;

// Use with CloudKit
let cloudkit = CloudKitClient::new(CloudKitConfig {
    container: "iCloud.com.company.app".to_string(),
    environment: Environment::Production,
    key_pair: key_pair.clone(),
})?;

// Use with App Store Server API
let appstore = AppStoreServerClient::new(AppStoreConfig {
    issuer_id: "issuer-id".to_string(),
    bundle_id: "com.company.app".to_string(),
    key_pair,
    environment: AppStoreEnvironment::Production,
})?;
```

## Error Handling

All operations return `Result<T, AppleError>`. Each module has specific error variants:

```rust
use apple::error::AppleError;

match result {
    Ok(record) => println!("Success"),
    Err(AppleError::CloudKitError(e)) => {
        println!("CloudKit error: {} - {}", e.server_error_code, e.reason);
        if let Some(retry) = e.retry_after {
            println!("Retry after {} seconds", retry);
        }
    }
    Err(AppleError::AppStoreError(e)) => {
        println!("App Store error {}: {}", e.error_code, e.error_message);
    }
    Err(AppleError::CertificateError(msg)) => {
        println!("Certificate validation failed: {}", msg);
    }
    Err(AppleError::ResponseError(e)) => println!("Auth error: {}", e),
    Err(e) => println!("Error: {}", e),
}
```

App Store error codes can be inspected for programmatic handling:

```rust
use apple::appstore::AppStoreErrorCode;

let code = AppStoreErrorCode::from_code(4040010);
assert_eq!(code, AppStoreErrorCode::SubscriptionNotFound);
println!("Error code: {}", code.code()); // 4040010
```

## License

MIT
