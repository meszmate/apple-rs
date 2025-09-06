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

pub const ERROR_RESPONSE_UNAUTHORIZED_CLIENT: AppleError = AppleError::ResponseError(ErrorResponse {
    error_type: ErrorResponseType::UnauthorizedClient,
    message: "The client is not authorized to use this authorization grant type.",
});

pub const ERROR_RESPONSE_UNSUPPORTED_GRANT_TYPE: AppleError = AppleError::ResponseError(ErrorResponse {
    error_type: ErrorResponseType::UnsupportedGrantType,
    message: "The authenticated client is not authorized to use this grant type.",
});

pub const ERROR_RESPONSE_INVALID_SCOPE: AppleError = AppleError::ResponseError(ErrorResponse {
    error_type: ErrorResponseType::InvalidScope,
    message: "The requested scope is invalid.",
});

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
        }
    }
}

impl std::error::Error for AppleError {}