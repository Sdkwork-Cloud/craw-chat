//! White-box unit tests for ConversationRuntime.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "runtime_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use super::*;
use std::panic::{self, AssertUnwindSafe};

fn drive_reference_for_test() -> DriveReference {
    DriveReference {
        drive_uri: "drive://spaces/space-im/nodes/node-image-1".into(),
        space_id: "space-im".into(),
        node_id: "node-image-1".into(),
        node_version: None,
    }
}

fn drive_media_resource_for_test(drive: &DriveReference) -> MediaResource {
    MediaResource {
        id: Some(drive.node_id.clone()),
        kind: im_domain_core::media::MediaKind::Image,
        source: MediaSource::Drive,
        url: None,
        public_url: None,
        uri: Some(drive.drive_uri.clone()),
        object_blob_id: None,
        file_name: Some("image.png".into()),
        mime_type: Some("image/png".into()),
        size_bytes: Some("42".into()),
        checksum: None,
        width: None,
        height: None,
        duration_seconds: None,
        alt_text: None,
        title: None,
        poster: None,
        thumbnails: None,
        variants: None,
        access: None,
        ai: None,
        metadata: None,
    }
}

fn media_message_body_for_test(resource: MediaResource, drive: DriveReference) -> MessageBody {
    MessageBody {
        summary: Some("image".into()),
        parts: vec![ContentPart::media(im_domain_core::message::MediaPart {
            resource,
            drive,
            media_role: Some("attachment".into()),
        })],
        render_hints: BTreeMap::new(),
        reply_to: None,
    }
}

#[test]
fn test_message_body_rejects_local_preview_urls_in_drive_media_resource() {
    let drive = drive_reference_for_test();
    let mut resource = drive_media_resource_for_test(&drive);
    resource.url = Some("blob://local-image".into());
    let body = media_message_body_for_test(resource, drive);

    let result = validate_message_body_contract(&body);

    assert!(
        matches!(
            result,
            Err(RuntimeError::InvalidInput(message))
                if message.contains("resource.url")
                    && message.contains("local preview URL")
        ),
        "local preview URLs must be rejected before IM message persistence"
    );
}

#[test]
fn test_message_body_rejects_nested_local_preview_urls_in_drive_media_resource() {
    let drive = drive_reference_for_test();
    let mut resource = drive_media_resource_for_test(&drive);
    let mut poster = drive_media_resource_for_test(&drive);
    poster.url = Some("data:image/png;base64,local-preview".into());
    resource.poster = Some(Box::new(poster));
    let body = media_message_body_for_test(resource, drive);

    let result = validate_message_body_contract(&body);

    assert!(
        matches!(
            result,
            Err(RuntimeError::InvalidInput(message))
                if message.contains("resource.poster.url")
                    && message.contains("local preview URL")
        ),
        "nested local preview URLs must be rejected before IM message persistence"
    );
}

fn poison_mutex<T>(mutex: &Mutex<T>) {
    let _ = panic::catch_unwind(AssertUnwindSafe(|| {
        let _guard = mutex.lock().expect("test poison lock should succeed");
        panic!("intentional poison for regression coverage");
    }));
}

fn poison_rwlock_write<T>(lock: &RwLock<T>) {
    let _ = panic::catch_unwind(AssertUnwindSafe(|| {
        let _guard = lock.write().expect("test poison lock should succeed");
        panic!("intentional poison for regression coverage");
    }));
}

#[test]
fn test_in_memory_journal_recorded_recovers_from_poisoned_lock() {
    let journal = InMemoryJournal::default();
    poison_mutex(&journal.events);

    let result = panic::catch_unwind(AssertUnwindSafe(|| journal.recorded()));
    assert!(
        result.is_ok(),
        "journal.recorded should not panic when journal lock is poisoned"
    );
}

#[test]
fn test_require_active_member_recovers_from_poisoned_runtime_state_lock() {
    let runtime = ConversationRuntime::new(InMemoryJournal::default());
    poison_rwlock_write(&runtime.state);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        runtime.require_active_member("100001", "c_demo", "1")
    }));
    assert!(
        result.is_ok(),
        "require_active_member should not panic when runtime state lock is poisoned"
    );
    let member_result = result.expect("panic status should be captured");
    assert!(member_result.is_err());
}

#[test]
fn test_post_message_recovers_from_poisoned_runtime_state_lock() {
    let runtime = ConversationRuntime::new(InMemoryJournal::default());
    poison_rwlock_write(&runtime.state);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        runtime.post_message(PostMessageCommand {
            tenant_id: "100001".into(),
           organization_id: "0".into(),
            conversation_id: "c_demo".into(),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: None,
                metadata: BTreeMap::new(),
            },
            client_msg_id: None,
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![im_domain_core::message::ContentPart::text("hello")],
                render_hints: BTreeMap::new(),
                reply_to: None,
            },
        })
    }));
    assert!(
        result.is_ok(),
        "post_message should not panic when runtime state lock is poisoned"
    );
    let post_result = result.expect("panic status should be captured");
    assert!(post_result.is_err());
}
