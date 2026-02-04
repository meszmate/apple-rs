#[cfg(feature = "cloudkit")]
mod cloudkit_client_tests {
    use apple::cloudkit::client::{CloudKitClient, CloudKitConfig};
    use apple::cloudkit::types::{Environment};
    use apple::signing::AppleKeyPair;

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

    fn make_client(env: Environment) -> CloudKitClient {
        let kp = AppleKeyPair::from_pem_bytes("test-key", &test_pem_bytes()).unwrap();
        CloudKitClient::new(CloudKitConfig {
            container: "iCloud.com.test.app".to_string(),
            environment: env,
            key_pair: kp,
        }).unwrap()
    }

    #[test]
    fn test_client_creation_development() {
        let client = make_client(Environment::Development);
        assert_eq!(client.config().container, "iCloud.com.test.app");
        assert_eq!(client.config().environment, Environment::Development);
        assert_eq!(client.config().key_pair.key_id(), "test-key");
    }

    #[test]
    fn test_client_creation_production() {
        let client = make_client(Environment::Production);
        assert_eq!(client.config().container, "iCloud.com.test.app");
        assert_eq!(client.config().environment, Environment::Production);
    }

    #[test]
    fn test_client_different_containers() {
        let kp = AppleKeyPair::from_pem_bytes("key", &test_pem_bytes()).unwrap();

        let client1 = CloudKitClient::new(CloudKitConfig {
            container: "iCloud.com.app.one".to_string(),
            environment: Environment::Development,
            key_pair: kp.clone(),
        }).unwrap();

        let client2 = CloudKitClient::new(CloudKitConfig {
            container: "iCloud.com.app.two".to_string(),
            environment: Environment::Production,
            key_pair: kp,
        }).unwrap();

        assert_ne!(client1.config().container, client2.config().container);
        assert_ne!(client1.config().environment, client2.config().environment);
    }

    #[test]
    fn test_client_shares_key_pair() {
        let kp = AppleKeyPair::from_pem_bytes("shared", &test_pem_bytes()).unwrap();
        let kp_clone = kp.clone();

        let client = CloudKitClient::new(CloudKitConfig {
            container: "iCloud.com.test".to_string(),
            environment: Environment::Development,
            key_pair: kp,
        }).unwrap();

        assert!(std::sync::Arc::ptr_eq(&client.config().key_pair, &kp_clone));
    }
}
