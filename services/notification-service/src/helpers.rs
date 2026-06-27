use std::collections::{BTreeSet, HashMap};

use axum::extract::Extension;
use axum::http::HeaderMap;
use im_app_context::{AppContext, resolve_app_context};
use im_domain_core::notification::{NotificationStatus, NotificationTask};
use sdkwork_im_contract_notification::NotificationTaskRecord;

use crate::dto::{NotificationRecipient, NotificationRequestDeliveryStatus, RequestNotification};
use crate::error::NotificationError;

pub(crate) const NOTIFICATION_MAX_TITLE_BYTES: usize = 8 * 1024;
pub(crate) const NOTIFICATION_MAX_BODY_BYTES: usize = 64 * 1024;
pub(crate) const NOTIFICATION_MAX_PAYLOAD_BYTES: usize = 256 * 1024;
pub(crate) const NOTIFICATION_MAX_NOTIFICATION_ID_BYTES: usize = 512;
pub(crate) const NOTIFICATION_MAX_SOURCE_EVENT_ID_BYTES: usize = 512;
pub(crate) const NOTIFICATION_MAX_SOURCE_EVENT_TYPE_BYTES: usize = 128;
pub(crate) const NOTIFICATION_MAX_CATEGORY_BYTES: usize = 128;
pub(crate) const NOTIFICATION_MAX_CHANNEL_BYTES: usize = 64;
pub(crate) const NOTIFICATION_MAX_RECIPIENT_ID_BYTES: usize = 256;
pub(crate) const NOTIFICATION_MAX_RECIPIENT_KIND_BYTES: usize = 64;

pub(crate) fn notification_scope_key(tenant_id: &str, notification_id: &str) -> String {
    scope_key_parts(&[tenant_id, notification_id])
}

pub(crate) fn notification_recipient_scope_key(
    tenant_id: &str,
    recipient_kind: &str,
    recipient_id: &str,
) -> String {
    scope_key_parts(&[tenant_id, recipient_kind, recipient_id])
}

pub(crate) fn scope_key_parts(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|part| format!("{}:{part}", part.len()))
        .collect::<Vec<_>>()
        .join("|")
}

pub(crate) fn record_notification_recipient_scope_key(record: &NotificationTaskRecord) -> String {
    notification_recipient_scope_key(
        record.tenant_id.as_str(),
        record.task.recipient_kind.as_str(),
        record.task.recipient_id.as_str(),
    )
}

pub(crate) fn insert_notification_recipient_index(
    index: &mut HashMap<String, BTreeSet<String>>,
    notification_key: &str,
    record: &NotificationTaskRecord,
) {
    index
        .entry(record_notification_recipient_scope_key(record))
        .or_default()
        .insert(notification_key.to_owned());
}

pub(crate) fn remove_notification_recipient_index(
    index: &mut HashMap<String, BTreeSet<String>>,
    notification_key: &str,
    record: &NotificationTaskRecord,
) {
    let recipient_key = record_notification_recipient_scope_key(record);
    if let Some(task_keys) = index.get_mut(recipient_key.as_str()) {
        task_keys.remove(notification_key);
        if task_keys.is_empty() {
            index.remove(recipient_key.as_str());
        }
    }
}

pub(crate) fn notification_request_key(tenant_id: &str, notification_id: &str) -> String {
    notification_scope_key(tenant_id, notification_id)
}

pub(crate) fn notification_sort_key(task: &NotificationTask) -> (&str, &str) {
    (
        task.dispatched_at
            .as_deref()
            .unwrap_or(task.requested_at.as_str()),
        task.requested_at.as_str(),
    )
}

pub(crate) fn delivery_status_from_notification_status(
    status: &NotificationStatus,
) -> NotificationRequestDeliveryStatus {
    match status {
        NotificationStatus::Requested => NotificationRequestDeliveryStatus::Accepted,
        NotificationStatus::Dispatched => NotificationRequestDeliveryStatus::Replayed,
        NotificationStatus::Failed => NotificationRequestDeliveryStatus::Failed,
    }
}

pub(crate) fn notification_matches_request(task: &NotificationTask, request: &RequestNotification) -> bool {
    task.notification_id == request.notification_id.as_str()
        && task.source_event_id == request.source_event_id.as_str()
        && task.source_event_type == request.source_event_type.as_str()
        && task.category == request.category.as_str()
        && task.channel == request.channel.as_str()
        && task.recipient_id == request.recipient_id.as_str()
        && task.recipient_kind == request.recipient_kind
        && task.title.as_ref() == request.title.as_ref()
        && task.body.as_ref() == request.body.as_ref()
        && task.payload.as_ref() == request.payload.as_ref()
}

pub(crate) fn ensure_notification_request_access(
    auth: &AppContext,
    recipient_id: &str,
    recipient_kind: &str,
) -> Result<(), NotificationError> {
    if (recipient_id == auth.actor_id && recipient_kind == auth.actor_kind.as_str())
        || auth.has_permission("notification.write")
    {
        return Ok(());
    }

    Err(NotificationError::forbidden(
        "permission_denied",
        "missing required permission to request notifications for other recipients: notification.write",
    ))
}

pub(crate) fn resolve_request_app_context(
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
) -> Result<AppContext, NotificationError> {
    match auth {
        Some(Extension(auth)) => Ok(auth),
        None => resolve_app_context(headers).map_err(NotificationError::from),
    }
}

pub(crate) fn notification_visible_to_actor(task: &NotificationTask, auth: &AppContext) -> bool {
    task.recipient_id == auth.actor_id && task.recipient_kind == auth.actor_kind
}

pub(crate) fn fanout_notification_id(notification_id_seed: &str, recipient: &NotificationRecipient) -> String {
    format!(
        "ntf_{}_{}_{}",
        notification_id_seed, recipient.recipient_kind, recipient.recipient_id
    )
}

pub(crate) fn automation_notification_id(actor_kind: &str, execution_id: &str) -> String {
    format!("ntf_automation_{actor_kind}_{execution_id}")
}

pub(crate) fn automation_notification_source_event_id(actor_kind: &str, execution_id: &str) -> String {
    format!("evt_{actor_kind}_{execution_id}_automation_execution_completed")
}

pub(crate) fn validate_payload_size(
    field: &'static str,
    payload: &str,
    max_bytes: usize,
) -> Result<(), NotificationError> {
    let payload_len = payload.len();
    if payload_len > max_bytes {
        return Err(NotificationError::payload_too_large(
            field,
            max_bytes,
            payload_len,
        ));
    }
    Ok(())
}

pub(crate) fn validate_notification_request_payload_size(
    request: &RequestNotification,
) -> Result<(), NotificationError> {
    validate_payload_size(
        "notificationId",
        request.notification_id.as_str(),
        NOTIFICATION_MAX_NOTIFICATION_ID_BYTES,
    )?;
    validate_payload_size(
        "sourceEventId",
        request.source_event_id.as_str(),
        NOTIFICATION_MAX_SOURCE_EVENT_ID_BYTES,
    )?;
    validate_payload_size(
        "sourceEventType",
        request.source_event_type.as_str(),
        NOTIFICATION_MAX_SOURCE_EVENT_TYPE_BYTES,
    )?;
    validate_payload_size(
        "category",
        request.category.as_str(),
        NOTIFICATION_MAX_CATEGORY_BYTES,
    )?;
    validate_payload_size(
        "channel",
        request.channel.as_str(),
        NOTIFICATION_MAX_CHANNEL_BYTES,
    )?;
    validate_payload_size(
        "recipientId",
        request.recipient_id.as_str(),
        NOTIFICATION_MAX_RECIPIENT_ID_BYTES,
    )?;
    validate_payload_size(
        "recipientKind",
        request.recipient_kind.as_str(),
        NOTIFICATION_MAX_RECIPIENT_KIND_BYTES,
    )?;
    if let Some(title) = request.title.as_deref() {
        validate_payload_size("title", title, NOTIFICATION_MAX_TITLE_BYTES)?;
    }
    if let Some(body) = request.body.as_deref() {
        validate_payload_size("body", body, NOTIFICATION_MAX_BODY_BYTES)?;
    }
    if let Some(payload) = request.payload.as_deref() {
        validate_payload_size("payload", payload, NOTIFICATION_MAX_PAYLOAD_BYTES)?;
    }
    Ok(())
}
