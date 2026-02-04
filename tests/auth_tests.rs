#[cfg(feature = "auth")]
mod auth_tests {
    use apple::auth::AppleAuthImpl;
    use apple::signing::AppleKeyPair;
    use std::sync::Arc;

    fn test_pem_bytes() -> Vec<u8> {
        use p256::ecdsa::SigningKey;

        let sk = SigningKey::from_slice(&[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
            0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
            0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
        ]).unwrap();

        let raw_bytes = sk.to_bytes();
        let pem_obj = pem::Pem::new("EC PRIVATE KEY", raw_bytes.as_slice());
        pem::encode(&pem_obj).into_bytes()
    }

    #[test]
    fn test_new_from_file() {
        let pem_bytes = test_pem_bytes();
        let path = std::env::temp_dir().join("apple_rs_auth_test.p8");
        std::fs::write(&path, &pem_bytes).unwrap();

        let auth = AppleAuthImpl::new("app-id", "team-id", "key-id", path.to_str().unwrap());
        assert!(auth.is_ok());

        let auth = auth.unwrap();
        assert_eq!(auth.key_pair().key_id(), "key-id");

        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_new_b64() {
        use base64::engine::general_purpose::STANDARD;
        use base64::Engine;

        let pem_bytes = test_pem_bytes();
        let b64 = STANDARD.encode(&pem_bytes);

        let auth = AppleAuthImpl::new_b64("app-id", "team-id", "key-id", &b64);
        assert!(auth.is_ok());
    }

    #[test]
    fn test_new_from_file_not_found() {
        let result = AppleAuthImpl::new("app-id", "team-id", "key-id", "/nonexistent.p8");
        assert!(result.is_err());
    }

    #[test]
    fn test_from_key_pair() {
        let pem_bytes = test_pem_bytes();
        let kp = AppleKeyPair::from_pem_bytes("shared-key", &pem_bytes).unwrap();

        let auth = AppleAuthImpl::from_key_pair("app-id", "team-id", kp.clone());
        assert!(auth.is_ok());

        let auth = auth.unwrap();
        assert!(Arc::ptr_eq(auth.key_pair(), &kp));
    }

    #[test]
    fn test_key_pair_accessor() {
        let pem_bytes = test_pem_bytes();
        let kp = AppleKeyPair::from_pem_bytes("test-key", &pem_bytes).unwrap();

        let auth = AppleAuthImpl::from_key_pair("app-id", "team-id", kp).unwrap();
        assert_eq!(auth.key_pair().key_id(), "test-key");
    }
}
