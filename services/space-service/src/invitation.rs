//! Invitation API handlers.

use axum::Json;
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use im_adapters_social_postgres::MemberInsertOutcome;
use im_adapters_social_postgres::governance_store::{
    InvitationRecord, InvitationTargetListQuery, SpaceMemberRecord,
};
use im_app_context::AppContext;
use im_time::{rfc3339_le, utc_now_rfc3339_millis};
use sdkwork_routes_web_framework_backend_api::response::{
    ApiProblem, ApiResult, finish_api_json, finish_api_response, no_content,
};
use sdkwork_utils_rust::{SdkWorkPageData, SdkWorkResourceData};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};

use crate::api_payload::{keyset_list_page, resource_item};
use crate::http::AppState;
use crate::id::next_entity_id;
use crate::list_query::{ListQuery, resolve_keyset_page};
use crate::space_access::{
    actor_can_manage_space, actor_can_read_space, ensure_user_not_banned_in_space, load_space,
    normalize_space_member_role, parse_entity_id, parse_space_id,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvitationRequest {
    pub invitee_user_id: Option<String>,
    pub invitee_email: Option<String>,
    pub invitee_phone: Option<String>,
    pub target_type: String,
    pub target_id: String,
    pub role: Option<String>,
    pub message: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvitationResponse {
    pub invitation_id: String,
    pub inviter_user_id: String,
    pub invitee_user_id: Option<String>,
    pub target_type: String,
    pub target_id: String,
    pub status: String,
    pub created_at: String,
}

impl From<InvitationRecord> for InvitationResponse {
    fn from(record: InvitationRecord) -> Self {
        Self {
            invitation_id: record.invitation_id.to_string(),
            inviter_user_id: record.inviter_user_id,
            invitee_user_id: record.invitee_user_id,
            target_type: record.target_type,
            target_id: record.target_id.to_string(),
            status: record.status,
            created_at: record.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ListInvitationsQuery {
    pub status: Option<String>,
    #[serde(flatten)]
    pub paging: ListQuery,
}

fn normalize_invitation_target_type(target_type: &str) -> Result<String, ApiProblem> {
    match target_type {
        "space" | "group" | "channel" => Ok(target_type.to_owned()),
        other => {
            tracing::warn!(target_type = other, "invalid invitation target_type");
            Err(ApiProblem::bad_request("invalid invitation target_type"))
        }
    }
}

fn ensure_invitee_specified(request: &CreateInvitationRequest) -> Result<(), ApiProblem> {
    if request
        .invitee_user_id
        .as_deref()
        .unwrap_or("")
        .trim()
        .is_empty()
        && request
            .invitee_email
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
        && request
            .invitee_phone
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
    {
        return Err(ApiProblem::bad_request(
            "invitee_user_id, invitee_email, or invitee_phone is required",
        ));
    }
    Ok(())
}

fn load_invitation_for_space(
    state: &AppState,
    auth: &AppContext,
    space_id: i64,
    invitation_id: i64,
) -> Result<InvitationRecord, ApiProblem> {
    let invitation = state
        .invitation_store
        .get_by_id(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            invitation_id,
        )
        .map_err(|error| {
            tracing::error!(error = ?error, "failed to load invitation");
            ApiProblem::internal_server_error("failed to load invitation")
        })?
        .ok_or_else(|| ApiProblem::not_found("invitation not found"))?;

    if invitation.target_type != "space" || invitation.target_id != space_id {
        return Err(ApiProblem::not_found("invitation not found"));
    }
    Ok(invitation)
}

fn ensure_invitation_actor(
    auth: &AppContext,
    invitation: &InvitationRecord,
    for_manage: bool,
) -> Result<(), ApiProblem> {
    if for_manage && invitation.inviter_user_id == auth.actor_id {
        return Ok(());
    }
    if !for_manage {
        if invitation.invitee_user_id.as_deref() == Some(auth.actor_id.as_str()) {
            return Ok(());
        }
        if invitation.inviter_user_id == auth.actor_id {
            return Ok(());
        }
    }
    Err(ApiProblem::forbidden("invitation access denied"))
}

pub async fn create_invitation(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
    Json(request): Json<CreateInvitationRequest>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<InvitationResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_manage_space(&state, &auth, &space)?;
        ensure_invitee_specified(&request)?;

        let target_type = normalize_invitation_target_type(request.target_type.as_str())?;
        let target_id = parse_entity_id(request.target_id.as_str(), "target_id")?;
        if target_type == "space" && target_id != space_id {
            return Err(ApiProblem::bad_request(
                "space invitation target_id must match path space_id",
            ));
        }

        let role = normalize_space_member_role(request.role.as_deref(), false)?;
        let now = chrono::Utc::now().to_rfc3339();
        let record = InvitationRecord {
            tenant_id: auth.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
            invitation_id: next_entity_id(&state.id_generator)?,
            inviter_user_id: auth.actor_id.clone(),
            invitee_user_id: request.invitee_user_id,
            invitee_email: request.invitee_email,
            invitee_phone: request.invitee_phone,
            target_type: target_type.clone(),
            target_id,
            role,
            status: "pending".to_owned(),
            message: request.message,
            expires_at: request.expires_at,
            accepted_at: None,
            created_at: now.clone(),
            updated_at: now,
        };

        state.invitation_store.insert(&record).map_err(|error| {
            tracing::error!(error = ?error, "failed to insert invitation");
            ApiProblem::internal_server_error("failed to create invitation")
        })?;
        Ok(resource_item(InvitationResponse::from(record)))
    })();
    finish_api_json(&ctx, result)
}

pub async fn list_invitations(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(space_id): Path<String>,
    Query(query): Query<ListInvitationsQuery>,
) -> Response {
    let result: ApiResult<SdkWorkPageData<InvitationResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_manage_space(&state, &auth, &space)?;
        let paging = resolve_keyset_page(&query.paging)?;
        let cursor_invitation_id = paging
            .cursor_entity
            .as_deref()
            .and_then(|s| s.parse::<i64>().ok());

        let records = state
            .invitation_store
            .list_by_target(InvitationTargetListQuery {
                tenant_id: auth.tenant_id.as_str(),
                organization_id: auth.organization_id.as_str(),
                target_type: "space",
                target_id: space_id,
                status: query.status.as_deref(),
                limit: paging.fetch_limit(),
                cursor_created_at: paging.cursor_sort_value.as_deref(),
                cursor_invitation_id,
            })
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to list invitations");
                ApiProblem::internal_server_error("failed to list invitations")
            })?;

        let items = records.into_iter().map(InvitationResponse::from).collect();
        Ok(keyset_list_page(
            items,
            paging.page_size,
            |item: &InvitationResponse| (item.created_at.clone(), item.invitation_id.clone()),
        ))
    })();
    finish_api_json(&ctx, result)
}

pub async fn get_invitation(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, invite_code)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<InvitationResponse>> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let invitation_id = parse_entity_id(invite_code.as_str(), "invite_code")?;
        let _space = load_space(&state, &auth, space_id)?;
        actor_can_read_space(&state, &auth, &_space)?;
        let invitation = load_invitation_for_space(&state, &auth, space_id, invitation_id)?;
        ensure_invitation_actor(&auth, &invitation, false)?;
        Ok(resource_item(InvitationResponse::from(invitation)))
    })();
    finish_api_json(&ctx, result)
}

pub async fn revoke_invitation(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, invite_code)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<()> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let invitation_id = parse_entity_id(invite_code.as_str(), "invite_code")?;
        let space = load_space(&state, &auth, space_id)?;
        actor_can_manage_space(&state, &auth, &space)?;
        let mut invitation = load_invitation_for_space(&state, &auth, space_id, invitation_id)?;
        if invitation.status != "pending" {
            return Err(ApiProblem::bad_request("invitation is not pending"));
        }
        invitation.status = "canceled".to_owned();
        invitation.updated_at = chrono::Utc::now().to_rfc3339();
        state
            .invitation_store
            .update(&invitation)
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to revoke invitation");
                ApiProblem::internal_server_error("failed to revoke invitation")
            })?;
        Ok(())
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}

pub async fn accept_invitation(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((space_id, invite_code)): Path<(String, String)>,
) -> Response {
    let result: ApiResult<()> = (|| {
        let space_id = parse_space_id(space_id.as_str())?;
        let invitation_id = parse_entity_id(invite_code.as_str(), "invite_code")?;
        let space = load_space(&state, &auth, space_id)?;
        let mut invitation = load_invitation_for_space(&state, &auth, space_id, invitation_id)?;
        ensure_invitation_actor(&auth, &invitation, false)?;

        if invitation.status != "pending" {
            return Err(ApiProblem::bad_request("invitation is not pending"));
        }
        if invitation
            .expires_at
            .as_deref()
            .is_some_and(|expires_at| rfc3339_le(expires_at, utc_now_rfc3339_millis().as_str()))
        {
            return Err(ApiProblem::bad_request("invitation has expired"));
        }
        if invitation
            .invitee_user_id
            .as_deref()
            .is_some_and(|user_id| user_id != auth.actor_id)
        {
            return Err(ApiProblem::forbidden(
                "invitation is not addressed to this user",
            ));
        }

        ensure_user_not_banned_in_space(&state, &auth, space_id, auth.actor_id.as_str())?;

        let now = chrono::Utc::now().to_rfc3339();
        let member = SpaceMemberRecord {
            tenant_id: auth.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
            space_id,
            user_id: auth.actor_id.clone(),
            role: invitation.role.clone(),
            nickname: None,
            joined_at: now.clone(),
            updated_at: now.clone(),
        };
        match state
            .space_member_store
            .insert_within_capacity(&member, space.max_members)
        {
            Ok(MemberInsertOutcome::Inserted | MemberInsertOutcome::AlreadyExists) => {}
            Ok(MemberInsertOutcome::CapacityFull) => {
                return Err(ApiProblem::bad_request("space member limit reached"));
            }
            Err(error) => {
                tracing::error!(error = ?error, "failed to insert space member from invitation");
                return Err(ApiProblem::internal_server_error(
                    "failed to accept invitation",
                ));
            }
        }

        invitation.status = "accepted".to_owned();
        invitation.accepted_at = Some(now.clone());
        invitation.updated_at = now;
        state
            .invitation_store
            .update(&invitation)
            .map_err(|error| {
                tracing::error!(error = ?error, "failed to mark invitation accepted");
                ApiProblem::internal_server_error("failed to accept invitation")
            })?;
        Ok(())
    })();
    finish_api_response(&ctx, result.and_then(|_| no_content(&ctx)))
}
