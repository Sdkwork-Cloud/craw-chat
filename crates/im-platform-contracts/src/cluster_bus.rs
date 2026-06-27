//! Cluster event bus contract for cross-node realtime event delivery.
//!
//! When a client route is bound to a remote session-gateway node, the
//! owning node publishes the event via the bus. The target node
//! subscribes and delivers locally.

/// Cross-node event bus for delivering realtime route events to remote
/// session-gateway nodes.
pub trait ClusterEventBus: Send + Sync {
    /// Publish a route event to the target node's channel.
    ///
    /// `event_json` is a JSON-encoded string containing the route event
    /// payload including tenant, principal, device, scope, and event
    /// identifiers.
    fn publish_route_event(&self, target_node_id: &str, event_json: &str) -> Result<(), String>;
}
