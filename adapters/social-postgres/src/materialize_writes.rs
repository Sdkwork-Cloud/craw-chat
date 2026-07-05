//! Transactional social commit materialization for multi-commit write batches.
//!
//! Friend accept and similar flows emit several commits (request status, friendship,
//! direct chat). A single PostgreSQL transaction keeps supplemental stores consistent
//! before journal append.

use im_domain_events::social::{
    DirectChatBoundPayload, FriendRequestAcceptedPayload, FriendRequestCanceledPayload,
    FriendRequestDeclinedPayload, FriendRequestExpiredPayload, FriendRequestSubmittedPayload,
    FriendshipActivatedPayload, FriendshipRemovedPayload, UserBlockedPayload,
    UserBlockReleasedPayload,
};
use im_platform_contracts::CommitEnvelope;

use crate::wire_id::social_entity_id_to_i64;
use crate::{postgres_pool_client, postgres_unavailable, run_postgres_io, SocialPostgresPool};

const FRIEND_REQUEST_INSERT_SQL: &str = r#"
INSERT INTO im_friend_requests (
    tenant_id, organization_id, request_id, requester_user_id, target_user_id,
    request_message, status, expired_at, created_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
ON CONFLICT (tenant_id, organization_id, request_id) DO NOTHING
"#;

const FRIEND_REQUEST_GET_BY_ID_SQL: &str = r#"
SELECT status
FROM im_friend_requests
WHERE tenant_id = $1 AND organization_id = $2 AND request_id = $3
"#;

const FRIEND_REQUEST_UPDATE_STATUS_SQL: &str = r#"
UPDATE im_friend_requests
SET status = $4, updated_at = $5
WHERE tenant_id = $1 AND organization_id = $2 AND request_id = $3 AND status = 'pending'
"#;

const FRIENDSHIP_UPSERT_ACTIVE_PAIR_SQL: &str = r#"
INSERT INTO im_friendships (
    tenant_id, organization_id, friendship_id, user_low_id, user_high_id,
    initiator_user_id, status, established_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
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
SET status = $4, updated_at = $5
WHERE tenant_id = $1 AND organization_id = $2 AND friendship_id = $3
"#;

const USER_BLOCK_INSERT_SQL: &str = r#"
INSERT INTO im_user_blocks (
    tenant_id, organization_id, block_id, blocker_user_id, blocked_user_id,
    scope, direct_chat_id, reason, expires_at, created_at, updated_at
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
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
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
ON CONFLICT (tenant_id, organization_id, direct_chat_id) DO NOTHING
"#;

/// Materialize a multi-commit social batch inside one PostgreSQL transaction.
pub fn materialize_commits_in_transaction(
    pool: &SocialPostgresPool,
    commits: &[CommitEnvelope],
) -> Result<(), String> {
    if commits.len() <= 1 {
        return Err(
            "materialize_commits_in_transaction requires at least two commits".to_owned(),
        );
    }
    let pool = pool.inner().clone();
    let commits = commits.to_vec();
    run_postgres_io(move || {
        let mut client = postgres_pool_client(&pool, "materialize_social_commits_batch")?;
        let mut txn = client
            .transaction()
            .map_err(|error| postgres_unavailable("materialize_social_commits_batch", error))?;
        for commit in &commits {
            materialize_commit_on(&mut txn, commit).map_err(|message| {
                im_platform_contracts::ContractError::Unavailable(message)
            })?;
        }
        txn.commit()
            .map_err(|error| postgres_unavailable("materialize_social_commits_batch", error))?;
        Ok(())
    })
    .map_err(|error| format!("{error:?}"))
}

fn materialize_commit_on(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    match commit.event_type.as_str() {
        "friend_request.submitted" => materialize_friend_request_submitted(txn, commit),
        "friend_request.accepted" => {
            materialize_friend_request_status(txn, commit, "accepted")
        }
        "friend_request.declined" => {
            materialize_friend_request_status(txn, commit, "declined")
        }
        "friend_request.canceled" => {
            materialize_friend_request_status(txn, commit, "canceled")
        }
        "friend_request.expired" => materialize_friend_request_status(txn, commit, "expired"),
        "friendship.activated" => materialize_friendship_activated(txn, commit),
        "friendship.removed" => materialize_friendship_removed(txn, commit),
        "user_block.blocked" => materialize_user_blocked(txn, commit),
        "user_block.released" => materialize_user_block_released(txn, commit),
        "direct_chat.bound" => materialize_direct_chat_bound(txn, commit),
        _ => Ok(()),
    }
}

fn materialize_friend_request_submitted(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: FriendRequestSubmittedPayload =
        serde_json::from_str(commit.payload.as_str())
            .map_err(|error| format!("invalid friend_request.submitted payload: {error}"))?;
    txn.execute(
        FRIEND_REQUEST_INSERT_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &social_entity_id_to_i64(payload.request_id.as_str()),
            &payload.requester_user_id,
            &payload.target_user_id,
            &payload.request_message,
            &"pending".to_string(),
            &payload.expires_at,
            &payload.requested_at,
            &payload.requested_at,
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
                serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                    format!("invalid friend_request.accepted payload: {error}")
                })?;
            (
                social_entity_id_to_i64(payload.request_id.as_str()),
                payload.accepted_at,
            )
        }
        "declined" => {
            let payload: FriendRequestDeclinedPayload =
                serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                    format!("invalid friend_request.declined payload: {error}")
                })?;
            (
                social_entity_id_to_i64(commit.aggregate_id.as_str()),
                payload.declined_at,
            )
        }
        "canceled" => {
            let payload: FriendRequestCanceledPayload =
                serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                    format!("invalid friend_request.canceled payload: {error}")
                })?;
            (
                social_entity_id_to_i64(commit.aggregate_id.as_str()),
                payload.canceled_at,
            )
        }
        "expired" => {
            let payload: FriendRequestExpiredPayload =
                serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                    format!("invalid friend_request.expired payload: {error}")
                })?;
            (
                social_entity_id_to_i64(commit.aggregate_id.as_str()),
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

fn update_friend_request_status_idempotent(
    txn: &mut postgres::Transaction<'_>,
    tenant_id: &str,
    organization_id: &str,
    request_id: i64,
    status: &str,
    updated_at: &str,
) -> Result<(), String> {
    let updated = txn
        .execute(
            FRIEND_REQUEST_UPDATE_STATUS_SQL,
            &[
                &tenant_id,
                &organization_id,
                &request_id,
                &status.to_string(),
                &updated_at.to_string(),
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
            "friend_request {request_id} not found for status update"
        )),
    }
}

fn materialize_friendship_activated(
    txn: &mut postgres::Transaction<'_>,
    commit: &CommitEnvelope,
) -> Result<(), String> {
    let payload: FriendshipActivatedPayload = serde_json::from_str(commit.payload.as_str())
        .map_err(|error| format!("invalid friendship.activated payload: {error}"))?;
    txn.execute(
        FRIENDSHIP_UPSERT_ACTIVE_PAIR_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &social_entity_id_to_i64(payload.friendship_id.as_str()),
            &payload.user_low_id,
            &payload.user_high_id,
            &payload.initiator_user_id,
            &"active".to_string(),
            &Some(payload.established_at.clone()),
            &payload.established_at,
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
    let updated = txn
        .execute(
            FRIENDSHIP_UPDATE_STATUS_SQL,
            &[
                &commit.tenant_id,
                &commit.organization_id,
                &social_entity_id_to_i64(payload.friendship_id.as_str()),
                &"removed".to_string(),
                &payload.removed_at,
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
    txn.execute(
        USER_BLOCK_INSERT_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &social_entity_id_to_i64(payload.block_id.as_str()),
            &payload.blocker_user_id,
            &payload.blocked_user_id,
            &payload.scope,
            &payload
                .direct_chat_id
                .as_deref()
                .map(social_entity_id_to_i64),
            &None::<String>,
            &payload.expires_at,
            &payload.effective_at,
            &payload.effective_at,
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
            &social_entity_id_to_i64(payload.block_id.as_str()),
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
    txn.execute(
        DIRECT_CHAT_INSERT_SQL,
        &[
            &commit.tenant_id,
            &commit.organization_id,
            &social_entity_id_to_i64(payload.direct_chat_id.as_str()),
            &"user".to_string(),
            &payload.left_actor_id,
            &"user".to_string(),
            &payload.right_actor_id,
            &payload.pair_hash,
            &"active".to_string(),
            &Some(payload.conversation_id.clone()),
            &payload.bound_at,
            &payload.bound_at,
        ],
    )
    .map_err(|error| format!("direct_chat insert failed: {error}"))
    .map(|_| ())
}