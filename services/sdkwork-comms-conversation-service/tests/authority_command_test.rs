use std::collections::BTreeSet;

use conversation_runtime::{
    AcceptAgentHandoffCommand, AddConversationMemberCommand, AddMessageReactionCommand,
    ChangeConversationMemberRoleCommand, CloseAgentHandoffCommand, CreateAgentDialogCommand,
    CreateAgentHandoffCommand, CreateConversationCommand, CreateSystemChannelCommand,
    EditMessageCommand, LeaveConversationCommand, PinMessageCommand, PostMessageCommand,
    PublishSystemChannelMessageCommand, RecallMessageCommand, RemoveConversationMemberCommand,
    RemoveMessageReactionCommand, ResolveAgentHandoffCommand, TransferConversationOwnerCommand,
    UnpinMessageCommand, UpdateReadCursorCommand,
};
use im_app_context::AppContext;
use im_domain_core::message::{ContentPart, MessageBody, MessageType};

fn demo_auth() -> AppContext {
    AppContext {
        tenant_id: "100001".into(),
        organization_id: "0".to_owned(),
        user_id: "svc_ops".into(),
        session_id: Some("s_system".into()),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: BTreeSet::new(),
        permission_scope: BTreeSet::new(),
        actor_id: "svc_ops".into(),
        actor_kind: "system".into(),
        device_id: Some("d_system".into()),
    }
}

fn demo_body(summary: &str, text: &str) -> MessageBody {
    MessageBody {
        summary: Some(summary.into()),
        parts: vec![ContentPart::text(text)],
        render_hints: Default::default(),
        reply_to: None,
    }
}

#[test]
fn test_message_mutation_commands_from_auth_context_preserve_authority_snapshot() {
    let auth = demo_auth();

    let post = PostMessageCommand::from_auth_context(
        &auth,
        "c_demo".into(),
        Some("client_demo".into()),
        MessageType::Standard,
        demo_body("post", "post"),
    );
    assert_eq!(post.tenant_id, "100001");
    assert_eq!(post.sender.id, "svc_ops");
    assert_eq!(post.sender.kind, "system");
    assert_eq!(post.sender.device_id.as_deref(), Some("d_system"));
    assert_eq!(post.sender.session_id.as_deref(), Some("s_system"));

    let publish = PublishSystemChannelMessageCommand::from_auth_context(
        &auth,
        "c_system".into(),
        Some("client_system".into()),
        demo_body("publish", "publish"),
    );
    assert_eq!(publish.tenant_id, "100001");
    assert_eq!(publish.publisher.id, "svc_ops");
    assert_eq!(publish.publisher.kind, "system");
    assert_eq!(publish.publisher.device_id.as_deref(), Some("d_system"));
    assert_eq!(publish.publisher.session_id.as_deref(), Some("s_system"));

    let edit =
        EditMessageCommand::from_auth_context(&auth, "msg_demo".into(), demo_body("edit", "edit"));
    assert_eq!(edit.tenant_id, "100001");
    assert_eq!(edit.editor.id, "svc_ops");
    assert_eq!(edit.editor.kind, "system");
    assert_eq!(edit.editor.device_id.as_deref(), Some("d_system"));
    assert_eq!(edit.editor.session_id.as_deref(), Some("s_system"));

    let recall = RecallMessageCommand::from_auth_context(&auth, "msg_demo".into());
    assert_eq!(recall.tenant_id, "100001");
    assert_eq!(recall.recalled_by.id, "svc_ops");
    assert_eq!(recall.recalled_by.kind, "system");
    assert_eq!(recall.recalled_by.device_id.as_deref(), Some("d_system"));
    assert_eq!(recall.recalled_by.session_id.as_deref(), Some("s_system"));

    let add_reaction =
        AddMessageReactionCommand::from_auth_context(&auth, "msg_demo".into(), "thumbs_up".into());
    assert_eq!(add_reaction.tenant_id, "100001");
    assert_eq!(add_reaction.message_id, "msg_demo");
    assert_eq!(add_reaction.reaction_key, "thumbs_up");
    assert_eq!(add_reaction.reacted_by.id, "svc_ops");
    assert_eq!(add_reaction.reacted_by.kind, "system");
    assert_eq!(
        add_reaction.reacted_by.device_id.as_deref(),
        Some("d_system")
    );
    assert_eq!(
        add_reaction.reacted_by.session_id.as_deref(),
        Some("s_system")
    );

    let remove_reaction = RemoveMessageReactionCommand::from_auth_context(
        &auth,
        "msg_demo".into(),
        "thumbs_up".into(),
    );
    assert_eq!(remove_reaction.tenant_id, "100001");
    assert_eq!(remove_reaction.message_id, "msg_demo");
    assert_eq!(remove_reaction.reaction_key, "thumbs_up");
    assert_eq!(remove_reaction.removed_by.id, "svc_ops");
    assert_eq!(remove_reaction.removed_by.kind, "system");
    assert_eq!(
        remove_reaction.removed_by.device_id.as_deref(),
        Some("d_system")
    );
    assert_eq!(
        remove_reaction.removed_by.session_id.as_deref(),
        Some("s_system")
    );

    let pin = PinMessageCommand::from_auth_context(&auth, "msg_demo".into());
    assert_eq!(pin.tenant_id, "100001");
    assert_eq!(pin.message_id, "msg_demo");
    assert_eq!(pin.pinned_by.id, "svc_ops");
    assert_eq!(pin.pinned_by.kind, "system");
    assert_eq!(pin.pinned_by.device_id.as_deref(), Some("d_system"));
    assert_eq!(pin.pinned_by.session_id.as_deref(), Some("s_system"));

    let unpin = UnpinMessageCommand::from_auth_context(&auth, "msg_demo".into());
    assert_eq!(unpin.tenant_id, "100001");
    assert_eq!(unpin.message_id, "msg_demo");
    assert_eq!(unpin.unpinned_by.id, "svc_ops");
    assert_eq!(unpin.unpinned_by.kind, "system");
    assert_eq!(unpin.unpinned_by.device_id.as_deref(), Some("d_system"));
    assert_eq!(unpin.unpinned_by.session_id.as_deref(), Some("s_system"));
}

#[test]
fn test_non_message_commands_from_auth_context_preserve_authority_identity() {
    let auth = demo_auth();

    let create =
        CreateConversationCommand::from_auth_context(&auth, "c_create".into(), "group".into());
    assert_eq!(create.tenant_id, "100001");
    assert_eq!(create.creator_id, "svc_ops");

    let agent_dialog =
        CreateAgentDialogCommand::from_auth_context(&auth, "c_agent".into(), "agent_1".into());
    assert_eq!(agent_dialog.tenant_id, "100001");
    assert_eq!(agent_dialog.requester_id, "svc_ops");

    let handoff = CreateAgentHandoffCommand::from_auth_context(
        &auth,
        "c_handoff".into(),
        "target_1".into(),
        "human".into(),
        "session_handoff".into(),
        Some("escalation".into()),
    );
    assert_eq!(handoff.tenant_id, "100001");
    assert_eq!(handoff.source_id, "svc_ops");

    let system_channel = CreateSystemChannelCommand::from_auth_context(
        &auth,
        "c_system".into(),
        "subscriber_1".into(),
    );
    assert_eq!(system_channel.tenant_id, "100001");
    assert_eq!(system_channel.requester_id, "svc_ops");

    let accept = AcceptAgentHandoffCommand::from_auth_context(&auth, "c_accept".into());
    assert_eq!(accept.tenant_id, "100001");
    assert_eq!(accept.accepted_by, "svc_ops");

    let resolve = ResolveAgentHandoffCommand::from_auth_context(&auth, "c_resolve".into());
    assert_eq!(resolve.tenant_id, "100001");
    assert_eq!(resolve.resolved_by, "svc_ops");

    let close = CloseAgentHandoffCommand::from_auth_context(&auth, "c_close".into());
    assert_eq!(close.tenant_id, "100001");
    assert_eq!(close.closed_by, "svc_ops");

    let add_member = AddConversationMemberCommand::from_auth_context(
        &auth,
        "c_add".into(),
        "member_1".into(),
        "human".into(),
        im_domain_core::conversation::MembershipRole::Member,
    );
    assert_eq!(add_member.tenant_id, "100001");
    assert_eq!(add_member.invited_by, "svc_ops");

    let remove_member = RemoveConversationMemberCommand::from_auth_context(
        &auth,
        "c_remove".into(),
        "member_2".into(),
    );
    assert_eq!(remove_member.tenant_id, "100001");
    assert_eq!(remove_member.removed_by, "svc_ops");

    let transfer = TransferConversationOwnerCommand::from_auth_context(
        &auth,
        "c_transfer".into(),
        "member_3".into(),
    );
    assert_eq!(transfer.tenant_id, "100001");
    assert_eq!(transfer.transferred_by, "svc_ops");

    let change_role = ChangeConversationMemberRoleCommand::from_auth_context(
        &auth,
        "c_role".into(),
        "member_4".into(),
        im_domain_core::conversation::MembershipRole::Owner,
    );
    assert_eq!(change_role.tenant_id, "100001");
    assert_eq!(change_role.changed_by, "svc_ops");

    let leave = LeaveConversationCommand::from_auth_context(&auth, "c_leave".into());
    assert_eq!(leave.tenant_id, "100001");
    assert_eq!(leave.principal_id, "svc_ops");

    let cursor = UpdateReadCursorCommand::from_auth_context(
        &auth,
        "c_cursor".into(),
        42,
        Some("msg_42".into()),
    );
    assert_eq!(cursor.tenant_id, "100001");
    assert_eq!(cursor.principal_id, "svc_ops");
}
