//! Shared-channel linked-member sync runtime control: lease ownership, stale reclaim,
//! pending/dead-letter inventory, delivery proofs, and operator repair surfaces.

use chrono::{DateTime, Duration};
#[cfg(test)]
use chrono::{FixedOffset, TimeZone};
use im_time::{rfc3339_le, utc_now_rfc3339_millis};
use serde::{Deserialize, Serialize};

use crate::runtime::{
    PendingSharedChannelSyncRequest, SocialControlState, StoredSharedChannelSyncDeliveryProof,
};
use crate::{
    SharedChannelLinkedMemberSyncRequest, SharedChannelSyncDeliveryProofStatus,
    SHARED_CHANNEL_SYNC_DEAD_LETTER_FAILURE_THRESHOLD,
};

pub const SHARED_CHANNEL_SYNC_CLAIM_LEASE_SECONDS_DEFAULT: i64 = 300;
const CLAIM_LEASE_SECONDS_ENV: &str = "SDKWORK_IM_SHARED_CHANNEL_SYNC_CLAIM_LEASE_SECONDS";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialSharedChannelSyncLeaseStatus {
    Unclaimed,
    Active,
    Stale,
    Untracked,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncInventoryItem {
    pub request_key: String,
    pub request: SharedChannelLinkedMemberSyncRequest,
    pub failure_count: u32,
    pub last_error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_actor_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_actor_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_expires_at: Option<String>,
    pub lease_status: SocialSharedChannelSyncLeaseStatus,
    pub takeover_eligible: bool,
    pub legacy_takeover_required: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncPendingInventoryResponse {
    pub status: &'static str,
    pub items: Vec<SocialSharedChannelSyncInventoryItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncDeadLetterInventoryResponse {
    pub status: &'static str,
    pub items: Vec<SocialSharedChannelSyncInventoryItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncDeliveredInventoryResponse {
    pub status: &'static str,
    pub items: Vec<SocialSharedChannelSyncDeliveredItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncDeliveryProof {
    pub delivered_at: String,
    pub status: SharedChannelSyncDeliveryProofStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

impl From<StoredSharedChannelSyncDeliveryProof> for SocialSharedChannelSyncDeliveryProof {
    fn from(proof: StoredSharedChannelSyncDeliveryProof) -> Self {
        Self {
            delivered_at: proof.delivered_at,
            status: proof.status,
            proof_version: proof.proof_version,
            target: proof.target,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncDeliveredItem {
    pub request_key: String,
    pub delivered_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<SocialSharedChannelSyncDeliveryProof>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncDeliveryStateInventoryResponse {
    pub status: &'static str,
    pub pending: usize,
    pub dead_letter: usize,
    pub delivered: usize,
    pub recent_deliveries: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncPendingStaleReclaimResponse {
    pub status: &'static str,
    pub pending_before: usize,
    pub reclaimed: usize,
    pub pending_after: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncRepairResponse {
    pub status: &'static str,
    pub reclaimed: usize,
    pub attempted: usize,
    pub repaired: usize,
    pub failed: usize,
    pub pending_after: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncDeadLetterRequeueResponse {
    pub status: &'static str,
    pub pending_before: usize,
    pub dead_letter_before: usize,
    pub requeued: usize,
    pub pending_after: usize,
    pub dead_letter_after: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncTargetedMutationResponse {
    pub status: &'static str,
    pub affected: usize,
    pub pending_after: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncRepublishResponse {
    pub status: &'static str,
    pub attempted: usize,
    pub repaired: usize,
    pub failed: usize,
    pub trigger_unconfigured: bool,
}

pub fn shared_channel_sync_claim_lease_seconds() -> i64 {
    std::env::var(CLAIM_LEASE_SECONDS_ENV)
        .ok()
        .and_then(|value| value.trim().parse().ok())
        .filter(|seconds| *seconds > 0)
        .unwrap_or(SHARED_CHANNEL_SYNC_CLAIM_LEASE_SECONDS_DEFAULT)
}

pub fn shared_channel_sync_request_key(request: &SharedChannelLinkedMemberSyncRequest) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}",
        request.tenant_id,
        request.conversation_id,
        request.shared_channel_policy_id,
        request.external_connection_id,
        request.local_actor_id,
        request.local_actor_kind,
        request.external_member_id
    )
}

fn lease_expires_at_from_now(now: &str, lease_seconds: i64) -> Option<String> {
    let parsed = DateTime::parse_from_rfc3339(now).ok()?;
    let expires = parsed + Duration::seconds(lease_seconds);
    Some(
        expires
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
            .replace("+00:00", "Z"),
    )
}

pub(crate) fn pending_lease_status(
    owner_actor_id: Option<&str>,
    lease_expires_at: Option<&str>,
    now: &str,
) -> SocialSharedChannelSyncLeaseStatus {
    let Some(owner) = owner_actor_id.filter(|value| !value.trim().is_empty()) else {
        return SocialSharedChannelSyncLeaseStatus::Unclaimed;
    };
    if owner.trim().is_empty() {
        return SocialSharedChannelSyncLeaseStatus::Unclaimed;
    }
    match lease_expires_at.filter(|value| !value.trim().is_empty()) {
        None => SocialSharedChannelSyncLeaseStatus::Untracked,
        Some(expires_at) if rfc3339_le(expires_at, now) => {
            SocialSharedChannelSyncLeaseStatus::Stale
        }
        Some(_) => SocialSharedChannelSyncLeaseStatus::Active,
    }
}

impl PendingSharedChannelSyncRequest {
    pub fn lease_status(&self, now: &str) -> SocialSharedChannelSyncLeaseStatus {
        pending_lease_status(
            self.owner_actor_id.as_deref(),
            self.lease_expires_at.as_deref(),
            now,
        )
    }

    pub fn takeover_eligible_for(
        &self,
        actor_id: &str,
        actor_kind: &str,
        now: &str,
    ) -> bool {
        let status = self.lease_status(now);
        let owner_id = self.owner_actor_id.as_deref().unwrap_or_default();
        let owner_kind = self.owner_actor_kind.as_deref().unwrap_or_default();
        if owner_id.is_empty() {
            return false;
        }
        if owner_id == actor_id && owner_kind == actor_kind {
            return false;
        }
        matches!(
            status,
            SocialSharedChannelSyncLeaseStatus::Stale
                | SocialSharedChannelSyncLeaseStatus::Untracked
        )
    }

    pub fn legacy_takeover_required(&self, now: &str) -> bool {
        self.lease_status(now) == SocialSharedChannelSyncLeaseStatus::Untracked
            && self
                .owner_actor_id
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty())
    }

    pub fn clear_claim_ownership(&mut self) {
        self.owner_actor_id = None;
        self.owner_actor_kind = None;
        self.claimed_at = None;
        self.lease_expires_at = None;
    }

    pub fn renew_claim_lease(
        &mut self,
        actor_id: &str,
        actor_kind: &str,
        now: &str,
        lease_seconds: i64,
    ) {
        self.owner_actor_id = Some(actor_id.to_owned());
        self.owner_actor_kind = Some(actor_kind.to_owned());
        self.claimed_at = Some(now.to_owned());
        self.lease_expires_at = lease_expires_at_from_now(now, lease_seconds);
    }
}

fn inventory_item_from_pending(
    request_key: String,
    pending: &PendingSharedChannelSyncRequest,
    viewer_actor_id: &str,
    viewer_actor_kind: &str,
    now: &str,
) -> SocialSharedChannelSyncInventoryItem {
    let lease_status = pending.lease_status(now);
    SocialSharedChannelSyncInventoryItem {
        request_key,
        request: pending.request.clone(),
        failure_count: pending.failure_count,
        last_error: pending.last_error.clone(),
        last_failed_at: pending.last_failed_at.clone(),
        owner_actor_id: pending.owner_actor_id.clone(),
        owner_actor_kind: pending.owner_actor_kind.clone(),
        claimed_at: pending.claimed_at.clone(),
        lease_expires_at: pending.lease_expires_at.clone(),
        takeover_eligible: pending.takeover_eligible_for(viewer_actor_id, viewer_actor_kind, now),
        legacy_takeover_required: pending.legacy_takeover_required(now),
        lease_status,
    }
}

impl SocialControlState {
    pub fn reclaim_stale_pending_shared_channel_sync_claims(&mut self, now: &str) -> usize {
        let mut reclaimed = 0usize;
        let keys: Vec<String> = self.pending_shared_channel_sync_requests.keys().cloned().collect();
        for request_key in keys {
            let Some(pending) = self.pending_shared_channel_sync_requests.get(&request_key) else {
                continue;
            };
            let should_reclaim = matches!(
                pending.lease_status(now),
                SocialSharedChannelSyncLeaseStatus::Stale
                    | SocialSharedChannelSyncLeaseStatus::Untracked
            );
            if !should_reclaim {
                continue;
            }
            let mut updated = pending.clone();
            updated.clear_claim_ownership();
            self.upsert_pending_shared_channel_sync_request(request_key, updated);
            reclaimed += 1;
        }
        reclaimed
    }

    pub fn mark_shared_channel_sync_delivered(
        &mut self,
        request: &SharedChannelLinkedMemberSyncRequest,
        status: SharedChannelSyncDeliveryProofStatus,
        delivered_at: &str,
        target: Option<String>,
    ) {
        let request_key = shared_channel_sync_request_key(request);
        if self
            .delivered_shared_channel_sync_requests
            .contains_key(request_key.as_str())
        {
            crate::shared_channel_sync_metrics::shared_channel_sync_metrics()
                .record_delivery_deduplicated();
            return;
        }
        crate::shared_channel_sync_metrics::shared_channel_sync_metrics()
            .record_delivery_proof_recorded();
        self.pending_shared_channel_sync_requests.remove(&request_key);
        self.dead_letter_shared_channel_sync_requests.remove(&request_key);
        self.delivered_shared_channel_sync_requests
            .insert(request_key.clone(), delivered_at.to_owned());
        self.delivered_shared_channel_sync_delivery_proofs.insert(
            request_key.clone(),
            StoredSharedChannelSyncDeliveryProof {
                delivered_at: delivered_at.to_owned(),
                status,
                proof_version: Some("v1".to_owned()),
                target,
            },
        );
        self.recent_shared_channel_sync_deliveries
            .insert(request_key, delivered_at.to_owned());
    }

    pub fn is_shared_channel_sync_delivered(
        &self,
        request: &SharedChannelLinkedMemberSyncRequest,
    ) -> bool {
        let request_key = shared_channel_sync_request_key(request);
        self.delivered_shared_channel_sync_requests
            .contains_key(request_key.as_str())
    }

    pub fn pending_shared_channel_sync_inventory(
        &self,
        viewer_actor_id: &str,
        viewer_actor_kind: &str,
        now: &str,
    ) -> SocialSharedChannelSyncPendingInventoryResponse {
        let items = self
            .pending_shared_channel_sync_requests
            .iter()
            .map(|(request_key, pending)| {
                inventory_item_from_pending(
                    request_key.clone(),
                    pending,
                    viewer_actor_id,
                    viewer_actor_kind,
                    now,
                )
            })
            .collect();
        SocialSharedChannelSyncPendingInventoryResponse {
            status: "snapshot",
            items,
        }
    }

    pub fn dead_letter_shared_channel_sync_inventory(
        &self,
        viewer_actor_id: &str,
        viewer_actor_kind: &str,
        now: &str,
    ) -> SocialSharedChannelSyncDeadLetterInventoryResponse {
        let items = self
            .dead_letter_shared_channel_sync_requests
            .iter()
            .map(|(request_key, pending)| {
                inventory_item_from_pending(
                    request_key.clone(),
                    pending,
                    viewer_actor_id,
                    viewer_actor_kind,
                    now,
                )
            })
            .collect();
        SocialSharedChannelSyncDeadLetterInventoryResponse {
            status: "snapshot",
            items,
        }
    }

    pub fn delivered_shared_channel_sync_inventory(
        &self,
    ) -> SocialSharedChannelSyncDeliveredInventoryResponse {
        let items = self
            .delivered_shared_channel_sync_requests
            .iter()
            .map(|(request_key, delivered_at)| SocialSharedChannelSyncDeliveredItem {
                request_key: request_key.clone(),
                delivered_at: delivered_at.clone(),
                proof: self
                    .delivered_shared_channel_sync_delivery_proofs
                    .get(request_key)
                    .cloned()
                    .map(Into::into),
            })
            .collect();
        SocialSharedChannelSyncDeliveredInventoryResponse {
            status: "snapshot",
            items,
        }
    }

    pub fn delivery_state_shared_channel_sync_inventory(
        &self,
    ) -> SocialSharedChannelSyncDeliveryStateInventoryResponse {
        SocialSharedChannelSyncDeliveryStateInventoryResponse {
            status: "snapshot",
            pending: self.pending_shared_channel_sync_requests.len(),
            dead_letter: self.dead_letter_shared_channel_sync_requests.len(),
            delivered: self.delivered_shared_channel_sync_requests.len(),
            recent_deliveries: self.recent_shared_channel_sync_deliveries.len(),
        }
    }

    pub fn requeue_dead_letter_shared_channel_sync_requests(&mut self) -> usize {
        let keys: Vec<String> = self
            .dead_letter_shared_channel_sync_requests
            .keys()
            .cloned()
            .collect();
        let mut requeued = 0usize;
        for request_key in keys {
            let Some(pending) = self
                .dead_letter_shared_channel_sync_requests
                .remove(&request_key)
            else {
                continue;
            };
            self.upsert_pending_shared_channel_sync_request(request_key, pending);
            requeued += 1;
        }
        requeued
    }

    pub fn requeue_dead_letter_shared_channel_sync_targeted(
        &mut self,
        request_keys: &[String],
    ) -> usize {
        let mut requeued = 0usize;
        for request_key in request_keys {
            let Some(pending) = self
                .dead_letter_shared_channel_sync_requests
                .remove(request_key.as_str())
            else {
                continue;
            };
            self.upsert_pending_shared_channel_sync_request(request_key.clone(), pending);
            requeued += 1;
        }
        requeued
    }

    pub fn claim_pending_shared_channel_sync_targeted(
        &mut self,
        request_keys: &[String],
        actor_id: &str,
        actor_kind: &str,
        now: &str,
    ) -> Result<usize, SharedChannelSyncOwnerConflict> {
        let lease_seconds = shared_channel_sync_claim_lease_seconds();
        let mut affected = 0usize;
        for request_key in request_keys {
            let Some(pending) = self
                .pending_shared_channel_sync_requests
                .get(request_key.as_str())
                .cloned()
            else {
                continue;
            };
            let status = pending.lease_status(now);
            if matches!(status, SocialSharedChannelSyncLeaseStatus::Active) {
                return Err(SharedChannelSyncOwnerConflict::from_pending(
                    request_key.clone(),
                    &pending,
                    actor_id,
                    actor_kind,
                    now,
                ));
            }
            let mut updated = pending;
            if matches!(
                status,
                SocialSharedChannelSyncLeaseStatus::Stale
                    | SocialSharedChannelSyncLeaseStatus::Untracked
            ) && updated.owner_actor_id.as_deref() == Some(actor_id)
                && updated.owner_actor_kind.as_deref() == Some(actor_kind)
            {
                updated.renew_claim_lease(actor_id, actor_kind, now, lease_seconds);
            } else {
                updated.renew_claim_lease(actor_id, actor_kind, now, lease_seconds);
            }
            self.upsert_pending_shared_channel_sync_request(request_key.clone(), updated);
            affected += 1;
        }
        Ok(affected)
    }

    pub fn release_pending_shared_channel_sync_targeted(
        &mut self,
        request_keys: &[String],
        actor_id: &str,
        actor_kind: &str,
        now: &str,
    ) -> Result<usize, SharedChannelSyncOwnerConflict> {
        let mut affected = 0usize;
        for request_key in request_keys {
            let Some(pending) = self
                .pending_shared_channel_sync_requests
                .get(request_key.as_str())
                .cloned()
            else {
                continue;
            };
            if pending.lease_status(now) == SocialSharedChannelSyncLeaseStatus::Active
                && (pending.owner_actor_id.as_deref() != Some(actor_id)
                    || pending.owner_actor_kind.as_deref() != Some(actor_kind))
            {
                return Err(SharedChannelSyncOwnerConflict::from_pending(
                    request_key.clone(),
                    &pending,
                    actor_id,
                    actor_kind,
                    now,
                ));
            }
            let mut updated = pending;
            updated.clear_claim_ownership();
            self.upsert_pending_shared_channel_sync_request(request_key.clone(), updated);
            affected += 1;
        }
        Ok(affected)
    }

    pub fn takeover_pending_shared_channel_sync_targeted(
        &mut self,
        request_keys: &[String],
        actor_id: &str,
        actor_kind: &str,
        now: &str,
        legacy_override: bool,
    ) -> Result<usize, SharedChannelSyncOwnerConflict> {
        let lease_seconds = shared_channel_sync_claim_lease_seconds();
        let mut affected = 0usize;
        for request_key in request_keys {
            let Some(pending) = self
                .pending_shared_channel_sync_requests
                .get(request_key.as_str())
                .cloned()
            else {
                continue;
            };
            let status = pending.lease_status(now);
            if status == SocialSharedChannelSyncLeaseStatus::Active {
                return Err(SharedChannelSyncOwnerConflict::from_pending(
                    request_key.clone(),
                    &pending,
                    actor_id,
                    actor_kind,
                    now,
                ));
            }
            if status == SocialSharedChannelSyncLeaseStatus::Untracked && !legacy_override {
                return Err(SharedChannelSyncOwnerConflict::from_pending(
                    request_key.clone(),
                    &pending,
                    actor_id,
                    actor_kind,
                    now,
                ));
            }
            if !pending.takeover_eligible_for(actor_id, actor_kind, now)
                && status != SocialSharedChannelSyncLeaseStatus::Unclaimed
            {
                return Err(SharedChannelSyncOwnerConflict::from_pending(
                    request_key.clone(),
                    &pending,
                    actor_id,
                    actor_kind,
                    now,
                ));
            }
            let mut updated = pending;
            updated.renew_claim_lease(actor_id, actor_kind, now, lease_seconds);
            self.upsert_pending_shared_channel_sync_request(request_key.clone(), updated);
            affected += 1;
        }
        Ok(affected)
    }

    pub fn ensure_pending_shared_channel_sync_request(
        &mut self,
        request: SharedChannelLinkedMemberSyncRequest,
        error: &str,
        now: &str,
    ) {
        let request_key = shared_channel_sync_request_key(&request);
        if self.is_shared_channel_sync_delivered(&request) {
            return;
        }
        let existing_failure_count = self
            .pending_shared_channel_sync_requests
            .get(request_key.as_str())
            .map(|pending| pending.failure_count)
            .unwrap_or(0);
        let pending = PendingSharedChannelSyncRequest {
            request,
            failure_count: existing_failure_count,
            last_error: error.to_owned(),
            last_failed_at: Some(now.to_owned()),
            owner_actor_id: None,
            owner_actor_kind: None,
            claimed_at: None,
            lease_expires_at: None,
        };
        self.upsert_pending_shared_channel_sync_request(request_key, pending);
    }

    pub fn record_failed_shared_channel_sync_requests_with_owner_preservation(
        &mut self,
        requests: &[SharedChannelLinkedMemberSyncRequest],
        error: &str,
        now: &str,
        preserve_owner: bool,
    ) -> bool {
        let mut changed = false;
        for request in requests {
            let request_key = shared_channel_sync_request_key(request);
            let existing = self
                .pending_shared_channel_sync_requests
                .get(request_key.as_str())
                .cloned();
            let existing_failure_count = existing.as_ref().map(|pending| pending.failure_count).unwrap_or(0);
            let mut pending = PendingSharedChannelSyncRequest {
                request: request.clone(),
                failure_count: existing_failure_count + 1,
                last_error: error.to_owned(),
                last_failed_at: Some(now.to_owned()),
                owner_actor_id: None,
                owner_actor_kind: None,
                claimed_at: None,
                lease_expires_at: None,
            };
            if preserve_owner {
                if let Some(existing) = existing {
                    pending.owner_actor_id = existing.owner_actor_id;
                    pending.owner_actor_kind = existing.owner_actor_kind;
                    pending.claimed_at = existing.claimed_at;
                    pending.lease_expires_at = existing.lease_expires_at;
                    if matches!(
                        pending.lease_status(now),
                        SocialSharedChannelSyncLeaseStatus::Stale
                    ) {
                        let lease_seconds = shared_channel_sync_claim_lease_seconds();
                        let owner_id = pending.owner_actor_id.clone();
                        let owner_kind = pending.owner_actor_kind.clone();
                        if let (Some(owner_id), Some(owner_kind)) =
                            (owner_id.as_deref(), owner_kind.as_deref())
                        {
                            pending.renew_claim_lease(owner_id, owner_kind, now, lease_seconds);
                        }
                    }
                }
            } else if let Some(existing) = existing {
                let status = existing.lease_status(now);
                if matches!(
                    status,
                    SocialSharedChannelSyncLeaseStatus::Stale
                        | SocialSharedChannelSyncLeaseStatus::Untracked
                ) {
                    pending.clear_claim_ownership();
                } else if status == SocialSharedChannelSyncLeaseStatus::Active {
                    pending.owner_actor_id = existing.owner_actor_id;
                    pending.owner_actor_kind = existing.owner_actor_kind;
                    pending.claimed_at = existing.claimed_at;
                    pending.lease_expires_at = existing.lease_expires_at;
                }
            }
            if pending.failure_count >= SHARED_CHANNEL_SYNC_DEAD_LETTER_FAILURE_THRESHOLD {
                pending.clear_claim_ownership();
                self.upsert_pending_shared_channel_sync_request(
                    request_key.clone(),
                    pending.clone(),
                );
                self.dead_letter_shared_channel_sync_requests
                    .insert(request_key, pending);
            } else {
                self.upsert_pending_shared_channel_sync_request(request_key, pending);
            }
            changed = true;
        }
        changed
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedChannelSyncOwnerConflict {
    pub code: &'static str,
    pub message: String,
    pub details: serde_json::Value,
}

impl SharedChannelSyncOwnerConflict {
    fn from_pending(
        request_key: String,
        pending: &PendingSharedChannelSyncRequest,
        actor_id: &str,
        actor_kind: &str,
        now: &str,
    ) -> Self {
        let lease_status = pending.lease_status(now);
        Self {
            code: "shared_channel_sync_owner_conflict",
            message: format!(
                "request {request_key} is owned by another operator while lease status is {lease_status:?}"
            ),
            details: serde_json::json!({
                "requestKey": request_key,
                "ownerActorId": pending.owner_actor_id,
                "ownerActorKind": pending.owner_actor_kind,
                "claimedAt": pending.claimed_at,
                "leaseExpiresAt": pending.lease_expires_at,
                "leaseStatus": serde_json::to_value(lease_status).unwrap_or(serde_json::Value::Null),
                "takeoverEligible": pending.takeover_eligible_for(actor_id, actor_kind, now),
                "legacyTakeoverRequired": pending.legacy_takeover_required(now),
            }),
        }
    }
}

#[cfg(test)]
pub fn stale_timestamp(now: &str, offset_seconds: i64) -> String {
    let parsed = DateTime::parse_from_rfc3339(now).unwrap_or_else(|_| {
        FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(2026, 1, 1, 0, 0, 0)
            .unwrap()
    });
    let shifted = parsed - Duration::seconds(offset_seconds);
    shifted
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
        .replace("+00:00", "Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_request() -> SharedChannelLinkedMemberSyncRequest {
        SharedChannelLinkedMemberSyncRequest {
            tenant_id: "100001".into(),
            conversation_id: "conv-1".into(),
            shared_channel_policy_id: "policy-1".into(),
            external_connection_id: "conn-1".into(),
            local_actor_id: "user-1".into(),
            local_actor_kind: "user".into(),
            external_member_id: "ext-1".into(),
        }
    }

    #[test]
    fn test_reclaim_stale_pending_shared_channel_sync_claims_clears_owner_metadata() {
        let now = utc_now_rfc3339_millis();
        let stale_at = stale_timestamp(now.as_str(), 600);
        let mut state = SocialControlState::default();
        let request = sample_request();
        let request_key = shared_channel_sync_request_key(&request);
        state.upsert_pending_shared_channel_sync_request(
            request_key,
            PendingSharedChannelSyncRequest {
                request,
                failure_count: 1,
                last_error: "boom".into(),
                last_failed_at: Some(now.clone()),
                owner_actor_id: Some("operator-1".into()),
                owner_actor_kind: Some("system".into()),
                claimed_at: Some(stale_timestamp(now.as_str(), 900)),
                lease_expires_at: Some(stale_at),
            },
        );

        let reclaimed = state.reclaim_stale_pending_shared_channel_sync_claims(now.as_str());
        assert_eq!(reclaimed, 1);
        let pending = state
            .pending_shared_channel_sync_requests
            .values()
            .next()
            .expect("pending item");
        assert_eq!(
            pending.lease_status(now.as_str()),
            SocialSharedChannelSyncLeaseStatus::Unclaimed
        );
        assert!(pending.owner_actor_id.is_none());
    }

    #[test]
    fn test_mark_shared_channel_sync_delivered_removes_pending_and_records_proof() {
        let now = utc_now_rfc3339_millis();
        let mut state = SocialControlState::default();
        let request = sample_request();
        state.ensure_pending_shared_channel_sync_request(
            request.clone(),
            "pending",
            now.as_str(),
        );
        state.mark_shared_channel_sync_delivered(
            &request,
            SharedChannelSyncDeliveryProofStatus::Applied,
            now.as_str(),
            None,
        );
        assert!(state.pending_shared_channel_sync_requests.is_empty());
        assert_eq!(state.delivered_shared_channel_sync_requests.len(), 1);
        assert_eq!(state.delivered_shared_channel_sync_delivery_proofs.len(), 1);
    }

    #[test]
    fn test_mark_shared_channel_sync_delivered_is_idempotent() {
        let now = utc_now_rfc3339_millis();
        let mut state = SocialControlState::default();
        let request = sample_request();
        state.mark_shared_channel_sync_delivered(
            &request,
            SharedChannelSyncDeliveryProofStatus::Applied,
            now.as_str(),
            None,
        );
        state.ensure_pending_shared_channel_sync_request(
            request.clone(),
            "should not requeue",
            now.as_str(),
        );
        state.mark_shared_channel_sync_delivered(
            &request,
            SharedChannelSyncDeliveryProofStatus::Applied,
            now.as_str(),
            None,
        );
        assert!(state.pending_shared_channel_sync_requests.is_empty());
        assert_eq!(state.delivered_shared_channel_sync_requests.len(), 1);
    }
}

use crate::runtime::SocialRuntime;

impl SocialRuntime {
    pub fn reclaim_stale_pending_shared_channel_sync_claims_persisted(
        &self,
    ) -> Result<SocialSharedChannelSyncPendingStaleReclaimResponse, String> {
        let now = utc_now_rfc3339_millis();
        let mut state = self
            .state
            .write()
            .map_err(|error| format!("social runtime lock poisoned: {error}"))?;
        let pending_before = state.pending_shared_channel_sync_requests.len();
        let reclaimed = state.reclaim_stale_pending_shared_channel_sync_claims(now.as_str());
        let pending_after = state.pending_shared_channel_sync_requests.len();
        drop(state);
        self.persist_state_snapshot("shared-channel sync stale reclaim")?;
        Ok(SocialSharedChannelSyncPendingStaleReclaimResponse {
            status: "reclaimed",
            pending_before,
            reclaimed,
            pending_after,
        })
    }

    pub fn pending_shared_channel_sync_inventory(
        &self,
        viewer_actor_id: &str,
        viewer_actor_kind: &str,
    ) -> SocialSharedChannelSyncPendingInventoryResponse {
        let now = utc_now_rfc3339_millis();
        self.state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .pending_shared_channel_sync_inventory(viewer_actor_id, viewer_actor_kind, now.as_str())
    }

    pub fn dead_letter_shared_channel_sync_inventory(
        &self,
        viewer_actor_id: &str,
        viewer_actor_kind: &str,
    ) -> SocialSharedChannelSyncDeadLetterInventoryResponse {
        let now = utc_now_rfc3339_millis();
        self.state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .dead_letter_shared_channel_sync_inventory(
                viewer_actor_id,
                viewer_actor_kind,
                now.as_str(),
            )
    }

    pub fn delivered_shared_channel_sync_inventory(
        &self,
    ) -> SocialSharedChannelSyncDeliveredInventoryResponse {
        self.state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .delivered_shared_channel_sync_inventory()
    }

    pub fn delivery_state_shared_channel_sync_inventory(
        &self,
    ) -> SocialSharedChannelSyncDeliveryStateInventoryResponse {
        self.state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .delivery_state_shared_channel_sync_inventory()
    }

    pub fn requeue_dead_letter_shared_channel_sync_persisted(
        &self,
        request_keys: Option<&[String]>,
    ) -> Result<SocialSharedChannelSyncDeadLetterRequeueResponse, String> {
        let mut state = self
            .state
            .write()
            .map_err(|error| format!("social runtime lock poisoned: {error}"))?;
        let pending_before = state.pending_shared_channel_sync_requests.len();
        let dead_letter_before = state.dead_letter_shared_channel_sync_requests.len();
        let requeued = match request_keys {
            Some(keys) => state.requeue_dead_letter_shared_channel_sync_targeted(keys),
            None => state.requeue_dead_letter_shared_channel_sync_requests(),
        };
        let response = SocialSharedChannelSyncDeadLetterRequeueResponse {
            status: "requeued",
            pending_before,
            dead_letter_before,
            requeued,
            pending_after: state.pending_shared_channel_sync_requests.len(),
            dead_letter_after: state.dead_letter_shared_channel_sync_requests.len(),
        };
        drop(state);
        self.persist_state_snapshot("shared-channel sync dead-letter requeue")?;
        Ok(response)
    }

    pub fn claim_pending_shared_channel_sync_targeted_persisted(
        &self,
        request_keys: &[String],
        actor_id: &str,
        actor_kind: &str,
    ) -> Result<SocialSharedChannelSyncTargetedMutationResponse, SharedChannelSyncOwnerConflict>
    {
        let now = utc_now_rfc3339_millis();
        let mut state = self
            .state
            .write()
            .map_err(|_| {
                SharedChannelSyncOwnerConflict {
                    code: "social_runtime_lock_poisoned",
                    message: "social runtime lock poisoned".into(),
                    details: serde_json::json!({}),
                }
            })?;
        let affected = state.claim_pending_shared_channel_sync_targeted(
            request_keys,
            actor_id,
            actor_kind,
            now.as_str(),
        )?;
        let response = SocialSharedChannelSyncTargetedMutationResponse {
            status: "claimed",
            affected,
            pending_after: state.pending_shared_channel_sync_requests.len(),
        };
        drop(state);
        self.persist_state_snapshot("shared-channel sync targeted claim")
            .map_err(|error| SharedChannelSyncOwnerConflict {
                code: "shared_channel_sync_persist_failed",
                message: error,
                details: serde_json::json!({}),
            })?;
        Ok(response)
    }

    pub fn release_pending_shared_channel_sync_targeted_persisted(
        &self,
        request_keys: &[String],
        actor_id: &str,
        actor_kind: &str,
    ) -> Result<SocialSharedChannelSyncTargetedMutationResponse, SharedChannelSyncOwnerConflict>
    {
        let now = utc_now_rfc3339_millis();
        let mut state = self
            .state
            .write()
            .map_err(|_| SharedChannelSyncOwnerConflict {
                code: "social_runtime_lock_poisoned",
                message: "social runtime lock poisoned".into(),
                details: serde_json::json!({}),
            })?;
        let affected = state.release_pending_shared_channel_sync_targeted(
            request_keys,
            actor_id,
            actor_kind,
            now.as_str(),
        )?;
        let response = SocialSharedChannelSyncTargetedMutationResponse {
            status: "released",
            affected,
            pending_after: state.pending_shared_channel_sync_requests.len(),
        };
        drop(state);
        self.persist_state_snapshot("shared-channel sync targeted release")
            .map_err(|error| SharedChannelSyncOwnerConflict {
                code: "shared_channel_sync_persist_failed",
                message: error,
                details: serde_json::json!({}),
            })?;
        Ok(response)
    }

    pub fn takeover_pending_shared_channel_sync_targeted_persisted(
        &self,
        request_keys: &[String],
        actor_id: &str,
        actor_kind: &str,
        legacy_override: bool,
    ) -> Result<SocialSharedChannelSyncTargetedMutationResponse, SharedChannelSyncOwnerConflict>
    {
        let now = utc_now_rfc3339_millis();
        let mut state = self
            .state
            .write()
            .map_err(|_| SharedChannelSyncOwnerConflict {
                code: "social_runtime_lock_poisoned",
                message: "social runtime lock poisoned".into(),
                details: serde_json::json!({}),
            })?;
        let affected = state.takeover_pending_shared_channel_sync_targeted(
            request_keys,
            actor_id,
            actor_kind,
            now.as_str(),
            legacy_override,
        )?;
        let response = SocialSharedChannelSyncTargetedMutationResponse {
            status: "taken_over",
            affected,
            pending_after: state.pending_shared_channel_sync_requests.len(),
        };
        drop(state);
        self.persist_state_snapshot("shared-channel sync targeted takeover")
            .map_err(|error| SharedChannelSyncOwnerConflict {
                code: "shared_channel_sync_persist_failed",
                message: error,
                details: serde_json::json!({}),
            })?;
        Ok(response)
    }

    pub fn repair_shared_channel_sync(
        &self,
    ) -> Result<SocialSharedChannelSyncRepairResponse, String> {
        let now = utc_now_rfc3339_millis();
        let requests = {
            let mut state = self
                .state
                .write()
                .map_err(|error| format!("social runtime lock poisoned: {error}"))?;
            let reclaimed =
                state.reclaim_stale_pending_shared_channel_sync_claims(now.as_str());
            let requests = state
                .pending_shared_channel_sync_requests
                .values()
                .map(|pending| pending.request.clone())
                .collect::<Vec<_>>();
            (reclaimed, requests, state.pending_shared_channel_sync_requests.len())
        };
        self.persist_state_snapshot("shared-channel sync repair reclaim")?;
        let (reclaimed, requests, _) = requests;
        let trigger = self.shared_channel_linked_member_sync_trigger();
        let Some(trigger) = trigger else {
            return Ok(SocialSharedChannelSyncRepairResponse {
                status: "trigger_unconfigured",
                reclaimed,
                attempted: 0,
                repaired: 0,
                failed: 0,
                pending_after: self
                    .state
                    .read()
                    .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
                    .pending_shared_channel_sync_requests
                    .len(),
            });
        };

        let mut repaired = 0usize;
        let mut failed = 0usize;
        for request in &requests {
            if self
                .state
                .read()
                .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
                .is_shared_channel_sync_delivered(request)
            {
                continue;
            }
            match trigger.trigger(request.clone()) {
                Ok(()) => {
                    let mut state = self
                        .state
                        .write()
                        .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
                    state.mark_shared_channel_sync_delivered(
                        request,
                        SharedChannelSyncDeliveryProofStatus::Applied,
                        now.as_str(),
                        None,
                    );
                    drop(state);
                    self.persist_state_snapshot("shared-channel sync repair delivery")?;
                    repaired += 1;
                }
                Err(error) => {
                    let mut state = self
                        .state
                        .write()
                        .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
                    state.record_failed_shared_channel_sync_requests_with_owner_preservation(
                        std::slice::from_ref(request),
                        error.as_str(),
                        now.as_str(),
                        true,
                    );
                    drop(state);
                    self.persist_state_snapshot("shared-channel sync repair failure")?;
                    failed += 1;
                }
            }
        }
        Ok(SocialSharedChannelSyncRepairResponse {
            status: if failed == 0 { "repaired" } else { "partial" },
            reclaimed,
            attempted: requests.len(),
            repaired,
            failed,
            pending_after: self
                .state
                .read()
                .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
                .pending_shared_channel_sync_requests
                .len(),
        })
    }

    pub fn republish_pending_shared_channel_sync_targeted(
        &self,
        actor_id: &str,
        actor_kind: &str,
        request_keys: Vec<String>,
    ) -> Result<SocialSharedChannelSyncRepublishResponse, SharedChannelSyncOwnerConflict> {
        let now = utc_now_rfc3339_millis();
        let requests = {
            let mut state = self
                .state
                .write()
                .map_err(|_| SharedChannelSyncOwnerConflict {
                    code: "social_runtime_lock_poisoned",
                    message: "social runtime lock poisoned".into(),
                    details: serde_json::json!({}),
                })?;
            state.reclaim_stale_pending_shared_channel_sync_claims(now.as_str());
            let mut selected = Vec::new();
            for request_key in &request_keys {
                let Some(pending) = state
                    .pending_shared_channel_sync_requests
                    .get(request_key.as_str())
                    .cloned()
                else {
                    continue;
                };
                let status = pending.lease_status(now.as_str());
                if status == SocialSharedChannelSyncLeaseStatus::Active
                    && (pending.owner_actor_id.as_deref() != Some(actor_id)
                        || pending.owner_actor_kind.as_deref() != Some(actor_kind))
                {
                    return Err(SharedChannelSyncOwnerConflict::from_pending(
                        request_key.clone(),
                        &pending,
                        actor_id,
                        actor_kind,
                        now.as_str(),
                    ));
                }
                if matches!(
                    status,
                    SocialSharedChannelSyncLeaseStatus::Stale
                        | SocialSharedChannelSyncLeaseStatus::Untracked
                ) && pending.owner_actor_id.as_deref() == Some(actor_id)
                    && pending.owner_actor_kind.as_deref() == Some(actor_kind)
                {
                    let mut renewed = pending.clone();
                    renewed.renew_claim_lease(
                        actor_id,
                        actor_kind,
                        now.as_str(),
                        shared_channel_sync_claim_lease_seconds(),
                    );
                    state.upsert_pending_shared_channel_sync_request(
                        request_key.clone(),
                        renewed,
                    );
                }
                selected.push(pending.request.clone());
            }
            drop(state);
            self.persist_state_snapshot("shared-channel sync targeted republish prep")
                .map_err(|error| SharedChannelSyncOwnerConflict {
                    code: "shared_channel_sync_persist_failed",
                    message: error,
                    details: serde_json::json!({}),
                })?;
            selected
        };

        let trigger = self.shared_channel_linked_member_sync_trigger();
        let Some(trigger) = trigger else {
            return Ok(SocialSharedChannelSyncRepublishResponse {
                status: "trigger_unconfigured",
                attempted: requests.len(),
                repaired: 0,
                failed: 0,
                trigger_unconfigured: true,
            });
        };

        let mut repaired = 0usize;
        let mut failed = 0usize;
        for request in &requests {
            match trigger.trigger(request.clone()) {
                Ok(()) => {
                    let mut state = self
                        .state
                        .write()
                        .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
                    state.mark_shared_channel_sync_delivered(
                        request,
                        SharedChannelSyncDeliveryProofStatus::Applied,
                        now.as_str(),
                        None,
                    );
                    drop(state);
                    self.persist_state_snapshot("shared-channel sync republish delivery")
                        .map_err(|error| SharedChannelSyncOwnerConflict {
                            code: "shared_channel_sync_persist_failed",
                            message: error,
                            details: serde_json::json!({}),
                        })?;
                    repaired += 1;
                }
                Err(error) => {
                    let mut state = self
                        .state
                        .write()
                        .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
                    state.record_failed_shared_channel_sync_requests_with_owner_preservation(
                        std::slice::from_ref(request),
                        error.as_str(),
                        now.as_str(),
                        true,
                    );
                    drop(state);
                    self.persist_state_snapshot("shared-channel sync republish failure")
                        .map_err(|error| SharedChannelSyncOwnerConflict {
                            code: "shared_channel_sync_persist_failed",
                            message: error,
                            details: serde_json::json!({}),
                        })?;
                    failed += 1;
                }
            }
        }
        Ok(SocialSharedChannelSyncRepublishResponse {
            status: if failed == 0 { "republished" } else { "partial" },
            attempted: requests.len(),
            repaired,
            failed,
            trigger_unconfigured: false,
        })
    }
}
