//! Redis Pub/Sub cluster event bus for cross-node realtime event delivery.
//!
//! Each node subscribes to `cluster:route:{node_id}` and publishes
//! route events for remote nodes to the target node's channel.
//!
//! ## Channel layout
//! - Publish: `cluster:route:{target_node_id}` → JSON payload
//! - Subscribe: `cluster:route:{own_node_id}` → receive JSON payload

use redis::{Commands, PubSubCommands};
use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};

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
}

impl RedisClusterBus {
    pub fn new(client: redis::Client, own_node_id: impl Into<String>) -> Self {
        Self {
            client,
            own_node_id: own_node_id.into(),
        }
    }

    fn connection(&self) -> Result<redis::Connection, ContractError> {
        self.client
            .get_connection()
            .map_err(|e| redis_unavailable("cluster_bus_connect", e))
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
        let mut conn = self.connection()?;
        let _: i32 = conn
            .publish(&channel, &payload)
            .map_err(|e| redis_unavailable("publish_route_event", e))?;
        Ok(())
    }

    /// Get the channel name for the local node's subscription.
    pub fn own_channel(&self) -> String {
        route_channel(&self.own_node_id)
    }

    /// Get a pubsub connection for subscribing to the local node's channel.
    /// The caller must provide a message handler callback that returns
    /// `redis::ControlFlow` to control the subscription loop.
    pub fn subscribe_connection<F, U>(&self, handler: F) -> Result<U, ContractError>
    where
        F: FnMut(redis::Msg) -> redis::ControlFlow<U> + Send,
        U: Send,
    {
        let mut conn = self.connection()?;
        conn.subscribe(&[self.own_channel().as_str()], handler)
            .map_err(|e| redis_unavailable("subscribe_route_events", e))
    }

    /// Get the own node ID.
    pub fn own_node_id(&self) -> &str {
        &self.own_node_id
    }
}

impl im_platform_contracts::ClusterEventBus for RedisClusterBus {
    fn publish_route_event(&self, target_node_id: &str, event_json: &str) -> Result<(), String> {
        let channel = route_channel(target_node_id);
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| format!("redis cluster_bus publish failed: {e}"))?;
        let _: i32 = conn
            .publish(&channel, event_json)
            .map_err(|e| format!("redis cluster_bus publish to {target_node_id} failed: {e}"))?;
        Ok(())
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
