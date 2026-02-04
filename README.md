# apple-rs

A Rust library for Apple Sign-In authentication and CloudKit Web Services.

## Features

- **Apple Sign-In** — Validate authorization codes, refresh tokens, generate authorization URLs, and parse user info from JWT ID tokens.
- **CloudKit Web Services** — Full CRUD for records, zones, subscriptions, change tracking, user discovery, asset uploads, and APNs token management.
- **Shared key management** — A single ECDSA P-256 key pair can be shared between Sign-In and CloudKit.
- **Async/await** — All network operations are async.

## Cargo Features

| Feature    | Default | Description                          |
|------------|---------|--------------------------------------|
| `auth`     | Yes     | Apple Sign-In authentication         |
| `cloudkit` | Yes     | CloudKit Web Services (adds `sha2`, `chrono`) |

```toml
[dependencies]
apple = "0.2.0"

# Or pick features:
# apple = { version = "0.2.0", default-features = false, features = ["auth"] }
# apple = { version = "0.2.0", default-features = false, features = ["cloudkit"] }
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
    }),
    zone_id: Some(ZoneID::new("MyZone")),
};

let sub = client.create_subscription(&DatabaseType::Private, subscription).await?;
let subs = client.list_subscriptions(&DatabaseType::Private).await?;
client.delete_subscription(&DatabaseType::Private, "my-sub", SubscriptionType::Zone).await?;
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

## Shared Key Management

You can share a single key pair between Apple Sign-In and CloudKit:

```rust
use apple::signing::AppleKeyPair;
use apple::auth::AppleAuthImpl;
use apple::cloudkit::{CloudKitClient, CloudKitConfig, Environment};

let key_pair = AppleKeyPair::from_file("key-id", "AuthKey.p8")?;

// Use with Sign-In
let auth = AppleAuthImpl::from_key_pair("app-id", "team-id", key_pair.clone())?;

// Use with CloudKit
let cloudkit = CloudKitClient::new(CloudKitConfig {
    container: "iCloud.com.company.app".to_string(),
    environment: Environment::Production,
    key_pair,
})?;
```

## Error Handling

All operations return `Result<T, AppleError>`. CloudKit-specific errors include the server error code and reason:

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
    Err(AppleError::ResponseError(e)) => println!("Auth error: {}", e),
    Err(e) => println!("Error: {}", e),
}
```

## License

MIT
