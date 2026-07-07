//! Background scheduler that expires stale pending friend requests.

use std::sync::Arc;
use std::time::Duration;

use im_domain_core::social::FriendRequestStatus;
use im_domain_events::social::{
    FriendRequestExpiredPayload, SocialCommitEnvelopeInput, SocialEventType, social_commit_envelope,
};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_time::{rfc3339_add_secs, rfc3339_le, utc_now_rfc3339_millis};
use tokio::task::JoinHandle;
use tokio::time::{self, MissedTickBehavior};
use tracing::{info, warn};

use crate::runtime::SocialRuntime;

const SCHEDULER_ENABLED_ENV: &str = "SDKWORK_IM_FRIEND_REQUEST_EXPIRATION_SCHEDULER_ENABLED";
const INTERVAL_SECONDS_ENV: &str = "SDKWORK_IM_FRIEND_REQUEST_EXPIRATION_INTERVAL_SECONDS";
const TTL_SECONDS_ENV: &str = "SDKWORK_IM_FRIEND_REQUEST_TTL_SECONDS";
const DEFAULT_INTERVAL_SECONDS: u64 = 300;
const MIN_INTERVAL_SECONDS: u64 = 60;
const MAX_INTERVAL_SECONDS: u64 = 3_600;
const DEFAULT_TTL_SECONDS: i64 = 7 * 86_400;
const MIN_TTL_SECONDS: i64 = 3_600;
const MAX_TTL_SECONDS: i64 = 90 * 86_400;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FriendRequestExpirationSchedulerConfig {
    pub interval: Duration,
}

impl FriendRequestExpirationSchedulerConfig {
    pub fn from_env() -> Option<Self> {
        if !scheduler_enabled_from_env() {
            return None;
        }
        Some(Self {
            interval: Duration::from_secs(read_u64_env(
                INTERVAL_SECONDS_ENV,
                DEFAULT_INTERVAL_SECONDS,
                MIN_INTERVAL_SECONDS,
                MAX_INTERVAL_SECONDS,
            )),
        })
    }
}

pub fn friend_request_ttl_seconds_from_env() -> i64 {
    std::env::var(TTL_SECONDS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<i64>().ok())
        .map(|value| value.clamp(MIN_TTL_SECONDS, MAX_TTL_SECONDS))
        .unwrap_or(DEFAULT_TTL_SECONDS)
}

pub fn resolve_friend_request_expires_at(requested_at: &str) -> Option<String> {
    let ttl_secs = friend_request_ttl_seconds_from_env();
    if ttl_secs <= 0 {
        return None;
    }
    rfc3339_add_secs(requested_at, ttl_secs)
}

pub fn friend_request_is_expired(expired_at: Option<&str>, created_at: &str) -> bool {
    friend_request_is_due_for_expiration(expired_at, created_at, utc_now_rfc3339_millis().as_str())
}

pub fn friend_request_is_due_for_expiration(
    expired_at: Option<&str>,
    created_at: &str,
    now: &str,
) -> bool {
    if let Some(expired_at) = expired_at.filter(|value| !value.trim().is_empty()) {
        return rfc3339_le(expired_at, now);
    }
    resolve_friend_request_expires_at(created_at)
        .is_some_and(|deadline| rfc3339_le(deadline.as_str(), now))
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FriendRequestExpirationTickResult {
    pub expired: usize,
}

pub fn spawn_friend_request_expiration_scheduler_from_env(
    runtime: Arc<SocialRuntime>,
) -> Option<JoinHandle<()>> {
    let config = FriendRequestExpirationSchedulerConfig::from_env()?;
    if runtime
        .friend_request_expiration_scheduler_started
        .swap(true, std::sync::atomic::Ordering::AcqRel)
    {
        warn!(
            target: "sdkwork.im",
            event = "im.friend_request.expiration.scheduler_duplicate",
            "friend request expiration scheduler already started"
        );
        return None;
    }
    Some(spawn_friend_request_expiration_scheduler(runtime, config))
}

pub fn spawn_friend_request_expiration_scheduler(
    runtime: Arc<SocialRuntime>,
    config: FriendRequestExpirationSchedulerConfig,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut ticker = time::interval(config.interval);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);
        loop {
            match runtime.expire_due_friend_requests_persisted() {
                Ok(result) => {
                    if result.expired > 0 {
                        info!(
                            target: "sdkwork.im",
                            event = "im.friend_request.expiration.scheduler_tick",
                            expired = result.expired,
                            "expired stale pending friend requests"
                        );
                    }
                }
                Err(error) => {
                    warn!(
                        target: "sdkwork.im",
                        event = "im.friend_request.expiration.scheduler_tick_failed",
                        error = %error,
                        "friend request expiration scheduler tick failed"
                    );
                }
            }
            ticker.tick().await;
        }
    })
}

impl SocialRuntime {
    pub fn expire_due_friend_requests_persisted(
        &self,
    ) -> Result<FriendRequestExpirationTickResult, String> {
        let now = utc_now_rfc3339_millis();
        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);

        const FRIEND_REQUEST_EXPIRATION_BATCH_LIMIT: usize = 1_024;
        let due_request_ids = state
            .friend_requests
            .values()
            .filter(|record| {
                matches!(record.friend_request.status, FriendRequestStatus::Pending)
                    && friend_request_is_due_for_expiration(
                        record.friend_request.expired_at.as_deref(),
                        record.friend_request.created_at.as_str(),
                        now.as_str(),
                    )
            })
            .map(|record| record.friend_request.request_id.clone())
            .take(FRIEND_REQUEST_EXPIRATION_BATCH_LIMIT)
            .collect::<Vec<_>>();

        if due_request_ids.is_empty() {
            return Ok(FriendRequestExpirationTickResult { expired: 0 });
        }

        let mut next_state = state.clone();
        let mut commits = Vec::with_capacity(due_request_ids.len());
        for request_id in due_request_ids {
            let Some(stored) = next_state.friend_requests.get(request_id.as_str()).cloned() else {
                continue;
            };
            if !matches!(stored.friend_request.status, FriendRequestStatus::Pending) {
                continue;
            }
            if !friend_request_is_due_for_expiration(
                stored.friend_request.expired_at.as_deref(),
                stored.friend_request.created_at.as_str(),
                now.as_str(),
            ) {
                continue;
            }

            let organization_id = organization_id_from_friend_request_commits(&stored.commits);
            let payload = FriendRequestExpiredPayload {
                request_id: request_id.clone(),
                requester_user_id: stored.friend_request.requester_user_id.clone(),
                target_user_id: stored.friend_request.target_user_id.clone(),
                expired_at: now.clone(),
            };
            let payload_json = serde_json::to_string(&payload).map_err(|error| {
                format!("serialize friend request expiration payload failed: {error}")
            })?;
            let event_id = format!("evt_fr_expire_{request_id}_{}", now.replace(':', ""));
            let commit = social_commit_envelope(SocialCommitEnvelopeInput {
                event_id: event_id.as_str(),
                tenant_id: stored.friend_request.tenant_id.as_str(),
                organization_id: organization_id.as_str(),
                aggregate_type: AggregateType::FriendRequest,
                aggregate_id: request_id.as_str(),
                event_type: SocialEventType::FriendRequestExpired,
                ordering_seq: stored.commits.len() as u64 + 1,
                actor: EventActor {
                    actor_id: "social-friend-request-expiration".into(),
                    actor_kind: "system".into(),
                    actor_session_id: None,
                },
                occurred_at: now.as_str(),
                committed_at: now.as_str(),
                payload: payload_json.as_str(),
            });

            let mut record = stored;
            record.friend_request.status = FriendRequestStatus::Expired;
            record.friend_request.updated_at = now.clone();
            record.friend_request.expired_at = Some(now.clone());
            record.commits.push(commit.clone());
            next_state.insert_friend_request_record(request_id, record);
            commits.push(commit);
        }

        if commits.is_empty() {
            return Ok(FriendRequestExpirationTickResult { expired: 0 });
        }

        let expired = commits.len();
        self.persist_state_transition_batch(&next_state, commits.as_slice())?;
        *state = next_state;
        Ok(FriendRequestExpirationTickResult { expired })
    }
}

fn organization_id_from_friend_request_commits(commits: &[CommitEnvelope]) -> String {
    commits
        .first()
        .map(|commit| commit.organization_id.clone())
        .unwrap_or_else(|| "0".into())
}

fn scheduler_enabled_from_env() -> bool {
    !matches!(
        std::env::var(SCHEDULER_ENABLED_ENV)
            .ok()
            .map(|value| value.trim().to_ascii_lowercase())
            .as_deref(),
        Some("0") | Some("false") | Some("off") | Some("no")
    )
}

fn read_u64_env(name: &str, default: u64, min: u64, max: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .map(|value| value.clamp(min, max))
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_friend_request_expires_at_adds_default_ttl() {
        let expires = resolve_friend_request_expires_at("2026-01-01T00:00:00.000Z")
            .expect("expires_at should be computed");
        assert!(expires.as_str() > "2026-01-01T00:00:00.000Z");
    }

    #[test]
    fn friend_request_is_due_when_deadline_passed() {
        assert!(friend_request_is_due_for_expiration(
            Some("2026-01-01T00:00:00.000Z"),
            "2025-12-25T00:00:00.000Z",
            "2026-01-02T00:00:00.000Z",
        ));
        assert!(!friend_request_is_due_for_expiration(
            Some("2026-02-01T00:00:00.000Z"),
            "2026-01-01T00:00:00.000Z",
            "2026-01-02T00:00:00.000Z",
        ));
    }
}
