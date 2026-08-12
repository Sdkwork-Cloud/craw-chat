//! Shared space authorization helpers for space-service handlers.

use im_adapters_social_postgres::organization_store::{ChannelRecord, SpaceRecord};
use im_app_context::AppContext;
use sdkwork_routes_web_framework_backend_api::response::ApiProblem;

use crate::http::AppState;

pub fn parse_space_id(space_id: &str) -> Result<i64, ApiProblem> {
    space_id.parse().map_err(|_| {
        tracing::warn!("invalid space_id path parameter: {space_id}");
        ApiProblem::bad_request("invalid space_id path parameter")
    })
}

pub fn parse_entity_id(entity_id: &str, field: &str) -> Result<i64, ApiProblem> {
    entity_id.parse().map_err(|_| {
        tracing::warn!("invalid {field} path parameter: {entity_id}");
        ApiProblem::bad_request(format!("invalid {field} path parameter"))
    })
}

pub fn load_space(
    state: &AppState,
    auth: &AppContext,
    space_id: i64,
) -> Result<SpaceRecord, ApiProblem> {
    state
        .space_store
        .get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            space_id,
        )
        .map_err(|error| {
            tracing::error!(error = ?error, space_id, "failed to load space");
            ApiProblem::internal_server_error("failed to load space")
        })?
        .ok_or_else(|| ApiProblem::not_found("space not found"))
}

pub fn actor_can_read_space(
    state: &AppState,
    auth: &AppContext,
    space: &SpaceRecord,
) -> Result<(), ApiProblem> {
    if space.owner_user_id == auth.actor_id {
        return Ok(());
    }
    match state.space_member_store.get_by_id(
        auth.tenant_id.as_str(),
        auth.organization_id.as_str(),
        space.space_id,
        auth.actor_id.as_str(),
    ) {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err(ApiProblem::forbidden("space membership required")),
        Err(error) => {
            tracing::error!(error = ?error, "failed to resolve space membership");
            Err(ApiProblem::internal_server_error(
                "failed to resolve space membership",
            ))
        }
    }
}

pub fn actor_can_manage_space(
    state: &AppState,
    auth: &AppContext,
    space: &SpaceRecord,
) -> Result<(), ApiProblem> {
    if space.owner_user_id == auth.actor_id {
        return Ok(());
    }
    match state.space_member_store.get_by_id(
        auth.tenant_id.as_str(),
        auth.organization_id.as_str(),
        space.space_id,
        auth.actor_id.as_str(),
    ) {
        Ok(Some(member)) if member.role == "admin" => Ok(()),
        Ok(Some(_)) => Err(ApiProblem::forbidden("space admin permission required")),
        Ok(None) => Err(ApiProblem::forbidden("space admin permission required")),
        Err(error) => {
            tracing::error!(error = ?error, "failed to resolve space admin membership");
            Err(ApiProblem::internal_server_error(
                "failed to resolve space admin membership",
            ))
        }
    }
}

pub fn ensure_user_not_banned_in_space(
    state: &AppState,
    auth: &AppContext,
    space_id: i64,
    user_id: &str,
) -> Result<(), ApiProblem> {
    match state.ban_store.get_active_by_user(
        auth.tenant_id.as_str(),
        auth.organization_id.as_str(),
        "space",
        space_id,
        user_id,
    ) {
        Ok(Some(_)) => Err(ApiProblem::forbidden("user is banned from this space")),
        Ok(None) => Ok(()),
        Err(error) => {
            tracing::error!(error = ?error, "failed to resolve space ban status");
            Err(ApiProblem::internal_server_error(
                "failed to resolve space ban status",
            ))
        }
    }
}

pub fn normalize_space_member_role(
    role: Option<&str>,
    allow_owner: bool,
) -> Result<String, ApiProblem> {
    match role.unwrap_or("member") {
        "owner" if allow_owner => Ok("owner".to_owned()),
        "owner" => Err(ApiProblem::bad_request(
            "owner role cannot be assigned directly",
        )),
        "admin" => Ok("admin".to_owned()),
        "member" => Ok("member".to_owned()),
        "guest" => Ok("guest".to_owned()),
        other => {
            tracing::warn!(role = other, "invalid space member role");
            Err(ApiProblem::bad_request("invalid space member role"))
        }
    }
}

pub fn load_channel_in_space(
    state: &AppState,
    auth: &AppContext,
    space_id: i64,
    channel_id: i64,
) -> Result<ChannelRecord, ApiProblem> {
    let channel = state
        .channel_store
        .get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            channel_id,
        )
        .map_err(|error| {
            tracing::error!(error = ?error, channel_id, "failed to load channel");
            ApiProblem::internal_server_error("failed to load channel")
        })?
        .ok_or_else(|| ApiProblem::not_found("channel not found"))?;

    if channel.space_id != space_id {
        tracing::warn!(
            path_space_id = space_id,
            record_space_id = channel.space_id,
            channel_id,
            "channel does not belong to requested space"
        );
        return Err(ApiProblem::not_found("channel not found"));
    }
    Ok(channel)
}

/// Enforces channel access rules for a channel operation.
///
/// Space owners and admins manage the rules themselves and always bypass
/// them. For every other member the effective rule decision applies:
/// an explicit deny forbids the permission, an explicit allow permits it,
/// and no rule keeps the membership-based default (the caller already
/// verified space membership and channel ownership).
pub fn enforce_channel_permission(
    state: &AppState,
    auth: &AppContext,
    space: &SpaceRecord,
    channel_id: i64,
    permission: &str,
) -> Result<(), ApiProblem> {
    if space.owner_user_id == auth.actor_id {
        return Ok(());
    }
    match state.space_member_store.get_by_id(
        auth.tenant_id.as_str(),
        auth.organization_id.as_str(),
        space.space_id,
        auth.actor_id.as_str(),
    ) {
        Ok(Some(member)) if member.role == "admin" => return Ok(()),
        Ok(_) => {}
        Err(error) => {
            tracing::error!(error = ?error, "failed to resolve space membership for channel rule enforcement");
            return Err(ApiProblem::internal_server_error(
                "failed to resolve space membership",
            ));
        }
    }
    match state.channel_access_rule_store.effective_permission(
        auth.tenant_id.as_str(),
        auth.organization_id.as_str(),
        channel_id,
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
        permission,
    ) {
        Ok(im_adapters_social_postgres::governance_store::ChannelRuleDecision::Deny) => {
            tracing::info!(
                tenant_id = auth.tenant_id.as_str(),
                channel_id,
                permission,
                "channel access rule denied member operation"
            );
            Err(ApiProblem::forbidden("channel access rule forbids this operation"))
        }
        Ok(
            im_adapters_social_postgres::governance_store::ChannelRuleDecision::Allow
            | im_adapters_social_postgres::governance_store::ChannelRuleDecision::NoRule,
        ) => Ok(()),
        Err(error) => {
            tracing::error!(error = ?error, channel_id, "failed to evaluate channel access rules");
            Err(ApiProblem::internal_server_error(
                "failed to evaluate channel access rules",
            ))
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use im_adapters_social_postgres::governance_store::{
        BanRecord, BanStore, BanTargetListQuery, ChannelAccessRuleRecord, ChannelAccessRuleStore,
        ChannelRuleDecision, InvitationRecord, InvitationStore, InvitationTargetListQuery,
        SpaceMemberRecord, SpaceMemberStore,
    };
    use im_adapters_social_postgres::member_capacity::MemberInsertOutcome;
    use im_adapters_social_postgres::organization_store::{
        ChannelRecord, ChannelStore, GroupMemberRecord, GroupMemberStore, GroupRecord, GroupStore,
        SpaceRecord, SpaceStore,
    };
    use im_platform_contracts::ContractError;
    use sdkwork_im_runtime_id::RuntimeSnowflakeIdGenerator;
    use std::sync::Arc;

    fn test_space(owner: &str) -> SpaceRecord {
        SpaceRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            space_id: 1,
            space_name: "test".into(),
            space_type: "team".into(),
            owner_user_id: owner.into(),
            description: None,
            avatar_url: None,
            max_members: 100,
            settings_json: "{}".into(),
            created_at: "2026-01-01T00:00:00.000Z".into(),
            updated_at: "2026-01-01T00:00:00.000Z".into(),
        }
    }

    fn test_auth(actor: &str) -> im_app_context::AppContext {
        im_app_context::local_service_app_context("100001", actor, "user", None, ["space.read"])
    }

    struct ScriptedRuleStore {
        decision: ChannelRuleDecision,
    }

    impl ChannelAccessRuleStore for ScriptedRuleStore {
        fn insert(&self, _record: &ChannelAccessRuleRecord) -> Result<(), ContractError> {
            Ok(())
        }
        fn list_by_channel(
            &self,
            _tenant_id: &str,
            _org_id: &str,
            _channel_id: i64,
            _cursor_created_at: Option<&str>,
            _cursor_rule_id: Option<i64>,
            _limit: i64,
        ) -> Result<Vec<ChannelAccessRuleRecord>, ContractError> {
            Ok(Vec::new())
        }
        fn delete(&self, _tenant_id: &str, _org_id: &str, _rule_id: i64) -> Result<(), ContractError> {
            Ok(())
        }
        fn effective_permission(
            &self,
            _tenant_id: &str,
            _org_id: &str,
            _channel_id: i64,
            _principal_kind: &str,
            _principal_id: &str,
            _permission: &str,
        ) -> Result<ChannelRuleDecision, ContractError> {
            Ok(self.decision)
        }
    }

    struct ScriptedSpaceMemberStore {
        member: Option<SpaceMemberRecord>,
    }

    impl SpaceMemberStore for ScriptedSpaceMemberStore {
        fn insert(&self, _record: &SpaceMemberRecord) -> Result<(), ContractError> {
            Ok(())
        }
        fn get_by_id(
            &self,
            _t: &str,
            _o: &str,
            _s: i64,
            _u: &str,
        ) -> Result<Option<SpaceMemberRecord>, ContractError> {
            Ok(self.member.clone())
        }
        fn list_by_space(
            &self,
            _t: &str,
            _o: &str,
            _s: i64,
            _cursor_joined_at: Option<&str>,
            _cursor_user_id: Option<&str>,
            _limit: i64,
        ) -> Result<Vec<SpaceMemberRecord>, ContractError> {
            Ok(Vec::new())
        }
        fn count_by_space(&self, _t: &str, _o: &str, _s: i64) -> Result<i64, ContractError> {
            Ok(0)
        }
        fn insert_within_capacity(
            &self,
            _record: &SpaceMemberRecord,
            _max_members: i32,
        ) -> Result<MemberInsertOutcome, ContractError> {
            Ok(MemberInsertOutcome::Inserted)
        }
        fn update(&self, _record: &SpaceMemberRecord) -> Result<(), ContractError> {
            Ok(())
        }
        fn delete(&self, _t: &str, _o: &str, _s: i64, _u: &str) -> Result<(), ContractError> {
            Ok(())
        }
        fn list_space_ids_by_user(
            &self,
            _t: &str,
            _o: &str,
            _u: &str,
            _limit: i64,
        ) -> Result<Vec<i64>, ContractError> {
            Ok(Vec::new())
        }
    }

    struct EmptyStores;

    impl SpaceStore for EmptyStores {
        fn insert(&self, _r: &SpaceRecord) -> Result<(), ContractError> {
            Ok(())
        }
        fn get_by_id(&self, _t: &str, _o: &str, _s: i64) -> Result<Option<SpaceRecord>, ContractError> {
            Ok(None)
        }
        fn list_by_owner(
            &self,
            _t: &str,
            _o: &str,
            _u: &str,
            _limit: i64,
        ) -> Result<Vec<SpaceRecord>, ContractError> {
            Ok(Vec::new())
        }
        fn list_accessible_by_user(
            &self,
            _t: &str,
            _o: &str,
            _u: &str,
            _cursor_created_at: Option<&str>,
            _cursor_space_id: Option<i64>,
            _limit: i64,
        ) -> Result<Vec<SpaceRecord>, ContractError> {
            Ok(Vec::new())
        }
        fn update(&self, _r: &SpaceRecord) -> Result<(), ContractError> {
            Ok(())
        }
        fn delete(&self, _t: &str, _o: &str, _s: i64) -> Result<(), ContractError> {
            Ok(())
        }
    }

    impl GroupStore for EmptyStores {
        fn insert(&self, _r: &GroupRecord) -> Result<(), ContractError> {
            Ok(())
        }
        fn insert_with_owner_member(
            &self,
            _g: &GroupRecord,
            _owner_member: &GroupMemberRecord,
        ) -> Result<(), ContractError> {
            Ok(())
        }
        fn get_by_id(&self, _t: &str, _o: &str, _g: i64) -> Result<Option<GroupRecord>, ContractError> {
            Ok(None)
        }
        fn list_by_space(
            &self,
            _t: &str,
            _o: &str,
            _s: i64,
            _cursor_created_at: Option<&str>,
            _cursor_group_id: Option<i64>,
            _limit: i64,
        ) -> Result<Vec<GroupRecord>, ContractError> {
            Ok(Vec::new())
        }
        fn list_by_owner(
            &self,
            _t: &str,
            _o: &str,
            _u: &str,
            _limit: i64,
        ) -> Result<Vec<GroupRecord>, ContractError> {
            Ok(Vec::new())
        }
        fn update(&self, _r: &GroupRecord) -> Result<(), ContractError> {
            Ok(())
        }
        fn transfer_owner(
            &self,
            _t: &str,
            _o: &str,
            _g: i64,
            _current_owner: &str,
            _new_owner: &str,
            _updated_at: &str,
        ) -> Result<GroupRecord, ContractError> {
            Err(ContractError::Unavailable("group not found".into()))
        }
        fn delete(&self, _t: &str, _o: &str, _g: i64) -> Result<(), ContractError> {
            Ok(())
        }
    }

    impl GroupMemberStore for EmptyStores {
        fn insert(&self, _r: &GroupMemberRecord) -> Result<(), ContractError> {
            Ok(())
        }
        fn get_by_id(
            &self,
            _t: &str,
            _o: &str,
            _g: i64,
            _u: &str,
        ) -> Result<Option<GroupMemberRecord>, ContractError> {
            Ok(None)
        }
        fn list_by_group(
            &self,
            _t: &str,
            _o: &str,
            _g: i64,
            _cursor_joined_at: Option<&str>,
            _cursor_user_id: Option<&str>,
            _limit: i64,
        ) -> Result<Vec<GroupMemberRecord>, ContractError> {
            Ok(Vec::new())
        }
        fn count_by_group(&self, _t: &str, _o: &str, _g: i64) -> Result<i64, ContractError> {
            Ok(0)
        }
        fn insert_within_capacity(
            &self,
            _record: &GroupMemberRecord,
            _max_members: i32,
        ) -> Result<MemberInsertOutcome, ContractError> {
            Ok(MemberInsertOutcome::Inserted)
        }
        fn update(&self, _r: &GroupMemberRecord) -> Result<(), ContractError> {
            Ok(())
        }
        fn delete(&self, _t: &str, _o: &str, _g: i64, _u: &str) -> Result<(), ContractError> {
            Ok(())
        }
    }

    impl BanStore for EmptyStores {
        fn insert(&self, _r: &BanRecord) -> Result<(), ContractError> {
            Ok(())
        }
        fn get_active_by_user(
            &self,
            _t: &str,
            _o: &str,
            _target_type: &str,
            _target_id: i64,
            _banned_user_id: &str,
        ) -> Result<Option<BanRecord>, ContractError> {
            Ok(None)
        }
        fn list_active_by_target(
            &self,
            _query: BanTargetListQuery,
        ) -> Result<Vec<BanRecord>, ContractError> {
            Ok(Vec::new())
        }
        fn update(&self, _record: &BanRecord) -> Result<(), ContractError> {
            Ok(())
        }
    }

    impl InvitationStore for EmptyStores {
        fn insert(&self, _r: &InvitationRecord) -> Result<(), ContractError> {
            Ok(())
        }
        fn get_by_id(
            &self,
            _t: &str,
            _o: &str,
            _invitation_id: i64,
        ) -> Result<Option<InvitationRecord>, ContractError> {
            Ok(None)
        }
        fn list_by_target(
            &self,
            _query: InvitationTargetListQuery,
        ) -> Result<Vec<InvitationRecord>, ContractError> {
            Ok(Vec::new())
        }
        fn update(&self, _record: &InvitationRecord) -> Result<(), ContractError> {
            Ok(())
        }
    }

    impl ChannelStore for EmptyStores {
        fn insert(&self, _r: &ChannelRecord) -> Result<(), ContractError> {
            Ok(())
        }
        fn get_by_id(&self, _t: &str, _o: &str, _c: i64) -> Result<Option<ChannelRecord>, ContractError> {
            Ok(None)
        }
        fn list_by_space(
            &self,
            _t: &str,
            _o: &str,
            _s: i64,
            _cursor_created_at: Option<&str>,
            _cursor_channel_id: Option<i64>,
            _limit: i64,
        ) -> Result<Vec<ChannelRecord>, ContractError> {
            Ok(Vec::new())
        }
        fn update(&self, _r: &ChannelRecord) -> Result<(), ContractError> {
            Ok(())
        }
        fn delete(&self, _t: &str, _o: &str, _c: i64) -> Result<(), ContractError> {
            Ok(())
        }
    }

    fn test_state(member: Option<SpaceMemberRecord>, decision: ChannelRuleDecision) -> AppState {
        AppState {
            postgres_pool: None,
            space_store: Arc::new(EmptyStores),
            group_store: Arc::new(EmptyStores),
            group_member_store: Arc::new(EmptyStores),
            space_member_store: Arc::new(ScriptedSpaceMemberStore { member }),
            ban_store: Arc::new(EmptyStores),
            invitation_store: Arc::new(EmptyStores),
            channel_access_rule_store: Arc::new(ScriptedRuleStore { decision }),
            channel_store: Arc::new(EmptyStores),
            id_generator: Arc::new(
                RuntimeSnowflakeIdGenerator::with_node_id(0)
                    .expect("snowflake node 0 must initialize"),
            ),
            group_conversation_binder: None,
            channel_conversation_binder: None,
            write_authority: None,
        }
    }

    fn member(role: &str) -> SpaceMemberRecord {
        SpaceMemberRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            space_id: 1,
            user_id: "user-2".into(),
            role: role.into(),
            nickname: None,
            joined_at: "2026-01-01T00:00:00.000Z".into(),
            updated_at: "2026-01-01T00:00:00.000Z".into(),
        }
    }

    #[test]
    fn owner_and_admin_bypass_deny_rules() {
        let owner_state = test_state(None, ChannelRuleDecision::Deny);
        assert!(enforce_channel_permission(
            &owner_state,
            &test_auth("user-1"),
            &test_space("user-1"),
            7,
            "manage",
        )
        .is_ok());

        let admin_state = test_state(Some(member("admin")), ChannelRuleDecision::Deny);
        assert!(enforce_channel_permission(
            &admin_state,
            &test_auth("user-2"),
            &test_space("user-1"),
            7,
            "view",
        )
        .is_ok());
    }

    #[test]
    fn member_deny_rule_forbids_operation() {
        let state = test_state(Some(member("member")), ChannelRuleDecision::Deny);
        let error = enforce_channel_permission(
            &state,
            &test_auth("user-2"),
            &test_space("user-1"),
            7,
            "view",
        )
        .expect_err("deny rule must forbid the member operation");
        assert!(
            error.message.contains("forbids"),
            "deny must surface the rule-forbidden problem, got: {}",
            error.message
        );
    }

    #[test]
    fn member_allow_rule_and_no_rule_permit_operation() {
        for decision in [ChannelRuleDecision::Allow, ChannelRuleDecision::NoRule] {
            let state = test_state(Some(member("member")), decision);
            assert!(enforce_channel_permission(
                &state,
                &test_auth("user-2"),
                &test_space("user-1"),
                7,
                "view",
            )
            .is_ok());
        }
    }
}
