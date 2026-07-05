use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_adapters_social_postgres::governance_store::{
    BanRecord, BanStore, ChannelAccessRuleRecord, ChannelAccessRuleStore, InvitationRecord,
    InvitationStore, SpaceMemberRecord, SpaceMemberStore,
};
use im_adapters_social_postgres::organization_store::{
    ChannelRecord, ChannelStore, GroupMemberRecord, GroupMemberStore, GroupRecord, GroupStore,
    SpaceRecord, SpaceStore,
};
use im_platform_contracts::ContractError;
use sdkwork_im_runtime_id::RuntimeSnowflakeIdGenerator;
use space_service::http::{AppState, build_app};
use tower::ServiceExt;

struct NoopSpaceStore;

impl SpaceStore for NoopSpaceStore {
    fn insert(&self, _record: &SpaceRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn get_by_id(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _space_id: i64,
    ) -> Result<Option<SpaceRecord>, ContractError> {
        Ok(None)
    }

    fn list_by_owner(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _owner_user_id: &str,
        _limit: i64,
    ) -> Result<Vec<SpaceRecord>, ContractError> {
        Ok(Vec::new())
    }

    fn list_accessible_by_user(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _user_id: &str,
        _limit: i64,
        _offset: i64,
    ) -> Result<Vec<SpaceRecord>, ContractError> {
        Ok(Vec::new())
    }

    fn update(&self, _record: &SpaceRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn delete(&self, _tenant_id: &str, _org_id: &str, _space_id: i64) -> Result<(), ContractError> {
        Ok(())
    }
}

struct NoopGroupStore;

impl GroupStore for NoopGroupStore {
    fn insert(&self, _record: &GroupRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn insert_with_owner_member(
        &self,
        _group: &GroupRecord,
        _owner_member: &GroupMemberRecord,
    ) -> Result<(), ContractError> {
        Ok(())
    }

    fn get_by_id(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _group_id: i64,
    ) -> Result<Option<GroupRecord>, ContractError> {
        Ok(None)
    }

    fn list_by_space(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _space_id: i64,
        _limit: i64,
        _offset: i64,
    ) -> Result<Vec<GroupRecord>, ContractError> {
        Ok(Vec::new())
    }

    fn list_by_owner(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _owner_user_id: &str,
        _limit: i64,
    ) -> Result<Vec<GroupRecord>, ContractError> {
        Ok(Vec::new())
    }

    fn update(&self, _record: &GroupRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn transfer_owner(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _group_id: i64,
        _current_owner_user_id: &str,
        _new_owner_user_id: &str,
        _updated_at: &str,
    ) -> Result<GroupRecord, ContractError> {
        Err(ContractError::UnsupportedCapability(
            "noop group store".to_owned(),
        ))
    }

    fn delete(&self, _tenant_id: &str, _org_id: &str, _group_id: i64) -> Result<(), ContractError> {
        Ok(())
    }
}

struct NoopGroupMemberStore;

impl GroupMemberStore for NoopGroupMemberStore {
    fn insert(&self, _record: &GroupMemberRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn get_by_id(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _group_id: i64,
        _user_id: &str,
    ) -> Result<Option<GroupMemberRecord>, ContractError> {
        Ok(None)
    }

    fn list_by_group(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _group_id: i64,
        _limit: i64,
        _offset: i64,
    ) -> Result<Vec<GroupMemberRecord>, ContractError> {
        Ok(Vec::new())
    }

    fn count_by_group(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _group_id: i64,
    ) -> Result<i64, ContractError> {
        Ok(0)
    }

    fn insert_within_capacity(
        &self,
        _record: &GroupMemberRecord,
        _max_members: i32,
    ) -> Result<im_adapters_social_postgres::MemberInsertOutcome, ContractError> {
        Ok(im_adapters_social_postgres::MemberInsertOutcome::Inserted)
    }

    fn update(&self, _record: &GroupMemberRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn delete(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _group_id: i64,
        _user_id: &str,
    ) -> Result<(), ContractError> {
        Ok(())
    }
}

struct NoopChannelStore;

impl ChannelStore for NoopChannelStore {
    fn insert(&self, _record: &ChannelRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn get_by_id(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _channel_id: i64,
    ) -> Result<Option<ChannelRecord>, ContractError> {
        Ok(None)
    }

    fn list_by_space(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _space_id: i64,
        _limit: i64,
        _offset: i64,
    ) -> Result<Vec<ChannelRecord>, ContractError> {
        Ok(Vec::new())
    }

    fn update(&self, _record: &ChannelRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn delete(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _channel_id: i64,
    ) -> Result<(), ContractError> {
        Ok(())
    }
}

struct NoopSpaceMemberStore;

impl SpaceMemberStore for NoopSpaceMemberStore {
    fn insert(&self, _record: &SpaceMemberRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn get_by_id(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _space_id: i64,
        _user_id: &str,
    ) -> Result<Option<SpaceMemberRecord>, ContractError> {
        Ok(None)
    }

    fn list_by_space(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _space_id: i64,
        _limit: i64,
        _offset: i64,
    ) -> Result<Vec<SpaceMemberRecord>, ContractError> {
        Ok(Vec::new())
    }

    fn count_by_space(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _space_id: i64,
    ) -> Result<i64, ContractError> {
        Ok(0)
    }

    fn insert_within_capacity(
        &self,
        _record: &SpaceMemberRecord,
        _max_members: i32,
    ) -> Result<im_adapters_social_postgres::MemberInsertOutcome, ContractError> {
        Ok(im_adapters_social_postgres::MemberInsertOutcome::Inserted)
    }

    fn update(&self, _record: &SpaceMemberRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn delete(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _space_id: i64,
        _user_id: &str,
    ) -> Result<(), ContractError> {
        Ok(())
    }

    fn list_space_ids_by_user(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _user_id: &str,
        _limit: i64,
    ) -> Result<Vec<i64>, ContractError> {
        Ok(Vec::new())
    }
}

struct NoopBanStore;

impl BanStore for NoopBanStore {
    fn insert(&self, _record: &BanRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn get_active_by_user(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _target_type: &str,
        _target_id: i64,
        _banned_user_id: &str,
    ) -> Result<Option<BanRecord>, ContractError> {
        Ok(None)
    }

    fn list_active_by_target(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _target_type: &str,
        _target_id: i64,
        _limit: i64,
        _offset: i64,
    ) -> Result<Vec<BanRecord>, ContractError> {
        Ok(Vec::new())
    }

    fn update(&self, _record: &BanRecord) -> Result<(), ContractError> {
        Ok(())
    }
}

struct NoopInvitationStore;

impl InvitationStore for NoopInvitationStore {
    fn insert(&self, _record: &InvitationRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn get_by_id(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _invitation_id: i64,
    ) -> Result<Option<InvitationRecord>, ContractError> {
        Ok(None)
    }

    fn list_by_target(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _target_type: &str,
        _target_id: i64,
        _status: Option<&str>,
        _limit: i64,
        _offset: i64,
    ) -> Result<Vec<InvitationRecord>, ContractError> {
        Ok(Vec::new())
    }

    fn update(&self, _record: &InvitationRecord) -> Result<(), ContractError> {
        Ok(())
    }
}

struct NoopChannelAccessRuleStore;

impl ChannelAccessRuleStore for NoopChannelAccessRuleStore {
    fn insert(&self, _record: &ChannelAccessRuleRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn list_by_channel(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        _channel_id: i64,
        _limit: i64,
        _offset: i64,
    ) -> Result<Vec<ChannelAccessRuleRecord>, ContractError> {
        Ok(Vec::new())
    }

    fn delete(&self, _tenant_id: &str, _org_id: &str, _rule_id: i64) -> Result<(), ContractError> {
        Ok(())
    }
}

fn test_app_state() -> AppState {
    AppState {
        postgres_pool: None,
        space_store: Arc::new(NoopSpaceStore),
        group_store: Arc::new(NoopGroupStore),
        group_member_store: Arc::new(NoopGroupMemberStore),
        space_member_store: Arc::new(NoopSpaceMemberStore),
        ban_store: Arc::new(NoopBanStore),
        invitation_store: Arc::new(NoopInvitationStore),
        channel_access_rule_store: Arc::new(NoopChannelAccessRuleStore),
        channel_store: Arc::new(NoopChannelStore),
        id_generator: Arc::new(
            RuntimeSnowflakeIdGenerator::with_node_id(0).expect("snowflake node 0 must initialize"),
        ),
        group_conversation_binder: None,
        channel_conversation_binder: None,
        write_authority: None,
    }
}

#[tokio::test]
async fn test_healthz_returns_ok() {
    let app = build_app(test_app_state());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .expect("healthz request should build"),
        )
        .await
        .expect("healthz request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("healthz body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(body.as_ref()).expect("healthz body should be valid json");
    assert_eq!(value["status"], "ok");
}

#[tokio::test]
async fn test_readyz_returns_service_readiness_status() {
    let app = build_app(test_app_state());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .expect("readyz request should build"),
        )
        .await
        .expect("readyz request should succeed");

    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::SERVICE_UNAVAILABLE
    );
}
