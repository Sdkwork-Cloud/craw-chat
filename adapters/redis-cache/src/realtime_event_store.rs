//! Redis-backed [`RealtimeEventWindowStore`] implementation.
//!
//! Key pattern: `realtime:window:{length-prefixed tenant/org/principal/device scope}`
//! Type: STRING (JSON-serialized [`RealtimeEventWindowRecord`])
//! TTL: 86400 seconds (24h)
//!
//! Uses a synchronous `redis::Client` because [`RealtimeEventWindowStore`] is a
//! synchronous trait. Long-running calls are bridged off the async runtime via
//! `tokio::task::spawn_blocking`, mirroring the pattern in `adapters/postgres-journal`.

use sdkwork_im_contract_control::{
    RealtimeDiagnosticsRequest, RealtimeEventWindowDiagnosticsSnapshot, RealtimeEventWindowRecord,
};
use sdkwork_im_contract_core::ContractError;

use crate::redis_blocking::{RedisBlockingTimeouts, run_bounded_redis_command};
use crate::redis_key::encode_redis_key_segments;

const REALTIME_WINDOW_TTL_SECONDS: u64 = 86400;
const REALTIME_WINDOW_KEY_PREFIX: &str = "realtime:window:";

fn window_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> String {
    format!(
        "{REALTIME_WINDOW_KEY_PREFIX}{}",
        encode_redis_key_segments([
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        ])
    )
}

/// Redis-backed realtime event window store.
#[derive(Clone)]
pub struct RedisRealtimeEventWindowStore {
    client: redis::Client,
    timeouts: RedisBlockingTimeouts,
}

impl RedisRealtimeEventWindowStore {
    pub fn new(client: redis::Client) -> Self {
        Self {
            client,
            timeouts: RedisBlockingTimeouts::from_env(),
        }
    }
}

impl sdkwork_im_contract_control::RealtimeEventWindowStore for RedisRealtimeEventWindowStore {
    fn load_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeEventWindowRecord>, ContractError> {
        let key = window_key(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        );
        let data: Option<String> = run_bounded_redis_command(
            &self.client,
            self.timeouts,
            "load_window",
            move |mut connection| async move {
                redis::cmd("GET")
                    .arg(key)
                    .query_async(&mut connection)
                    .await
            },
        )?;
        match data {
            Some(json) => {
                let record: RealtimeEventWindowRecord =
                    serde_json::from_str(&json).map_err(|e| {
                        ContractError::Unavailable(format!(
                            "deserialize realtime window failed: {e}"
                        ))
                    })?;
                Ok(Some(record))
            }
            None => Ok(None),
        }
    }

    fn save_windows(&self, records: Vec<RealtimeEventWindowRecord>) -> Result<(), ContractError> {
        run_bounded_redis_command(
            &self.client,
            self.timeouts,
            "save_window",
            move |mut connection| async move {
                for record in records {
                    let key = window_key(
                        record.tenant_id.as_str(),
                        record.organization_id.as_str(),
                        record.principal_kind.as_str(),
                        record.principal_id.as_str(),
                        record.device_id.as_str(),
                    );
                    let data = serde_json::to_string(&record).map_err(|e| {
                        redis::RedisError::from((
                            redis::ErrorKind::TypeError,
                            "serialize realtime window failed",
                            e.to_string(),
                        ))
                    })?;
                    redis::cmd("SET")
                        .arg(&key)
                        .arg(&data)
                        .arg("EX")
                        .arg(REALTIME_WINDOW_TTL_SECONDS)
                        .query_async::<()>(&mut connection)
                        .await?;
                }
                Ok(())
            },
        )
    }

    fn clear_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        let key = window_key(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        );
        let deleted: i32 = run_bounded_redis_command(
            &self.client,
            self.timeouts,
            "clear_window",
            move |mut connection| async move {
                redis::cmd("DEL")
                    .arg(key)
                    .query_async(&mut connection)
                    .await
            },
        )?;
        Ok(deleted > 0)
    }

    fn diagnostics_snapshot(
        &self,
        _request: RealtimeDiagnosticsRequest<'_>,
    ) -> Result<RealtimeEventWindowDiagnosticsSnapshot, ContractError> {
        // Redis STRING store cannot efficiently scan all keys for diagnostics.
        // The in-memory runtime layer provides authoritative diagnostics.
        Ok(RealtimeEventWindowDiagnosticsSnapshot {
            client_route_window_count: 0,
            pending_event_count: 0,
            max_client_route_window_event_count: 0,
            max_trimmed_through_seq: 0,
            capacity_trimmed_event_count: 0,
            max_capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            oldest_pending_occurred_at: None,
            high_risk_windows: Vec::new(),
        })
    }

    fn trim_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        _acked_through_seq: u64,
    ) -> Result<(), ContractError> {
        // For STRING store, trim is a no-op: the entire window is replaced
        // on each save. The caller is responsible for only saving events
        // above the acked-through watermark.
        let _ = (
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_key_is_segment_safe() {
        let k1 = window_key("tenant:a", "default", "user", "b", "d1");
        let k2 = window_key("tenant", "a:default", "user", "b", "d1");
        assert_ne!(k1, k2, "segment-safe keys must not collide");
    }

    #[test]
    fn test_window_key_contains_all_identity_segments() {
        let key = window_key("t1", "default", "user", "u1", "d1");
        assert!(key.contains("t1"));
        assert!(key.contains("user"));
        assert!(key.contains("u1"));
        assert!(key.contains("d1"));
    }
}
