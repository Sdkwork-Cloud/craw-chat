#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SessionFenceDecision {
    Allow,
    ClearAndAllow,
    RequireReconnect,
}

pub(crate) fn decide_session_fence(
    fence_required: bool,
    fenced_session_id: Option<&str>,
    current_session_id: Option<&str>,
) -> SessionFenceDecision {
    if !fence_required {
        return SessionFenceDecision::Allow;
    }
    match fenced_session_id {
        Some(fenced_session_id) if Some(fenced_session_id) != current_session_id => {
            SessionFenceDecision::ClearAndAllow
        }
        Some(_) | None => SessionFenceDecision::RequireReconnect,
    }
}

#[cfg(test)]
mod tests {
    use super::{SessionFenceDecision, decide_session_fence};

    #[test]
    fn session_fence_policy_is_fail_closed_and_session_scoped() {
        assert_eq!(
            decide_session_fence(false, Some("old"), Some("old")),
            SessionFenceDecision::Allow
        );
        assert_eq!(
            decide_session_fence(true, Some("old"), Some("new")),
            SessionFenceDecision::ClearAndAllow
        );
        assert_eq!(
            decide_session_fence(true, Some("old"), Some("old")),
            SessionFenceDecision::RequireReconnect
        );
        assert_eq!(
            decide_session_fence(true, None, Some("new")),
            SessionFenceDecision::RequireReconnect
        );
    }
}
