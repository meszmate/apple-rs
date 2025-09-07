# Apple Auth Rust Library

A Rust library for handling Apple Sign-In token validation and authentication with Apple servers. This library provides functionality to validate authorization codes, refresh tokens, generate authorization URLs, and parse user information from JWT ID tokens.

## Features

- Validate Apple Sign-In authorization codes and refresh tokens.
- Generate Apple Sign-In authorization URLs with customizable parameters.
- Parse user information from JWT ID tokens.
- Comprehensive error handling for Apple's authentication responses.
- Supports both file-based and base64-encoded private keys.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
apple = "0.1.1"
```

Ensure you have the required dependencies installed. The library depends on:

- reqwest for HTTP requests
- serde for JSON serialization/deserialization
- jsonwebtoken for JWT handling
- p256 for ECDSA private key parsing
- pem for PEM-encoded key parsing
- base64 for base64 decoding
- url for URL manipulation
- futures for async runtime compatibility


## Usage

### Initializing the AppleAuth Client
Create an `AppleAuthImpl` instance using either a file path to a `.p8` key or a base64-encoded key.
```rust
use apple::auth::{AppleAuth, AppleAuthImpl};

fn main() -> Result<(), apple_auth::error::AppleError> {
    // Using a .p8 file
    let auth = AppleAuthImpl::new(
        "your-app-id",
        "your-team-id",
        "your-key-id",
        "path/to/AuthKey_ABCDE12345.p8",
    )?;

    // Or using a base64-encoded key
    let auth = AppleAuthImpl::new_b64(
        "your-app-id",
        "your-team-id",
        "your-key-id",
        "base64-encoded-key",
    )?;

    Ok(())
}
```

### Validating an Authorization Code
Validate an authorization code to obtain a `TokenResponse` containing access tokens, refresh tokens, and ID tokens.
```rust
use apple::auth::AppleAuth;

fn main() -> Result<(), apple_auth::error::AppleError> {
    let auth = AppleAuthImpl::new(
        "your-app-id",
        "your-team-id",
        "your-key-id",
        "path/to/AuthKey_ABCDE12345.p8",
    )?;

    let token_response = auth.validate_code("authorization-code")?;
    println!("Token Response: {:?}", token_response);

    Ok(())
}
```

### Validating with Redirect URI
Validate an authorization code with a redirect URI.
```rust
use apple::auth::AppleAuth;

fn main() -> Result<(), apple_auth::error::AppleError> {
    let auth = AppleAuthImpl::new(
        "your-app-id",
        "your-team-id",
        "your-key-id",
        "path/to/AuthKey_ABCDE12345.p8",
    )?;

    let token_response = auth.validate_code_with_redirect_uri(
        "authorization-code",
        "https://your-redirect-uri.com",
    )?;
    println!("Token Response: {:?}", token_response);

    Ok(())
}
```

### Validating a Refresh Token
Validate a refresh token to obtain a new `TokenResponse`.
```rust
use apple::auth::AppleAuth;

fn main() -> Result<(), apple_auth::error::AppleError> {
    let auth = AppleAuthImpl::new(
        "your-app-id",
        "your-team-id",
        "your-key-id",
        "path/to/AuthKey_ABCDE12345.p8",
    )?;

    let token_response = auth.validate_refresh_token("refresh-token")?;
    println!("Token Response: {:?}", token_response);

    Ok(())
}
```

### Generating an Authorization URL
Generate an authorization URL for Apple Sign-In.
```rust
use apple::url::{AuthorizeURLConfig, ResponseMode, ResponseType};

fn main() {
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
    println!("Authorization URL: {}", url);
}
```

### Parsing User Information from ID Token
Extract user information from a JWT ID token.
```rust
use apple::user::get_user_info_from_id_token;

fn main() -> Result<(), apple_auth::error::AppleError> {
    let id_token = "your-id-token";
    let user = get_user_info_from_id_token(id_token)?;
    println!("User Info: {:?}", user);

    Ok(())
}
```

### Error Handling
The library provides comprehensive error handling for Apple's authentication responses. Errors are represented by the `AppleError` enum, which includes specific response errors like `ErrorResponseInvalidRequest`, `ErrorResponseInvalidClient`, etc.
```rust
use apple::error::AppleError;

fn main() {
    match some_auth_operation() {
        Ok(response) => println!("Success: {:?}", response),
        Err(AppleError::ResponseError(err)) => println!("Apple Error: {}", err),
        Err(e) => println!("Other Error: {}", e),
    }
}
```

# Notes

- **JWT Validation**: The get_user_info_from_id_token function does not validate the JWT signature for simplicity. In a production environment, you should validate the signature using Apple's public keys.
- **Async/Sync**: The library uses asynchronous calls for compatibility with most of the web frameworks. For sync usage, consider modifying the AppleAuth trait to use async methods.
- **Security**: Store private keys and tokens securely. Avoid hardcoding sensitive information in your code.

# License
This project is licensed under the MIT License.
