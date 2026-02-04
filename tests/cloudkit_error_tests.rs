#[cfg(feature = "cloudkit")]
mod cloudkit_error_tests {
    use apple::error::{AppleError, CloudKitErrorCode};

    // The parse_cloudkit_error function is pub(crate), so we test it
    // indirectly through the error types and through JSON parsing.

    #[test]
    fn test_cloudkit_error_code_roundtrip() {
        let codes = vec![
            ("BAD_REQUEST", CloudKitErrorCode::BadRequest),
            ("AUTHENTICATION_FAILED", CloudKitErrorCode::AuthenticationFailed),
            ("ACCESS_DENIED", CloudKitErrorCode::AccessDenied),
            ("NOT_FOUND", CloudKitErrorCode::NotFound),
            ("ZONE_NOT_FOUND", CloudKitErrorCode::ZoneNotFound),
            ("CONFLICT", CloudKitErrorCode::Conflict),
            ("EXISTS", CloudKitErrorCode::Exists),
            ("QUOTA_EXCEEDED", CloudKitErrorCode::QuotaExceeded),
            ("AUTHENTICATION_REQUIRED", CloudKitErrorCode::AuthenticationRequired),
            ("THROTTLED", CloudKitErrorCode::Throttled),
            ("INTERNAL_ERROR", CloudKitErrorCode::InternalError),
            ("TRY_AGAIN_LATER", CloudKitErrorCode::TryAgainLater),
        ];

        for (str_code, expected) in codes {
            let parsed = CloudKitErrorCode::parse(str_code);
            assert_eq!(parsed, expected);
            // Display should round-trip
            assert_eq!(parsed.to_string(), str_code);
        }
    }

    #[test]
    fn test_cloudkit_error_code_unknown_roundtrip() {
        let parsed = CloudKitErrorCode::parse("BRAND_NEW_ERROR");
        assert_eq!(parsed, CloudKitErrorCode::Unknown("BRAND_NEW_ERROR".into()));
        assert_eq!(parsed.to_string(), "BRAND_NEW_ERROR");
    }

    #[test]
    fn test_cloudkit_error_response_fields() {
        let err = apple::error::CloudKitErrorResponse {
            server_error_code: CloudKitErrorCode::Throttled,
            reason: "too many requests".into(),
            uuid: Some("abc-123".into()),
            retry_after: Some(60),
        };

        assert_eq!(err.server_error_code, CloudKitErrorCode::Throttled);
        assert_eq!(err.reason, "too many requests");
        assert_eq!(err.uuid.as_deref(), Some("abc-123"));
        assert_eq!(err.retry_after, Some(60));
    }

    #[test]
    fn test_cloudkit_error_in_apple_error() {
        let err = AppleError::CloudKitError(apple::error::CloudKitErrorResponse {
            server_error_code: CloudKitErrorCode::NotFound,
            reason: "record not found".into(),
            uuid: None,
            retry_after: None,
        });

        let display = err.to_string();
        assert!(display.contains("NOT_FOUND"));
        assert!(display.contains("record not found"));
    }

    #[test]
    fn test_cloudkit_error_clone() {
        let err = apple::error::CloudKitErrorResponse {
            server_error_code: CloudKitErrorCode::Conflict,
            reason: "conflict".into(),
            uuid: Some("id".into()),
            retry_after: Some(5),
        };

        let cloned = err.clone();
        assert_eq!(cloned.server_error_code, CloudKitErrorCode::Conflict);
        assert_eq!(cloned.reason, "conflict");
        assert_eq!(cloned.uuid.as_deref(), Some("id"));
        assert_eq!(cloned.retry_after, Some(5));
    }
}
