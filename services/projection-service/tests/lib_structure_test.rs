#[test]
fn test_projection_service_lib_rs_stays_below_step02_redline() {
    let line_count = include_str!("../src/lib.rs").lines().count();

    assert!(
        line_count <= 1000,
        "services/projection-service/src/lib.rs must stay below 1000 lines for Step 02, found {line_count}"
    );
}

#[test]
fn test_projection_service_http_surface_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");

    for forbidden_symbol in [
        "struct RegisterDeviceRequest {",
        "struct SyncFeedQuery {",
        "struct ListQuery {",
        "struct TimelineResponse {",
        "struct InboxResponse {",
        "struct ClientRouteSyncFeedResponse {",
        "pub struct ProjectionApiError {",
        "pub fn build_default_app(",
        "pub fn build_public_app(",
        "pub fn build_public_app_with_service(",
        "pub fn build_app(",
        "async fn require_public_bearer_auth(",
        "async fn healthz(",
        "async fn readyz(",
        "async fn register_client_route(",
        "async fn get_client_route_sync_feed(",
        "async fn get_timeline(",
        "async fn get_inbox(",
        "async fn get_conversation_summary(",
        "async fn get_read_cursor(",
        "fn resolve_requested_device_id(",
        "fn validate_device_scope(",
        "fn ensure_conversation_member_access(",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "services/projection-service/src/lib.rs should not keep http surface symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_projection_service_handoff_projection_surface_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");

    for forbidden_symbol in [
        "fn handoff_view_from_created_payload(",
        "fn handoff_view_from_state_payload(",
        "fn projection_actor_to_view(",
        "fn latest_summary_activity_at(",
        "pub enum ProjectionError {",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "services/projection-service/src/lib.rs should not keep projection helper symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_projection_service_access_module_exposes_auth_context_entrypoints() {
    let access_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/access.rs"))
            .expect("services/projection-service/src/access.rs should exist");

    for required_symbol in [
        "pub struct ProjectionAccessError {",
        "pub struct ClientRouteSyncStateSnapshot {",
        "pub fn ensure_active_member_from_auth_context(",
        "pub fn active_conversation_principal_recipients_from_auth_context(",
        "pub fn message_posted_notification_recipients_from_auth_context(",
        "pub fn register_client_route_from_auth_context(",
        "pub fn ensure_client_route_registration_allowed_from_auth_context(",
        "pub fn registered_client_routes_from_auth_context(",
        "pub fn client_route_sync_state_snapshot_from_auth_context(",
        "pub fn realtime_fanout_targets_for_recipients_from_auth_context(",
        "pub fn latest_client_route_sync_seq_from_auth_context(",
        "pub fn client_route_sync_feed_window_from_auth_context(",
        "pub fn inbox_from_auth_context(",
        "pub fn contacts_from_auth_context(",
        "pub fn timeline_window_from_auth_context(",
        "pub fn conversation_summary_from_auth_context(",
        "pub fn message_interaction_summary_from_auth_context(",
        "pub fn pinned_messages_from_auth_context(",
        "pub fn read_cursor_from_auth_context(",
    ] {
        assert!(
            access_source.contains(required_symbol),
            "projection-service access module should own principal-context authority wrapper: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        "pub fn client_route_sync_feed_from_auth_context(",
        "pub fn timeline_from_auth_context(",
    ] {
        assert!(
            !access_source.contains(forbidden_symbol),
            "projection-service access module must not expose unbounded principal-context read entrypoint: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_projection_service_exposes_realtime_fanout_target_owner_seam() {
    let lib_source = include_str!("../src/lib.rs");

    assert!(
        lib_source.contains("pub fn client_route_sync_fanout_targets_for_conversation("),
        "services/projection-service/src/lib.rs should expose a projection-owned realtime fanout target seam for typed recipient-to-device resolution"
    );
}

#[test]
fn test_projection_service_principal_identity_queries_are_strictly_typed() {
    let lib_source = include_str!("../src/lib.rs");
    let inbox_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inbox.rs"))
            .expect("services/projection-service/src/inbox.rs should exist");
    let member_store_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/member_store.rs"))
            .expect("services/projection-service/src/member_store.rs should exist");
    let access_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/access.rs"))
            .expect("services/projection-service/src/access.rs should exist");

    for forbidden_symbol in [
        "pub fn inbox(&self, tenant_id: &str, principal_id: &str)",
        "principal_kind: Option<&str>",
        "active_member_scopes_for_principal(",
        "conversation_members_by_principal",
        "fn member_principal_index_key(",
        "pub fn read_cursor(\n        &self,\n        tenant_id: &str,\n        conversation_id: &str,\n        principal_id: &str,",
        "pub fn member_snapshot(\n        &self,\n        tenant_id: &str,\n        conversation_id: &str,\n        principal_id: &str,",
        "pub fn is_active_member(\n        &self,\n        tenant_id: &str,\n        conversation_id: &str,\n        principal_id: &str,",
        "pub fn realtime_fanout_targets_for_principals(",
        "pub fn active_conversation_principal_ids_from_auth_context(",
        "pub fn realtime_fanout_targets_from_auth_context(",
        "pub(crate) fn active_conversation_principal_ids(",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol)
                && !inbox_source.contains(forbidden_symbol)
                && !member_store_source.contains(forbidden_symbol)
                && !access_source.contains(forbidden_symbol),
            "projection-service principal identity query path must include required principal_kind: {forbidden_symbol}"
        );
    }

    for required_symbol in [
        "pub fn inbox_for_principal_kind(",
        "pub fn read_cursor_for_principal_kind(",
        "pub fn member_snapshot_for_principal_kind(",
        "pub fn is_active_member_for_principal_kind(",
    ] {
        assert!(
            lib_source.contains(required_symbol) || inbox_source.contains(required_symbol),
            "projection-service should expose typed principal query API: {required_symbol}"
        );
    }
}

#[test]
fn test_projection_service_exposes_conversation_client_route_sync_target_owner_seam() {
    let lib_source = include_str!("../src/lib.rs");

    assert!(
        lib_source.contains("pub fn client_route_sync_fanout_targets_for_conversation("),
        "services/projection-service/src/lib.rs should expose a projection-owned conversation device-sync target seam for active member plus fallback principal resolution"
    );
    assert!(
        lib_source
            .matches(".client_route_sync_fanout_targets_for_conversation(")
            .count()
            >= 4,
        "services/projection-service/src/lib.rs should route conversation-scoped device-sync fanout callers through the shared projection owner seam"
    );
    assert!(
        lib_source
            .matches("for device in self.registered_client_routes(")
            .count()
            <= 1,
        "services/projection-service/src/lib.rs should not duplicate raw principal-to-device device-sync fanout loops across multiple projection handlers"
    );
}

#[test]
fn test_projection_service_http_surface_uses_auth_context_entrypoints() {
    let http_source = include_str!("../src/http.rs");

    for required_symbol in [
        ".inbox_window_from_auth_context(",
        ".contact_window_from_auth_context(",
        ".timeline_window_from_auth_context(",
        ".conversation_summary_from_auth_context(",
        ".message_interaction_summary_from_auth_context(",
        ".pinned_messages_from_auth_context(",
        ".read_cursor_from_auth_context(",
    ] {
        assert!(
            http_source.contains(required_symbol),
            "projection-service http surface should consume projection principal-context entrypoint: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        "/im/v3/api/devices/register",
        "/im/v3/api/devices/{device_id}/sync_feed",
        "async fn register_client_route(",
        "async fn get_client_route_sync_feed(",
        ".register_client_route_from_auth_context(",
        ".client_route_sync_feed_window_from_auth_context(",
        ".register_client_route(\n        auth.tenant_id.as_str(),",
        ".client_route_sync_feed(\n            auth.tenant_id.as_str(),",
        ".inbox(auth.tenant_id.as_str(), auth.actor_id.as_str())",
        ".timeline(auth.tenant_id.as_str(), \"default\", conversation_id.as_str())",
        ".conversation_summary(auth.tenant_id.as_str(), \"default\", conversation_id.as_str())",
        ".read_cursor(\n            auth.tenant_id.as_str(),",
        "ensure_conversation_member_access(",
        "resolve_requested_device_id(",
        "validate_device_scope(",
    ] {
        assert!(
            !http_source.contains(forbidden_symbol),
            "projection-service http surface should not keep raw auth authority capture outside access module: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_projection_service_client_route_sync_entry_assembly_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");
    let client_route_sync_source = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/client_route_sync.rs"
    ))
    .expect("services/projection-service/src/client_route_sync.rs should exist");

    for required_symbol in [
        "pub(crate) struct ClientRouteSyncEntryDraft {",
        "impl ClientRouteSyncEntryDraft {",
        "pub(crate) fn build_for_target(",
    ] {
        assert!(
            client_route_sync_source.contains(required_symbol),
            "projection-service device-sync entry owner module should expose shared assembly symbol: {required_symbol}"
        );
    }

    assert_eq!(
        lib_source.matches("ClientRouteSyncFeedEntry {").count(),
        0,
        "services/projection-service/src/lib.rs should not keep inline device-sync entry assembly after the shared owner seam is extracted"
    );
    assert!(
        lib_source
            .matches(".append_client_route_sync_draft(")
            .count()
            >= 5,
        "services/projection-service/src/lib.rs should route message, mutation, read_cursor, handoff, and member-governance sync fanout through the shared device-sync draft owner seam"
    );
}

#[test]
fn test_projection_service_client_route_sync_feed_store_uses_sequence_index() {
    let lib_source = include_str!("../src/lib.rs");
    let model_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/model.rs"))
            .expect("services/projection-service/src/model.rs should exist");
    let client_route_sync_source = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/client_route_sync.rs"
    ))
    .expect("services/projection-service/src/client_route_sync.rs should exist");

    assert!(
        lib_source
            .contains("HashMap<ClientRouteFeedScopeKey, BTreeMap<u64, ClientRouteSyncFeedEntry>>"),
        "projection-service device-sync feed store must be keyed by sync_seq so cursor reads can seek instead of scanning an unindexed Vec"
    );
    assert!(
        !lib_source.contains("HashMap<ClientRouteFeedScopeKey, Vec<ClientRouteSyncFeedEntry>>"),
        "projection-service device-sync feed store must not keep per-device Vec feeds"
    );
    assert!(
        client_route_sync_source.contains(".range((Excluded(min_seq), Unbounded))"),
        "projection-service device-sync feed windows should seek by sync_seq range"
    );
    for forbidden_symbol in [
        "pub fn client_route_sync_feed(",
        "pub fn client_route_sync_feed_for_principal_kind(",
        "fn client_route_sync_feed_for_principal_kind(",
    ] {
        assert!(
            !client_route_sync_source.contains(forbidden_symbol),
            "projection-service must not expose unbounded device-sync feed helper: {forbidden_symbol}"
        );
    }
    assert!(
        !client_route_sync_source.contains(".iter().filter(|entry| entry.sync_seq > min_seq)"),
        "projection-service device-sync feed windows should not scan every entry after min_seq with a Vec iterator"
    );
    assert!(
        lib_source.contains("PROJECTION_CLIENT_ROUTE_SYNC_FEED_MAX_RETAINED_EVENTS"),
        "projection-service device-sync feed cache must define a bounded retention contract"
    );
    assert!(
        model_source.contains("pub trimmed_through_seq: u64,"),
        "projection-service device-sync feed windows must expose trimmed_through_seq so clients can detect expired cursors"
    );
    assert!(
        client_route_sync_source.contains(".pop_first()"),
        "projection-service device-sync feed append path must trim the oldest indexed entries when retention is exceeded"
    );
}

#[test]
fn test_projection_service_client_route_sync_identity_is_strictly_typed() {
    let model_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/model.rs"))
            .expect("services/projection-service/src/model.rs should exist");
    let scope_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/scope.rs"))
            .expect("services/projection-service/src/scope.rs should exist");
    let snapshot_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/snapshot.rs"))
            .expect("services/projection-service/src/snapshot.rs should exist");
    let client_route_sync_source = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/client_route_sync.rs"
    ))
    .expect("services/projection-service/src/client_route_sync.rs should exist");

    for forbidden_symbol in [
        "pub principal_kind: Option<String>,",
        "principal_kind: Option<&str>,",
        "principal_kind: principal_kind.map(str::to_owned)",
        "principal_kind.as_deref()",
        "client_route_principal_scope_key(tenant_id, principal_id, None)",
        "client_route_feed_scope_key(tenant_id, principal_id, None",
    ] {
        assert!(
            !model_source.contains(forbidden_symbol)
                && !scope_source.contains(forbidden_symbol)
                && !snapshot_source.contains(forbidden_symbol)
                && !client_route_sync_source.contains(forbidden_symbol),
            "projection-service client route sync identity must not keep optional principalKind path: {forbidden_symbol}"
        );
    }

    assert!(
        model_source.contains("pub principal_kind: String,"),
        "RegisteredClientRouteView and RealtimeFanoutTarget must carry a required principal_kind"
    );
    assert!(
        scope_source.contains("pub(super) organization_id: String,"),
        "device principal/feed cache keys must include organization_id"
    );
    assert!(
        scope_source.contains("organization_id: &str,")
            && scope_source.contains("principal_kind: &str,")
            && scope_source.contains("principal_id: &str,")
            && scope_source
                .find("organization_id: &str,")
                .unwrap()
                < scope_source.find("principal_kind: &str,").unwrap()
            && scope_source
                .find("principal_kind: &str,")
                .unwrap()
                < scope_source.find("principal_id: &str,").unwrap(),
        "client route scope keys must use organization_id and principal_kind before principal_id"
    );
    assert!(
        model_source.contains("pub organization_id: String,"),
        "RegisteredClientRouteView must carry organization_id"
    );
}

#[test]
fn test_projection_service_timeline_store_uses_sequence_index() {
    let lib_source = include_str!("../src/lib.rs");

    assert!(
        lib_source.contains("entries: Mutex<HashMap<String, BTreeMap<u64, TimelineViewEntry>>>"),
        "projection-service timeline store must be keyed by message_seq so cursor reads can seek instead of scanning a Vec"
    );
    assert!(
        !lib_source.contains("entries: Mutex<HashMap<String, Vec<TimelineViewEntry>>>"),
        "projection-service timeline store must not keep per-conversation Vec entries"
    );
    assert!(
        lib_source.contains(".range((Excluded(after_seq), Unbounded))"),
        "projection-service timeline windows should seek by message_seq range"
    );
    assert!(
        !lib_source.contains("partition_point(|entry| entry.message_seq <= after_seq)"),
        "projection-service timeline windows should not binary-search a Vec now that message_seq is the map key"
    );
}

#[test]
fn test_projection_service_runtime_keys_use_segment_safe_encoding() {
    let lib_source = include_str!("../src/lib.rs");
    let scope_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/scope.rs"))
            .expect("services/projection-service/src/scope.rs should exist");
    let contacts_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/contacts.rs"))
            .expect("services/projection-service/src/contacts.rs should exist");
    let snapshot_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/snapshot.rs"))
            .expect("services/projection-service/src/snapshot.rs should exist");

    for required_symbol in [
        "pub(super) fn encode_projection_key_segments",
        "encode_projection_key_segments([",
        "organization_id: &str,",
        "pub(super) struct ContactOwnerScopeKey",
        "pub(super) fn contact_owner_scope_key",
        "organization_id: &str,",
        "owner_user_id: &str,",
    ] {
        assert!(
            scope_source.contains(required_symbol),
            "projection-service scope keys should use segment-safe key encoding: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        "format!(\"{tenant_id}:{conversation_id}\")",
        "format!(\"{tenant_id}:{principal_id}\")",
        "split_once(':')",
        "parse_contact_scope(",
        "format!(\"{tenant_id}:{conversation_id}\")",
        "format!(\"legacy:{conversation_id}\")",
        "format!(\"{PRINCIPAL_SNAPSHOT_SCOPE_PREFIX}:{tenant_id}:{principal_id}\")",
        "\"{PRINCIPAL_SNAPSHOT_SCOPE_PREFIX}:typed:{}:{}:{}\"",
        "\"{DEVICE_SYNC_SNAPSHOT_SCOPE_PREFIX}:typed:{}:{}:{}:{}\"",
        "format!(\"{}:{}\", snapshot.scope, snapshot.key)",
        "format!(\"{scope}:{key}\")",
        "format!(\"{}#{}#{}\", tenant_id.len(), tenant_id, timeline_scope)",
    ] {
        assert!(
            !scope_source.contains(forbidden_symbol)
                && !contacts_source.contains(forbidden_symbol)
                && !snapshot_source.contains(forbidden_symbol),
            "projection-service runtime/snapshot keys must not use delimiter-composed ids: {forbidden_symbol}"
        );
    }

    assert!(
        lib_source.contains("HashMap<ContactOwnerScopeKey, HashMap<String, ContactView>>"),
        "projection-service contact store should be keyed by typed owner scope, not a parseable string"
    );
    assert!(
        contacts_source.contains(
            "HashMap<ContactConversationIndexKey, ContactDirectChatBindingKey>"
        ),
        "projection-service direct-chat conversation index should be keyed by typed tenant/organization/conversation tuple"
    );
}

#[test]
fn test_projection_service_direct_chat_binding_store_uses_conversation_index() {
    let lib_source = include_str!("../src/lib.rs");
    let access_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/access.rs"))
            .expect("services/projection-service/src/access.rs should exist");
    let contacts_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/contacts.rs"))
            .expect("services/projection-service/src/contacts.rs should exist");

    assert!(
        lib_source.contains(
            "direct_chat_bindings: Mutex<contacts::ContactDirectChatBindingRuntimeStore>"
        ),
        "projection-service direct-chat bindings must use a typed runtime store with secondary indexes"
    );
    assert!(
        contacts_source.contains("organization_id: String,"),
        "projection-service contact conversation index must include organization_id"
    );
    assert!(
        contacts_source.contains(
            "direct_chat_id_by_conversation: HashMap<ContactConversationIndexKey, ContactDirectChatBindingKey>"
        ),
        "projection-service direct-chat binding runtime store must index tenant+organization+conversation to directChatId"
    );
    assert!(
        contacts_source.contains("pub(crate) fn get_by_conversation("),
        "projection-service direct-chat binding runtime store must expose indexed conversation lookup"
    );
    assert!(
        !access_source.contains(".values().find(|binding|")
            && !access_source.contains(".values()\n        .find(|binding|"),
        "projection-service access checks must not scan every direct-chat binding by conversation"
    );
}

#[test]
fn test_projection_service_member_store_uses_principal_inbox_index() {
    let lib_source = include_str!("../src/lib.rs");
    let member_store_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/member_store.rs"))
            .expect("services/projection-service/src/member_store.rs should exist");
    let inbox_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inbox.rs"))
            .expect("services/projection-service/src/inbox.rs should exist");
    let access_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/access.rs"))
            .expect("services/projection-service/src/access.rs should exist");
    let snapshot_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/snapshot.rs"))
            .expect("services/projection-service/src/snapshot.rs should exist");

    assert!(
        lib_source.contains("members: Mutex<ProjectionMemberRuntimeStore>"),
        "projection-service member cache must use a typed runtime store with secondary indexes"
    );
    assert!(
        member_store_source
            .contains("conversation_members_by_typed_principal: HashMap<String, BTreeSet<String>>"),
        "projection-service member cache must index tenant+principalKind+principalId to active conversation scopes for typed inbox reads"
    );
    assert!(
        member_store_source.contains(
            "encode_member_index_key_segments([tenant_id, principal_kind, principal_id])"
        ),
        "projection-service member principal index key must use segment-safe length-prefixed encoding"
    );
    assert!(
        !member_store_source.contains("format!(\"{tenant_id}:{principal_kind}:{principal_id}\")"),
        "projection-service member principal index key must not use delimiter-joined identifiers"
    );
    assert!(
        member_store_source.contains("fn active_member_scopes_for_principal_kind("),
        "projection-service member cache must expose indexed typed active scope lookup for principal-context inbox reads"
    );
    assert!(
        !lib_source.contains("for (scope, scope_members) in members.iter()"),
        "projection-service inbox must not scan every conversation membership bucket"
    );
    assert!(
        inbox_source.contains("pub fn inbox_for_principal_kind("),
        "projection-service must expose a typed inbox path so principal-context reads do not fetch same principalId across other actor kinds"
    );
    assert!(
        !access_source.contains("self.inbox(auth.tenant_id.as_str(), auth.actor_id.as_str())"),
        "projection-service principal-context inbox must not call the untyped inbox and then filter by actor kind"
    );
    assert!(
        snapshot_source.contains(".insert_member("),
        "projection-service snapshot restore must rebuild member secondary indexes through the runtime store"
    );
}

#[test]
fn test_projection_service_uses_received_message_index_for_unread_counts() {
    let lib_source = include_str!("../src/lib.rs");
    let inbox_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inbox.rs"))
            .expect("services/projection-service/src/inbox.rs should exist");
    let snapshot_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/snapshot.rs"))
            .expect("services/projection-service/src/snapshot.rs should exist");

    assert!(
        lib_source.contains("received_messages: Mutex<ReceivedMessageIndex>"),
        "projection-service should keep an internal received-message index for high-frequency unread counts"
    );
    assert!(
        lib_source.contains(".append_message("),
        "message projection should update the received-message unread index once when messages are posted"
    );
    assert!(
        snapshot_source.contains(".rebuild_conversation("),
        "snapshot restore should rebuild the derived unread index from persisted timeline and members"
    );
    assert!(
        lib_source.contains(".unread_count_after(")
            && inbox_source.contains(".unread_count_after("),
        "read cursor and inbox views should read unread counts from the received-message index"
    );
    assert!(
        !lib_source.contains(".range((Excluded(cursor.read_seq), Unbounded))")
            && !inbox_source.contains(".range((Excluded(read_seq), Unbounded))"),
        "high-frequency read cursor and inbox paths must not scan timeline entries to calculate unread counts"
    );
}

#[test]
fn test_projection_service_source_does_not_keep_legacy_device_sync_symbols() {
    let source_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let forbidden_symbols = [
        "mod device_sync;",
        "/src/device_sync.rs",
        "DeviceSync",
        "device_sync",
        "RegisteredDevice",
        "registered_devices",
        "register_device",
        "DevicePrincipalScopeKey",
        "DeviceFeedScopeKey",
    ];

    for entry in
        std::fs::read_dir(&source_dir).expect("services/projection-service/src should be readable")
    {
        let entry = entry.expect("projection-service src entry should be readable");
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("rs") {
            continue;
        }
        let source =
            std::fs::read_to_string(&path).expect("projection-service Rust source should read");
        for forbidden_symbol in forbidden_symbols {
            assert!(
                !source.contains(forbidden_symbol),
                "{} must use client_route_sync naming instead of legacy client route sync symbol: {forbidden_symbol}",
                path.display()
            );
        }
    }
}
