use apple::signing::AppleKeyPair;
use p256::ecdsa::{Signature, VerifyingKey, signature::Verifier};

/// Generate a test P-256 PEM key for use in tests.
fn test_pem_bytes() -> Vec<u8> {
    use p256::ecdsa::SigningKey;

    let sk = SigningKey::from_slice(&[
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ])
    .unwrap();

    let raw_bytes = sk.to_bytes();
    let pem_obj = pem::Pem::new("EC PRIVATE KEY", raw_bytes.as_slice());
    pem::encode(&pem_obj).into_bytes()
}

#[test]
fn test_from_pem_bytes() {
    let pem_bytes = test_pem_bytes();
    let kp = AppleKeyPair::from_pem_bytes("test-key-id", &pem_bytes).unwrap();
    assert_eq!(kp.key_id(), "test-key-id");
}

#[test]
fn test_from_base64() {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;

    let pem_bytes = test_pem_bytes();
    let b64 = STANDARD.encode(&pem_bytes);
    let kp = AppleKeyPair::from_base64("b64-key", &b64).unwrap();
    assert_eq!(kp.key_id(), "b64-key");
}

#[test]
fn test_from_file() {
    let pem_bytes = test_pem_bytes();
    let dir = std::env::temp_dir().join("apple_rs_test_key.p8");
    std::fs::write(&dir, &pem_bytes).unwrap();

    let kp = AppleKeyPair::from_file("file-key", dir.to_str().unwrap()).unwrap();
    assert_eq!(kp.key_id(), "file-key");

    std::fs::remove_file(&dir).ok();
}

#[test]
fn test_from_file_not_found() {
    let result = AppleKeyPair::from_file("key", "/nonexistent/path.p8");
    assert!(result.is_err());
}

#[test]
fn test_from_base64_invalid() {
    let result = AppleKeyPair::from_base64("key", "!!!not-base64!!!");
    assert!(result.is_err());
}

#[test]
fn test_from_pem_bytes_invalid() {
    let result = AppleKeyPair::from_pem_bytes("key", b"not pem data");
    assert!(result.is_err());
}

#[test]
fn test_sign_produces_verifiable_signature() {
    let pem_bytes = test_pem_bytes();
    let kp = AppleKeyPair::from_pem_bytes("sign-key", &pem_bytes).unwrap();

    let message = b"hello cloudkit";
    let sig_der = kp.sign(message);
    assert!(!sig_der.is_empty());

    // Verify the signature with the corresponding public key
    let verifying_key = VerifyingKey::from(kp.signing_key());
    let sig = Signature::from_der(&sig_der).unwrap();
    verifying_key.verify(message, &sig).unwrap();
}

#[test]
fn test_sign_different_messages_different_signatures() {
    let pem_bytes = test_pem_bytes();
    let kp = AppleKeyPair::from_pem_bytes("key", &pem_bytes).unwrap();

    let sig1 = kp.sign(b"message one");
    let sig2 = kp.sign(b"message two");

    // Signatures should differ (with overwhelming probability)
    assert_ne!(sig1, sig2);
}

#[test]
fn test_key_pair_clone() {
    let pem_bytes = test_pem_bytes();
    let kp = AppleKeyPair::from_pem_bytes("clone-key", &pem_bytes).unwrap();
    let kp2 = kp.clone();
    assert_eq!(kp.key_id(), kp2.key_id());
}

#[test]
fn test_key_pair_arc_shared() {
    let pem_bytes = test_pem_bytes();
    let kp = AppleKeyPair::from_pem_bytes("shared-key", &pem_bytes).unwrap();

    // Arc can be shared
    let kp2 = kp.clone();
    assert_eq!(kp.key_id(), kp2.key_id());
    assert!(std::sync::Arc::ptr_eq(&kp, &kp2));
}
