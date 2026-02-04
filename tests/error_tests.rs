use apple::error::*;

#[test]
fn test_error_response_type_display() {
    assert_eq!(ErrorResponseType::InvalidRequest.to_string(), "invalid_request");
    assert_eq!(ErrorResponseType::InvalidClient.to_string(), "invalid_client");
    assert_eq!(ErrorResponseType::InvalidGrant.to_string(), "invalid_grant");
    assert_eq!(ErrorResponseType::UnauthorizedClient.to_string(), "unauthorized_client");
    assert_eq!(ErrorResponseType::UnsupportedGrantType.to_string(), "unsupported_grant_type");
    assert_eq!(ErrorResponseType::InvalidScope.to_string(), "invalid_scope");
}

#[test]
fn test_apple_error_display_variants() {
    assert_eq!(AppleError::IoError("disk".into()).to_string(), "IO error: disk");
    assert_eq!(AppleError::Base64Error("bad".into()).to_string(), "Base64 error: bad");
    assert_eq!(AppleError::PemError("pem".into()).to_string(), "PEM error: pem");
    assert_eq!(AppleError::KeyParseError("key".into()).to_string(), "Key parse error: key");
    assert_eq!(AppleError::JwtError("jwt".into()).to_string(), "JWT error: jwt");
    assert_eq!(AppleError::HttpError("http".into()).to_string(), "HTTP error: http");
    assert_eq!(AppleError::JsonError("json".into()).to_string(), "JSON error: json");
    assert_eq!(AppleError::TimeError("time".into()).to_string(), "Time error: time");
    assert_eq!(AppleError::UnrecognizedError("unk".into()).to_string(), "Unrecognized error: unk");
}

#[test]
fn test_error_response_display() {
    let err = ErrorResponse {
        error_type: ErrorResponseType::InvalidClient,
        message: "bad client",
    };
    let display = err.to_string();
    assert!(display.contains("invalid_client"));
    assert!(display.contains("bad client"));
}

#[test]
fn test_error_constants() {
    match &ERROR_RESPONSE_INVALID_REQUEST {
        AppleError::ResponseError(e) => {
            assert_eq!(e.error_type, ErrorResponseType::InvalidRequest);
            assert!(!e.message.is_empty());
        }
        _ => panic!("Expected ResponseError"),
    }

    match &ERROR_RESPONSE_INVALID_CLIENT {
        AppleError::ResponseError(e) => assert_eq!(e.error_type, ErrorResponseType::InvalidClient),
        _ => panic!("Expected ResponseError"),
    }

    match &ERROR_RESPONSE_INVALID_GRANT {
        AppleError::ResponseError(e) => assert_eq!(e.error_type, ErrorResponseType::InvalidGrant),
        _ => panic!("Expected ResponseError"),
    }

    match &ERROR_RESPONSE_UNAUTHORIZED_CLIENT {
        AppleError::ResponseError(e) => assert_eq!(e.error_type, ErrorResponseType::UnauthorizedClient),
        _ => panic!("Expected ResponseError"),
    }

    match &ERROR_RESPONSE_UNSUPPORTED_GRANT_TYPE {
        AppleError::ResponseError(e) => assert_eq!(e.error_type, ErrorResponseType::UnsupportedGrantType),
        _ => panic!("Expected ResponseError"),
    }

    match &ERROR_RESPONSE_INVALID_SCOPE {
        AppleError::ResponseError(e) => assert_eq!(e.error_type, ErrorResponseType::InvalidScope),
        _ => panic!("Expected ResponseError"),
    }
}

#[test]
fn test_apple_error_is_error_trait() {
    let err: Box<dyn std::error::Error> = Box::new(AppleError::IoError("test".into()));
    assert!(err.to_string().contains("IO error"));
}

#[test]
fn test_apple_error_clone() {
    let err = AppleError::HttpError("timeout".into());
    let err2 = err.clone();
    assert_eq!(err.to_string(), err2.to_string());
}

#[cfg(feature = "cloudkit")]
mod cloudkit_error_tests {
    use apple::error::*;

    #[test]
    fn test_cloudkit_error_code_parse_known() {
        assert_eq!(CloudKitErrorCode::parse("BAD_REQUEST"), CloudKitErrorCode::BadRequest);
        assert_eq!(CloudKitErrorCode::parse("AUTHENTICATION_FAILED"), CloudKitErrorCode::AuthenticationFailed);
        assert_eq!(CloudKitErrorCode::parse("ACCESS_DENIED"), CloudKitErrorCode::AccessDenied);
        assert_eq!(CloudKitErrorCode::parse("NOT_FOUND"), CloudKitErrorCode::NotFound);
        assert_eq!(CloudKitErrorCode::parse("ZONE_NOT_FOUND"), CloudKitErrorCode::ZoneNotFound);
        assert_eq!(CloudKitErrorCode::parse("CONFLICT"), CloudKitErrorCode::Conflict);
        assert_eq!(CloudKitErrorCode::parse("EXISTS"), CloudKitErrorCode::Exists);
        assert_eq!(CloudKitErrorCode::parse("QUOTA_EXCEEDED"), CloudKitErrorCode::QuotaExceeded);
        assert_eq!(CloudKitErrorCode::parse("AUTHENTICATION_REQUIRED"), CloudKitErrorCode::AuthenticationRequired);
        assert_eq!(CloudKitErrorCode::parse("THROTTLED"), CloudKitErrorCode::Throttled);
        assert_eq!(CloudKitErrorCode::parse("INTERNAL_ERROR"), CloudKitErrorCode::InternalError);
        assert_eq!(CloudKitErrorCode::parse("TRY_AGAIN_LATER"), CloudKitErrorCode::TryAgainLater);
    }

    #[test]
    fn test_cloudkit_error_code_parse_unknown() {
        assert_eq!(
            CloudKitErrorCode::parse("SOMETHING_NEW"),
            CloudKitErrorCode::Unknown("SOMETHING_NEW".to_string())
        );
    }

    #[test]
    fn test_cloudkit_error_code_display() {
        assert_eq!(CloudKitErrorCode::BadRequest.to_string(), "BAD_REQUEST");
        assert_eq!(CloudKitErrorCode::NotFound.to_string(), "NOT_FOUND");
        assert_eq!(CloudKitErrorCode::Throttled.to_string(), "THROTTLED");
        assert_eq!(
            CloudKitErrorCode::Unknown("CUSTOM".into()).to_string(),
            "CUSTOM"
        );
    }

    #[test]
    fn test_cloudkit_error_response_display() {
        let resp = CloudKitErrorResponse {
            server_error_code: CloudKitErrorCode::AccessDenied,
            reason: "not allowed".into(),
            uuid: Some("uuid-123".into()),
            retry_after: None,
        };
        let s = resp.to_string();
        assert!(s.contains("ACCESS_DENIED"));
        assert!(s.contains("not allowed"));
    }

    #[test]
    fn test_apple_error_cloudkit_variant_display() {
        let err = AppleError::CloudKitError(CloudKitErrorResponse {
            server_error_code: CloudKitErrorCode::Throttled,
            reason: "rate limited".into(),
            uuid: None,
            retry_after: Some(30),
        });
        let s = err.to_string();
        assert!(s.contains("THROTTLED"));
        assert!(s.contains("rate limited"));
    }

    #[test]
    fn test_apple_error_signature_variant_display() {
        let err = AppleError::SignatureError("bad sig".into());
        assert_eq!(err.to_string(), "Signature error: bad sig");
    }
}
