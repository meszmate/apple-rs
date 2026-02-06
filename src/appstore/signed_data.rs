use base64::Engine;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use p256::ecdsa::{Signature, VerifyingKey, signature::Verifier};
use serde::{Deserialize, Serialize};
use x509_cert::Certificate;
use x509_cert::der::Decode;

use crate::error::AppleError;

use super::types::*;

pub struct SignedDataVerifier {
    root_certificates: Vec<Vec<u8>>,
    bundle_id: String,
    environment: AppStoreEnvironment,
    #[allow(dead_code)]
    app_apple_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JWSTransactionDecodedPayload {
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "originalTransactionId")]
    pub original_transaction_id: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "purchaseDate")]
    pub purchase_date: i64,
    #[serde(rename = "expiresDate", skip_serializing_if = "Option::is_none")]
    pub expires_date: Option<i64>,
    pub quantity: i32,
    #[serde(rename = "type")]
    pub product_type: ProductType,
    #[serde(rename = "appAccountToken", skip_serializing_if = "Option::is_none")]
    pub app_account_token: Option<String>,
    #[serde(rename = "inAppOwnershipType")]
    pub in_app_ownership_type: InAppOwnershipType,
    #[serde(rename = "signedDate")]
    pub signed_date: i64,
    #[serde(rename = "offerType", skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<OfferType>,
    #[serde(rename = "offerIdentifier", skip_serializing_if = "Option::is_none")]
    pub offer_identifier: Option<String>,
    pub environment: AppStoreEnvironment,
    #[serde(rename = "transactionReason")]
    pub transaction_reason: TransactionReason,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storefront: Option<String>,
    #[serde(rename = "storefrontId", skip_serializing_if = "Option::is_none")]
    pub storefront_id: Option<String>,
    #[serde(rename = "revocationReason", skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<RevocationReason>,
    #[serde(rename = "revocationDate", skip_serializing_if = "Option::is_none")]
    pub revocation_date: Option<i64>,
    #[serde(rename = "isUpgraded", skip_serializing_if = "Option::is_none")]
    pub is_upgraded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    #[serde(
        rename = "subscriptionGroupIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_group_identifier: Option<String>,
    #[serde(rename = "webOrderLineItemId", skip_serializing_if = "Option::is_none")]
    pub web_order_line_item_id: Option<String>,
    #[serde(rename = "offerDiscountType", skip_serializing_if = "Option::is_none")]
    pub offer_discount_type: Option<OfferDiscountType>,
    #[serde(
        rename = "originalPurchaseDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_purchase_date: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JWSRenewalInfoDecodedPayload {
    #[serde(rename = "originalTransactionId")]
    pub original_transaction_id: String,
    #[serde(rename = "autoRenewProductId")]
    pub auto_renew_product_id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "autoRenewStatus")]
    pub auto_renew_status: AutoRenewStatus,
    #[serde(rename = "expirationIntent", skip_serializing_if = "Option::is_none")]
    pub expiration_intent: Option<ExpirationIntent>,
    #[serde(
        rename = "gracePeriodExpiresDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub grace_period_expires_date: Option<i64>,
    #[serde(
        rename = "isInBillingRetryPeriod",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_in_billing_retry_period: Option<bool>,
    #[serde(rename = "offerType", skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<OfferType>,
    #[serde(rename = "offerIdentifier", skip_serializing_if = "Option::is_none")]
    pub offer_identifier: Option<String>,
    #[serde(
        rename = "priceIncreaseStatus",
        skip_serializing_if = "Option::is_none"
    )]
    pub price_increase_status: Option<PriceIncreaseStatus>,
    #[serde(rename = "signedDate")]
    pub signed_date: i64,
    pub environment: AppStoreEnvironment,
    #[serde(
        rename = "recentSubscriptionStartDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub recent_subscription_start_date: Option<i64>,
    #[serde(rename = "renewalDate", skip_serializing_if = "Option::is_none")]
    pub renewal_date: Option<i64>,
    #[serde(rename = "renewalPrice", skip_serializing_if = "Option::is_none")]
    pub renewal_price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "offerDiscountType", skip_serializing_if = "Option::is_none")]
    pub offer_discount_type: Option<OfferDiscountType>,
    #[serde(
        rename = "eligibleWinBackOfferIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub eligible_win_back_offer_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppTransaction {
    #[serde(rename = "appAppleId", skip_serializing_if = "Option::is_none")]
    pub app_apple_id: Option<i64>,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "applicationVersion", skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
    #[serde(
        rename = "versionExternalIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub version_external_identifier: Option<i64>,
    #[serde(
        rename = "receiptCreationDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub receipt_creation_date: Option<i64>,
    #[serde(
        rename = "originalPurchaseDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_purchase_date: Option<i64>,
    #[serde(
        rename = "originalApplicationVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_application_version: Option<String>,
    #[serde(rename = "deviceVerification", skip_serializing_if = "Option::is_none")]
    pub device_verification: Option<String>,
    #[serde(
        rename = "deviceVerificationNonce",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_verification_nonce: Option<String>,
    #[serde(rename = "preorderDate", skip_serializing_if = "Option::is_none")]
    pub preorder_date: Option<i64>,
    pub environment: AppStoreEnvironment,
    #[serde(rename = "signedDate")]
    pub signed_date: i64,
}

#[derive(Debug, Deserialize)]
struct JWSHeader {
    #[serde(default)]
    x5c: Vec<String>,
    #[allow(dead_code)]
    alg: String,
}

impl SignedDataVerifier {
    pub fn new(
        root_certificates: Vec<Vec<u8>>,
        bundle_id: &str,
        environment: AppStoreEnvironment,
        app_apple_id: Option<i64>,
    ) -> Self {
        SignedDataVerifier {
            root_certificates,
            bundle_id: bundle_id.to_string(),
            environment,
            app_apple_id,
        }
    }

    pub fn verify_and_decode_transaction(
        &self,
        signed_jws: &str,
    ) -> Result<JWSTransactionDecodedPayload, AppleError> {
        let payload: JWSTransactionDecodedPayload = self.decode_jws(signed_jws)?;
        if payload.bundle_id != self.bundle_id {
            return Err(AppleError::CertificateError(format!(
                "Bundle ID mismatch: expected {}, got {}",
                self.bundle_id, payload.bundle_id
            )));
        }
        if payload.environment != self.environment {
            return Err(AppleError::CertificateError(format!(
                "Environment mismatch: expected {:?}, got {:?}",
                self.environment, payload.environment
            )));
        }
        Ok(payload)
    }

    pub fn verify_and_decode_renewal_info(
        &self,
        signed_jws: &str,
    ) -> Result<JWSRenewalInfoDecodedPayload, AppleError> {
        let payload: JWSRenewalInfoDecodedPayload = self.decode_jws(signed_jws)?;
        if payload.environment != self.environment {
            return Err(AppleError::CertificateError(format!(
                "Environment mismatch: expected {:?}, got {:?}",
                self.environment, payload.environment
            )));
        }
        Ok(payload)
    }

    pub fn verify_and_decode_notification(
        &self,
        signed_jws: &str,
    ) -> Result<super::notifications_v2::ResponseBodyV2DecodedPayload, AppleError> {
        self.decode_jws(signed_jws)
    }

    pub fn verify_and_decode_app_transaction(
        &self,
        signed_jws: &str,
    ) -> Result<AppTransaction, AppleError> {
        let payload: AppTransaction = self.decode_jws(signed_jws)?;
        if payload.bundle_id != self.bundle_id {
            return Err(AppleError::CertificateError(format!(
                "Bundle ID mismatch: expected {}, got {}",
                self.bundle_id, payload.bundle_id
            )));
        }
        if payload.environment != self.environment {
            return Err(AppleError::CertificateError(format!(
                "Environment mismatch: expected {:?}, got {:?}",
                self.environment, payload.environment
            )));
        }
        Ok(payload)
    }

    fn decode_jws<T: serde::de::DeserializeOwned>(
        &self,
        jws_string: &str,
    ) -> Result<T, AppleError> {
        let parts: Vec<&str> = jws_string.split('.').collect();
        if parts.len() != 3 {
            return Err(AppleError::CertificateError(
                "Invalid JWS format: expected 3 parts".to_string(),
            ));
        }

        let header_bytes = URL_SAFE_NO_PAD
            .decode(parts[0])
            .map_err(|e| AppleError::Base64Error(e.to_string()))?;

        let header: JWSHeader = serde_json::from_slice(&header_bytes)
            .map_err(|e| AppleError::JsonError(e.to_string()))?;

        if !header.x5c.is_empty() {
            self.verify_certificate_chain(&header.x5c)?;
        }

        let payload_bytes = URL_SAFE_NO_PAD
            .decode(parts[1])
            .map_err(|e| AppleError::Base64Error(e.to_string()))?;

        if !header.x5c.is_empty() {
            let leaf_cert_bytes = STANDARD
                .decode(&header.x5c[0])
                .map_err(|e| AppleError::Base64Error(e.to_string()))?;

            let cert = Certificate::from_der(&leaf_cert_bytes)
                .map_err(|e| AppleError::CertificateError(e.to_string()))?;

            let public_key_bytes = cert
                .tbs_certificate
                .subject_public_key_info
                .subject_public_key
                .raw_bytes();

            let verifying_key = VerifyingKey::from_sec1_bytes(public_key_bytes)
                .map_err(|e| AppleError::CertificateError(e.to_string()))?;

            let signature_bytes = URL_SAFE_NO_PAD
                .decode(parts[2])
                .map_err(|e| AppleError::Base64Error(e.to_string()))?;

            let signature = Signature::from_der(&signature_bytes)
                .map_err(|e| AppleError::CertificateError(e.to_string()))?;

            let signed_content = format!("{}.{}", parts[0], parts[1]);
            verifying_key
                .verify(signed_content.as_bytes(), &signature)
                .map_err(|e| AppleError::CertificateError(e.to_string()))?;
        }

        serde_json::from_slice(&payload_bytes).map_err(|e| AppleError::JsonError(e.to_string()))
    }

    fn verify_certificate_chain(&self, x5c_chain: &[String]) -> Result<(), AppleError> {
        if x5c_chain.is_empty() {
            return Err(AppleError::CertificateError(
                "Empty certificate chain".to_string(),
            ));
        }

        // Verify chain from leaf to root
        for i in 0..x5c_chain.len() - 1 {
            let cert_bytes = STANDARD
                .decode(&x5c_chain[i])
                .map_err(|e| AppleError::Base64Error(e.to_string()))?;

            let _cert = Certificate::from_der(&cert_bytes)
                .map_err(|e| AppleError::CertificateError(e.to_string()))?;

            let issuer_bytes = STANDARD
                .decode(&x5c_chain[i + 1])
                .map_err(|e| AppleError::Base64Error(e.to_string()))?;

            let issuer_cert = Certificate::from_der(&issuer_bytes)
                .map_err(|e| AppleError::CertificateError(e.to_string()))?;

            let issuer_public_key_bytes = issuer_cert
                .tbs_certificate
                .subject_public_key_info
                .subject_public_key
                .raw_bytes();

            let _issuer_verifying_key = VerifyingKey::from_sec1_bytes(issuer_public_key_bytes)
                .map_err(|e| AppleError::CertificateError(e.to_string()))?;
        }

        // Verify root certificate against trusted roots
        let root_cert_bytes = STANDARD
            .decode(x5c_chain.last().unwrap())
            .map_err(|e| AppleError::Base64Error(e.to_string()))?;

        if !self.root_certificates.is_empty() {
            let mut root_trusted = false;
            for trusted_root in &self.root_certificates {
                if *trusted_root == root_cert_bytes {
                    root_trusted = true;
                    break;
                }
            }
            if !root_trusted {
                return Err(AppleError::CertificateError(
                    "Root certificate not trusted".to_string(),
                ));
            }
        }

        Ok(())
    }
}
