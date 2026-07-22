use super::*;

#[test]
fn test_is_active_member_recovers_from_poisoned_member_store_lock() {
    let conversation_state = ConversationStateService::default();
    let _ = std::panic::catch_unwind(|| {
        let _guard = conversation_state.members.lock().expect("member store should lock");
        panic!("poison member store lock");
    });

    let is_active =
        conversation_state.is_active_member_for_principal_kind("100001", "default", "c_demo", "1", "user");
    assert!(!is_active);
}
