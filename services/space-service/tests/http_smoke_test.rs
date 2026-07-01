use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_adapters_social_postgres::organization_store::{
    ChannelRecord, ChannelStore, GroupRecord, GroupStore, SpaceRecord, SpaceStore,
};
use sdkwork_im_contract_core::ContractError;
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

    fn delete(&self, _tenant_id: &str, _org_id: &str, _group_id: i64) -> Result<(), ContractError> {
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

fn test_app_state() -> AppState {
    AppState {
        postgres_pool: None,
        space_store: Arc::new(NoopSpaceStore),
        group_store: Arc::new(NoopGroupStore),
        channel_store: Arc::new(NoopChannelStore),
        id_generator: Arc::new(
            RuntimeSnowflakeIdGenerator::with_node_id(0).expect("snowflake node 0 must initialize"),
        ),
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
