//! Realtime fanout for social domain commits.

use im_domain_events::social::{
    DirectChatBoundPayload, FriendRequestAcceptedPayload, FriendRequestCanceledPayload,
    FriendRequestDeclinedPayload, FriendRequestExpiredPayload, FriendRequestSubmittedPayload,
    FriendshipActivatedPayload, FriendshipRemovedPayload, UserBlockedPayload,
    UserBlockReleasedPayload,
};
use im_platform_contracts::{
    CommitEnvelope, OutboxEventRecord, OutboxPublishStatus,
};
use im_time::utc_now_rfc3339_millis;
use sdkwork_utils_rust::sha256_hash;
use serde_json::json;
use tracing::warn;

pub const SOCIAL_OUTBOX_AGGREGATE_TYPE: &str = "social";
const REQUIRE_REALTIME_PUBLISHER_ENV: &str = "SDKWORK_IM_REQUIRE_REALTIME_PUBLISHER";

pub fn env_flag_enabled(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

pub fn commit_requires_realtime_delivery(commit: &CommitEnvelope) -> Result<bool, String> {
    let (recipients, _) = social_realtime_recipients_for_commit(commit)?;
    Ok(!recipients.is_empty())
}

pub fn ensure_realtime_delivery_configured(
    has_fanout: bool,
    has_outbox: bool,
    commits: &[CommitEnvelope],
) -> Result<(), String> {
    if !env_flag_enabled(REQUIRE_REALTIME_PUBLISHER_ENV) {
        return Ok(());
    }
    if has_fanout || has_outbox {
        return Ok(());
    }
    for commit in commits {
        if commit_requires_realtime_delivery(commit).unwrap_or(true) {
            return Err(
                "social realtime delivery is required in production when neither embedded fanout nor outbox is configured"
                    .into(),
            );
        }
    }
    Ok(())
}

/// Publish social commits to connected realtime subscribers.
pub trait SocialRealtimeFanout: Send + Sync {
    fn publish_social_commit(&self, envelope: &CommitEnvelope) -> Result<(), String>;
}

pub struct LoggingSocialRealtimeFanout;

impl SocialRealtimeFanout for LoggingSocialRealtimeFanout {
    fn publish_social_commit(&self, envelope: &CommitEnvelope) -> Result<(), String> {
        tracing::debug!(
            event_type = envelope.event_type.as_str(),
            aggregate_id = envelope.aggregate_id.as_str(),
            "social realtime fanout skipped (logging-only adapter)"
        );
        Ok(())
    }
}

pub fn try_publish_social_commits(
    fanout: Option<&dyn SocialRealtimeFanout>,
    commits: &[CommitEnvelope],
) {
    let Some(fanout) = fanout else {
        return;
    };
    for commit in commits {
        if let Err(error) = fanout.publish_social_commit(commit) {
            warn!(
                event_id = commit.event_id.as_str(),
                event_type = commit.event_type.as_str(),
                error = %error,
                "social realtime fanout skipped for commit"
            );
        }
    }
}

pub fn build_social_realtime_outbox_record(
    commit: &CommitEnvelope,
    id_generator: &dyn im_platform_contracts::IdGenerator,
) -> Result<Option<OutboxEventRecord>, String> {
    let (recipients, _) = social_realtime_recipients_for_commit(commit)?;
    if recipients.is_empty() {
        return Ok(None);
    }
    let recipient_principal_ids = recipients
        .iter()
        .map(|(principal_id, _)| principal_id.as_str())
        .collect::<Vec<_>>();
    let payload_json = json!({
        "recipientPrincipalIds": recipient_principal_ids,
        "commitPayload": serde_json::from_str::<serde_json::Value>(commit.payload.as_str())
            .unwrap_or_else(|_| json!(commit.payload)),
    });
    let payload_json = serde_json::to_string(&payload_json)
        .map_err(|error| format!("social outbox payload encode failed: {error}"))?;
    let payload_hash = sha256_hash(payload_json.as_bytes());
    let now = utc_now_rfc3339_millis();
    let outbox_id = id_generator
        .next_id()
        .map_err(|error| format!("social outbox id allocation failed: {error:?}"))?
        .to_string();
    Ok(Some(OutboxEventRecord {
        tenant_id: commit.tenant_id.clone(),
        organization_id: commit.organization_id.clone(),
        outbox_id,
        aggregate_type: SOCIAL_OUTBOX_AGGREGATE_TYPE.into(),
        aggregate_id: commit.aggregate_id.clone(),
        event_id: commit.event_id.clone(),
        event_type: commit.event_type.clone(),
        payload_json,
        payload_hash,
        publish_status: OutboxPublishStatus::Pending,
        attempt_count: 0,
        available_at: now.clone(),
        published_at: None,
        created_at: now.clone(),
        updated_at: now,
    }))
}

pub fn social_realtime_recipients_for_commit(
    commit: &CommitEnvelope,
) -> Result<(Vec<(String, String)>, String), String> {
    let payload = build_realtime_payload(commit)?;
    let recipients = match commit.event_type.as_str() {
        "friend_request.submitted" => {
            let body: FriendRequestSubmittedPayload =
                serde_json::from_str(commit.payload.as_str())
                    .map_err(|error| format!("invalid friend_request.submitted payload: {error}"))?;
            friend_request_party_recipients(
                body.requester_user_id.as_str(),
                body.target_user_id.as_str(),
            )
        }
        "friend_request.accepted" => {
            let body: FriendRequestAcceptedPayload =
                serde_json::from_str(commit.payload.as_str())
                    .map_err(|error| format!("invalid friend_request.accepted payload: {error}"))?;
            friend_request_outcome_recipients(
                body.requester_user_id.as_str(),
                body.target_user_id.as_str(),
                body.accepted_by_user_id.as_str(),
            )
        }
        "friend_request.declined" => {
            let body: FriendRequestDeclinedPayload =
                serde_json::from_str(commit.payload.as_str())
                    .map_err(|error| format!("invalid friend_request.declined payload: {error}"))?;
            friend_request_outcome_recipients(
                body.requester_user_id.as_str(),
                body.target_user_id.as_str(),
                body.declined_by_user_id.as_str(),
            )
        }
        "friend_request.canceled" => {
            let body: FriendRequestCanceledPayload =
                serde_json::from_str(commit.payload.as_str())
                    .map_err(|error| format!("invalid friend_request.canceled payload: {error}"))?;
            friend_request_outcome_recipients(
                body.requester_user_id.as_str(),
                body.target_user_id.as_str(),
                body.canceled_by_user_id.as_str(),
            )
        }
        "friend_request.expired" => {
            let body: FriendRequestExpiredPayload = serde_json::from_str(commit.payload.as_str())
                .map_err(|error| format!("invalid friend_request.expired payload: {error}"))?;
            friend_request_party_recipients(
                body.requester_user_id.as_str(),
                body.target_user_id.as_str(),
            )
        }
        "friendship.activated" => {
            let body: FriendshipActivatedPayload =
                serde_json::from_str(commit.payload.as_str())
                    .map_err(|error| format!("invalid friendship.activated payload: {error}"))?;
            vec![
                (body.user_low_id, "user".to_string()),
                (body.user_high_id, "user".to_string()),
            ]
        }
        "friendship.removed" => {
            let body: FriendshipRemovedPayload =
                serde_json::from_str(commit.payload.as_str())
                    .map_err(|error| format!("invalid friendship.removed payload: {error}"))?;
            vec![
                (body.user_low_id, "user".to_string()),
                (body.user_high_id, "user".to_string()),
            ]
        }
        "user_block.blocked" => {
            let body: UserBlockedPayload = serde_json::from_str(commit.payload.as_str())
                .map_err(|error| format!("invalid user_block.blocked payload: {error}"))?;
            friend_request_party_recipients(
                body.blocker_user_id.as_str(),
                body.blocked_user_id.as_str(),
            )
        }
        "user_block.released" => {
            let body: UserBlockReleasedPayload = serde_json::from_str(commit.payload.as_str())
                .map_err(|error| format!("invalid user_block.released payload: {error}"))?;
            friend_request_party_recipients(
                body.blocker_user_id.as_str(),
                body.blocked_user_id.as_str(),
            )
        }
        "direct_chat.bound" => {
            let body: DirectChatBoundPayload = serde_json::from_str(commit.payload.as_str())
                .map_err(|error| format!("invalid direct_chat.bound payload: {error}"))?;
            vec![
                (body.left_actor_id, "user".to_string()),
                (body.right_actor_id, "user".to_string()),
            ]
        }
        _ => Vec::new(),
    };
    Ok((recipients, payload))
}

fn actor_recipient(user_id: &str) -> Vec<(String, String)> {
    vec![(user_id.to_owned(), "user".to_owned())]
}

fn friend_request_party_recipients(
    requester_user_id: &str,
    target_user_id: &str,
) -> Vec<(String, String)> {
    vec![
        (requester_user_id.to_owned(), "user".to_owned()),
        (target_user_id.to_owned(), "user".to_owned()),
    ]
}

fn friend_request_outcome_recipients(
    requester_user_id: &str,
    target_user_id: &str,
    actor_user_id: &str,
) -> Vec<(String, String)> {
    if !requester_user_id.trim().is_empty() && !target_user_id.trim().is_empty() {
        return friend_request_party_recipients(requester_user_id, target_user_id);
    }
    actor_recipient(actor_user_id)
}

fn build_realtime_payload(commit: &CommitEnvelope) -> Result<String, String> {
    Ok(json!({
        "eventId": commit.event_id,
        "eventType": commit.event_type,
        "aggregateId": commit.aggregate_id,
        "tenantId": commit.tenant_id,
        "organizationId": commit.organization_id,
        "occurredAt": commit.occurred_at,
        "payload": serde_json::from_str::<serde_json::Value>(commit.payload.as_str())
            .unwrap_or_else(|_| json!(commit.payload)),
    })
    .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_events::CommitEnvelope;
    use sdkwork_im_runtime_id::RuntimeSnowflakeIdGenerator;

    #[test]
    fn build_social_realtime_outbox_record_includes_recipient_principal_ids() {
        let id_generator =
            RuntimeSnowflakeIdGenerator::with_node_id(0).expect("snowflake node 0 must initialize");
        let commit = CommitEnvelope::minimal(
            "evt-friend-1",
            "100001",
            "friend_request.submitted",
            "friend_request",
            "fr-1",
            1,
        )
        .with_payload(
            "FriendRequestSubmittedPayload",
            &serde_json::json!({
                "requestId": "fr-1",
                "requesterUserId": "u_alice",
                "targetUserId": "u_bob",
                "requestedAt": "2026-01-01T00:00:00.000Z",
            })
            .to_string(),
        );

        let record = build_social_realtime_outbox_record(&commit, &id_generator)
            .expect("outbox record should build")
            .expect("friend request should produce outbox row");
        assert_eq!(record.aggregate_type, SOCIAL_OUTBOX_AGGREGATE_TYPE);
        let payload: serde_json::Value =
            serde_json::from_str(record.payload_json.as_str()).expect("payload json");
        assert_eq!(
            payload["recipientPrincipalIds"],
            serde_json::json!(["u_alice", "u_bob"])
        );
    }

    #[test]
    fn ensure_realtime_delivery_configured_fails_when_required_and_unconfigured() {
        let commit = CommitEnvelope::minimal(
            "evt-friend-2",
            "100001",
            "friend_request.submitted",
            "friend_request",
            "fr-2",
            1,
        )
        .with_payload(
            "FriendRequestSubmittedPayload",
            &serde_json::json!({
                "requestId": "fr-2",
                "requesterUserId": "u_alice",
                "targetUserId": "u_bob",
                "requestedAt": "2026-01-01T00:00:00.000Z",
            })
            .to_string(),
        );
        unsafe {
            std::env::set_var(REQUIRE_REALTIME_PUBLISHER_ENV, "1");
        }
        let result =
            ensure_realtime_delivery_configured(false, false, std::slice::from_ref(&commit));
        unsafe {
            std::env::remove_var(REQUIRE_REALTIME_PUBLISHER_ENV);
        }
        assert!(result.is_err());
    }

    #[test]
    fn friend_request_outcome_notifies_both_parties() {
        let commit = CommitEnvelope::minimal(
            "evt-friend-accept-1",
            "100001",
            "friend_request.accepted",
            "friend_request",
            "fr-1",
            2,
        )
        .with_payload(
            "FriendRequestAcceptedPayload",
            &serde_json::json!({
                "requestId": "fr-1",
                "requesterUserId": "u_alice",
                "targetUserId": "u_bob",
                "acceptedByUserId": "u_bob",
                "acceptedAt": "2026-01-01T00:00:01.000Z",
            })
            .to_string(),
        );
        let (recipients, _) =
            social_realtime_recipients_for_commit(&commit).expect("recipients should resolve");
        assert_eq!(recipients.len(), 2);
        assert!(recipients.iter().any(|(id, _)| id == "u_alice"));
        assert!(recipients.iter().any(|(id, _)| id == "u_bob"));
    }
}
