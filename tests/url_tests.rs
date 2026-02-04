#[cfg(feature = "auth")]
mod auth_url_tests {
    use apple::url::*;

    #[test]
    fn test_response_mode_display() {
        assert_eq!(ResponseMode::Query.to_string(), "query");
        assert_eq!(ResponseMode::Fragment.to_string(), "fragment");
        assert_eq!(ResponseMode::FormPost.to_string(), "form_post");
    }

    #[test]
    fn test_response_type_display() {
        assert_eq!(ResponseType::Code.to_string(), "code");
        assert_eq!(ResponseType::CodeId.to_string(), "code id_token");
    }

    #[test]
    fn test_response_mode_default() {
        let mode: ResponseMode = Default::default();
        assert_eq!(mode.to_string(), "form_post");
    }

    #[test]
    fn test_response_type_default() {
        let rt: ResponseType = Default::default();
        assert_eq!(rt.to_string(), "code id_token");
    }

    #[test]
    fn test_authorize_url_defaults() {
        let cfg = AuthorizeURLConfig {
            client_id: "com.example.app".to_string(),
            redirect_uri: "https://example.com/callback".to_string(),
            state: None,
            scope: None,
            nonce: None,
            response_mode: None,
            response_type: None,
        };

        let url = authorize_url(cfg);
        assert!(url.starts_with("https://appleid.apple.com/auth/authorize?"));
        assert!(url.contains("client_id=com.example.app"));
        assert!(url.contains("redirect_uri=https%3A%2F%2Fexample.com%2Fcallback"));
        assert!(url.contains("response_type=code+id_token"));
        assert!(url.contains("response_mode=form_post"));
    }

    #[test]
    fn test_authorize_url_with_all_params() {
        let cfg = AuthorizeURLConfig {
            client_id: "com.test.app".to_string(),
            redirect_uri: "https://test.com/cb".to_string(),
            state: Some("mystate".to_string()),
            scope: Some(vec!["email".to_string(), "name".to_string()]),
            nonce: Some("mynonce".to_string()),
            response_mode: Some(ResponseMode::Query),
            response_type: Some(ResponseType::Code),
        };

        let url = authorize_url(cfg);
        assert!(url.contains("client_id=com.test.app"));
        assert!(url.contains("state=mystate"));
        assert!(url.contains("nonce=mynonce"));
        assert!(url.contains("scope=email+name"));
        assert!(url.contains("response_mode=query"));
        assert!(url.contains("response_type=code"));
    }

    #[test]
    fn test_authorize_url_with_fragment_mode() {
        let cfg = AuthorizeURLConfig {
            client_id: "com.test.app".to_string(),
            redirect_uri: "https://test.com/cb".to_string(),
            state: None,
            scope: None,
            nonce: None,
            response_mode: Some(ResponseMode::Fragment),
            response_type: Some(ResponseType::CodeId),
        };

        let url = authorize_url(cfg);
        assert!(url.contains("response_mode=fragment"));
    }

    #[test]
    fn test_authorize_url_serde_roundtrip() {
        let mode = ResponseMode::FormPost;
        let json = serde_json::to_string(&mode).unwrap();
        let deserialized: ResponseMode = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.to_string(), "form_post");

        let rt = ResponseType::Code;
        let json = serde_json::to_string(&rt).unwrap();
        let deserialized: ResponseType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.to_string(), "code");
    }
}
