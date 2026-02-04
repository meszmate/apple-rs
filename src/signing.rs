use crate::error::AppleError;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use p256::ecdsa::{Signature, SigningKey, signature::Signer};
use std::sync::Arc;

/// A shared ECDSA P-256 key pair used for both Apple Sign-In (JWT signing)
/// and CloudKit (request signing).
#[derive(Clone)]
pub struct AppleKeyPair {
    key_id: String,
    signing_key: SigningKey,
}

impl AppleKeyPair {
    /// Load a key pair from a `.p8` file on disk.
    pub fn from_file(key_id: &str, path: &str) -> Result<Arc<Self>, AppleError> {
        let bytes = std::fs::read(path).map_err(|e| AppleError::IoError(e.to_string()))?;
        Self::from_pem_bytes(key_id, &bytes)
    }

    /// Load a key pair from a base64-encoded PEM string.
    pub fn from_base64(key_id: &str, b64: &str) -> Result<Arc<Self>, AppleError> {
        let bytes = STANDARD
            .decode(b64)
            .map_err(|e| AppleError::Base64Error(e.to_string()))?;
        Self::from_pem_bytes(key_id, &bytes)
    }

    /// Load a key pair from raw PEM bytes.
    pub fn from_pem_bytes(key_id: &str, bytes: &[u8]) -> Result<Arc<Self>, AppleError> {
        let pem = pem::parse(bytes).map_err(|e| AppleError::PemError(e.to_string()))?;
        let signing_key = SigningKey::from_slice(pem.contents())
            .map_err(|e| AppleError::KeyParseError(e.to_string()))?;
        Ok(Arc::new(AppleKeyPair {
            key_id: key_id.to_string(),
            signing_key,
        }))
    }

    /// Returns the key ID.
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    /// Returns a reference to the underlying signing key.
    pub fn signing_key(&self) -> &SigningKey {
        &self.signing_key
    }

    /// Sign arbitrary bytes and return the DER-encoded signature.
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let sig: Signature = self.signing_key.sign(message);
        sig.to_der().as_bytes().to_vec()
    }
}
