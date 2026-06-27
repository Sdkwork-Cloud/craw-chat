#[test]
fn test_conversation_runtime_support_module_no_longer_owns_member_and_cursor_domain_builders() {
    let support_source = include_str!("../src/runtime/support.rs");

    for forbidden_symbol in [
        "pub(super) fn build_conversation_member(",
        "pub(super) fn build_conversation_member_with_attributes(",
        "pub(super) fn build_default_read_cursor(",
        "pub(super) fn member_id(",
        "pub(super) fn member_episode_id(",
    ] {
        assert!(
            !support_source.contains(forbidden_symbol),
            "services/sdkwork-comms-conversation-service/src/runtime/support.rs should not keep domain builder symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_conversation_runtime_state_uses_domain_roster_for_member_and_cursor_state() {
    let runtime_source = include_str!("../src/runtime.rs");

    for forbidden_symbol in [
        "members: BTreeMap<String, ConversationMember>,",
        "principal_members: HashMap<String, String>,",
        "read_cursors: BTreeMap<String, ConversationReadCursor>,",
    ] {
        assert!(
            !runtime_source.contains(forbidden_symbol),
            "services/sdkwork-comms-conversation-service/src/runtime.rs should not keep direct roster field: {forbidden_symbol}"
        );
    }

    assert!(
        runtime_source.contains("roster: ConversationRoster,"),
        "services/sdkwork-comms-conversation-service/src/runtime.rs should use ConversationRoster as the domain owner for member/read_cursor state"
    );
}

#[test]
fn test_conversation_runtime_state_uses_domain_message_log_for_message_state() {
    let runtime_source = include_str!("../src/runtime.rs");

    for forbidden_symbol in [
        "messages: HashMap<String, StoredMessage>,",
        "struct StoredMessage {",
    ] {
        assert!(
            !runtime_source.contains(forbidden_symbol),
            "services/sdkwork-comms-conversation-service/src/runtime.rs should not keep direct message state symbol: {forbidden_symbol}"
        );
    }

    assert!(
        runtime_source.contains("message_log: ConversationMessageLog,"),
        "services/sdkwork-comms-conversation-service/src/runtime.rs should use ConversationMessageLog as the domain owner for conversation message state"
    );
}

#[test]
fn test_runtime_state_uses_domain_message_locator_for_cross_conversation_lookup() {
    let runtime_source = include_str!("../src/runtime.rs");
    let recovery_source = include_str!("../src/runtime/recovery.rs");

    for forbidden_symbol in ["message_index: HashMap<String, String>,", ".message_index"] {
        let found =
            runtime_source.contains(forbidden_symbol) || recovery_source.contains(forbidden_symbol);
        assert!(
            !found,
            "conversation runtime should not keep direct message lookup map symbol: {forbidden_symbol}"
        );
    }

    assert!(
        runtime_source.contains("message_locator: MessageLocatorIndex,"),
        "services/sdkwork-comms-conversation-service/src/runtime.rs should use MessageLocatorIndex as the domain owner for cross-conversation message lookup"
    );
}

#[test]
fn test_conversation_runtime_state_uses_rwlock_and_query_read_guards() {
    let runtime_source = include_str!("../src/runtime.rs").replace("\r\n", "\n");
    let membership_source = include_str!("../src/runtime/membership.rs").replace("\r\n", "\n");
    let binding_source = include_str!("../src/runtime/binding.rs").replace("\r\n", "\n");

    assert!(
        !runtime_source.contains("state: Mutex<RuntimeState>,"),
        "conversation runtime state must not use a global exclusive Mutex; query paths need shared read guards"
    );
    assert!(
        runtime_source.contains("state: RwLock<RuntimeState>,"),
        "conversation runtime state should use RwLock<RuntimeState> so independent reads can proceed concurrently"
    );
    assert!(
        runtime_source.contains("fn read_runtime_state")
            && runtime_source.contains("fn write_runtime_state"),
        "conversation runtime should expose explicit read/write guard helpers for runtime state"
    );

    for required_symbol in [
        "pub fn conversation_id_for_message(",
        "pub fn require_active_member_with_kind(",
        "pub fn require_active_member(",
    ] {
        let method = runtime_source
            .split(required_symbol)
            .nth(1)
            .and_then(|source| source.split("\n    pub fn ").next())
            .expect("runtime.rs should keep expected query method");
        assert!(
            method.contains("read_runtime_state("),
            "conversation-runtime query method should use shared read guard: {required_symbol}"
        );
    }

    for required_symbol in [
        "pub fn list_members(",
        "pub fn read_cursor_view(",
        "pub fn read_cursor_view_with_actor_kind(",
        "pub fn list_messages_window(",
        "pub fn list_messages_with_actor_kind(",
    ] {
        let method = membership_source
            .split(required_symbol)
            .nth(1)
            .and_then(|source| source.split("\n    pub fn ").next())
            .expect("membership.rs should keep expected query method");
        assert!(
            method.contains("read_runtime_state("),
            "conversation-runtime membership query should use shared read guard: {required_symbol}"
        );
    }

    assert!(
        binding_source.contains("read_runtime_state("),
        "conversation business binding query should use shared read guard"
    );
}

#[test]
fn test_message_mutation_commands_offer_auth_context_constructors() {
    let runtime_source = include_str!("../src/runtime.rs").replace("\r\n", "\n");

    assert!(
        runtime_source.contains("fn sender_from_auth_context(auth: &AppContext) -> Sender {"),
        "services/sdkwork-comms-conversation-service/src/runtime.rs should centralize message sender snapshot projection behind sender_from_auth_context"
    );

    for required_symbol in [
        "impl PostMessageCommand {\n    pub fn from_auth_context(",
        "impl PublishSystemChannelMessageCommand {\n    pub fn from_auth_context(",
        "impl EditMessageCommand {\n    pub fn from_auth_context(",
        "impl RecallMessageCommand {\n    pub fn from_auth_context(",
    ] {
        assert!(
            runtime_source.contains(required_symbol),
            "services/sdkwork-comms-conversation-service/src/runtime.rs should expose principal-context-backed message mutation constructor: {required_symbol}"
        );
    }
}

#[test]
fn test_http_message_surface_uses_auth_context_command_constructors() {
    let http_source = include_str!("../src/runtime/http.rs");

    assert!(
        http_source.contains("from_auth_context("),
        "services/sdkwork-comms-conversation-service/src/runtime/http.rs should construct message mutation commands from AppContext in one place"
    );

    for forbidden_symbol in [
        "sender: Sender {",
        "publisher: Sender {",
        "editor: Sender {",
        "recalled_by: Sender {",
    ] {
        assert!(
            !http_source.contains(forbidden_symbol),
            "services/sdkwork-comms-conversation-service/src/runtime/http.rs should not hand-build message mutation sender snapshot: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_non_message_commands_offer_auth_context_constructors() {
    let runtime_source = include_str!("../src/runtime.rs").replace("\r\n", "\n");

    for required_symbol in [
        "impl CreateConversationCommand {\n    pub fn from_auth_context(",
        "impl BindDirectChatConversationCommand {\n    pub fn from_auth_context(",
        "impl SyncSharedChannelLinkedMemberCommand {\n    pub fn from_auth_context(",
        "impl CreateAgentDialogCommand {\n    pub fn from_auth_context(",
        "impl CreateAgentHandoffCommand {\n    pub fn from_auth_context(",
        "impl CreateSystemChannelCommand {\n    pub fn from_auth_context(",
        "impl AcceptAgentHandoffCommand {\n    pub fn from_auth_context(",
        "impl ResolveAgentHandoffCommand {\n    pub fn from_auth_context(",
        "impl CloseAgentHandoffCommand {\n    pub fn from_auth_context(",
        "impl AddConversationMemberCommand {\n    pub fn from_auth_context(",
        "impl RemoveConversationMemberCommand {\n    pub fn from_auth_context(",
        "impl LeaveConversationCommand {\n    pub fn from_auth_context(",
        "impl TransferConversationOwnerCommand {\n    pub fn from_auth_context(",
        "impl ChangeConversationMemberRoleCommand {\n    pub fn from_auth_context(",
        "impl UpdateReadCursorCommand {\n    pub fn from_auth_context(",
    ] {
        assert!(
            runtime_source.contains(required_symbol),
            "services/sdkwork-comms-conversation-service/src/runtime.rs should expose principal-context-backed non-message constructor: {required_symbol}"
        );
    }
}

#[test]
fn test_http_non_message_surface_uses_auth_context_command_constructors() {
    let http_source = include_str!("../src/runtime/http.rs");

    for required_symbol in [
        ".create_conversation_from_auth_context(",
        ".bind_direct_chat_conversation_from_auth_context(",
        ".sync_shared_channel_linked_member_from_auth_context_with_result(",
        ".create_agent_dialog_from_auth_context(",
        ".create_agent_handoff_from_auth_context(",
        ".create_system_channel_from_auth_context(",
        ".accept_agent_handoff_from_auth_context(",
        ".resolve_agent_handoff_from_auth_context(",
        ".close_agent_handoff_from_auth_context(",
        ".add_member_from_auth_context(",
        ".remove_member_from_auth_context(",
        ".transfer_conversation_owner_from_auth_context(",
        ".change_conversation_member_role_from_auth_context(",
        ".leave_conversation_from_auth_context(",
        ".update_read_cursor_from_auth_context(",
    ] {
        assert!(
            http_source.contains(required_symbol),
            "services/sdkwork-comms-conversation-service/src/runtime/http.rs should consume runtime non-message principal-context entrypoint in one place: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        "CreateConversationCommand::from_auth_context(",
        "BindDirectChatConversationCommand::from_auth_context(",
        "SyncSharedChannelLinkedMemberCommand::from_auth_context(",
        "CreateAgentDialogCommand::from_auth_context(",
        "CreateAgentHandoffCommand::from_auth_context(",
        "CreateSystemChannelCommand::from_auth_context(",
        "AcceptAgentHandoffCommand::from_auth_context(",
        "ResolveAgentHandoffCommand::from_auth_context(",
        "CloseAgentHandoffCommand::from_auth_context(",
        "AddConversationMemberCommand::from_auth_context(",
        "RemoveConversationMemberCommand::from_auth_context(",
        "TransferConversationOwnerCommand::from_auth_context(",
        "ChangeConversationMemberRoleCommand::from_auth_context(",
        "LeaveConversationCommand::from_auth_context(",
        "UpdateReadCursorCommand::from_auth_context(",
        "create_conversation_with_creator_kind(",
        "create_agent_dialog_with_requester_kind(",
        "create_agent_handoff_with_source_kind(",
        "create_system_channel_with_requester_kind(",
        "accept_agent_handoff_with_actor_kind(",
        "resolve_agent_handoff_with_actor_kind(",
        "close_agent_handoff_with_actor_kind(",
        "add_member_with_actor_kind(",
        "remove_member_with_actor_kind(",
        "transfer_conversation_owner_with_actor_kind(",
        "change_conversation_member_role_with_actor_kind(",
        "leave_conversation_with_actor_kind(",
        "update_read_cursor_with_actor_kind(",
    ] {
        assert!(
            !http_source.contains(forbidden_symbol),
            "services/sdkwork-comms-conversation-service/src/runtime/http.rs should not keep non-message authority capture outside runtime principal-context entrypoint: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_runtime_exposes_non_message_auth_context_entrypoints() {
    let combined = format!(
        "{}\n{}\n{}",
        include_str!("../src/runtime/creation.rs"),
        include_str!("../src/runtime/membership.rs"),
        include_str!("../src/runtime/handoff.rs"),
    );

    for required_symbol in [
        "pub fn create_conversation_from_auth_context(",
        "pub fn bind_direct_chat_conversation_from_auth_context(",
        "pub fn sync_shared_channel_linked_member_from_auth_context(",
        "pub fn create_agent_dialog_from_auth_context(",
        "pub fn create_agent_handoff_from_auth_context(",
        "pub fn create_system_channel_from_auth_context(",
        "pub fn accept_agent_handoff_from_auth_context(",
        "pub fn resolve_agent_handoff_from_auth_context(",
        "pub fn close_agent_handoff_from_auth_context(",
        "pub fn add_member_from_auth_context(",
        "pub fn remove_member_from_auth_context(",
        "pub fn leave_conversation_from_auth_context(",
        "pub fn transfer_conversation_owner_from_auth_context(",
        "pub fn change_conversation_member_role_from_auth_context(",
        "pub fn update_read_cursor_from_auth_context(",
    ] {
        assert!(
            combined.contains(required_symbol),
            "conversation-runtime should expose principal-context-backed non-message runtime entrypoint: {required_symbol}"
        );
    }
}

#[test]
fn test_runtime_exposes_read_query_auth_context_entrypoints() {
    let combined = format!(
        "{}\n{}\n{}\n{}",
        include_str!("../src/runtime.rs"),
        include_str!("../src/runtime/binding.rs"),
        include_str!("../src/runtime/membership.rs"),
        include_str!("../src/runtime/handoff.rs"),
    );

    for required_symbol in [
        "pub fn require_active_member_from_auth_context(",
        "pub fn conversation_business_binding_from_auth_context(",
        "pub fn list_members_from_auth_context(",
        "pub fn list_members_window_from_auth_context(",
        "pub fn list_messages_window_from_auth_context(",
        "pub fn read_cursor_view_from_auth_context(",
        "pub fn get_agent_handoff_state_from_auth_context(",
    ] {
        assert!(
            combined.contains(required_symbol),
            "conversation-runtime should expose principal-context-backed read query entrypoint: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        "pub fn list_messages_from_auth_context(",
        "pub fn list_messages(\n",
        "usize::MAX",
    ] {
        assert!(
            !combined.contains(forbidden_symbol),
            "conversation-runtime must not expose unbounded message history read path: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_runtime_exposes_conversation_bound_write_access_auth_context_entrypoint() {
    let runtime_source = include_str!("../src/runtime.rs");

    assert!(
        runtime_source
            .contains("pub fn ensure_conversation_bound_write_allowed_from_auth_context("),
        "services/sdkwork-comms-conversation-service/src/runtime.rs should expose principal-context-backed conversation-bound write access guard"
    );
}

#[test]
fn test_http_non_message_surface_uses_runtime_auth_context_entrypoints() {
    let http_source = include_str!("../src/runtime/http.rs");

    for required_symbol in [
        ".create_conversation_from_auth_context(",
        ".bind_direct_chat_conversation_from_auth_context(",
        ".sync_shared_channel_linked_member_from_auth_context_with_result(",
        ".create_agent_dialog_from_auth_context(",
        ".create_agent_handoff_from_auth_context(",
        ".create_system_channel_from_auth_context(",
        ".accept_agent_handoff_from_auth_context(",
        ".resolve_agent_handoff_from_auth_context(",
        ".close_agent_handoff_from_auth_context(",
        ".add_member_from_auth_context(",
        ".remove_member_from_auth_context(",
        ".transfer_conversation_owner_from_auth_context(",
        ".change_conversation_member_role_from_auth_context(",
        ".leave_conversation_from_auth_context(",
        ".update_read_cursor_from_auth_context(",
    ] {
        assert!(
            http_source.contains(required_symbol),
            "services/sdkwork-comms-conversation-service/src/runtime/http.rs should consume runtime principal-context entrypoint: {required_symbol}"
        );
    }

    assert!(
        !http_source.contains("with_actor_kind(")
            && !http_source.contains("with_creator_kind(")
            && !http_source.contains("with_requester_kind(")
            && !http_source.contains("with_source_kind("),
        "services/sdkwork-comms-conversation-service/src/runtime/http.rs should not thread non-message authority through the old *with_*kind entrypoints once runtime owns the principal-context boundary"
    );
}

#[test]
fn test_http_read_query_surface_uses_runtime_auth_context_entrypoints() {
    let http_source = include_str!("../src/runtime/http.rs").replace("\r\n", "\n");

    for required_symbol in [
        ".conversation_business_binding_from_auth_context(",
        ".get_agent_handoff_state_from_auth_context(",
        ".list_members_window_from_auth_context(",
        ".list_messages_window_from_auth_context(",
        ".read_cursor_view_from_auth_context(",
    ] {
        assert!(
            http_source.contains(required_symbol),
            "services/sdkwork-comms-conversation-service/src/runtime/http.rs should consume runtime read query principal-context entrypoint: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        ".get_agent_handoff_state(\n        auth.tenant_id.as_str(),",
        ".require_active_member(\n        auth.tenant_id.as_str(),",
        ".list_members(auth.tenant_id.as_str(), conversation_id.as_str())",
        ".list_messages(auth.tenant_id.as_str(), conversation_id.as_str(),",
        ".read_cursor_view(\n        auth.tenant_id.as_str(),",
    ] {
        assert!(
            !http_source.contains(forbidden_symbol),
            "services/sdkwork-comms-conversation-service/src/runtime/http.rs should not keep read query authority capture outside runtime principal-context entrypoint: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_auth_context_runtime_entrypoints_keep_typed_principal_identity() {
    let runtime_source = include_str!("../src/runtime.rs").replace("\r\n", "\n");
    let binding_source = include_str!("../src/runtime/binding.rs").replace("\r\n", "\n");
    let membership_source = include_str!("../src/runtime/membership.rs").replace("\r\n", "\n");
    let handoff_source = include_str!("../src/runtime/handoff.rs").replace("\r\n", "\n");
    let governance_source = include_str!("../src/runtime/governance.rs").replace("\r\n", "\n");

    for (source_name, source) in [
        ("runtime.rs", runtime_source.as_str()),
        ("binding.rs", binding_source.as_str()),
        ("membership.rs", membership_source.as_str()),
        ("handoff.rs", handoff_source.as_str()),
        ("governance.rs", governance_source.as_str()),
    ] {
        assert!(
            !source.contains("from_auth_context(\n") || source.contains("auth.actor_kind.as_str()"),
            "services/sdkwork-comms-conversation-service/src/runtime/{source_name} principal-context entrypoints must keep actor_kind in the runtime boundary"
        );
    }

    assert!(
        !binding_source
            .contains("self.require_active_member(\n                auth.tenant_id.as_str(),"),
        "conversation_business_binding_from_auth_context must not use untyped member lookup"
    );
    assert!(
        binding_source.contains(
            "self.require_active_member_with_kind(\n                auth.tenant_id.as_str(),"
        ),
        "conversation_business_binding_from_auth_context should resolve membership by tenant, conversation, actor id, and actor kind"
    );
}

#[test]
fn test_conversation_runtime_uses_domain_handoff_view_and_transition_logic() {
    let runtime_source = include_str!("../src/runtime.rs");
    let handoff_source = include_str!("../src/runtime/handoff.rs");

    for forbidden_symbol in [
        "pub struct ChangeAgentHandoffStatusView",
        "pub struct AgentHandoffStateView",
    ] {
        assert!(
            !runtime_source.contains(forbidden_symbol),
            "services/sdkwork-comms-conversation-service/src/runtime.rs should not keep handoff view symbol: {forbidden_symbol}"
        );
    }

    for forbidden_symbol in [
        "handoff_state.status =",
        "handoff_state.accepted_at =",
        "handoff_state.accepted_by =",
        "handoff_state.resolved_at =",
        "handoff_state.resolved_by =",
        "handoff_state.closed_at =",
        "handoff_state.closed_by =",
    ] {
        assert!(
            !handoff_source.contains(forbidden_symbol),
            "services/sdkwork-comms-conversation-service/src/runtime/handoff.rs should not keep direct handoff mutation symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_runtime_policy_uses_domain_scenario_owner_for_direct_group_channel_paths() {
    let policy_source = include_str!("../src/runtime/policy.rs");
    let domain_source = include_str!("../../../crates/im-domain-core/src/conversation.rs");

    assert!(
        domain_source.contains("pub enum ConversationScenario {"),
        "crates/im-domain-core/src/conversation.rs should expose ConversationScenario as the domain owner for conversation-type-specific main-path policy"
    );
    assert!(
        domain_source.contains("pub fn scenario(&self) -> ConversationScenario {"),
        "ConversationAggregateState should expose scenario() so runtime policy can consume the aggregate owner instead of branching on raw conversation_type strings"
    );

    assert!(
        policy_source.contains(".scenario()"),
        "services/sdkwork-comms-conversation-service/src/runtime/policy.rs should consume ConversationAggregateState::scenario() for direct/group/channel main-path branching"
    );

    for forbidden_symbol in [
        "match conversation.aggregate.conversation_type() {",
        "if conversation.aggregate.conversation_type() == \"agent_handoff\"",
        "if conversation.aggregate.conversation_type() == \"system_channel\"",
    ] {
        assert!(
            !policy_source.contains(forbidden_symbol),
            "services/sdkwork-comms-conversation-service/src/runtime/policy.rs should not keep raw conversation_type branching once ConversationScenario owns direct/group/channel closure: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_conversation_runtime_state_uses_domain_aggregate_for_metadata_fields() {
    let runtime_source = include_str!("../src/runtime.rs");
    let domain_source = include_str!("../../../crates/im-domain-core/src/conversation.rs");

    for forbidden_symbol in [
        "\n    conversation_type: String,\n",
        "\n    member_epoch: u64,\n",
        "\n    handoff_status_epoch: u64,\n",
        "\n    handoff_state: Option<AgentHandoffStateView>,\n",
    ] {
        assert!(
            !runtime_source.contains(forbidden_symbol),
            "services/sdkwork-comms-conversation-service/src/runtime.rs should not keep direct aggregate metadata field: {forbidden_symbol}"
        );
    }

    assert!(
        runtime_source.contains("aggregate: ConversationAggregateState,"),
        "services/sdkwork-comms-conversation-service/src/runtime.rs should use ConversationAggregateState as the domain owner for conversation metadata"
    );
    assert!(
        domain_source.contains("pub struct ConversationBusinessBinding {"),
        "crates/im-domain-core/src/conversation.rs should expose ConversationBusinessBinding as the domain owner for business binding metadata"
    );
    assert!(
        domain_source
            .contains("pub fn business_binding(&self) -> Option<&ConversationBusinessBinding> {")
            && domain_source.contains("pub fn replace_business_binding("),
        "ConversationAggregateState should own business binding accessors instead of pushing business metadata back into runtime state"
    );
}

#[test]
fn test_conversation_runtime_modules_use_domain_aggregate_owner_for_type_and_epochs() {
    let creation_source = include_str!("../src/runtime/creation.rs");
    let membership_source = include_str!("../src/runtime/membership.rs");
    let recovery_source = include_str!("../src/runtime/recovery.rs");
    let handoff_source = include_str!("../src/runtime/handoff.rs");

    for forbidden_symbol in [
        "conversation.conversation_type =",
        "conversation.member_epoch =",
        "conversation.member_epoch +=",
        "conversation.handoff_state =",
        "conversation.handoff_status_epoch =",
        "conversation.handoff_status_epoch +=",
    ] {
        let found = creation_source.contains(forbidden_symbol)
            || membership_source.contains(forbidden_symbol)
            || recovery_source.contains(forbidden_symbol)
            || handoff_source.contains(forbidden_symbol);
        assert!(
            !found,
            "conversation runtime modules should delegate aggregate metadata ownership to ConversationAggregateState instead of using direct symbol: {forbidden_symbol}"
        );
    }
}
