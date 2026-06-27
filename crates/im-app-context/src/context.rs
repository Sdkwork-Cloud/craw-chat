use std::collections::BTreeSet;

use axum::http::{HeaderMap, HeaderValue, header};
use sdkwork_im_ccp_core::{CcpActor, CcpAuthority, CcpSender};
use sdkwork_web_core::{
    ServerRequestId, WebAuthMode, WebLoginScope, WebRequestContext, WebRequestContextProfile,
    WebRequestPrincipal, WebSubjectType, WebTransportFacts, classify_api_surface, new_request_id,
};
use serde_json::json;

use crate::env::{
    APP_CONTEXT_JWT_TENANT_ID_ENV, APP_CONTEXT_REQUIRE_SIGNATURE_ENV,
    APP_CONTEXT_SIGNATURE_SECRET_ENV, APP_CONTEXT_SIGNATURE_SECRET_FILE_ENV,
    format_auth_level, format_deployment_mode, format_environment, parse_auth_level,
    parse_deployment_mode, parse_environment, parse_truthy_env_flag, resolve_secret_from_env_or_file,
};
use crate::error::AppContextError;
use crate::headers::{extract_access_token, extract_bearer_token, require_app_context_signature};
use crate::jwt::{TokenClaims, encode_local_jwt_claims};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppContext {
    pub tenant_id: String,
    pub organization_id: String,
    pub user_id: String,
    pub session_id: Option<String>,
    pub app_id: Option<String>,
    pub environment: Option<String>,
    pub deployment_mode: Option<String>,
    pub auth_level: Option<String>,
    pub data_scope: BTreeSet<String>,
    pub permission_scope: BTreeSet<String>,
    pub actor_id: String,
    pub actor_kind: String,
    pub device_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedAppContext {
    pub app_request_context: WebRequestContext,
    pub app_context: AppContext,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppContextSignatureConfig {
    pub require_signature: bool,
    pub shared_secret: Option<String>,
}

impl AppContextSignatureConfig {
    pub fn from_env() -> Self {
        Self {
            require_signature: parse_truthy_env_flag(
                std::env::var(APP_CONTEXT_REQUIRE_SIGNATURE_ENV).ok(),
            ),
            shared_secret: resolve_secret_from_env_or_file(
                APP_CONTEXT_SIGNATURE_SECRET_ENV,
                APP_CONTEXT_SIGNATURE_SECRET_FILE_ENV,
            ),
        }
    }
}

/// Tenant, organization, and user scope extracted from an authenticated request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppRequestScope {
    pub tenant_id: String,
    pub organization_id: String,
    pub user_id: String,
}

impl From<AppContext> for AppRequestScope {
    fn from(auth: AppContext) -> Self {
        Self {
            tenant_id: auth.tenant_id,
            organization_id: auth.organization_id,
            user_id: auth.user_id,
        }
    }
}

impl AppContext {
    pub fn has_permission(&self, permission: &str) -> bool {
        if permission.trim().is_empty() {
            return false;
        }

        if self.permission_scope.contains("*")
            || self.permission_scope.contains("tenant.admin")
            || self.permission_scope.contains(permission)
        {
            return true;
        }

        let segments: Vec<&str> = permission.split('.').collect();
        for index in 1..segments.len() {
            let wildcard = format!("{}.*", segments[..index].join("."));
            if self.permission_scope.contains(wildcard.as_str()) {
                return true;
            }
        }

        false
    }

    pub fn ccp_authority(&self) -> CcpAuthority {
        CcpAuthority::new(
            self.tenant_id.clone(),
            CcpSender::new(
                self.actor_id.clone(),
                self.device_id.clone(),
                self.session_id.clone(),
            ),
            CcpActor::new(self.actor_id.clone(), self.actor_kind.clone()),
        )
    }
}

pub fn local_service_app_context<I, S>(
    tenant_id: &str,
    user_id: &str,
    actor_kind: &str,
    device_id: Option<&str>,
    permission_scope: I,
) -> AppContext
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    AppContext {
        tenant_id: tenant_id.to_owned(),
        organization_id: "0".to_owned(),
        user_id: user_id.to_owned(),
        session_id: Some("s_local_service".to_owned()),
        app_id: Some("sdkwork-im".to_owned()),
        environment: Some("dev".to_owned()),
        deployment_mode: Some("saas".to_owned()),
        auth_level: Some("password".to_owned()),
        data_scope: BTreeSet::from(["tenant".to_owned()]),
        permission_scope: permission_scope
            .into_iter()
            .map(|value| value.as_ref().trim().to_owned())
            .filter(|value| !value.is_empty())
            .collect(),
        actor_id: user_id.to_owned(),
        actor_kind: actor_kind.to_owned(),
        device_id: device_id.map(ToOwned::to_owned),
    }
}

pub(crate) fn local_service_app_context_from_env() -> AppContext {
    let tenant_id = std::env::var(APP_CONTEXT_JWT_TENANT_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "100001".to_owned());
    eprintln!(
        "WARN: with_updated_local_dual_token_context fell back to local_service_app_context; \
         caller did not forward AppContext headers. Using tenant_id={tenant_id} actor=system with no permissions."
    );
    local_service_app_context(&tenant_id, "1", "system", None, Vec::<&str>::new())
}

/// Maps a framework-resolved [`WebRequestContext`] into the IM domain [`AppContext`].
pub fn app_context_from_web_request(context: &WebRequestContext) -> Option<AppContext> {
    let principal = context.principal.as_ref()?;
    Some(app_context_from_web_principal(principal))
}

pub fn app_context_from_web_principal(principal: &WebRequestPrincipal) -> AppContext {
    let environment = Some(format_environment(&principal.app.environment).to_owned());
    let deployment_mode = Some(format_deployment_mode(&principal.app.deployment_mode).to_owned());
    let auth_level = Some(format_auth_level(&principal.auth.auth_level).to_owned());
    let actor_kind = match principal.subject.subject_type {
        WebSubjectType::User => "user".to_owned(),
        WebSubjectType::Service => "service".to_owned(),
        WebSubjectType::System => "system".to_owned(),
        WebSubjectType::ApiKey => "api_key".to_owned(),
    };

    AppContext {
        tenant_id: principal.tenant_id().to_owned(),
        organization_id: principal
            .organization_id()
            .map(str::to_owned)
            .unwrap_or_else(|| "0".to_owned()),
        user_id: principal.user_id().to_owned(),
        session_id: principal.session_id().map(str::to_owned),
        app_id: Some(principal.app_id().to_owned()),
        environment,
        deployment_mode,
        auth_level,
        data_scope: principal.scopes.data_scope.iter().cloned().collect(),
        permission_scope: principal.scopes.permission_scope.iter().cloned().collect(),
        actor_id: principal.user_id().to_owned(),
        actor_kind,
        device_id: None,
    }
}

pub fn resolve_app_context(headers: &HeaderMap) -> Result<AppContext, AppContextError> {
    resolve_app_context_for_request(headers, "", "").map(|resolved| resolved.app_context)
}

/// Resolve `AppContext` from middleware-injected extensions or request headers.
pub fn resolve_handler_app_context(
    auth: Option<axum::extract::Extension<AppContext>>,
    headers: &HeaderMap,
) -> Result<AppContext, AppContextError> {
    match auth {
        Some(axum::extract::Extension(context)) => Ok(context),
        None => resolve_app_context(headers),
    }
}

/// Require authenticated request scope for HTTP handlers.
pub fn require_handler_request_scope(
    auth: Option<axum::extract::Extension<AppContext>>,
    headers: &HeaderMap,
) -> Result<AppRequestScope, AppContextError> {
    resolve_handler_app_context(auth, headers).map(Into::into)
}

pub fn resolve_app_context_with_signature_config(
    headers: &HeaderMap,
    signature_config: AppContextSignatureConfig,
) -> Result<AppContext, AppContextError> {
    require_app_context_signature(headers, &signature_config)?;
    resolve_app_context_for_request_inner(headers, "", "").map(|resolved| resolved.app_context)
}

pub fn resolve_app_request_context(
    headers: &HeaderMap,
    path: &str,
    method: &str,
) -> Result<WebRequestContext, AppContextError> {
    resolve_app_context_for_request(headers, path, method)
        .map(|resolved| resolved.app_request_context)
}

pub fn resolve_app_context_for_request(
    headers: &HeaderMap,
    path: &str,
    method: &str,
) -> Result<ResolvedAppContext, AppContextError> {
    require_app_context_signature(headers, &AppContextSignatureConfig::from_env())?;
    resolve_app_context_for_request_inner(headers, path, method)
}

fn resolve_app_context_for_request_inner(
    headers: &HeaderMap,
    path: &str,
    method: &str,
) -> Result<ResolvedAppContext, AppContextError> {
    let auth_token = extract_bearer_token(headers).ok_or_else(AppContextError::auth_token_missing)?;
    let access_token =
        extract_access_token(headers).ok_or_else(AppContextError::access_token_missing)?;
    let auth_claims = TokenClaims::parse(auth_token.as_str())?;
    let access_claims = TokenClaims::parse(access_token.as_str())?;
    let principal = resolve_principal(&auth_claims, &access_claims)?;
    let app_context = app_context_from_claims(&principal, &auth_claims, &access_claims);
    let request_context = WebRequestContext {
        request_id: ServerRequestId(new_request_id()),
        api_surface: classify_api_surface(path, &WebRequestContextProfile::default()),
        auth_mode: WebAuthMode::DualToken,
        principal: Some(principal),
        transport: WebTransportFacts {
            path: path.to_owned(),
            method: method.to_owned(),
            auth_token_present: true,
            access_token_present: true,
            api_key_present: false,
            oauth_bearer_present: false,
            agent_token_present: false,
        },
        locale: None,
        client_kind: None,
        operation: None,
        trace_id: None,
    };

    Ok(ResolvedAppContext {
        app_request_context: request_context,
        app_context,
    })
}

fn resolve_principal(
    auth_claims: &TokenClaims,
    access_claims: &TokenClaims,
) -> Result<WebRequestPrincipal, AppContextError> {
    require_optional_match(
        "tenant_id",
        auth_claims.optional(&["tenant_id", "tenantId"]),
        access_claims.optional(&["tenant_id", "tenantId"]),
    )?;
    require_optional_match(
        "organization_id",
        auth_claims.optional(&["organization_id", "organizationId"]),
        access_claims.optional(&["organization_id", "organizationId"]),
    )?;
    require_optional_match(
        "user_id",
        Some(auth_claims.required(&["user_id", "userId", "sub"], "auth token user_id")?),
        access_claims.optional(&["user_id", "userId", "sub"]),
    )?;
    require_optional_match(
        "session_id",
        auth_claims.optional(&["session_id", "sessionId", "sid"]),
        access_claims.optional(&["session_id", "sessionId", "sid"]),
    )?;
    require_optional_match(
        "app_id",
        auth_claims.optional(&["app_id", "appId", "azp", "aud"]),
        access_claims.optional(&["app_id", "appId", "azp", "aud"]),
    )?;

    let organization_id = normalize_organization_id(
        access_claims
            .optional(&["organization_id", "organizationId"])
            .or_else(|| auth_claims.optional(&["organization_id", "organizationId"])),
    );
    let access_login_scope = parse_login_scope(
        access_claims.optional(&["login_scope", "loginScope"]),
        organization_id.as_deref(),
    )?;
    let auth_login_scope = parse_login_scope(
        auth_claims.optional(&["login_scope", "loginScope"]),
        organization_id.as_deref(),
    )?;
    if auth_login_scope != access_login_scope {
        return Err(AppContextError::invalid(
            "auth token and access token login_scope contexts do not match",
        ));
    }

    Ok(WebRequestPrincipal::builder()
        .tenant_id(access_claims.required(&["tenant_id", "tenantId"], "access token tenant_id")?)
        .organization_id(organization_id)
        .login_scope(access_login_scope)
        .user_id(auth_claims.required(&["user_id", "userId", "sub"], "auth token user_id")?)
        .session_id(
            auth_claims
                .optional(&["session_id", "sessionId", "sid"])
                .or_else(|| access_claims.optional(&["session_id", "sessionId", "sid"])),
        )
        .app_id(access_claims.required(&["app_id", "appId", "azp", "aud"], "access token app_id")?)
        .environment(parse_environment(
            access_claims.optional(&["environment", "env"]),
        ))
        .deployment_mode(parse_deployment_mode(
            access_claims.optional(&["deployment_mode", "deploymentMode"]),
        ))
        .auth_level(parse_auth_level(auth_claims.optional(&[
            "auth_level",
            "authLevel",
            "acr",
        ])))
        .data_scope(scope_vec(
            access_claims
                .optional(&["data_scope", "dataScope"])
                .or_else(|| auth_claims.optional(&["data_scope", "dataScope"])),
        ))
        .permission_scope(scope_vec(
            access_claims
                .optional(&["permission_scope", "permissionScope", "scope", "scp"])
                .or_else(|| {
                    auth_claims.optional(&["permission_scope", "permissionScope", "scope", "scp"])
                }),
        ))
        .api_key_id(None)
        .subject_type(WebSubjectType::User)
        .build())
}

fn app_context_from_claims(
    principal: &WebRequestPrincipal,
    auth_claims: &TokenClaims,
    access_claims: &TokenClaims,
) -> AppContext {
    let actor_id = access_claims
        .optional(&["actor_id", "actorId"])
        .or_else(|| auth_claims.optional(&["actor_id", "actorId"]))
        .unwrap_or_else(|| principal.user_id().to_owned());
    let actor_kind = access_claims
        .optional(&["actor_kind", "actorKind"])
        .or_else(|| auth_claims.optional(&["actor_kind", "actorKind"]))
        .unwrap_or_else(|| format!("{:?}", principal.subject.subject_type).to_ascii_lowercase());

    AppContext {
        tenant_id: principal.tenant_id().to_owned(),
        organization_id: principal
            .organization_id()
            .map(str::to_owned)
            .unwrap_or_else(|| "0".to_owned()),
        user_id: principal.user_id().to_owned(),
        session_id: principal.session_id().map(str::to_owned),
        app_id: Some(principal.app_id().to_owned()),
        environment: Some(format_environment(&principal.app.environment).to_owned()),
        deployment_mode: Some(format_deployment_mode(&principal.app.deployment_mode).to_owned()),
        auth_level: Some(format_auth_level(&principal.auth.auth_level).to_owned()),
        data_scope: principal.scopes.data_scope.iter().cloned().collect(),
        permission_scope: principal.scopes.permission_scope.iter().cloned().collect(),
        actor_id,
        actor_kind,
        device_id: access_claims
            .optional(&["device_id", "deviceId"])
            .or_else(|| auth_claims.optional(&["device_id", "deviceId"])),
    }
}

pub fn build_dual_token_headers_for_context<I, S>(
    context: &AppContext,
    permission_scope: I,
) -> HeaderMap
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let permission_scope = permission_scope
        .into_iter()
        .map(|value| value.as_ref().trim().to_owned())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    let permission_scope = if permission_scope.is_empty() {
        context
            .permission_scope
            .iter()
            .cloned()
            .collect::<Vec<_>>()
    } else {
        permission_scope
    };
    let data_scope = context.data_scope.iter().cloned().collect::<Vec<_>>();
    let login_scope = if is_tenant_level_organization_id(&context.organization_id) {
        "TENANT"
    } else {
        "ORGANIZATION"
    };
    let organization_id = dual_token_organization_id_claim(login_scope, &context.organization_id);
    let session_id = context
        .session_id
        .clone()
        .unwrap_or_else(|| "local-service-session".to_owned());
    let app_id = context
        .app_id
        .clone()
        .unwrap_or_else(|| "sdkwork-im".to_owned());

    let auth_token = encode_local_jwt_claims(json!({
        "tenant_id": context.tenant_id,
        "organization_id": organization_id,
        "login_scope": login_scope,
        "user_id": context.user_id,
        "session_id": session_id,
        "app_id": app_id,
        "auth_level": context.auth_level.as_deref().unwrap_or("password"),
        "subject_type": context.actor_kind,
    }));
    let access_token = encode_local_jwt_claims(json!({
        "tenant_id": context.tenant_id,
        "organization_id": organization_id,
        "login_scope": login_scope,
        "user_id": context.user_id,
        "session_id": session_id,
        "app_id": app_id,
        "environment": context.environment.as_deref().unwrap_or("dev"),
        "deployment_mode": context.deployment_mode.as_deref().unwrap_or("saas"),
        "auth_level": context.auth_level.as_deref().unwrap_or("password"),
        "actor_id": context.actor_id,
        "actor_kind": context.actor_kind,
        "device_id": context.device_id,
        "data_scope": data_scope,
        "permission_scope": permission_scope,
        "subject_type": context.actor_kind,
    }));

    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(format!("Bearer {auth_token}").as_str())
            .expect("local auth token must be a valid header value"),
    );
    headers.insert(
        "Access-Token",
        HeaderValue::from_str(access_token.as_str())
            .expect("local access token must be a valid header value"),
    );
    headers
}

fn require_optional_match(
    claim_name: &str,
    left: Option<String>,
    right: Option<String>,
) -> Result<(), AppContextError> {
    match (left, right) {
        (Some(left), Some(right)) if left != right => Err(AppContextError::invalid(format!(
            "auth token and access token {claim_name} contexts do not match"
        ))),
        _ => Ok(()),
    }
}

fn normalize_organization_id(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty() && value != "0" && value != "default")
}

fn parse_login_scope(
    value: Option<String>,
    organization_id: Option<&str>,
) -> Result<WebLoginScope, AppContextError> {
    match value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        Some(value) if value.eq_ignore_ascii_case("TENANT") => {
            if organization_id.is_some() {
                return Err(AppContextError::invalid(
                    "login_scope TENANT requires organization_id to be absent or 0",
                ));
            }
            Ok(WebLoginScope::Tenant)
        }
        Some(value) if value.eq_ignore_ascii_case("ORGANIZATION") => {
            if organization_id.is_none() {
                return Err(AppContextError::invalid(
                    "login_scope ORGANIZATION requires a non-zero organization_id",
                ));
            }
            Ok(WebLoginScope::Organization)
        }
        Some(value) => Err(AppContextError::invalid(format!(
            "unsupported login_scope claim: {value}"
        ))),
        None if organization_id.is_some() => Ok(WebLoginScope::Organization),
        None => Ok(WebLoginScope::Tenant),
    }
}

fn scope_vec(value: Option<String>) -> Vec<String> {
    value
        .map(|value| {
            value
                .split([',', ' '])
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

pub(crate) fn split_scope(value: &str) -> BTreeSet<String> {
    value
        .split(|ch: char| ch.is_whitespace() || ch == ',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn is_tenant_level_organization_id(organization_id: &str) -> bool {
    matches!(organization_id.trim(), "" | "default" | "0")
}

fn dual_token_organization_id_claim(login_scope: &str, organization_id: &str) -> String {
    if login_scope.eq_ignore_ascii_case("TENANT") {
        return "0".to_owned();
    }
    organization_id.to_owned()
}
