use std::fmt;

#[derive(Debug, Clone)]
pub enum AppleError {
    IoError(String),
    Base64Error(String),
    PemError(String),
    KeyParseError(String),
    JwtError(String),
    HttpError(String),
    JsonError(String),
    TimeError(String),
    UnrecognizedError(String),
    ResponseError(ErrorResponse),
    #[cfg(feature = "cloudkit")]
    CloudKitError(CloudKitErrorResponse),
    #[cfg(feature = "cloudkit")]
    SignatureError(String),
}

#[derive(Debug, Clone)]
pub struct ErrorResponse {
    pub error_type: ErrorResponseType,
    pub message: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorResponseType {
    InvalidRequest,
    InvalidClient,
    InvalidGrant,
    UnauthorizedClient,
    UnsupportedGrantType,
    InvalidScope,
}

pub const ERROR_RESPONSE_INVALID_REQUEST: AppleError = AppleError::ResponseError(ErrorResponse {
    error_type: ErrorResponseType::InvalidRequest,
    message: "The request is malformed, typically because it is missing a parameter, contains an unsupported parameter, includes multiple credentials, or uses more than one mechanism for authenticating the client.",
});

pub const ERROR_RESPONSE_INVALID_CLIENT: AppleError = AppleError::ResponseError(ErrorResponse {
    error_type: ErrorResponseType::InvalidClient,
    message: "The client authentication failed, typically due to a mismatched or invalid client identifier, invalid client secret (expired token, malformed claims, or invalid signature), or mismatched or invalid redirect URI.",
});

pub const ERROR_RESPONSE_INVALID_GRANT: AppleError = AppleError::ResponseError(ErrorResponse {
    error_type: ErrorResponseType::InvalidGrant,
    message: "The authorization grant or refresh token is invalid, typically due to a mismatched or invalid client identifier, invalid code (expired or previously used authorization code), or invalid refresh token.",
});

pub const ERROR_RESPONSE_UNAUTHORIZED_CLIENT: AppleError =
    AppleError::ResponseError(ErrorResponse {
        error_type: ErrorResponseType::UnauthorizedClient,
        message: "The client is not authorized to use this authorization grant type.",
    });

pub const ERROR_RESPONSE_UNSUPPORTED_GRANT_TYPE: AppleError =
    AppleError::ResponseError(ErrorResponse {
        error_type: ErrorResponseType::UnsupportedGrantType,
        message: "The authenticated client is not authorized to use this grant type.",
    });

pub const ERROR_RESPONSE_INVALID_SCOPE: AppleError = AppleError::ResponseError(ErrorResponse {
    error_type: ErrorResponseType::InvalidScope,
    message: "The requested scope is invalid.",
});

#[cfg(feature = "cloudkit")]
#[derive(Debug, Clone)]
pub struct CloudKitErrorResponse {
    pub server_error_code: CloudKitErrorCode,
    pub reason: String,
    pub uuid: Option<String>,
    pub retry_after: Option<u64>,
}

#[cfg(feature = "cloudkit")]
#[derive(Debug, Clone, PartialEq)]
pub enum CloudKitErrorCode {
    BadRequest,
    AuthenticationFailed,
    AccessDenied,
    NotFound,
    ZoneNotFound,
    Conflict,
    Exists,
    QuotaExceeded,
    AuthenticationRequired,
    Throttled,
    InternalError,
    TryAgainLater,
    Unknown(String),
}

#[cfg(feature = "cloudkit")]
impl fmt::Display for CloudKitErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CloudKitErrorCode::BadRequest => write!(f, "BAD_REQUEST"),
            CloudKitErrorCode::AuthenticationFailed => write!(f, "AUTHENTICATION_FAILED"),
            CloudKitErrorCode::AccessDenied => write!(f, "ACCESS_DENIED"),
            CloudKitErrorCode::NotFound => write!(f, "NOT_FOUND"),
            CloudKitErrorCode::ZoneNotFound => write!(f, "ZONE_NOT_FOUND"),
            CloudKitErrorCode::Conflict => write!(f, "CONFLICT"),
            CloudKitErrorCode::Exists => write!(f, "EXISTS"),
            CloudKitErrorCode::QuotaExceeded => write!(f, "QUOTA_EXCEEDED"),
            CloudKitErrorCode::AuthenticationRequired => write!(f, "AUTHENTICATION_REQUIRED"),
            CloudKitErrorCode::Throttled => write!(f, "THROTTLED"),
            CloudKitErrorCode::InternalError => write!(f, "INTERNAL_ERROR"),
            CloudKitErrorCode::TryAgainLater => write!(f, "TRY_AGAIN_LATER"),
            CloudKitErrorCode::Unknown(code) => write!(f, "{}", code),
        }
    }
}

#[cfg(feature = "cloudkit")]
impl CloudKitErrorCode {
    pub fn parse(s: &str) -> Self {
        match s {
            "BAD_REQUEST" => CloudKitErrorCode::BadRequest,
            "AUTHENTICATION_FAILED" => CloudKitErrorCode::AuthenticationFailed,
            "ACCESS_DENIED" => CloudKitErrorCode::AccessDenied,
            "NOT_FOUND" => CloudKitErrorCode::NotFound,
            "ZONE_NOT_FOUND" => CloudKitErrorCode::ZoneNotFound,
            "CONFLICT" => CloudKitErrorCode::Conflict,
            "EXISTS" => CloudKitErrorCode::Exists,
            "QUOTA_EXCEEDED" => CloudKitErrorCode::QuotaExceeded,
            "AUTHENTICATION_REQUIRED" => CloudKitErrorCode::AuthenticationRequired,
            "THROTTLED" => CloudKitErrorCode::Throttled,
            "INTERNAL_ERROR" => CloudKitErrorCode::InternalError,
            "TRY_AGAIN_LATER" => CloudKitErrorCode::TryAgainLater,
            other => CloudKitErrorCode::Unknown(other.to_string()),
        }
    }
}

#[cfg(feature = "cloudkit")]
impl fmt::Display for CloudKitErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CloudKit error {}: {}",
            self.server_error_code, self.reason
        )
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error_type, self.message)
    }
}

impl fmt::Display for ErrorResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorResponseType::InvalidRequest => write!(f, "invalid_request"),
            ErrorResponseType::InvalidClient => write!(f, "invalid_client"),
            ErrorResponseType::InvalidGrant => write!(f, "invalid_grant"),
            ErrorResponseType::UnauthorizedClient => write!(f, "unauthorized_client"),
            ErrorResponseType::UnsupportedGrantType => write!(f, "unsupported_grant_type"),
            ErrorResponseType::InvalidScope => write!(f, "invalid_scope"),
        }
    }
}

impl fmt::Display for AppleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppleError::IoError(msg) => write!(f, "IO error: {}", msg),
            AppleError::Base64Error(msg) => write!(f, "Base64 error: {}", msg),
            AppleError::PemError(msg) => write!(f, "PEM error: {}", msg),
            AppleError::KeyParseError(msg) => write!(f, "Key parse error: {}", msg),
            AppleError::JwtError(msg) => write!(f, "JWT error: {}", msg),
            AppleError::HttpError(msg) => write!(f, "HTTP error: {}", msg),
            AppleError::JsonError(msg) => write!(f, "JSON error: {}", msg),
            AppleError::TimeError(msg) => write!(f, "Time error: {}", msg),
            AppleError::UnrecognizedError(msg) => write!(f, "Unrecognized error: {}", msg),
            AppleError::ResponseError(err) => write!(f, "{}", err),
            #[cfg(feature = "cloudkit")]
            AppleError::CloudKitError(err) => write!(f, "{}", err),
            #[cfg(feature = "cloudkit")]
            AppleError::SignatureError(msg) => write!(f, "Signature error: {}", msg),
        }
    }
}

impl std::error::Error for AppleError {}
