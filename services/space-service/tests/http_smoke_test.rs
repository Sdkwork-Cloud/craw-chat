use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_adapters_social_postgres::governance_store::{
    BanRecord, BanStore, BanTargetListQuery, ChannelAccessRuleRecord, ChannelAccessRuleStore,
    InvitationRecord, InvitationStore, InvitationTargetListQuery, SpaceMemberRecord,
    SpaceMemberStore,
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
        _cursor_created_at: Option<&str>,
        _cursor_space_id: Option<i64>,
        _limit: i64,
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
        _cursor_created_at: Option<&str>,
        _cursor_group_id: Option<i64>,
        _limit: i64,
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
        _cursor_joined_at: Option<&str>,
        _cursor_user_id: Option<&str>,
        _limit: i64,
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
        _cursor_created_at: Option<&str>,
        _cursor_channel_id: Option<i64>,
        _limit: i64,
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
        _cursor_joined_at: Option<&str>,
        _cursor_user_id: Option<&str>,
        _limit: i64,
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
        _query: BanTargetListQuery<'_>,
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
        _query: InvitationTargetListQuery<'_>,
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
    ) -> Result<im_adapters_social_postgres::governance_store::ChannelRuleDecision, ContractError>
    {
        Ok(im_adapters_social_postgres::governance_store::ChannelRuleDecision::NoRule)
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

// ---------------------------------------------------------------------------
// Invitation API tests — contract/implementation alignment coverage.
// ---------------------------------------------------------------------------

use std::sync::Mutex;

use im_adapters_social_postgres::member_capacity::MemberInsertOutcome;
use sdkwork_web_core::{
    ServerRequestId, WebApiSurface, WebAuthMode, WebRequestContext, WebTransportFacts,
};

const INVITE_TENANT: &str = "tenant-invite";
const INVITE_OWNER: &str = "user-owner";
const INVITE_SPACE_ID: i64 = 1001;

struct ScriptedSpaceStore;

impl SpaceStore for ScriptedSpaceStore {
    fn insert(&self, _record: &SpaceRecord) -> Result<(), ContractError> {
        Ok(())
    }

    fn get_by_id(
        &self,
        tenant_id: &str,
        org_id: &str,
        space_id: i64,
    ) -> Result<Option<SpaceRecord>, ContractError> {
        if tenant_id != INVITE_TENANT || space_id != INVITE_SPACE_ID {
            return Ok(None);
        }
        Ok(Some(SpaceRecord {
            tenant_id: tenant_id.to_owned(),
            organization_id: org_id.to_owned(),
            space_id,
            space_name: "invite-space".to_owned(),
            space_type: "organization".to_owned(),
            owner_user_id: INVITE_OWNER.to_owned(),
            description: None,
            avatar_url: None,
            max_members: 100,
            settings_json: "{}".to_owned(),
            created_at: "2026-01-01T00:00:00.000Z".to_owned(),
            updated_at: "2026-01-01T00:00:00.000Z".to_owned(),
        }))
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
        _cursor_created_at: Option<&str>,
        _cursor_space_id: Option<i64>,
        _limit: i64,
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

#[derive(Default)]
struct ScriptedInvitationStore {
    records: Mutex<Vec<InvitationRecord>>,
}

impl ScriptedInvitationStore {
    fn insert_record(&self, record: InvitationRecord) {
        self.records
            .lock()
            .expect("invitation store lock")
            .push(record);
    }
}

impl InvitationStore for ScriptedInvitationStore {
    fn insert(&self, record: &InvitationRecord) -> Result<(), ContractError> {
        self.insert_record(record.clone());
        Ok(())
    }

    fn get_by_id(
        &self,
        _tenant_id: &str,
        _org_id: &str,
        invitation_id: i64,
    ) -> Result<Option<InvitationRecord>, ContractError> {
        Ok(self
            .records
            .lock()
            .expect("invitation store lock")
            .iter()
            .find(|record| record.invitation_id == invitation_id)
            .cloned())
    }

    fn list_by_target(
        &self,
        _query: InvitationTargetListQuery<'_>,
    ) -> Result<Vec<InvitationRecord>, ContractError> {
        Ok(Vec::new())
    }

    fn update(&self, record: &InvitationRecord) -> Result<(), ContractError> {
        let mut records = self.records.lock().expect("invitation store lock");
        if let Some(existing) = records
            .iter_mut()
            .find(|existing| existing.invitation_id == record.invitation_id)
        {
            *existing = record.clone();
        }
        Ok(())
    }
}

struct ScriptedSpaceMemberStore;

impl SpaceMemberStore for ScriptedSpaceMemberStore {
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
        _cursor_joined_at: Option<&str>,
        _cursor_user_id: Option<&str>,
        _limit: i64,
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
    ) -> Result<MemberInsertOutcome, ContractError> {
        Ok(MemberInsertOutcome::Inserted)
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

struct NoActiveBanStore;

impl BanStore for NoActiveBanStore {
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
        _query: BanTargetListQuery<'_>,
    ) -> Result<Vec<BanRecord>, ContractError> {
        Ok(Vec::new())
    }

    fn update(&self, _record: &BanRecord) -> Result<(), ContractError> {
        Ok(())
    }
}

fn invitation_app_state(invitation_store: Arc<ScriptedInvitationStore>) -> AppState {
    AppState {
        postgres_pool: None,
        space_store: Arc::new(ScriptedSpaceStore),
        group_store: Arc::new(NoopGroupStore),
        group_member_store: Arc::new(NoopGroupMemberStore),
        space_member_store: Arc::new(ScriptedSpaceMemberStore),
        ban_store: Arc::new(NoActiveBanStore),
        invitation_store,
        channel_access_rule_store: Arc::new(NoopChannelAccessRuleStore),
        channel_store: Arc::new(NoopChannelStore),
        id_generator: Arc::new(
            RuntimeSnowflakeIdGenerator::with_node_id(1).expect("snowflake node 1 must initialize"),
        ),
        group_conversation_binder: None,
        channel_conversation_binder: None,
        write_authority: None,
    }
}

fn invitation_request(method: &str, uri: &str, body: &str, actor_user_id: &str) -> Request<Body> {
    let mut request = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_owned()))
        .expect("invitation request should build");
    request.extensions_mut().insert(WebRequestContext {
        request_id: ServerRequestId("request-invitation".to_owned()),
        api_surface: WebApiSurface::OpenApi,
        auth_mode: WebAuthMode::DualToken,
        principal: None,
        transport: WebTransportFacts {
            path: uri.to_owned(),
            method: method.to_owned(),
            auth_token_present: true,
            access_token_present: true,
            api_key_present: false,
            ingress_token_present: false,
            oauth_bearer_present: false,
            agent_token_present: false,
        },
        locale: None,
        client_kind: None,
        operation: None,
        trace_id: Some("trace-invitation".to_owned()),
        idempotency_key: None,
    });
    request
        .extensions_mut()
        .insert(im_app_context::local_service_app_context(
            INVITE_TENANT,
            actor_user_id,
            "user",
            None,
            ["conversation.read"],
        ));
    request
}

async fn invitation_response_body(response: axum::response::Response) -> serde_json::Value {
    let body = response
        .into_body()
        .collect()
        .await
        .expect("invitation response body should collect")
        .to_bytes();
    serde_json::from_slice(body.as_ref()).expect("invitation response body should be valid json")
}

fn seed_invitation(
    store: &ScriptedInvitationStore,
    invitation_id: i64,
    invitee_user_id: Option<&str>,
    status: &str,
) -> InvitationRecord {
    let now = "2026-01-01T00:00:00.000Z".to_owned();
    let record = InvitationRecord {
        tenant_id: INVITE_TENANT.to_owned(),
        organization_id: "0".to_owned(),
        invitation_id,
        inviter_user_id: INVITE_OWNER.to_owned(),
        invitee_user_id: invitee_user_id.map(str::to_owned),
        invitee_email: None,
        invitee_phone: None,
        target_type: "space".to_owned(),
        target_id: INVITE_SPACE_ID,
        role: "member".to_owned(),
        status: status.to_owned(),
        message: None,
        expires_at: None,
        accepted_at: None,
        created_at: now.clone(),
        updated_at: now,
        retention_until: Some("2027-01-01T00:00:00.000Z".to_owned()),
    };
    store.insert_record(record.clone());
    record
}

#[tokio::test]
async fn create_invitation_rejects_non_space_target() {
    let invitation_store = Arc::new(ScriptedInvitationStore::default());
    let app = build_app(invitation_app_state(invitation_store));

    let response = app
        .oneshot(invitation_request(
            "POST",
            &format!("/im/v3/api/spaces/{INVITE_SPACE_ID}/invites"),
            r#"{"inviteeUserId":"user-b","targetType":"group","targetId":"1"}"#,
            INVITE_OWNER,
        ))
        .await
        .expect("invitation create should respond");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_invitation_rejects_past_expiry() {
    let invitation_store = Arc::new(ScriptedInvitationStore::default());
    let app = build_app(invitation_app_state(invitation_store));

    let response = app
        .oneshot(invitation_request(
            "POST",
            &format!("/im/v3/api/spaces/{INVITE_SPACE_ID}/invites"),
            r#"{"inviteeUserId":"user-b","targetType":"space","targetId":"1001","expiresAt":"2020-01-01T00:00:00.000Z"}"#,
            INVITE_OWNER,
        ))
        .await
        .expect("invitation create should respond");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_invitation_returns_201_with_invitation_id() {
    let invitation_store = Arc::new(ScriptedInvitationStore::default());
    let app = build_app(invitation_app_state(invitation_store.clone()));

    let response = app
        .oneshot(invitation_request(
            "POST",
            &format!("/im/v3/api/spaces/{INVITE_SPACE_ID}/invites"),
            r#"{"inviteeUserId":"user-b","targetType":"space","targetId":"1001","role":"admin"}"#,
            INVITE_OWNER,
        ))
        .await
        .expect("invitation create should respond");

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = invitation_response_body(response).await;
    assert_eq!(body["code"], 0);
    let item = &body["data"]["item"];
    assert!(item["invitationId"].is_string());
    assert_eq!(item["targetType"], "space");
    assert_eq!(item["status"], "pending");
    assert_eq!(item["role"], "admin");
    assert_eq!(item["inviterUserId"], INVITE_OWNER);

    let stored = invitation_store
        .records
        .lock()
        .expect("invitation store lock");
    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].invitee_user_id.as_deref(), Some("user-b"));
    assert!(
        stored[0].retention_until.is_some(),
        "created invitation must carry a retention window"
    );
}

#[tokio::test]
async fn get_invitation_available_to_non_member_invitee() {
    let invitation_store = Arc::new(ScriptedInvitationStore::default());
    seed_invitation(&invitation_store, 555, Some("user-b"), "pending");
    let app = build_app(invitation_app_state(invitation_store));

    let response = app
        .oneshot(invitation_request(
            "GET",
            &format!("/im/v3/api/spaces/{INVITE_SPACE_ID}/invites/555"),
            "",
            "user-b",
        ))
        .await
        .expect("invitation retrieve should respond");

    assert_eq!(response.status(), StatusCode::OK);
    let body = invitation_response_body(response).await;
    assert_eq!(body["data"]["item"]["invitationId"], "555");
    assert_eq!(body["data"]["item"]["inviteeUserId"], "user-b");
}

#[tokio::test]
async fn get_invitation_denied_to_unrelated_user() {
    let invitation_store = Arc::new(ScriptedInvitationStore::default());
    seed_invitation(&invitation_store, 555, Some("user-b"), "pending");
    let app = build_app(invitation_app_state(invitation_store));

    let response = app
        .oneshot(invitation_request(
            "GET",
            &format!("/im/v3/api/spaces/{INVITE_SPACE_ID}/invites/555"),
            "",
            "user-unrelated",
        ))
        .await
        .expect("invitation retrieve should respond");

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn accept_invitation_returns_command_success_and_replays_idempotently() {
    let invitation_store = Arc::new(ScriptedInvitationStore::default());
    seed_invitation(&invitation_store, 777, Some("user-b"), "pending");
    let app = build_app(invitation_app_state(invitation_store.clone()));

    let first = app
        .clone()
        .oneshot(invitation_request(
            "POST",
            &format!("/im/v3/api/spaces/{INVITE_SPACE_ID}/invites/777/accept"),
            "",
            "user-b",
        ))
        .await
        .expect("invitation accept should respond");
    assert_eq!(first.status(), StatusCode::OK);
    let body = invitation_response_body(first).await;
    assert_eq!(body["code"], 0);
    assert_eq!(body["data"]["accepted"], true);
    assert_eq!(body["data"]["resourceId"], "1001");
    assert_eq!(body["data"]["status"], "accepted");

    let replayed = app
        .oneshot(invitation_request(
            "POST",
            &format!("/im/v3/api/spaces/{INVITE_SPACE_ID}/invites/777/accept"),
            "",
            "user-b",
        ))
        .await
        .expect("invitation accept replay should respond");
    assert_eq!(
        replayed.status(),
        StatusCode::OK,
        "repeated accept of an accepted invitation must replay idempotently"
    );
}
