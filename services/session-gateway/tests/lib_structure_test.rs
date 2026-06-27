#[test]
fn test_session_gateway_lib_rs_stays_below_step02_redline() {
    let line_count = include_str!("../src/lib.rs").lines().count();

    assert!(
        line_count <= 1000,
        "services/session-gateway/src/lib.rs must stay below 1000 lines for Step 02, found {line_count}"
    );
}

#[test]
fn test_session_gateway_cluster_disconnect_surface_moves_out_of_cluster_impl() {
    let cluster_source = include_str!("../src/cluster.rs");

    for forbidden_symbol in [
        "struct RealtimeDisconnectFence {",
        "struct ClusterMemoryDisconnectFenceStore {",
        "pub fn mark_client_route_disconnected(",
        "pub fn clear_client_route_disconnect_fence(",
        "pub fn ensure_client_route_resume_not_required(",
        "pub fn disconnect_fence_matches_client_route_session(",
        "fn load_disconnect_fence(",
        "fn disconnect_fence_store_error(",
    ] {
        assert!(
            !cluster_source.contains(forbidden_symbol),
            "services/session-gateway/src/cluster.rs should not keep disconnect-fence symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_realtime_cluster_rejects_implicit_user_identity_surfaces() {
    let cluster_source = include_str!("../src/cluster.rs").replace("\r\n", "\n");
    let disconnect_source = include_str!("../src/cluster/disconnect.rs")
        .replace("\r\n", "\n")
        .split("#[cfg(test)]")
        .next()
        .expect("disconnect source should contain production module")
        .to_owned();

    for forbidden_symbol in [
        "pub fn bind_client_route(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
        "pub fn resolve_client_route(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
        "pub fn release_client_route(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
        "pub fn ensure_route_session_current(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
        "pub fn ensure_client_route_local(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
        "pub fn publish_client_route_event(\n        &self,\n        origin_node_id: &str,\n        tenant_id: &str,\n        principal_id: &str,",
    ] {
        assert!(
            !cluster_source.contains(forbidden_symbol),
            "RealtimeClusterBridge must require explicit principal_kind and reject legacy implicit-user route API: {forbidden_symbol}"
        );
    }

    for forbidden_symbol in [
        "pub fn mark_client_route_disconnected(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
        "pub fn clear_client_route_disconnect_fence(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
        "pub fn ensure_client_route_resume_not_required(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
        "pub fn disconnect_fence_matches_client_route_session(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
    ] {
        assert!(
            !disconnect_source.contains(forbidden_symbol),
            "disconnect fence runtime must require explicit principal_kind and reject implicit user/default identity: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_disconnect_fence_token_uses_segment_safe_encoding() {
    let disconnect_source = include_str!("../src/cluster/disconnect.rs")
        .replace("\r\n", "\n")
        .split("#[cfg(test)]")
        .next()
        .expect("disconnect source should contain production module")
        .to_owned();

    for forbidden_symbol in [
        "\"fence:{tenant_id}:",
        "session_id.unwrap_or(\"sessionless\")",
        "session_id.unwrap_or(\"\")",
        "format!(\n        \"fence:",
    ] {
        assert!(
            !disconnect_source.contains(forbidden_symbol),
            "disconnect fence token must use segment-safe encoding instead of delimiter/default sentinels: {forbidden_symbol}"
        );
    }

    assert!(
        disconnect_source.contains("encode_disconnect_fence_token_segments("),
        "disconnect fence token should be built with the shared segment-safe token encoder"
    );
    assert!(
        disconnect_source.contains("Some(session_id) => (\"some-session\", session_id)")
            && disconnect_source.contains("None => (\"no-session\", \"\")"),
        "disconnect fence token must encode Some/None session state as an explicit segment"
    );
}

#[test]
fn test_session_gateway_realtime_storage_surface_moves_out_of_realtime_impl() {
    let realtime_source = include_str!("../src/realtime.rs");

    for forbidden_symbol in [
        "struct RuntimeMemoryCheckpointStore {",
        "struct RuntimeMemorySubscriptionStore {",
        "fn persist_checkpoint(",
        "fn persist_subscriptions(",
        "fn checkpoint_record(",
        "fn subscription_record(",
    ] {
        assert!(
            !realtime_source.contains(forbidden_symbol),
            "services/session-gateway/src/realtime.rs should not keep realtime storage symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_realtime_control_contracts_use_explicit_principal_kind() {
    let contract_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/sdkwork-im-contract-control/src/lib.rs"),
    )
    .expect("sdkwork-im-contract-control source should exist")
    .replace("\r\n", "\n");

    for required_symbol in [
        "pub struct RealtimeCheckpointRecord {\n    pub tenant_id: String,\n    #[serde(default = \"default_organization_id\")]\n    pub organization_id: String,\n    pub principal_kind: String,\n    pub principal_id: String,",
        "pub struct RealtimeDisconnectFenceRecord {\n    pub tenant_id: String,\n    #[serde(default = \"default_organization_id\")]\n    pub organization_id: String,\n    pub principal_kind: String,\n    pub principal_id: String,",
        "pub struct RealtimeSubscriptionRecord {\n    pub tenant_id: String,\n    #[serde(default = \"default_organization_id\")]\n    pub organization_id: String,\n    pub principal_kind: String,\n    pub principal_id: String,",
        "pub struct PresenceStateRecord {\n    pub tenant_id: String,\n    #[serde(default = \"default_organization_id\")]\n    pub organization_id: String,\n    pub principal_kind: String,\n    pub principal_id: String,",
        "fn load_checkpoint(\n        &self,\n        tenant_id: &str,\n        organization_id: &str,\n        principal_kind: &str,\n        principal_id: &str,",
        "fn save_checkpoints(&self, records: Vec<RealtimeCheckpointRecord>)",
        "fn load_fence(\n        &self,\n        tenant_id: &str,\n        organization_id: &str,\n        principal_kind: &str,\n        principal_id: &str,",
        "fn load_subscriptions(\n        &self,\n        tenant_id: &str,\n        organization_id: &str,\n        principal_kind: &str,\n        principal_id: &str,",
        "fn load_state(\n        &self,\n        tenant_id: &str,\n        organization_id: &str,\n        principal_kind: &str,\n        principal_id: &str,",
        "fn list_states_for_principal(\n        &self,\n        tenant_id: &str,\n        organization_id: &str,\n        principal_kind: &str,\n        principal_id: &str,",
    ] {
        assert!(
            contract_source.contains(required_symbol),
            "realtime/presence control contract must expose explicit principal_kind: {required_symbol}"
        );
    }
}

#[test]
fn test_realtime_subscription_store_requires_durable_fanout_query_implementation() {
    let contract_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/sdkwork-im-contract-control/src/lib.rs"),
    )
    .expect("sdkwork-im-contract-control source should exist")
    .replace("\r\n", "\n");
    let trait_source = contract_source
        .split("pub trait RealtimeSubscriptionStore")
        .nth(1)
        .expect("RealtimeSubscriptionStore trait should exist")
        .split("pub trait PresenceStateStore")
        .next()
        .expect("RealtimeSubscriptionStore trait should precede PresenceStateStore");

    assert!(
        trait_source.contains("fn load_matching_subscriptions("),
        "RealtimeSubscriptionStore must expose a durable scope/event fanout query"
    );
    assert!(
        !trait_source.contains("for device_id in candidate_device_ids"),
        "RealtimeSubscriptionStore must not provide an N-device default implementation for fanout queries"
    );
}

#[test]
fn test_session_gateway_realtime_runtime_requires_explicit_principal_kind() {
    let realtime_source = include_str!("../src/realtime.rs").replace("\r\n", "\n");
    let realtime_storage_source = include_str!("../src/realtime/storage.rs").replace("\r\n", "\n");

    for forbidden_symbol in [
        "actor_client_route_scope_key",
        "principal_kind: Option<&str>",
        "principal_kind.unwrap_or(\"user\")",
        "fn client_route_scope_key(\n    tenant_id: &str,\n    principal_id: &str,\n    principal_kind: Option<&str>,",
        "pub fn ensure_client_route_state(",
        "pub fn subscribe_device(",
        "pub fn subscribe_disconnect_signal(",
        "pub fn disconnect_generation(",
        "pub fn signal_device_disconnect(",
        "pub fn window_checkpoint(",
        "pub fn sync_subscriptions(",
        "pub fn clear_client_route_subscriptions(",
        "pub fn list_events(",
        "pub fn ack_events(",
        "pub fn take_client_route_state(",
        "pub fn publish_scope_event(",
        "restore_client_route_state_for_principal_kind(",
    ] {
        assert!(
            !realtime_source.contains(forbidden_symbol),
            "services/session-gateway/src/realtime.rs must require explicit principal_kind and avoid legacy realtime identity surface: {forbidden_symbol}"
        );
    }

    for forbidden_symbol in [
        "principal_kind: Option<&str>",
        "principal_kind.unwrap_or(\"user\")",
        "Some(principal_kind)",
        "Some(record.principal_kind.as_str())",
    ] {
        assert!(
            !realtime_storage_source.contains(forbidden_symbol),
            "services/session-gateway/src/realtime/storage.rs must persist realtime state with required principal_kind: {forbidden_symbol}"
        );
    }

    assert!(
        realtime_source.contains(
            "pub struct RealtimeClientRouteStateSnapshot {\n    pub tenant_id: String,\n    pub organization_id: String,\n    pub principal_kind: String,\n    pub principal_id: String,"
        ),
        "RealtimeClientRouteStateSnapshot must carry tenant, organization, and principal_kind so route migration cannot restore into an implicit default identity"
    );
    assert!(
        realtime_source.contains("pub disconnect_generation: u64,"),
        "RealtimeClientRouteStateSnapshot must carry disconnect_generation so runtime migration preserves websocket disconnect signal epochs"
    );
}

#[test]
fn test_session_gateway_realtime_window_store_uses_sequence_index() {
    let realtime_source = include_str!("../src/realtime.rs").replace("\r\n", "\n");

    assert!(
        !realtime_source.contains("windows: Arc<Mutex<HashMap<String, Vec<RealtimeEvent>>>>"),
        "realtime delivery windows must not store client route events in a Vec; cursor reads need a sequence index"
    );
    assert!(
        realtime_source
            .contains("windows: Arc<Mutex<HashMap<String, BTreeMap<u64, RealtimeEvent>>>>"),
        "realtime delivery windows should use BTreeMap<u64, RealtimeEvent> per client route scope"
    );
    assert!(
        realtime_source.contains("let effective_after_seq = after_seq.max(trimmed_through_seq);"),
        "realtime list_events should clamp the read cursor to the trimmed boundary"
    );
    assert!(
        realtime_source.contains(".range((Excluded(effective_after_seq), Unbounded))"),
        "realtime list_events should range-seek from the effective cursor"
    );
}

#[test]
fn test_session_gateway_realtime_subscription_store_uses_scope_index() {
    let realtime_source = include_str!("../src/realtime.rs").replace("\r\n", "\n");

    assert!(
        realtime_source.contains("fn realtime_subscription_scope_key("),
        "realtime subscription duplicate detection should use a centralized segment-safe scope key"
    );
    assert!(
        realtime_source.contains("encode_realtime_key_segments([scope_type, scope_id])"),
        "realtime subscription scope keys should encode type/id boundaries explicitly"
    );
    assert!(
        !realtime_source.contains("format!(\"{}:{}\", item.scope_type, item.scope_id)"),
        "realtime subscription duplicate detection must not collapse delimiter-shaped scope segments"
    );
    assert!(
        !realtime_source
            .contains("subscriptions: Arc<Mutex<HashMap<String, Vec<RealtimeSubscription>>>>"),
        "realtime subscriptions must not store each client route's subscriptions as a Vec; fanout needs scope lookup"
    );
    assert!(
        realtime_source.contains(
            "subscriptions: Arc<Mutex<HashMap<String, RealtimeClientRouteSubscriptions>>>"
        ),
        "realtime subscriptions should use an indexed per-client-route subscription store"
    );
    assert!(
        realtime_source
            .contains("by_scope: HashMap<RealtimeSubscriptionScopeKey, RealtimeSubscription>"),
        "per-client-route realtime subscriptions should index by scope type/id"
    );
    assert!(
        realtime_source.contains("fn subscription_matches_event("),
        "realtime fanout should evaluate event type filters from indexed subscription records"
    );
    assert!(
        realtime_source.contains("candidate_subscriptions\n        .into_iter()"),
        "realtime fanout should iterate scope-index candidates instead of scanning per-client-route subscriptions"
    );
}

#[test]
fn test_session_gateway_realtime_fanout_uses_scope_client_route_index() {
    let realtime_source = include_str!("../src/realtime.rs").replace("\r\n", "\n");

    assert!(
        realtime_source.contains(
            "subscription_scope_index:\n        Arc<Mutex<HashMap<RealtimePrincipalScopeKey, BTreeMap<String, RealtimeSubscription>>>>,"
        ),
        "realtime runtime should keep a scope -> client route index so publish fanout avoids probing every registered client route"
    );
    assert!(
        realtime_source.contains("fn index_client_route_subscriptions("),
        "realtime runtime should centralize subscription scope index maintenance"
    );
    assert!(
        realtime_source.contains("fn remove_device_subscription_index("),
        "realtime runtime should remove stale scope-index entries when subscriptions clear, move, or restore"
    );
    assert!(
        realtime_source
            .contains("subscription_scope_index\n        .get(&RealtimePrincipalScopeKey::new("),
        "realtime publish should read candidate client routes from the scope index"
    );
    let publish_source = realtime_source
        .split("fn publish_scope_event_internal(")
        .nth(1)
        .expect("realtime runtime should keep publish_scope_event_internal")
        .split("fn index_client_route_subscriptions(")
        .next()
        .expect("publish implementation should precede subscription index helpers");
    assert!(
        publish_source.contains(".load_matching_subscriptions("),
        "realtime publish should use the durable subscription store's scope/event fanout query before restoring cold devices"
    );
    assert!(
        publish_source
            .matches("collect_matched_delivery_targets(")
            .count()
            >= 2,
        "realtime publish should re-read the scope fanout index after restoring durable matching devices"
    );
    assert!(
        publish_source.contains("unmatched_registered_client_routes"),
        "realtime publish should only ask durable storage for client routes missing from the hot fanout index"
    );
    assert!(
        !publish_source.contains(
            "for device_id in &registered_client_routes {\n            self.ensure_client_route_state_internal("
        ),
        "realtime publish must not restore every registered client route before checking durable scope/event matches"
    );
    assert!(
        !realtime_source.contains("subscriptions: &HashMap<String, RealtimeClientRouteSubscriptions>,\n    tenant_id: &str,\n    principal_id: &str,\n    principal_kind: &str,"),
        "collect_matched_delivery_targets must not require the full subscription map once a scope fanout index exists"
    );
    assert!(
        !realtime_source.contains("registered_client_routes\n        .into_iter()\n        .collect::<BTreeSet<_>>()\n        .into_iter()\n        .filter_map(|device_id|"),
        "realtime publish must not iterate every registered client route to discover subscriptions"
    );
}

#[test]
fn test_cluster_delivery_result_separates_route_state_from_runtime_error() {
    let cluster_source = include_str!("../src/cluster.rs").replace("\r\n", "\n");
    assert!(
        cluster_source.contains("pub delivery_error_code: Option<String>,"),
        "RealtimeRouteDeliveryResult should expose runtime delivery errors separately from route_state"
    );
    assert!(
        cluster_source.contains("pub delivery_error_message: Option<String>,"),
        "RealtimeRouteDeliveryResult should preserve runtime delivery error details for diagnostics"
    );
    assert!(
        cluster_source.contains("route_state: route_state.to_string(),"),
        "publish results should keep route resolution state even when runtime delivery fails"
    );
    assert!(
        !cluster_source.contains("Err(error) => (error.code.to_string(), 0)"),
        "runtime delivery errors must not overwrite route_state or collapse into delivered=0"
    );
}

#[test]
fn test_session_gateway_presence_memory_store_uses_principal_index() {
    let presence_source = include_str!("../src/presence.rs").replace("\r\n", "\n");

    assert!(
        presence_source.contains("by_principal: HashMap<String, BTreeSet<String>>"),
        "presence memory state store should maintain a tenant/principal -> client-route-key index"
    );
    assert!(
        presence_source.contains("by_device: HashMap<String, PresenceStateRecord>"),
        "presence memory state store should keep client route records in the same indexed state object"
    );
    assert!(
        presence_source.contains("online_by_seen_at: BTreeSet<PresenceOnlineSeenAtKey>"),
        "presence memory state store should maintain an online last-seen index for lease expiration"
    );
    assert!(
        presence_source.contains("fn list_online_states_seen_at_or_before("),
        "presence state store should expose indexed stale-online listing for expiration jobs"
    );
    assert!(
        !presence_source.contains(".values()\n            .filter(|record| record.tenant_id == tenant_id && record.principal_id == principal_id)"),
        "presence memory state store must not full-scan all client route records for principal snapshots"
    );
}

#[test]
fn test_session_gateway_websocket_upgrade_transport_seam_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");

    for forbidden_symbol in [
        "fn realtime_websocket_subprotocols()",
        "fn select_realtime_websocket_mode(",
        "fn map_runtime_link_websocket_mode(",
        "fn prepare_realtime_websocket_upgrade(",
        "async fn serve_realtime_websocket_upgrade(",
        "let ws = ws.protocols(realtime_websocket_subprotocols());",
        ".on_upgrade(move |socket| upgrade.execute(socket, serve_realtime_websocket_upgrade))",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "services/session-gateway/src/lib.rs should not keep websocket upgrade seam symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_websocket_route_handler_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");
    let upgrade_source = include_str!("../src/websocket_upgrade.rs").replace("\r\n", "\n");

    for forbidden_symbol in [
        "use axum::extract::ws::WebSocketUpgrade;",
        "async fn realtime_websocket(",
        "websocket_upgrade::upgrade_realtime_websocket(",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "services/session-gateway/src/lib.rs should not keep websocket route handler symbol: {forbidden_symbol}"
        );
    }

    assert!(
        upgrade_source.contains("pub(crate) async fn realtime_websocket("),
        "services/session-gateway/src/websocket_upgrade.rs should host realtime websocket route handler"
    );
}

#[test]
fn test_session_gateway_websocket_pending_backlog_math_moves_out_of_service_impl() {
    let websocket_source = include_str!("../src/websocket.rs");

    assert!(
        !websocket_source.contains("fn outbound_pending_events("),
        "services/session-gateway/src/websocket.rs should not keep local backlog math helper"
    );
}

#[test]
fn test_session_gateway_websocket_outbound_queue_state_moves_out_of_service_impl() {
    let websocket_source = include_str!("../src/websocket.rs");

    for forbidden_symbol in [
        "let mut last_sent_seq =",
        "let mut push_cursor =",
        "push_cursor: &mut LinkPushCursor",
        "last_sent_seq: &mut u64",
    ] {
        assert!(
            !websocket_source.contains(forbidden_symbol),
            "services/session-gateway/src/websocket.rs should not keep local outbound queue state symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_websocket_buffered_push_drain_loop_moves_out_of_service_impl() {
    let websocket_source = include_str!("../src/websocket.rs");

    assert!(
        !websocket_source.contains("async fn flush_buffered_push_windows("),
        "services/session-gateway/src/websocket.rs should not keep the buffered push drain loop"
    );
}

#[test]
fn test_session_gateway_websocket_upgrade_module_stays_pure_axum_adapter() {
    let upgrade_source = include_str!("../src/websocket_upgrade.rs");
    let route_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/websocket_route.rs"),
    )
    .expect("services/session-gateway/src/websocket_route.rs should exist");

    for forbidden_symbol in [
        "resolve_app_context(&headers)?",
        "resolve_requested_device_id(",
    ] {
        assert!(
            !upgrade_source.contains(forbidden_symbol),
            "services/session-gateway/src/websocket_upgrade.rs should keep only the Axum transport adapter, found: {forbidden_symbol}"
        );
    }

    for required_symbol in [
        "pub(crate) struct RealtimeWebsocketRouteContext",
        "pub(crate) async fn prepare_realtime_websocket_route(",
        "resolve_request_app_context(",
        "resolve_requested_device_id(&auth, query_device_id)?",
        "state.prepare_active_client_route(",
    ] {
        assert!(
            route_source.contains(required_symbol),
            "services/session-gateway/src/websocket_route.rs should host websocket route preflight symbol: {required_symbol}"
        );
    }

    assert!(
        !route_source.contains("WebSocketUpgrade"),
        "services/session-gateway/src/websocket_route.rs should not own the Axum WebSocketUpgrade adapter"
    );
    assert!(
        upgrade_source.contains("use axum::extract::WebSocketUpgrade;"),
        "services/session-gateway/src/websocket_upgrade.rs should remain the only Axum WebSocketUpgrade adapter seam"
    );
    assert!(
        upgrade_source.contains("pub(crate) async fn realtime_websocket("),
        "services/session-gateway/src/websocket_upgrade.rs should still host the Axum route entrypoint"
    );
    assert!(
        upgrade_source.contains("websocket_route::prepare_realtime_websocket_route("),
        "services/session-gateway/src/websocket_upgrade.rs should delegate route preflight into websocket_route"
    );
    for required_symbol in [
        "ws.on_upgrade(move |socket| {",
        "upgrade.execute(socket, move |socket, context, mode| {",
    ] {
        assert!(
            upgrade_source.contains(required_symbol),
            "services/session-gateway/src/websocket_upgrade.rs should keep the Axum on_upgrade seam: {required_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_presence_surface_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");
    let presence_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/presence_routes.rs"),
    )
    .expect("services/session-gateway/src/presence_routes.rs should exist");

    for forbidden_symbol in ["async fn get_presence_me(", "async fn heartbeat_presence("] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "services/session-gateway/src/lib.rs should not keep presence handler symbol: {forbidden_symbol}"
        );
    }

    for required_symbol in [
        "pub(crate) async fn get_presence_me(",
        "pub(crate) async fn heartbeat_presence(",
    ] {
        assert!(
            presence_source.contains(required_symbol),
            "services/session-gateway/src/presence_routes.rs should host presence handler symbol: {required_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_presence_paths_use_client_route_state_snapshot_owner_seam() {
    let presence_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/presence_routes.rs"),
    )
    .expect("services/session-gateway/src/presence_routes.rs should exist");

    for required_symbol in [
        "fn client_route_state_snapshot(",
        "state.client_route_state_snapshot(",
    ] {
        assert!(
            presence_source.contains(required_symbol),
            "services/session-gateway/src/presence_routes.rs should consume the shared route sync-state owner seam: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        "state.registered_route_keys(auth.tenant_id.as_str(), auth.actor_id.as_str())",
        "state.latest_route_sync_seq(",
    ] {
        assert!(
            !presence_source.contains(forbidden_symbol),
            "services/session-gateway/src/presence_routes.rs should not keep raw route sync-state reads once the owner seam exists: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_sync_state_owner_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs").replace("\r\n", "\n");
    let owner_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/client_route_state.rs"),
    )
    .expect("services/session-gateway/src/client_route_state.rs should exist")
    .replace("\r\n", "\n");

    for forbidden_symbol in [
        "registered_route_keys: Arc<Mutex<HashMap<String, BTreeSet<String>>>>",
        "latest_sync_sequences: Arc<Mutex<HashMap<String, u64>>>",
        "fn registered_route_keys(&self, tenant_id: &str, principal_id: &str) -> Vec<String> {",
        "fn latest_route_sync_seq(&self, tenant_id: &str, principal_id: &str, device_id: &str) -> u64 {",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "services/session-gateway/src/lib.rs should not keep raw session sync-state owner detail: {forbidden_symbol}"
        );
    }

    for required_symbol in [
        "mod client_route_state;",
        "client_route_state: ClientRouteState,",
        "self.client_route_state\n            .client_route_state_snapshot(",
    ] {
        assert!(
            lib_source.contains(required_symbol),
            "services/session-gateway/src/lib.rs should delegate session sync-state ownership through ClientRouteState: {required_symbol}"
        );
    }

    for required_symbol in [
        "pub(crate) struct ClientRouteState",
        "pub(crate) fn ensure_route_key_available(",
        "pub(crate) fn register_route_key(",
        "pub(crate) fn client_route_state_snapshot(",
        "fn registered_route_keys(\n        &self,\n        tenant_id: &str,\n        organization_id: &str,\n        principal_id: &str,\n        principal_kind: &str,\n    ) -> Vec<String> {",
        "fn latest_route_sync_seq(",
    ] {
        assert!(
            owner_source.contains(required_symbol),
            "services/session-gateway/src/client_route_state.rs should host session sync-state owner implementation: {required_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_client_route_registration_owner_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs").replace("\r\n", "\n");
    let owner_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/client_route_registration.rs"),
    )
    .expect("services/session-gateway/src/client_route_registration.rs should exist")
    .replace("\r\n", "\n");

    for forbidden_symbol in [
        "self.presence_runtime\n            .register_client_route(",
        "self.realtime_runtime\n            .ensure_client_route_state(",
        "self.client_route_state\n            .register_route_key(",
        "self.realtime_cluster.bind_client_route(",
        "self.realtime_cluster.clear_client_route_disconnect_fence(",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "services/session-gateway/src/lib.rs should not keep legacy route registration owner detail: {forbidden_symbol}"
        );
    }

    for required_symbol in [
        "mod client_route_registration;",
        "client_route_registration: ClientRouteRegistration,",
        "self.client_route_registration.prepare_active_client_route(",
    ] {
        assert!(
            lib_source.contains(required_symbol),
            "services/session-gateway/src/lib.rs should delegate client route registration ownership through ClientRouteRegistration: {required_symbol}"
        );
    }

    for required_symbol in [
        "pub(crate) struct ClientRouteRegistration",
        "pub(crate) fn new(",
        "pub(crate) fn register_client_route(",
        "pub(crate) fn prepare_active_client_route(",
        "self.presence_runtime\n            .register_client_route(",
        "self.realtime_runtime\n            .ensure_client_route_state_for_principal_kind(",
        "self.client_route_state.register_route_key(",
        "self.realtime_cluster.bind_client_route_for_principal_kind(",
    ] {
        assert!(
            owner_source.contains(required_symbol),
            "services/session-gateway/src/client_route_registration.rs should host client route registration owner implementation: {required_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_route_preflight_owner_moves_out_of_entrypoints() {
    let lib_source = include_str!("../src/lib.rs");
    let presence_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/presence_routes.rs"),
    )
    .expect("services/session-gateway/src/presence_routes.rs should exist");
    let realtime_http_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/realtime_http_routes.rs"),
    )
    .expect("services/session-gateway/src/realtime_http_routes.rs should exist");
    let route_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/websocket_route.rs"),
    )
    .expect("services/session-gateway/src/websocket_route.rs should exist");
    let owner_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/client_route_registration.rs"),
    )
    .expect("services/session-gateway/src/client_route_registration.rs should exist");

    {
        let forbidden_symbol = "state.realtime_cluster.ensure_route_session_current(";
        for (path, source) in [
            ("lib.rs", lib_source),
            ("presence_routes.rs", presence_source.as_str()),
            ("realtime_http_routes.rs", realtime_http_source.as_str()),
            ("websocket_route.rs", route_source.as_str()),
        ] {
            assert!(
                !source.contains(forbidden_symbol),
                "services/session-gateway/src/{path} should not keep raw route preflight glue once the owner seam exists: {forbidden_symbol}"
            );
        }
    }

    assert!(
        !route_source.contains("state.register_client_route("),
        "services/session-gateway/src/websocket_route.rs should delegate route preflight instead of calling register_client_route directly"
    );

    for required_symbol in [
        "fn prepare_active_client_route(",
    ] {
        assert!(
            lib_source.contains(required_symbol),
            "services/session-gateway/src/lib.rs should expose the route preflight owner seam: {required_symbol}"
        );
    }

    {
        let required_symbol = "state.prepare_active_client_route(";
        for (path, source) in [
            ("presence_routes.rs", presence_source.as_str()),
            ("realtime_http_routes.rs", realtime_http_source.as_str()),
            ("websocket_route.rs", route_source.as_str()),
        ] {
            assert!(
                source.contains(required_symbol),
                "services/session-gateway/src/{path} should consume the shared route preflight owner seam: {required_symbol}"
            );
        }
    }

    for required_symbol in [
        "pub(crate) fn prepare_active_client_route(",
        "fn ensure_route_session_current(",
        "self.ensure_route_session_current(",
        "self.register_client_route(",
    ] {
        assert!(
            owner_source.contains(required_symbol),
            "services/session-gateway/src/client_route_registration.rs should host route preflight owner detail: {required_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_disconnect_lifecycle_is_not_exposed_by_presence_entrypoints() {
    let lib_source = include_str!("../src/lib.rs").replace("\r\n", "\n");
    let presence_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/presence_routes.rs"),
    )
    .expect("services/session-gateway/src/presence_routes.rs should exist");
    let owner_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/client_route_registration.rs"),
    )
    .expect("services/session-gateway/src/client_route_registration.rs should exist");

    for forbidden_symbol in [
        "state.realtime_cluster.disconnect_fence_matches_client_route_session(",
        "state.realtime_runtime.clear_client_route_subscriptions(",
        "state.realtime_cluster.release_client_route(",
        "state.realtime_cluster.mark_client_route_disconnected(",
    ] {
        assert!(
            !presence_source.contains(forbidden_symbol),
            "services/session-gateway/src/presence_routes.rs should not keep raw disconnect lifecycle glue once the owner seam exists: {forbidden_symbol}"
        );
    }

    for required_symbol in [
        "fn release_active_client_route_if_current_session(",
        "self.client_route_registration\n            .release_active_client_route_if_current_session(",
    ] {
        assert!(
            lib_source.contains(required_symbol),
            "services/session-gateway/src/lib.rs should expose the disconnect lifecycle owner seam: {required_symbol}"
        );
    }

    assert!(
        !presence_source.contains("state.disconnect_active_client_route("),
        "services/session-gateway/src/presence_routes.rs must not expose retired HTTP disconnect behavior"
    );

    for required_symbol in [
        "pub(crate) fn release_active_client_route_if_current_session(",
        "ensure_route_session_current(",
        "release_client_route_for_principal_kind(",
    ] {
        assert!(
            owner_source.contains(required_symbol),
            "services/session-gateway/src/client_route_registration.rs should host disconnect lifecycle owner detail: {required_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_source_does_not_keep_legacy_device_route_symbols() {
    let source_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let forbidden_symbols = [
        "RealtimeDeviceRoute",
        "RealtimeDeviceStateSnapshot",
        "RealtimeDeviceSubscriptions",
        "DeviceRoute",
        "device_route",
        "bind_device",
        "release_device",
        "ensure_device_state",
        "ensure_device_resume",
        "register_device(",
        "mark_device_disconnected",
        "clear_device_disconnect",
        "typed_device_scope_key",
        "tenant_device_scope_key",
    ];

    for entry in
        std::fs::read_dir(&source_dir).expect("services/session-gateway/src should be readable")
    {
        let entry = entry.expect("session-gateway src entry should be readable");
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("rs") {
            continue;
        }
        let source =
            std::fs::read_to_string(&path).expect("session-gateway Rust source should read");
        for forbidden_symbol in forbidden_symbols {
            assert!(
                !source.contains(forbidden_symbol),
                "{} must use client_route naming instead of legacy route symbol: {forbidden_symbol}",
                path.display()
            );
        }
    }
}

use std::path::PathBuf;
