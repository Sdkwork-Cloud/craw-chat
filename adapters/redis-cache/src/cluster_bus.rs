//! Redis Pub/Sub cluster event bus for cross-node realtime event delivery.
//!
//! Each node subscribes to `cluster:route:{node_id}` and publishes
//! route events for remote nodes to the target node's channel.
//!
//! ## Channel layout
//! - Publish: `cluster:route:{target_node_id}` → JSON payload
//! - Subscribe: `cluster:route:{own_node_id}` → receive JSON payload

use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};

use crate::redis_blocking::{RedisBlockingTimeouts, run_bounded_redis_command};
use crate::redis_unavailable;

/// A route event published across the cluster bus.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClusterRouteEvent {
    pub tenant_id: String,
    pub principal_id: String,
    pub principal_kind: String,
    pub device_id: String,
    pub scope_type: String,
    pub scope_id: String,
    pub event_type: String,
    pub payload: String,
}

fn route_channel(node_id: &str) -> String {
    format!("cluster:route:{node_id}")
}

/// Redis-backed cluster event bus for publishing route events to remote
/// nodes and subscribing to events targeted at the local node.
#[derive(Clone)]
pub struct RedisClusterBus {
    client: redis::Client,
    own_node_id: String,
    timeouts: RedisBlockingTimeouts,
}

impl RedisClusterBus {
    pub fn new(client: redis::Client, own_node_id: impl Into<String>) -> Self {
        Self {
            client,
            own_node_id: own_node_id.into(),
            timeouts: RedisBlockingTimeouts::from_env(),
        }
    }

    /// Publish a route event to a target node's channel.
    pub fn publish_route_event(
        &self,
        target_node_id: &str,
        event: &ClusterRouteEvent,
    ) -> Result<(), ContractError> {
        let channel = route_channel(target_node_id);
        let payload = serde_json::to_string(event).map_err(|e| {
            ContractError::Unavailable(format!("serialize cluster route event failed: {e}"))
        })?;
        run_bounded_redis_command(
            &self.client,
            self.timeouts,
            "publish_route_event",
            move |mut connection| async move {
                redis::cmd("PUBLISH")
                    .arg(channel)
                    .arg(payload)
                    .query_async::<i32>(&mut connection)
                    .await
                    .map(|_| ())
            },
        )
    }

    /// Get the channel name for the local node's subscription.
    pub fn own_channel(&self) -> String {
        route_channel(&self.own_node_id)
    }

    pub async fn subscribe_async(&self) -> Result<redis::aio::PubSub, ContractError> {
        let mut pubsub = tokio::time::timeout(
            self.timeouts.connect_timeout(),
            self.client.get_async_pubsub(),
        )
        .await
        .map_err(|_| ContractError::Unavailable("subscribe_route_events connect timed out".into()))?
        .map_err(|error| redis_unavailable("subscribe_route_events_connect", error))?;
        tokio::time::timeout(
            self.timeouts.command_timeout(),
            pubsub.subscribe(self.own_channel()),
        )
        .await
        .map_err(|_| ContractError::Unavailable("subscribe_route_events timed out".into()))?
        .map_err(|error| redis_unavailable("subscribe_route_events", error))?;
        Ok(pubsub)
    }

    /// Get the own node ID.
    pub fn own_node_id(&self) -> &str {
        &self.own_node_id
    }
}

impl im_platform_contracts::ClusterEventBus for RedisClusterBus {
    fn publish_route_event(&self, target_node_id: &str, event_json: &str) -> Result<(), String> {
        let channel = route_channel(target_node_id);
        let event_json = event_json.to_owned();
        run_bounded_redis_command(
            &self.client,
            self.timeouts,
            "cluster_bus_publish",
            move |mut connection| async move {
                redis::cmd("PUBLISH")
                    .arg(channel)
                    .arg(event_json)
                    .query_async::<i32>(&mut connection)
                    .await
                    .map(|_| ())
            },
        )
        .map_err(|error| format!("redis cluster_bus publish to {target_node_id} failed: {error:?}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_channel_contains_node_id() {
        let channel = route_channel("node-1");
        assert!(channel.contains("node-1"));
        assert!(channel.starts_with("cluster:route:"));
    }

    #[test]
    fn test_route_channel_is_unique_per_node() {
        assert_ne!(route_channel("node-a"), route_channel("node-b"));
    }

    #[test]
    fn test_own_channel_matches_own_node_id() {
        let bus = RedisClusterBus {
            client: redis::Client::open("redis://localhost:6379").unwrap(),
            own_node_id: "node-x".into(),
            timeouts: RedisBlockingTimeouts::from_env(),
        };
        assert_eq!(bus.own_channel(), "cluster:route:node-x");
    }

    #[test]
    fn test_cluster_route_event_serialization_roundtrip() {
        let event = ClusterRouteEvent {
            tenant_id: "t1".into(),
            principal_id: "u1".into(),
            principal_kind: "user".into(),
            device_id: "d1".into(),
            scope_type: "conversation".into(),
            scope_id: "c1".into(),
            event_type: "message.new".into(),
            payload: r#"{"text":"hello"}"#.into(),
        };
        let json = serde_json::to_string(&event).unwrap();
        let restored: ClusterRouteEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.tenant_id, "t1");
        assert_eq!(restored.device_id, "d1");
    }
}
