//! Authorization helpers for Postgres supplemental social routes.

use im_adapters_social_postgres::direct_chat_store::DirectChatRecord;
use im_adapters_social_postgres::friendship_store::FriendshipRecord;
use im_adapters_social_postgres::user_block_store::UserBlockRecord;
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::ApiProblem;

/// Canonical social-graph principal id for list/read supplemental routes.
pub fn social_principal_user_id(auth: &AppContext) -> &str {
    auth.social_principal_user_id()
}

pub fn ensure_friendship_participant(
    auth: &AppContext,
    record: &FriendshipRecord,
) -> Result<(), ApiProblem> {
    let principal = social_principal_user_id(auth);
    if principal == record.user_low_id.as_str() || principal == record.user_high_id.as_str() {
        return Ok(());
    }
    Err(ApiProblem::forbidden(
        "authenticated user must be a friendship participant",
    ))
}

pub fn ensure_block_owner(auth: &AppContext, record: &UserBlockRecord) -> Result<(), ApiProblem> {
    if social_principal_user_id(auth) == record.blocker_user_id.as_str() {
        return Ok(());
    }
    Err(ApiProblem::forbidden(
        "authenticated user must be the block owner",
    ))
}

pub fn ensure_direct_chat_participant(
    auth: &AppContext,
    record: &DirectChatRecord,
) -> Result<(), ApiProblem> {
    let principal = social_principal_user_id(auth);
    if principal == record.left_actor_id.as_str() || principal == record.right_actor_id.as_str() {
        return Ok(());
    }
    Err(ApiProblem::forbidden(
        "authenticated user must be a direct chat participant",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn auth_for(user_id: &str) -> AppContext {
        im_app_context::local_service_app_context(
            "100001",
            user_id,
            "user",
            None,
            Vec::<&str>::new(),
        )
    }

    fn sample_friendship_record(user_low_id: &str, user_high_id: &str) -> FriendshipRecord {
        FriendshipRecord {
            tenant_id: "100001".to_owned(),
            organization_id: "default".to_owned(),
            friendship_id: 1,
            user_low_id: user_low_id.to_owned(),
            user_high_id: user_high_id.to_owned(),
            initiator_user_id: user_low_id.to_owned(),
            status: "active".to_owned(),
            established_at: None,
            updated_at: "2026-01-01T00:00:00Z".to_owned(),
        }
    }

    fn sample_block_record(blocker_user_id: &str) -> UserBlockRecord {
        UserBlockRecord {
            tenant_id: "100001".to_owned(),
            organization_id: "default".to_owned(),
            block_id: 1,
            blocker_user_id: blocker_user_id.to_owned(),
            blocked_user_id: "blocked".to_owned(),
            scope: "all".to_owned(),
            direct_chat_id: None,
            reason: None,
            expires_at: None,
            created_at: "2026-01-01T00:00:00Z".to_owned(),
            updated_at: "2026-01-01T00:00:00Z".to_owned(),
        }
    }

    #[test]
    fn friendship_participant_may_read_supplemental_record() {
        let record = sample_friendship_record("user-a", "user-b");
        ensure_friendship_participant(&auth_for("user-a"), &record).expect("low participant");
        ensure_friendship_participant(&auth_for("user-b"), &record).expect("high participant");
    }

    #[test]
    fn non_participant_cannot_read_supplemental_friendship() {
        let record = sample_friendship_record("user-a", "user-b");
        assert!(ensure_friendship_participant(&auth_for("user-c"), &record).is_err());
    }

    #[test]
    fn block_owner_may_read_supplemental_block() {
        let record = sample_block_record("blocker");
        ensure_block_owner(&auth_for("blocker"), &record).expect("block owner");
    }

    #[test]
    fn non_owner_cannot_read_supplemental_block() {
        let record = sample_block_record("blocker");
        assert!(ensure_block_owner(&auth_for("other"), &record).is_err());
    }

    fn sample_direct_chat_record(left: &str, right: &str) -> DirectChatRecord {
        DirectChatRecord {
            tenant_id: "100001".to_owned(),
            organization_id: "default".to_owned(),
            direct_chat_id: 1,
            left_actor_kind: "user".to_owned(),
            left_actor_id: left.to_owned(),
            right_actor_kind: "user".to_owned(),
            right_actor_id: right.to_owned(),
            pair_hash: "pair".to_owned(),
            status: "active".to_owned(),
            conversation_id: Some("conv-1".to_owned()),
            created_at: "2026-01-01T00:00:00Z".to_owned(),
            updated_at: "2026-01-01T00:00:00Z".to_owned(),
        }
    }

    #[test]
    fn direct_chat_participant_may_read_supplemental_record() {
        let record = sample_direct_chat_record("user-a", "user-b");
        ensure_direct_chat_participant(&auth_for("user-a"), &record).expect("left participant");
        ensure_direct_chat_participant(&auth_for("user-b"), &record).expect("right participant");
    }

    #[test]
    fn non_participant_cannot_read_supplemental_direct_chat() {
        let record = sample_direct_chat_record("user-a", "user-b");
        assert!(ensure_direct_chat_participant(&auth_for("user-c"), &record).is_err());
    }
}
