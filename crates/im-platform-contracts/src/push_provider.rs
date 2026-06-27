//! Push notification provider contract.
//!
//! Defines the trait that push notification adapters (APNs, FCM, etc.)
//! must implement. Follows the same provider-plugin pattern as RTC adapters.

use crate::provider::ProviderHealthSnapshot;
use sdkwork_im_contract_core::ContractError;

/// A push notification message to be delivered to a device.
#[derive(Clone, Debug)]
pub struct PushMessage {
    /// Platform-specific device token (APNs device token or FCM registration token).
    pub device_token: String,
    /// Notification title (shown in notification tray).
    pub title: Option<String>,
    /// Notification body text.
    pub body: Option<String>,
    /// Optional JSON payload for deep-linking or custom data.
    pub payload: Option<String>,
    /// Notification category / channel identifier.
    pub category: String,
    /// Whether this is a content-available (silent) push.
    pub content_available: bool,
}

/// Result of a push notification delivery attempt.
#[derive(Clone, Debug)]
pub struct PushDeliveryResult {
    /// Whether the notification was accepted by the push service.
    pub accepted: bool,
    /// Provider-specific message ID or error token.
    pub provider_message_id: Option<String>,
    /// Error description if the delivery failed.
    pub error: Option<String>,
    /// Whether the device token should be considered invalid (unregistered).
    pub token_invalid: bool,
}

/// Push notification provider trait.
///
/// Each adapter (APNs, FCM, Huawei, etc.) implements this trait.
/// The notification service dispatches to the configured provider
/// based on the notification channel and device platform.
pub trait PushProvider: Send + Sync {
    /// Send a push notification to a single device.
    fn send(&self, message: &PushMessage) -> Result<PushDeliveryResult, ContractError>;

    /// Send push notifications to multiple devices (batch).
    fn send_batch(
        &self,
        messages: &[PushMessage],
    ) -> Result<Vec<PushDeliveryResult>, ContractError> {
        messages.iter().map(|m| self.send(m)).collect()
    }

    /// Return a health snapshot for operator diagnostics.
    fn provider_health(&self) -> ProviderHealthSnapshot;

    /// The provider plugin identifier (e.g. "push-apns", "push-fcm").
    fn plugin_id(&self) -> &'static str;

    /// Whether this provider supports the given notification category.
    fn supports_category(&self, _category: &str) -> bool {
        true
    }
}
