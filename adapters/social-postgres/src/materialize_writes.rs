//! Normalized Social writes executed inside the journal-owned PostgreSQL transaction.

use im_domain_events::social::{
    DirectChatBoundPayload, ExternalConnectionEstablishedPayload, ExternalMemberLinkBoundPayload,
    FriendRequestAcceptedPayload, FriendRequestCanceledPayload, FriendRequestDeclinedPayload,
    FriendRequestExpiredPayload, FriendRequestSubmittedPayload, FriendshipActivatedPayload,
    FriendshipRemovedPayload, SharedChannelPolicyAppliedPayload, UserBlockReleasedPayload,
    UserBlockedPayload,
};
use im_platform_contracts::{CommitEnvelope, ContractError};

use crate::wire_id::parse_social_entity_id;
use crate::{optional_postgres_timestamptz, postgres_timestamptz};

fn social_entity_id(value: &str) -> Result<i64, String> {
    parse_social_entity_id(value).map_err(|error| format!("{error:?}"))
}

fn social_materialize_timestamptz(
    value: &str,
    field: &'static str,
) -> Result<chrono::DateTime<chrono::Utc>, String> {
    postgres_timestamptz(value, field).map_err(|error| format!("{error:?}"))
}

fn social_materialize_optional_timestamptz(
    value: Option<&str>,
    field: &'static str,
) -> Result<Option<chrono::DateTime<chrono::Utc>>, String> {
    optional_postgres_timestamptz(value, field).map_err(|error| format!("{error:?}"))
}

const FRIEND_REQUEST_INSERT_SQL: &str = r#"
INSERT INTO im_friend_requests (
    tenant_id, organization_id, request_id, requester_user_id, target_user_id,
    request_message, status, expired_at, created_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8::timestamptz, $9::timestamptz, $10::timestamptz)
ON CONFLICT (tenant_id, organization_id, request_id) DO NOTHING
"#;

const FRIEND_REQUEST_GET_BY_ID_SQL: &str = r#"
SELECT status
FROM im_friend_requests
WHERE tenant_id = $1 AND organization_id = $2 AND request_id = $3
"#;

const FRIEND_REQUEST_UPDATE_STATUS_SQL: &str = r#"
UPDATE im_friend_requests
SET status = $4, updated_at = $5::timestamptz
WHERE tenant_id = $1 AND organization_id = $2 AND request_id = $3 AND status = 'pending'
"#;

const FRIENDSHIP_UPSERT_ACTIVE_PAIR_SQL: &str = r#"
INSERT INTO im_friendships (
    tenant_id, organization_id, friendship_id, user_low_id, user_high_id,
    initiator_user_id, status, established_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8::timestamptz, $9::timestamptz)
ON CONFLICT (tenant_id, organization_id, user_low_id, user_high_id)
DO UPDATE SET
    friendship_id = EXCLUDED.friendship_id,
    initiator_user_id = EXCLUDED.initiator_user_id,
    status = EXCLUDED.status,
    established_at = EXCLUDED.established_at,
    updated_at = EXCLUDED.updated_at
"#;

const FRIENDSHIP_UPDATE_STATUS_SQL: &str = r#"
UPDATE im_friendships
SET status = $4, updated_at = $5::timestamptz
WHERE tenant_id = $1 AND organization_id = $2 AND friendship_id = $3
"#;

const USER_BLOCK_INSERT_SQL: &str = r#"
INSERT INTO im_user_blocks (
    tenant_id, organization_id, block_id, blocker_user_id, blocked_user_id,
    scope, direct_chat_id, reason, expires_at, created_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9::timestamptz, $10::timestamptz, $11::timestamptz)
ON CONFLICT (tenant_id, organization_id, block_id) DO NOTHING
"#;

const USER_BLOCK_DELETE_BY_BLOCKER_SQL: &str = r#"
DELETE FROM im_user_blocks
WHERE tenant_id = $1 AND organization_id = $2 AND block_id = $3 AND blocker_user_id = $4
"#;

const DIRECT_CHAT_INSERT_SQL: &str = r#"
INSERT INTO im_direct_chats (
    tenant_id, organization_id, direct_chat_id, left_actor_kind, left_actor_id,
    right_actor_kind, right_actor_id, pair_hash, status, conversation_id,
    created_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11::timestamptz, $12::timestamptz)
ON CONFLICT (tenant_id, organization_id, direct_chat_id) DO NOTHING
"#;

const EXTERNAL_CONNECTION_INSERT_SQL: &str = r#"
INSERT INTO im_external_connections (
    tenant_id, organization_id, connection_id, external_tenant_id,
    external_org_name, connection_kind, status, established_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, 'active', $7::timestamptz, $7::timestamptz)
ON CONFLICT (tenant_id, organization_id, connection_id) DO NOTHING
"#;

const EXTERNAL_CONNECTION_MATCH_SQL: &str = r#"
SELECT EXISTS (
    SELECT 1
    FROM im_external_connections
    WHERE tenant_id = $1
      AND organization_id = $2
      AND connection_id = $3
      AND external_tenant_id = $4
      AND external_org_name IS NOT DISTINCT FROM $5
      AND connection_kind = $6
      AND status = 'active'
      AND established_at = $7::timestamptz
)
"#;

const EXTERNAL_MEMBER_LINK_INSERT_SQL: &str = r#"
INSERT INTO im_external_member_links (
    tenant_id, organization_id, link_id, connection_id, local_actor_kind,
    local_actor_id, external_member_id, external_display_name, status,
    linked_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'active', $9::timestamptz, $9::timestamptz)
ON CONFLICT (tenant_id, organization_id, link_id) DO NOTHING
"#;

const EXTERNAL_MEMBER_LINK_MATCH_SQL: &str = r#"
SELECT EXISTS (
    SELECT 1
    FROM im_external_member_links
    WHERE tenant_id = $1
      AND organization_id = $2
      AND link_id = $3
      AND connection_id = $4
      AND local_actor_kind = $5
      AND local_actor_id = $6
      AND external_member_id = $7
      AND external_display_name IS NOT DISTINCT FROM $8
      AND status = 'active'
      AND linked_at = $9::timestamptz
)
"#;

const SHARED_CHANNEL_POLICY_INSERT_SQL: &str = r#"
INSERT INTO im_shared_channel_policies (
    tenant_id, organization_id, policy_id, connection_id, channel_id,
    conversation_id, policy_version, history_visibility, status,
    applied_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'active', $9::timestamptz, $9::timestamptz)
ON CONFLICT (tenant_id, organization_id, policy_id) DO NOTHING
"#;

const SHARED_CHANNEL_POLICY_MATCH_SQL: &str = r#"
SELECT EXISTS (
    SELECT 1
    FROM im_shared_channel_policies
    WHERE tenant_id = $1
      AND organization_id = $2
      AND policy_id = $3
      AND connection_id = $4
      AND channel_id = $5
      AND conversation_id IS NOT DISTINCT FROM $6
      AND policy_version = $7
      AND history_visibility = $8
      AND status = 'active'
      AND applied_at = $9::timestamptz
)
"#;

/// Write newly committed Social state changes on a caller-owned transaction.
///
/// The online PostgreSQL write authority uses this entrypoint so journal rows and
/// normalized Social rows commit or roll back as one database unit.
pub fn materialize_commits_on_transaction(
    txn: &mut postgres::Transaction<'_>,
    commits: &[CommitEnvelope],
) -> Result<(), ContractError> {
    for commit in commits {
        materialize_commit_on(txn, commit).map_err(|error| {
            ContractError::Unavailable(format!("social postgres materialization failed: {error}"))
        })?;
    }
    Ok(())
}

fn materialize_commit_on(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    match commit.event_type.as_str() {
        "friend_request.submitted" => materialize_friend_request_submitted(txn, commit),
        "friend_request.accepted" => materialize_friend_request_status(txn, commit, "accepted"),
        "friend_request.declined" => materialize_friend_request_status(txn, commit, "declined"),
        "friend_request.canceled" => materialize_friend_request_status(txn, commit, "canceled"),
        "friend_request.expired" => materialize_friend_request_status(txn, commit, "expired"),
        "friendship.activated" => materialize_friendship_activated(txn, commit),
        "friendship.removed" => materialize_friendship_removed(txn, commit),
        "user_block.blocked" => materialize_user_blocked(txn, commit),
        "user_block.released" => materialize_user_block_released(txn, commit),
        "direct_chat.bound" => materialize_direct_chat_bound(txn, commit),
        "external_connection.established" => materialize_external_connection(txn, commit),
        "external_member_link.bound" => materialize_external_member_link(txn, commit),
        "shared_channel_policy.applied" => materialize_shared_channel_policy(txn, commit),
        event_type => Err(format!(
            "unsupported social normalized write event type {event_type}"
        )),
    }
}

fn materialize_friend_request_submitted(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: FriendRequestSubmittedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid friend_request.submitted payload: {error}"))?;
    let expired_at =
        social_materialize_optional_timestamptz(payload.expires_at.as_deref(), "expires_at")?;
    let requested_at =
        social_materialize_timestamptz(payload.requested_at.as_str(), "requested_at")?;
    txn.execute(
        FRIEND_REQUEST_INSERT_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &social_entity_id(payload.request_id.as_str())?,
            &payload.requester_user_id,
            &payload.target_user_id,
            &payload.request_message,
            &"pending".to_string(),
            &expired_at,
            &requested_at,
            &requested_at,
        ],
    )
    .map_err(|error| format!("friend_request insert failed: {error}"))
    .map(|_| ())
}

fn materialize_friend_request_status(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
    status: &str,
) -> Result<(), String> {
    let (request_id, updated_at) = match status {
        "accepted" => {
            let payload: FriendRequestAcceptedPayload =
                serde_json::from_str(commit.payload.as_str())
                    .map_err(|error| format!("invalid friend_request.accepted payload: {error}"))?;
            (
                social_entity_id(payload.request_id.as_str())?,
                payload.accepted_at,
            )
        }
        "declined" => {
            let payload: FriendRequestDeclinedPayload =
                serde_json::from_str(commit.payload.as_str())
                    .map_err(|error| format!("invalid friend_request.declined payload: {error}"))?;
            (
                social_entity_id(commit.aggregate_id.as_str())?,
                payload.declined_at,
            )
        }
        "canceled" => {
            let payload: FriendRequestCanceledPayload =
                serde_json::from_str(commit.payload.as_str())
                    .map_err(|error| format!("invalid friend_request.canceled payload: {error}"))?;
            (
                social_entity_id(commit.aggregate_id.as_str())?,
                payload.canceled_at,
            )
        }
        "expired" => {
            let payload: FriendRequestExpiredPayload =
                serde_json::from_str(commit.payload.as_str())
                    .map_err(|error| format!("invalid friend_request.expired payload: {error}"))?;
            (
                social_entity_id(commit.aggregate_id.as_str())?,
                payload.expired_at,
            )
        }
        _ => return Ok(()),
    };
    update_friend_request_status_idempotent(
        txn,
        commit.tenant_id.as_str(),
        commit.organization_id.as_str(),
        request_id,
        status,
        updated_at.as_str(),
    )
}

#[allow(clippy::too_many_arguments)]
fn update_friend_request_status_idempotent(
    txn: &mut postgres::Transaction<'_>,
    tenant_id: &str,
    organization_id: &str,
    request_id: i64,
    status: &str,
    updated_at: &str,
) -> Result<(), String> {
    let updated_at_ts = social_materialize_timestamptz(updated_at, "updated_at")?;
    let updated = txn
        .execute(
            FRIEND_REQUEST_UPDATE_STATUS_SQL,
            &[
                &tenant_id,
                &organization_id,
                &request_id,
                &status.to_string(),
                &updated_at_ts,
            ],
        )
        .map_err(|error| format!("friend_request update failed: {error}"))?;
    if updated > 0 {
        return Ok(());
    }
    let existing = txn
        .query_opt(
            FRIEND_REQUEST_GET_BY_ID_SQL,
            &[&tenant_id, &organization_id, &request_id],
        )
        .map_err(|error| format!("friend_request load failed: {error}"))?;
    match existing {
        Some(row) => {
            let current_status: String = row.get("status");
            if current_status == status {
                Ok(())
            } else {
                Err(format!(
                    "friend_request {request_id} is not pending (current status: {current_status})"
                ))
            }
        }
        None => Err(format!(
            "friend_request {request_id} does not exist in normalized PostgreSQL state"
        )),
    }
}

fn materialize_friendship_activated(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: FriendshipActivatedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid friendship.activated payload: {error}"))?;
    let established_at = social_materialize_optional_timestamptz(
        Some(payload.established_at.as_str()),
        "established_at",
    )?;
    let updated_at = social_materialize_timestamptz(payload.established_at.as_str(), "updated_at")?;
    txn.execute(
        FRIENDSHIP_UPSERT_ACTIVE_PAIR_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &social_entity_id(payload.friendship_id.as_str())?,
            &payload.user_low_id,
            &payload.user_high_id,
            &payload.initiator_user_id,
            &"active".to_string(),
            &established_at,
            &updated_at,
        ],
    )
    .map_err(|error| format!("friendship upsert failed: {error}"))
    .map(|_| ())
}

fn materialize_friendship_removed(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: FriendshipRemovedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid friendship.removed payload: {error}"))?;
    let removed_at = social_materialize_timestamptz(payload.removed_at.as_str(), "removed_at")?;
    let updated = txn
        .execute(
            FRIENDSHIP_UPDATE_STATUS_SQL,
            &[
                &commit.tenant_id,
                &commit.organization_id,
                &social_entity_id(payload.friendship_id.as_str())?,
                &"removed".to_string(),
                &removed_at,
            ],
        )
        .map_err(|error| format!("friendship update failed: {error}"))?;
    if updated == 0 {
        return Err(format!(
            "friendship {} does not exist in tenant scope",
            payload.friendship_id
        ));
    }
    Ok(())
}

fn materialize_user_blocked(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: UserBlockedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid user_block.blocked payload: {error}"))?;
    let expires_at =
        social_materialize_optional_timestamptz(payload.expires_at.as_deref(), "expires_at")?;
    let effective_at =
        social_materialize_timestamptz(payload.effective_at.as_str(), "effective_at")?;
    let direct_chat_id = payload
        .direct_chat_id
        .as_deref()
        .map(social_entity_id)
        .transpose()?;
    txn.execute(
        USER_BLOCK_INSERT_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &social_entity_id(payload.block_id.as_str())?,
            &payload.blocker_user_id,
            &payload.blocked_user_id,
            &payload.scope,
            &direct_chat_id,
            &None::<String>,
            &expires_at,
            &effective_at,
            &effective_at,
        ],
    )
    .map_err(|error| format!("user_block insert failed: {error}"))
    .map(|_| ())
}

fn materialize_user_block_released(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: UserBlockReleasedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid user_block.released payload: {error}"))?;
    txn.execute(
        USER_BLOCK_DELETE_BY_BLOCKER_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &social_entity_id(payload.block_id.as_str())?,
            &payload.blocker_user_id,
        ],
    )
    .map_err(|error| format!("user_block release failed: {error}"))
    .map(|_| ())
}

fn materialize_direct_chat_bound(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: DirectChatBoundPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid direct_chat.bound payload: {error}"))?;
    let bound_at = social_materialize_timestamptz(payload.bound_at.as_str(), "bound_at")?;
    txn.execute(
        DIRECT_CHAT_INSERT_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &social_entity_id(payload.direct_chat_id.as_str())?,
            &"user".to_string(),
            &payload.left_actor_id,
            &"user".to_string(),
            &payload.right_actor_id,
            &payload.pair_hash,
            &"active".to_string(),
            &Some(payload.conversation_id.clone()),
            &bound_at,
            &bound_at,
        ],
    )
    .map_err(|error| format!("direct_chat insert failed: {error}"))
    .map(|_| ())
}

fn materialize_external_connection(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: ExternalConnectionEstablishedPayload =
        serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid external_connection.established payload: {error}"))?;
    let connection_id = social_entity_id(payload.connection_id.as_str())?;
    let established_at =
        social_materialize_timestamptz(payload.established_at.as_str(), "established_at")?;
    let params: [&(dyn postgres::types::ToSql + Sync); 7] = [
        &commit.tenant_id,
        &commit.organization_id,
        &connection_id,
        &payload.external_tenant_id,
        &payload.external_org_name,
        &payload.connection_kind,
        &established_at,
    ];
    txn.execute(EXTERNAL_CONNECTION_INSERT_SQL, &params)
        .map_err(|error| format!("external_connection insert failed: {error}"))?;
    ensure_normalized_row_matches(
        txn,
        EXTERNAL_CONNECTION_MATCH_SQL,
        &params,
        "external_connection",
        payload.connection_id.as_str(),
    )
}

fn materialize_external_member_link(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: ExternalMemberLinkBoundPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid external_member_link.bound payload: {error}"))?;
    let link_id = social_entity_id(payload.link_id.as_str())?;
    let connection_id = social_entity_id(payload.connection_id.as_str())?;
    let linked_at = social_materialize_timestamptz(payload.linked_at.as_str(), "linked_at")?;
    let params: [&(dyn postgres::types::ToSql + Sync); 9] = [
        &commit.tenant_id,
        &commit.organization_id,
        &link_id,
        &connection_id,
        &payload.local_actor_kind,
        &payload.local_actor_id,
        &payload.external_member_id,
        &payload.external_display_name,
        &linked_at,
    ];
    txn.execute(EXTERNAL_MEMBER_LINK_INSERT_SQL, &params)
        .map_err(|error| format!("external_member_link insert failed: {error}"))?;
    ensure_normalized_row_matches(
        txn,
        EXTERNAL_MEMBER_LINK_MATCH_SQL,
        &params,
        "external_member_link",
        payload.link_id.as_str(),
    )
}

fn materialize_shared_channel_policy(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: SharedChannelPolicyAppliedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid shared_channel_policy.applied payload: {error}"))?;
    let policy_id = social_entity_id(payload.policy_id.as_str())?;
    let connection_id = social_entity_id(payload.connection_id.as_str())?;
    let policy_version = i64::try_from(payload.policy_version)
        .map_err(|_| "shared_channel_policy policy_version exceeds PostgreSQL BIGINT".to_owned())?;
    let applied_at = social_materialize_timestamptz(payload.applied_at.as_str(), "applied_at")?;
    let params: [&(dyn postgres::types::ToSql + Sync); 9] = [
        &commit.tenant_id,
        &commit.organization_id,
        &policy_id,
        &connection_id,
        &payload.channel_id,
        &payload.conversation_id,
        &policy_version,
        &payload.history_visibility,
        &applied_at,
    ];
    txn.execute(SHARED_CHANNEL_POLICY_INSERT_SQL, &params)
        .map_err(|error| format!("shared_channel_policy insert failed: {error}"))?;
    ensure_normalized_row_matches(
        txn,
        SHARED_CHANNEL_POLICY_MATCH_SQL,
        &params,
        "shared_channel_policy",
        payload.policy_id.as_str(),
    )
}

fn ensure_normalized_row_matches(
    txn: &mut postgres::Transaction<'_>,
    query: &str,
    params: &[&(dyn postgres::types::ToSql + Sync)],
    resource: &str,
    resource_id: &str,
) -> Result<(), String> {
    let matches: bool = txn
        .query_one(query, params)
        .map_err(|error| format!("{resource} idempotency verification failed: {error}"))?
        .get(0);
    if matches {
        Ok(())
    } else {
        Err(format!(
            "{resource} {resource_id} conflicts with the normalized PostgreSQL row"
        ))
    }
}
