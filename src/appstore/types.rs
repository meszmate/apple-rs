#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AppStoreEnvironment {
    Sandbox,
    Production,
    Xcode,
    LocalTesting,
}

impl fmt::Display for AppStoreEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppStoreEnvironment::Sandbox => write!(f, "Sandbox"),
            AppStoreEnvironment::Production => write!(f, "Production"),
            AppStoreEnvironment::Xcode => write!(f, "Xcode"),
            AppStoreEnvironment::LocalTesting => write!(f, "LocalTesting"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SubscriptionStatus {
    Active,
    Expired,
    BillingRetryPeriod,
    BillingGracePeriod,
    Revoked,
}

impl TryFrom<i32> for SubscriptionStatus {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SubscriptionStatus::Active),
            2 => Ok(SubscriptionStatus::Expired),
            3 => Ok(SubscriptionStatus::BillingRetryPeriod),
            4 => Ok(SubscriptionStatus::BillingGracePeriod),
            5 => Ok(SubscriptionStatus::Revoked),
            _ => Err(format!("Unknown SubscriptionStatus: {}", value)),
        }
    }
}

impl From<SubscriptionStatus> for i32 {
    fn from(value: SubscriptionStatus) -> Self {
        match value {
            SubscriptionStatus::Active => 1,
            SubscriptionStatus::Expired => 2,
            SubscriptionStatus::BillingRetryPeriod => 3,
            SubscriptionStatus::BillingGracePeriod => 4,
            SubscriptionStatus::Revoked => 5,
        }
    }
}

impl Serialize for SubscriptionStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(i32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for SubscriptionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        SubscriptionStatus::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AutoRenewStatus {
    Off,
    On,
}

impl TryFrom<i32> for AutoRenewStatus {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AutoRenewStatus::Off),
            1 => Ok(AutoRenewStatus::On),
            _ => Err(format!("Unknown AutoRenewStatus: {}", value)),
        }
    }
}

impl From<AutoRenewStatus> for i32 {
    fn from(value: AutoRenewStatus) -> Self {
        match value {
            AutoRenewStatus::Off => 0,
            AutoRenewStatus::On => 1,
        }
    }
}

impl Serialize for AutoRenewStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(i32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for AutoRenewStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        AutoRenewStatus::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpirationIntent {
    Cancelled,
    BillingError,
    PriceIncreaseNotConsented,
    ProductUnavailable,
    Unknown,
}

impl TryFrom<i32> for ExpirationIntent {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ExpirationIntent::Cancelled),
            2 => Ok(ExpirationIntent::BillingError),
            3 => Ok(ExpirationIntent::PriceIncreaseNotConsented),
            4 => Ok(ExpirationIntent::ProductUnavailable),
            5 => Ok(ExpirationIntent::Unknown),
            _ => Err(format!("Unknown ExpirationIntent: {}", value)),
        }
    }
}

impl From<ExpirationIntent> for i32 {
    fn from(value: ExpirationIntent) -> Self {
        match value {
            ExpirationIntent::Cancelled => 1,
            ExpirationIntent::BillingError => 2,
            ExpirationIntent::PriceIncreaseNotConsented => 3,
            ExpirationIntent::ProductUnavailable => 4,
            ExpirationIntent::Unknown => 5,
        }
    }
}

impl Serialize for ExpirationIntent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(i32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for ExpirationIntent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        ExpirationIntent::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProductType {
    #[serde(rename = "Auto-Renewable Subscription")]
    AutoRenewableSubscription,
    #[serde(rename = "Non-Consumable")]
    NonConsumable,
    #[serde(rename = "Consumable")]
    Consumable,
    #[serde(rename = "Non-Renewing Subscription")]
    NonRenewingSubscription,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OfferType {
    IntroductoryOffer,
    PromotionalOffer,
    OfferCode,
    WinBackOffer,
}

impl TryFrom<i32> for OfferType {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(OfferType::IntroductoryOffer),
            2 => Ok(OfferType::PromotionalOffer),
            3 => Ok(OfferType::OfferCode),
            4 => Ok(OfferType::WinBackOffer),
            _ => Err(format!("Unknown OfferType: {}", value)),
        }
    }
}

impl From<OfferType> for i32 {
    fn from(value: OfferType) -> Self {
        match value {
            OfferType::IntroductoryOffer => 1,
            OfferType::PromotionalOffer => 2,
            OfferType::OfferCode => 3,
            OfferType::WinBackOffer => 4,
        }
    }
}

impl Serialize for OfferType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(i32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for OfferType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        OfferType::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OfferDiscountType {
    FREE_TRIAL,
    PAY_AS_YOU_GO,
    PAY_UP_FRONT,
    ONE_TIME,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InAppOwnershipType {
    FAMILY_SHARED,
    PURCHASED,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionReason {
    PURCHASE,
    RENEWAL,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RevocationReason {
    RefundedDueToIssue,
    RefundedForOtherReason,
}

impl TryFrom<i32> for RevocationReason {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RevocationReason::RefundedForOtherReason),
            1 => Ok(RevocationReason::RefundedDueToIssue),
            _ => Err(format!("Unknown RevocationReason: {}", value)),
        }
    }
}

impl From<RevocationReason> for i32 {
    fn from(value: RevocationReason) -> Self {
        match value {
            RevocationReason::RefundedForOtherReason => 0,
            RevocationReason::RefundedDueToIssue => 1,
        }
    }
}

impl Serialize for RevocationReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(i32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for RevocationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        RevocationReason::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    Undeclared,
    Apple,
    NonApple,
}

impl TryFrom<i32> for Platform {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Platform::Undeclared),
            1 => Ok(Platform::Apple),
            2 => Ok(Platform::NonApple),
            _ => Err(format!("Unknown Platform: {}", value)),
        }
    }
}

impl From<Platform> for i32 {
    fn from(value: Platform) -> Self {
        match value {
            Platform::Undeclared => 0,
            Platform::Apple => 1,
            Platform::NonApple => 2,
        }
    }
}

impl Serialize for Platform {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(i32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for Platform {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        Platform::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PurchasePlatform {
    #[serde(rename = "iOS")]
    IOS,
    #[serde(rename = "macOS")]
    MacOS,
    #[serde(rename = "tvOS")]
    TvOS,
    #[serde(rename = "visionOS")]
    VisionOS,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PriceIncreaseStatus {
    NotResponded,
    Consented,
}

impl TryFrom<i32> for PriceIncreaseStatus {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PriceIncreaseStatus::NotResponded),
            1 => Ok(PriceIncreaseStatus::Consented),
            _ => Err(format!("Unknown PriceIncreaseStatus: {}", value)),
        }
    }
}

impl From<PriceIncreaseStatus> for i32 {
    fn from(value: PriceIncreaseStatus) -> Self {
        match value {
            PriceIncreaseStatus::NotResponded => 0,
            PriceIncreaseStatus::Consented => 1,
        }
    }
}

impl Serialize for PriceIncreaseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(i32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for PriceIncreaseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        PriceIncreaseStatus::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderLookupStatus {
    Valid,
    Invalid,
}

impl TryFrom<i32> for OrderLookupStatus {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OrderLookupStatus::Valid),
            1 => Ok(OrderLookupStatus::Invalid),
            _ => Err(format!("Unknown OrderLookupStatus: {}", value)),
        }
    }
}

impl From<OrderLookupStatus> for i32 {
    fn from(value: OrderLookupStatus) -> Self {
        match value {
            OrderLookupStatus::Valid => 0,
            OrderLookupStatus::Invalid => 1,
        }
    }
}

impl Serialize for OrderLookupStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(i32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for OrderLookupStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        OrderLookupStatus::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExtendReasonCode {
    Undeclared,
    CustomerSatisfaction,
    OtherReason,
    ServiceIssueOrOutage,
}

impl TryFrom<i32> for ExtendReasonCode {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ExtendReasonCode::Undeclared),
            1 => Ok(ExtendReasonCode::CustomerSatisfaction),
            2 => Ok(ExtendReasonCode::OtherReason),
            3 => Ok(ExtendReasonCode::ServiceIssueOrOutage),
            _ => Err(format!("Unknown ExtendReasonCode: {}", value)),
        }
    }
}

impl From<ExtendReasonCode> for i32 {
    fn from(value: ExtendReasonCode) -> Self {
        match value {
            ExtendReasonCode::Undeclared => 0,
            ExtendReasonCode::CustomerSatisfaction => 1,
            ExtendReasonCode::OtherReason => 2,
            ExtendReasonCode::ServiceIssueOrOutage => 3,
        }
    }
}

impl Serialize for ExtendReasonCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(i32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for ExtendReasonCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        ExtendReasonCode::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationTypeV2 {
    SUBSCRIBED,
    DID_RENEW,
    DID_CHANGE_RENEWAL_PREF,
    DID_CHANGE_RENEWAL_STATUS,
    DID_FAIL_TO_RENEW,
    EXPIRED,
    GRACE_PERIOD_EXPIRED,
    OFFER_REDEEMED,
    PRICE_INCREASE,
    REFUND,
    REFUND_DECLINED,
    REFUND_REVERSED,
    RENEWAL_EXTENDED,
    RENEWAL_EXTENSION,
    REVOKE,
    TEST,
    CONSUMPTION_REQUEST,
    EXTERNAL_PURCHASE_TOKEN,
    ONE_TIME_CHARGE,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Subtype {
    INITIAL_BUY,
    RESUBSCRIBE,
    DOWNGRADE,
    UPGRADE,
    AUTO_RENEW_ENABLED,
    AUTO_RENEW_DISABLED,
    VOLUNTARY,
    BILLING_RETRY,
    PRICE_INCREASE,
    GRACE_PERIOD,
    PENDING,
    ACCEPTED,
    BILLING_RECOVERY,
    PRODUCT_NOT_FOR_SALE,
    SUMMARY,
    FAILURE,
    UNREPORTED,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationTypeV1 {
    CANCEL,
    DID_CHANGE_RENEWAL_PREF,
    DID_CHANGE_RENEWAL_STATUS,
    DID_FAIL_TO_RENEW,
    DID_RECOVER,
    DID_RENEW,
    INITIAL_BUY,
    INTERACTIVE_RENEWAL,
    PRICE_INCREASE_CONSENT,
    REFUND,
    RENEWAL,
    REVOKE,
    CONSUMPTION_REQUEST,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SendAttemptResult {
    SUCCESS,
    TIMED_OUT,
    TLS_ISSUE,
    CIRCULAR_REDIRECT,
    NO_RESPONSE,
    SOCKET_ISSUE,
    UNSUPPORTED_CHARSET,
    INVALID_RESPONSE,
    PREMATURE_CLOSE,
    UNSUCCESSFUL_HTTP_RESPONSE_CODE,
    OTHER,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConsumptionRequestReason {
    UNINTENDED_PURCHASE,
    FULFILLMENT_ISSUE,
    UNSATISFIED,
    SUSPICIOUS,
    OTHER,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeliveryStatus {
    DeliveredAndWorking,
    DidNotDeliverDueToQualityIssue,
    DeliveredWrongItem,
    DidNotDeliverDueToServerOutage,
    DidNotDeliverDueToInGameCurrencyChange,
    DidNotDeliverForOtherReason,
}

impl TryFrom<i32> for DeliveryStatus {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DeliveryStatus::DeliveredAndWorking),
            1 => Ok(DeliveryStatus::DidNotDeliverDueToQualityIssue),
            2 => Ok(DeliveryStatus::DeliveredWrongItem),
            3 => Ok(DeliveryStatus::DidNotDeliverDueToServerOutage),
            4 => Ok(DeliveryStatus::DidNotDeliverDueToInGameCurrencyChange),
            5 => Ok(DeliveryStatus::DidNotDeliverForOtherReason),
            _ => Err(format!("Unknown DeliveryStatus: {}", value)),
        }
    }
}

impl From<DeliveryStatus> for i32 {
    fn from(value: DeliveryStatus) -> Self {
        match value {
            DeliveryStatus::DeliveredAndWorking => 0,
            DeliveryStatus::DidNotDeliverDueToQualityIssue => 1,
            DeliveryStatus::DeliveredWrongItem => 2,
            DeliveryStatus::DidNotDeliverDueToServerOutage => 3,
            DeliveryStatus::DidNotDeliverDueToInGameCurrencyChange => 4,
            DeliveryStatus::DidNotDeliverForOtherReason => 5,
        }
    }
}

impl Serialize for DeliveryStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(i32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for DeliveryStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        DeliveryStatus::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefundPreference {
    DECLINE,
    GRANT_FULL,
    GRANT_PRORATED,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AccountTenure {
    Undeclared,
    ZeroToThreeDays,
    ThreeToTenDays,
    TenToThirtyDays,
    ThirtyToNinetyDays,
    NinetyToOneEightyDays,
    OneEightyToThreeSixtyFiveDays,
    OverThreeSixtyFiveDays,
}

impl TryFrom<i32> for AccountTenure {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AccountTenure::Undeclared),
            1 => Ok(AccountTenure::ZeroToThreeDays),
            2 => Ok(AccountTenure::ThreeToTenDays),
            3 => Ok(AccountTenure::TenToThirtyDays),
            4 => Ok(AccountTenure::ThirtyToNinetyDays),
            5 => Ok(AccountTenure::NinetyToOneEightyDays),
            6 => Ok(AccountTenure::OneEightyToThreeSixtyFiveDays),
            7 => Ok(AccountTenure::OverThreeSixtyFiveDays),
            _ => Err(format!("Unknown AccountTenure: {}", value)),
        }
    }
}

impl From<AccountTenure> for i32 {
    fn from(value: AccountTenure) -> Self {
        match value {
            AccountTenure::Undeclared => 0,
            AccountTenure::ZeroToThreeDays => 1,
            AccountTenure::ThreeToTenDays => 2,
            AccountTenure::TenToThirtyDays => 3,
            AccountTenure::ThirtyToNinetyDays => 4,
            AccountTenure::NinetyToOneEightyDays => 5,
            AccountTenure::OneEightyToThreeSixtyFiveDays => 6,
            AccountTenure::OverThreeSixtyFiveDays => 7,
        }
    }
}

impl Serialize for AccountTenure {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(i32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for AccountTenure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        AccountTenure::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConsumptionStatus {
    Undeclared,
    NotConsumed,
    PartiallyConsumed,
    FullyConsumed,
}

impl TryFrom<i32> for ConsumptionStatus {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ConsumptionStatus::Undeclared),
            1 => Ok(ConsumptionStatus::NotConsumed),
            2 => Ok(ConsumptionStatus::PartiallyConsumed),
            3 => Ok(ConsumptionStatus::FullyConsumed),
            _ => Err(format!("Unknown ConsumptionStatus: {}", value)),
        }
    }
}

impl From<ConsumptionStatus> for i32 {
    fn from(value: ConsumptionStatus) -> Self {
        match value {
            ConsumptionStatus::Undeclared => 0,
            ConsumptionStatus::NotConsumed => 1,
            ConsumptionStatus::PartiallyConsumed => 2,
            ConsumptionStatus::FullyConsumed => 3,
        }
    }
}

impl Serialize for ConsumptionStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(i32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for ConsumptionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        ConsumptionStatus::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayTime {
    Undeclared,
    ZeroToFiveMinutes,
    FiveToSixtyMinutes,
    OneToSixHours,
    SixToTwentyFourHours,
    OneToFourDays,
    FourToSixteenDays,
    OverSixteenDays,
}

impl TryFrom<i32> for PlayTime {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PlayTime::Undeclared),
            1 => Ok(PlayTime::ZeroToFiveMinutes),
            2 => Ok(PlayTime::FiveToSixtyMinutes),
            3 => Ok(PlayTime::OneToSixHours),
            4 => Ok(PlayTime::SixToTwentyFourHours),
            5 => Ok(PlayTime::OneToFourDays),
            6 => Ok(PlayTime::FourToSixteenDays),
            7 => Ok(PlayTime::OverSixteenDays),
            _ => Err(format!("Unknown PlayTime: {}", value)),
        }
    }
}

impl From<PlayTime> for i32 {
    fn from(value: PlayTime) -> Self {
        match value {
            PlayTime::Undeclared => 0,
            PlayTime::ZeroToFiveMinutes => 1,
            PlayTime::FiveToSixtyMinutes => 2,
            PlayTime::OneToSixHours => 3,
            PlayTime::SixToTwentyFourHours => 4,
            PlayTime::OneToFourDays => 5,
            PlayTime::FourToSixteenDays => 6,
            PlayTime::OverSixteenDays => 7,
        }
    }
}

impl Serialize for PlayTime {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(i32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for PlayTime {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        PlayTime::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LifetimeDollarsPurchased {
    Undeclared,
    Zero,
    OneToFortyNine,
    FiftyToNinetyNine,
    OneHundredToFourNinetyNine,
    FiveHundredToNineNinetyNine,
    OneThousandToOneThousandNineNinetyNine,
    OverTwoThousand,
}

impl TryFrom<i32> for LifetimeDollarsPurchased {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LifetimeDollarsPurchased::Undeclared),
            1 => Ok(LifetimeDollarsPurchased::Zero),
            2 => Ok(LifetimeDollarsPurchased::OneToFortyNine),
            3 => Ok(LifetimeDollarsPurchased::FiftyToNinetyNine),
            4 => Ok(LifetimeDollarsPurchased::OneHundredToFourNinetyNine),
            5 => Ok(LifetimeDollarsPurchased::FiveHundredToNineNinetyNine),
            6 => Ok(LifetimeDollarsPurchased::OneThousandToOneThousandNineNinetyNine),
            7 => Ok(LifetimeDollarsPurchased::OverTwoThousand),
            _ => Err(format!("Unknown LifetimeDollarsPurchased: {}", value)),
        }
    }
}

impl From<LifetimeDollarsPurchased> for i32 {
    fn from(value: LifetimeDollarsPurchased) -> Self {
        match value {
            LifetimeDollarsPurchased::Undeclared => 0,
            LifetimeDollarsPurchased::Zero => 1,
            LifetimeDollarsPurchased::OneToFortyNine => 2,
            LifetimeDollarsPurchased::FiftyToNinetyNine => 3,
            LifetimeDollarsPurchased::OneHundredToFourNinetyNine => 4,
            LifetimeDollarsPurchased::FiveHundredToNineNinetyNine => 5,
            LifetimeDollarsPurchased::OneThousandToOneThousandNineNinetyNine => 6,
            LifetimeDollarsPurchased::OverTwoThousand => 7,
        }
    }
}

impl Serialize for LifetimeDollarsPurchased {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(i32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for LifetimeDollarsPurchased {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        LifetimeDollarsPurchased::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LifetimeDollarsRefunded {
    Undeclared,
    Zero,
    OneToFortyNine,
    FiftyToNinetyNine,
    OneHundredToFourNinetyNine,
    FiveHundredToNineNinetyNine,
    OneThousandToOneThousandNineNinetyNine,
    OverTwoThousand,
}

impl TryFrom<i32> for LifetimeDollarsRefunded {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LifetimeDollarsRefunded::Undeclared),
            1 => Ok(LifetimeDollarsRefunded::Zero),
            2 => Ok(LifetimeDollarsRefunded::OneToFortyNine),
            3 => Ok(LifetimeDollarsRefunded::FiftyToNinetyNine),
            4 => Ok(LifetimeDollarsRefunded::OneHundredToFourNinetyNine),
            5 => Ok(LifetimeDollarsRefunded::FiveHundredToNineNinetyNine),
            6 => Ok(LifetimeDollarsRefunded::OneThousandToOneThousandNineNinetyNine),
            7 => Ok(LifetimeDollarsRefunded::OverTwoThousand),
            _ => Err(format!("Unknown LifetimeDollarsRefunded: {}", value)),
        }
    }
}

impl From<LifetimeDollarsRefunded> for i32 {
    fn from(value: LifetimeDollarsRefunded) -> Self {
        match value {
            LifetimeDollarsRefunded::Undeclared => 0,
            LifetimeDollarsRefunded::Zero => 1,
            LifetimeDollarsRefunded::OneToFortyNine => 2,
            LifetimeDollarsRefunded::FiftyToNinetyNine => 3,
            LifetimeDollarsRefunded::OneHundredToFourNinetyNine => 4,
            LifetimeDollarsRefunded::FiveHundredToNineNinetyNine => 5,
            LifetimeDollarsRefunded::OneThousandToOneThousandNineNinetyNine => 6,
            LifetimeDollarsRefunded::OverTwoThousand => 7,
        }
    }
}

impl Serialize for LifetimeDollarsRefunded {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(i32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for LifetimeDollarsRefunded {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        LifetimeDollarsRefunded::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserStatus {
    Undeclared,
    Active,
    Suspended,
    Terminated,
    LimitedAccess,
}

impl TryFrom<i32> for UserStatus {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(UserStatus::Undeclared),
            1 => Ok(UserStatus::Active),
            2 => Ok(UserStatus::Suspended),
            3 => Ok(UserStatus::Terminated),
            4 => Ok(UserStatus::LimitedAccess),
            _ => Err(format!("Unknown UserStatus: {}", value)),
        }
    }
}

impl From<UserStatus> for i32 {
    fn from(value: UserStatus) -> Self {
        match value {
            UserStatus::Undeclared => 0,
            UserStatus::Active => 1,
            UserStatus::Suspended => 2,
            UserStatus::Terminated => 3,
            UserStatus::LimitedAccess => 4,
        }
    }
}

impl Serialize for UserStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(i32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for UserStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        UserStatus::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionHistoryProductType {
    AUTO_RENEWABLE,
    NON_RENEWABLE,
    CONSUMABLE,
    NON_CONSUMABLE,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Order {
    ASCENDING,
    DESCENDING,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImageState {
    PENDING_REVIEW,
    APPROVED,
    REJECTED,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageState {
    PENDING_REVIEW,
    APPROVED,
    REJECTED,
}
