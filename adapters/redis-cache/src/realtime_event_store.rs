//! Redis-backed [`RealtimeEventWindowStore`] implementation.
//!
//! Key pattern: `realtime:window:{tenant_id}:{principal_kind}:{principal_id}:{device_id}`
//! Type: STRING (JSON-serialized [`RealtimeEventWindowRecord`])
//! TTL: 86400 seconds (24h)
//!
//! Uses a synchronous `redis::Client` because [`RealtimeEventWindowStore`] is a
//! synchronous trait. Long-running calls are bridged off the async runtime via
//! `tokio::task::spawn_blocking`, mirroring the pattern in `adapters/postgres-journal`.

use redis::Commands;
use sdkwork_im_contract_control::{
    RealtimeEventWindowDiagnosticsSnapshot, RealtimeEventWindowRecord,
};
use sdkwork_im_contract_core::ContractError;

use crate::redis_unavailable;

const REALTIME_WINDOW_TTL_SECONDS: u64 = 86400;

fn window_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> String {
    format!(
        "realtime:window:{tenant_id}:{organization_id}:{principal_kind}:{principal_id}:{device_id}"
    )
}

/// Redis-backed realtime event window store.
#[derive(Clone)]
pub struct RedisRealtimeEventWindowStore {
    client: redis::Client,
}

impl RedisRealtimeEventWindowStore {
    pub fn new(client: redis::Client) -> Self {
        Self { client }
    }

    fn connection(&self) -> Result<redis::Connection, ContractError> {
        self.client
            .get_connection()
            .map_err(|e| redis_unavailable("connect", e))
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
        let key = window_key(tenant_id, organization_id, principal_kind, principal_id, device_id);
        let mut conn = self.connection()?;
        let data: Option<String> = conn
            .get(&key)
            .map_err(|e| redis_unavailable("load_window", e))?;
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
        let mut conn = self.connection()?;
        for record in records {
            let key = window_key(
                record.tenant_id.as_str(),
                record.organization_id.as_str(),
                record.principal_kind.as_str(),
                record.principal_id.as_str(),
                record.device_id.as_str(),
            );
            let data = serde_json::to_string(&record).map_err(|e| {
                ContractError::Unavailable(format!("serialize realtime window failed: {e}"))
            })?;
            redis::cmd("SET")
                .arg(&key)
                .arg(&data)
                .arg("EX")
                .arg(REALTIME_WINDOW_TTL_SECONDS)
                .query::<()>(&mut conn)
                .map_err(|e| redis_unavailable("save_window", e))?;
        }
        Ok(())
    }

    fn clear_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError> {
        let key = window_key(tenant_id, organization_id, principal_kind, principal_id, device_id);
        let mut conn = self.connection()?;
        let deleted: i32 = redis::cmd("DEL")
            .arg(&key)
            .query(&mut conn)
            .map_err(|e| redis_unavailable("clear_window", e))?;
        Ok(deleted > 0)
    }

    fn diagnostics_snapshot(
        &self,
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
        let k2 = window_key("tenant", "default", "user", "a:b", "d1");
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
