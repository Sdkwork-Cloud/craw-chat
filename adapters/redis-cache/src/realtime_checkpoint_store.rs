//! Redis-backed [`RealtimeCheckpointStore`] implementation.
//!
//! Key pattern: `realtime:checkpoint:{length-prefixed tenant/org/principal/device scope}`
//! Type: HASH (each checkpoint field is a hash field)
//! TTL: 86400 seconds (24h)

use sdkwork_im_contract_control::RealtimeCheckpointRecord;
use sdkwork_im_contract_core::ContractError;

use crate::redis_blocking::{RedisBlockingTimeouts, run_bounded_redis_command};
use crate::redis_key::encode_redis_key_segments;

const REALTIME_CHECKPOINT_TTL_SECONDS: u64 = 86400;
const REALTIME_CHECKPOINT_KEY_PREFIX: &str = "realtime:checkpoint:";

fn checkpoint_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> String {
    format!(
        "{REALTIME_CHECKPOINT_KEY_PREFIX}{}",
        encode_redis_key_segments([
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        ])
    )
}

/// Redis-backed realtime checkpoint store.
#[derive(Clone)]
pub struct RedisRealtimeCheckpointStore {
    client: redis::Client,
    timeouts: RedisBlockingTimeouts,
}

impl RedisRealtimeCheckpointStore {
    pub fn new(client: redis::Client) -> Self {
        Self {
            client,
            timeouts: RedisBlockingTimeouts::from_env(),
        }
    }
}

impl sdkwork_im_contract_control::RealtimeCheckpointStore for RedisRealtimeCheckpointStore {
    fn load_checkpoint(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        let key = checkpoint_key(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        );
        let fields: Vec<String> = run_bounded_redis_command(
            &self.client,
            self.timeouts,
            "load_checkpoint",
            move |mut connection| async move {
                redis::cmd("HGETALL")
                    .arg(key)
                    .query_async(&mut connection)
                    .await
            },
        )?;
        if fields.is_empty() {
            return Ok(None);
        }
        let record = parse_checkpoint_fields(
            &fields,
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            device_id,
        )?;
        Ok(Some(record))
    }

    fn save_checkpoints(
        &self,
        records: Vec<RealtimeCheckpointRecord>,
    ) -> Result<(), ContractError> {
        run_bounded_redis_command(
            &self.client,
            self.timeouts,
            "save_checkpoint",
            move |mut connection| async move {
                for record in records {
                    let key = checkpoint_key(
                        record.tenant_id.as_str(),
                        record.organization_id.as_str(),
                        record.principal_kind.as_str(),
                        record.principal_id.as_str(),
                        record.device_id.as_str(),
                    );
                    let fields: &[(&str, String)] = &[
                        ("tenant_id", record.tenant_id.clone()),
                        ("organization_id", record.organization_id.clone()),
                        ("principal_kind", record.principal_kind.clone()),
                        ("principal_id", record.principal_id.clone()),
                        ("device_id", record.device_id.clone()),
                        (
                            "latest_realtime_seq",
                            record.latest_realtime_seq.to_string(),
                        ),
                        ("acked_through_seq", record.acked_through_seq.to_string()),
                        (
                            "trimmed_through_seq",
                            record.trimmed_through_seq.to_string(),
                        ),
                        (
                            "capacity_trimmed_event_count",
                            record.capacity_trimmed_event_count.to_string(),
                        ),
                        (
                            "capacity_trimmed_through_seq",
                            record.capacity_trimmed_through_seq.to_string(),
                        ),
                        (
                            "last_capacity_trimmed_at",
                            record.last_capacity_trimmed_at.clone().unwrap_or_default(),
                        ),
                        ("updated_at", record.updated_at.clone()),
                    ];
                    redis::cmd("HSET")
                        .arg(&key)
                        .arg(fields)
                        .query_async::<()>(&mut connection)
                        .await?;
                    redis::cmd("EXPIRE")
                        .arg(&key)
                        .arg(REALTIME_CHECKPOINT_TTL_SECONDS)
                        .query_async::<()>(&mut connection)
                        .await?;
                }
                Ok(())
            },
        )
    }
}

fn parse_checkpoint_fields(
    fields: &[String],
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> Result<RealtimeCheckpointRecord, ContractError> {
    // fields are alternating key-value pairs: [k1, v1, k2, v2, ...]
    let mut map = std::collections::HashMap::new();
    for chunk in fields.chunks(2) {
        if chunk.len() == 2 {
            map.insert(chunk[0].clone(), chunk[1].clone());
        }
    }
    let get_u64 = |name: &str| -> Result<u64, ContractError> {
        map.get(name)
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);
        Ok(map
            .get(name)
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0))
    };
    Ok(RealtimeCheckpointRecord {
        tenant_id: tenant_id.to_owned(),
        organization_id: organization_id.to_owned(),
        principal_kind: principal_kind.to_owned(),
        principal_id: principal_id.to_owned(),
        device_id: device_id.to_owned(),
        latest_realtime_seq: get_u64("latest_realtime_seq")?,
        acked_through_seq: get_u64("acked_through_seq")?,
        trimmed_through_seq: get_u64("trimmed_through_seq")?,
        capacity_trimmed_event_count: get_u64("capacity_trimmed_event_count")?,
        capacity_trimmed_through_seq: get_u64("capacity_trimmed_through_seq")?,
        last_capacity_trimmed_at: map.get("last_capacity_trimmed_at").cloned(),
        updated_at: map.get("updated_at").cloned().unwrap_or_default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkpoint_key_is_segment_safe() {
        let k1 = checkpoint_key("tenant:a", "default", "user", "b", "d1");
        let k2 = checkpoint_key("tenant", "a:default", "user", "b", "d1");
        assert_ne!(k1, k2, "segment-safe keys must not collide");
    }

    #[test]
    fn test_checkpoint_key_contains_all_identity_segments() {
        let key = checkpoint_key("t1", "default", "user", "u1", "d1");
        assert!(key.contains("t1"));
        assert!(key.contains("user"));
        assert!(key.contains("u1"));
        assert!(key.contains("d1"));
    }
}
