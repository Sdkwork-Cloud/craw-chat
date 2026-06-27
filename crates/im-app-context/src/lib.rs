use std::collections::{BTreeMap, BTreeSet};

use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, HeaderValue, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use sdkwork_im_ccp_core::{CcpActor, CcpAuthority, CcpSender};
use sdkwork_utils_rust::{base64url_decode, base64url_encode, hmac_sha256_base64url};
use sdkwork_web_core::{
    EnvBootstrapTenantSigningKeyLookup, JwtVerifier, ServerRequestId, TenantBoundJwtVerifier,
    WebAuthLevel, WebAuthMode, WebDeploymentMode, WebEnvironment, WebLoginScope, WebRequestContext,
    WebRequestContextProfile, WebRequestPrincipal, WebSubjectType, WebTransportFacts,
    classify_api_surface, new_request_id,
};
use serde_json::{Value, json};

const APP_CONTEXT_REQUIRE_SIGNATURE_ENV: &str = "SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE";
const APP_CONTEXT_SIGNATURE_SECRET_ENV: &str = "SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET";
const APP_CONTEXT_SIGNATURE_SECRET_FILE_ENV: &str = "SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET_FILE";
const APP_CONTEXT_JWT_TENANT_ID_ENV: &str = "SDKWORK_IM_APP_CONTEXT_JWT_TENANT_ID";
const APP_CONTEXT_JWT_KEY_ID_ENV: &str = "SDKWORK_IM_APP_CONTEXT_JWT_KEY_ID";
const APP_CONTEXT_JWT_SIGNING_SECRET_ENV: &str = "SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET";
const APP_CONTEXT_JWT_SIGNING_SECRET_FILE_ENV: &str =
    "SDKWORK_IM_APP_CONTEXT_JWT_SIGNING_SECRET_FILE";
const APP_CONTEXT_JWT_KEY_ID_DEFAULT: &str = "bootstrap";
const SDKWORK_CONTEXT_SIGNATURE_HEADER: &str = "x-sdkwork-context-signature";
const SIGNED_APP_CONTEXT_HEADER_NAMES: &[&str] = &[
    "x-sdkwork-app-id",
    "x-sdkwork-tenant-id",
    "x-sdkwork-organization-id",
    "x-sdkwork-user-id",
    "x-sdkwork-session-id",
    "x-sdkwork-environment",
    "x-sdkwork-deployment-mode",
    "x-sdkwork-auth-level",
    "x-sdkwork-data-scope",
    "x-sdkwork-permission-scope",
    "x-sdkwork-actor-id",
    "x-sdkwork-actor-kind",
    "x-sdkwork-device-id",
];

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
pub struct AppContextError {
    code: &'static str,
    message: String,
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

pub trait DualTokenRequestBuilderExt {
    fn with_dual_token_context<I, S>(
        self,
        tenant_id: &str,
        user_id: &str,
        actor_kind: &str,
        device_id: Option<&str>,
        permission_scope: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>;

    fn with_dual_token_tenant<S>(self, tenant_id: S) -> Self
    where
        S: AsRef<str>;

    fn with_dual_token_organization<S>(self, organization_id: S) -> Self
    where
        S: AsRef<str>;

    fn with_dual_token_user<S>(self, user_id: S) -> Self
    where
        S: AsRef<str>;

    fn with_dual_token_actor<S>(self, actor_id: S) -> Self
    where
        S: AsRef<str>;

    fn with_dual_token_actor_kind<S>(self, actor_kind: S) -> Self
    where
        S: AsRef<str>;

    fn with_dual_token_session<S>(self, session_id: S) -> Self
    where
        S: AsRef<str>;

    fn with_dual_token_device<S>(self, device_id: S) -> Self
    where
        S: AsRef<str>;

    fn with_dual_token_app<S>(self, app_id: S) -> Self
    where
        S: AsRef<str>;

    fn with_dual_token_permission_scope<S>(self, permission_scope: S) -> Self
    where
        S: AsRef<str>;
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct TokenClaims {
    values: BTreeMap<String, String>,
}

impl AppContextError {
    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub fn missing(message: impl Into<String>) -> Self {
        Self {
            code: "app_context_missing",
            message: message.into(),
        }
    }

    pub fn invalid(message: impl Into<String>) -> Self {
        Self {
            code: "app_context_invalid",
            message: message.into(),
        }
    }

    pub fn auth_token_missing() -> Self {
        Self {
            code: "auth_token_missing",
            message: "authorization header must provide a bearer token".to_owned(),
        }
    }

    pub fn access_token_missing() -> Self {
        Self {
            code: "access_token_missing",
            message: "access-token header is required".to_owned(),
        }
    }
}

impl std::fmt::Display for AppContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppContextError {}

impl DualTokenRequestBuilderExt for axum::http::request::Builder {
    fn with_dual_token_context<I, S>(
        self,
        tenant_id: &str,
        user_id: &str,
        actor_kind: &str,
        device_id: Option<&str>,
        permission_scope: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let context =
            local_service_app_context(tenant_id, user_id, actor_kind, device_id, permission_scope);
        let headers =
            build_dual_token_headers_for_context(&context, context.permission_scope.iter());
        headers
            .iter()
            .fold(self, |builder, (name, value)| builder.header(name, value))
    }

    fn with_dual_token_tenant<S>(self, tenant_id: S) -> Self
    where
        S: AsRef<str>,
    {
        let tenant_id = tenant_id.as_ref().to_owned();
        with_updated_local_dual_token_context(self, move |context| {
            context.tenant_id = tenant_id;
        })
    }

    fn with_dual_token_organization<S>(self, organization_id: S) -> Self
    where
        S: AsRef<str>,
    {
        let organization_id = organization_id.as_ref().to_owned();
        with_updated_local_dual_token_context(self, move |context| {
            context.organization_id = organization_id;
        })
    }

    fn with_dual_token_user<S>(self, user_id: S) -> Self
    where
        S: AsRef<str>,
    {
        let user_id = user_id.as_ref().to_owned();
        with_updated_local_dual_token_context(self, move |context| {
            context.user_id = user_id.clone();
            context.actor_id = user_id;
        })
    }

    fn with_dual_token_actor<S>(self, actor_id: S) -> Self
    where
        S: AsRef<str>,
    {
        let actor_id = actor_id.as_ref().to_owned();
        with_updated_local_dual_token_context(self, move |context| {
            context.actor_id = actor_id;
        })
    }

    fn with_dual_token_actor_kind<S>(self, actor_kind: S) -> Self
    where
        S: AsRef<str>,
    {
        let actor_kind = actor_kind.as_ref().to_owned();
        with_updated_local_dual_token_context(self, move |context| {
            context.actor_kind = actor_kind;
        })
    }

    fn with_dual_token_session<S>(self, session_id: S) -> Self
    where
        S: AsRef<str>,
    {
        let session_id = session_id.as_ref().to_owned();
        with_updated_local_dual_token_context(self, move |context| {
            context.session_id = Some(session_id);
        })
    }

    fn with_dual_token_device<S>(self, device_id: S) -> Self
    where
        S: AsRef<str>,
    {
        let device_id = device_id.as_ref().to_owned();
        with_updated_local_dual_token_context(self, move |context| {
            context.device_id = Some(device_id);
        })
    }

    fn with_dual_token_app<S>(self, app_id: S) -> Self
    where
        S: AsRef<str>,
    {
        let app_id = app_id.as_ref().to_owned();
        with_updated_local_dual_token_context(self, move |context| {
            context.app_id = Some(app_id);
        })
    }

    fn with_dual_token_permission_scope<S>(self, permission_scope: S) -> Self
    where
        S: AsRef<str>,
    {
        let permission_scope = split_scope(permission_scope.as_ref());
        with_updated_local_dual_token_context(self, move |context| {
            context.permission_scope = permission_scope;
        })
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

fn local_service_app_context_from_env() -> AppContext {
    let tenant_id = std::env::var(APP_CONTEXT_JWT_TENANT_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "100001".to_owned());
    eprintln!(
        "WARN: with_updated_local_dual_token_context fell back to local_service_app_context; \
         caller did not forward AppContext headers. Using tenant_id={tenant_id} actor=system with no permissions."
    );
    local_service_app_context(&tenant_id, "0", "system", None, Vec::<&str>::new())
}

fn with_updated_local_dual_token_context<F>(
    mut builder: axum::http::request::Builder,
    update: F,
) -> axum::http::request::Builder
where
    F: FnOnce(&mut AppContext),
{
    let mut context = builder
        .headers_ref()
        .and_then(|headers| resolve_app_context(headers).ok())
        .unwrap_or_else(local_service_app_context_from_env);
    update(&mut context);
    let headers = build_dual_token_headers_for_context(&context, context.permission_scope.iter());
    if let Some(target_headers) = builder.headers_mut() {
        target_headers.remove(header::AUTHORIZATION);
        target_headers.remove("Access-Token");
        for (name, value) in headers.iter() {
            target_headers.insert(name, value.clone());
        }
    }
    builder
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

pub fn sign_app_context_headers(
    headers: &HeaderMap,
    shared_secret: &str,
) -> Result<String, AppContextError> {
    let shared_secret = shared_secret.trim();
    if shared_secret.is_empty() {
        return Err(AppContextError::invalid(
            "AppContext signature shared secret must not be empty",
        ));
    }

    let payload = canonical_app_context_signature_payload(headers);
    Ok(hmac_sha256_base64url(
        payload.as_bytes(),
        shared_secret.as_bytes(),
    ))
}

pub fn require_app_context_signature(
    headers: &HeaderMap,
    signature_config: &AppContextSignatureConfig,
) -> Result<(), AppContextError> {
    if !signature_config.require_signature {
        return Ok(());
    }

    let shared_secret = signature_config
        .shared_secret
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            AppContextError::invalid(format!(
                "{APP_CONTEXT_SIGNATURE_SECRET_ENV} is required when {APP_CONTEXT_REQUIRE_SIGNATURE_ENV}=true"
            ))
        })?;
    let actual_signature = headers
        .get(SDKWORK_CONTEXT_SIGNATURE_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            AppContextError::invalid(format!(
                "{SDKWORK_CONTEXT_SIGNATURE_HEADER} header is required when signed AppContext projection is required"
            ))
        })?;
    let expected_signature = sign_app_context_headers(headers, shared_secret)?;
    if !constant_time_eq(actual_signature.as_bytes(), expected_signature.as_bytes()) {
        return Err(AppContextError::invalid(format!(
            "{SDKWORK_CONTEXT_SIGNATURE_HEADER} signature validation failed"
        )));
    }

    Ok(())
}

pub fn build_dual_token_headers_for_context<I, S>(
    context: &AppContext,
    permission_scope: I,
) -> HeaderMap
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    ensure_local_dual_token_environment_for_unconfigured_process();
    let permission_scope = permission_scope
        .into_iter()
        .map(|value| value.as_ref().trim().to_owned())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    let permission_scope = if permission_scope.is_empty() {
        context.permission_scope.iter().cloned().collect::<Vec<_>>()
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
    let auth_token =
        extract_bearer_token(headers).ok_or_else(AppContextError::auth_token_missing)?;
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

pub async fn inject_app_request_context_middleware(
    mut request: Request<Body>,
    next: Next,
) -> Response {
    if request.method() == axum::http::Method::OPTIONS {
        return next.run(request).await;
    }

    if has_any_dual_token_header(request.headers()) {
        match resolve_app_context_for_request(
            request.headers(),
            request.uri().path(),
            request.method().as_str(),
        ) {
            Ok(resolved) => {
                request
                    .extensions_mut()
                    .insert(resolved.app_request_context);
                request.extensions_mut().insert(resolved.app_context);
            }
            Err(error) => return app_context_error_response(error),
        }
    }

    next.run(request).await
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

fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .and_then(|value| {
            let (scheme, token) = value.split_once(' ')?;
            if scheme.eq_ignore_ascii_case("bearer") && !token.trim().is_empty() {
                return Some(token.trim().to_owned());
            }
            None
        })
}

fn extract_access_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("access-token")
        .or_else(|| headers.get("Access-Token"))
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| {
            value
                .strip_prefix("Bearer ")
                .or_else(|| value.strip_prefix("bearer "))
                .unwrap_or(value)
                .trim()
                .to_owned()
        })
}

fn canonical_app_context_signature_payload(headers: &HeaderMap) -> String {
    SIGNED_APP_CONTEXT_HEADER_NAMES
        .iter()
        .map(|name| {
            let value = headers
                .get(*name)
                .and_then(|value| value.to_str().ok())
                .map(str::trim)
                .unwrap_or("");
            format!("{name}:{value}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    let max_len = left.len().max(right.len());
    let mut diff = left.len() ^ right.len();
    for index in 0..max_len {
        let left_byte = left.get(index).copied().unwrap_or(0);
        let right_byte = right.get(index).copied().unwrap_or(0);
        diff |= usize::from(left_byte ^ right_byte);
    }
    diff == 0
}

fn parse_truthy_env_flag(raw: Option<String>) -> bool {
    raw.as_deref().map(str::trim).is_some_and(|value| {
        matches!(
            value.to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}

/// Resolve a secret value from either a direct env var or a `_FILE` env var.
///
/// Follows the Docker/Kubernetes secrets pattern: if `<key>_FILE` is set,
/// the secret is read from the referenced file path; otherwise, the value
/// of `<key>` is used directly. The `_FILE` variant takes precedence.
fn resolve_secret_from_env_or_file(direct_key: &str, file_key: &str) -> Option<String> {
    // Check _FILE variant first (Docker/Kubernetes secrets pattern).
    if let Ok(file_path) = std::env::var(file_key) {
        let trimmed_path = file_path.trim();
        if !trimmed_path.is_empty() {
            return std::fs::read_to_string(trimmed_path)
                .ok()
                .map(|content| content.trim().to_owned())
                .filter(|value| !value.is_empty())
                .or_else(|| {
                    tracing::error!(
                        target: "sdkwork.im.app_context",
                        "failed to read secret file `{trimmed_path}` referenced by {file_key}"
                    );
                    None
                });
        }
    }
    // Fall back to direct env var.
    std::env::var(direct_key)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

/// Returns true when the upgrade request already carries dual-token credentials in headers.
///
/// Browser clients cannot set these headers; native/mobile/Node runtimes use this path and
/// skip the post-upgrade `auth.init` frame.
pub fn has_websocket_upgrade_auth_headers(headers: &HeaderMap) -> bool {
    headers.contains_key(header::AUTHORIZATION)
        || headers.contains_key("access-token")
        || headers.contains_key("Access-Token")
}

/// Extracts `deviceId` from a websocket path-and-query string (`/path?deviceId=...`).
pub fn websocket_query_device_id_from_path_and_query(path_and_query: &str) -> Option<String> {
    let (_path, query) = path_and_query.split_once('?')?;
    query.split('&').find_map(|pair| {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        if key.trim().eq_ignore_ascii_case("deviceId") {
            Some(value.trim().to_owned()).filter(|value| !value.is_empty())
        } else {
            None
        }
    })
}

/// Prefers the `auth.init` frame `deviceId`, then falls back to the upgrade query value.
pub fn coalesce_websocket_device_id(
    frame_device_id: Option<String>,
    query_device_id: Option<String>,
) -> Option<String> {
    frame_device_id
        .or(query_device_id)
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn has_any_dual_token_header(headers: &HeaderMap) -> bool {
    has_websocket_upgrade_auth_headers(headers)
}

impl TokenClaims {
    fn parse(raw: &str) -> Result<Self, AppContextError> {
        let raw = raw.trim();
        if raw.is_empty() {
            return Err(AppContextError::invalid("token must not be empty"));
        }
        let environment = resolve_web_environment_from_process_env();
        let dev_or_test = matches!(environment, WebEnvironment::Dev | WebEnvironment::Test);
        if raw.starts_with('{') {
            if !dev_or_test {
                return Err(AppContextError::invalid(
                    "raw JSON bearer tokens are not allowed outside dev/test environments",
                ));
            }
            return Self::from_json_str(raw);
        }
        if is_jwt_token(raw) {
            validate_jwt_token(raw)?;
            let value = decode_jwt_payload(raw)?
                .ok_or_else(|| AppContextError::invalid("token payload must be present"))?;
            return Self::from_json_value(value);
        }
        if !dev_or_test {
            return Err(AppContextError::invalid(
                "key-value bearer tokens are not allowed outside dev/test environments; use signed JWT",
            ));
        }
        Ok(Self {
            values: parse_key_value_claims(raw),
        })
    }

    fn from_json_str(raw: &str) -> Result<Self, AppContextError> {
        let value = serde_json::from_str::<Value>(raw)
            .map_err(|error| AppContextError::invalid(format!("invalid token json: {error}")))?;
        Self::from_json_value(value)
    }

    fn from_json_value(value: Value) -> Result<Self, AppContextError> {
        let object = value
            .as_object()
            .ok_or_else(|| AppContextError::invalid("token claims must be a JSON object"))?;
        Ok(Self {
            values: object
                .iter()
                .filter_map(|(key, value)| {
                    claim_value_to_string(value).map(|value| (key.clone(), value))
                })
                .collect(),
        })
    }

    fn optional(&self, names: &[&str]) -> Option<String> {
        names.iter().find_map(|name| {
            self.values
                .get(*name)
                .map(String::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned)
        })
    }

    fn required(&self, names: &[&str], label: &str) -> Result<String, AppContextError> {
        self.optional(names)
            .ok_or_else(|| AppContextError::missing(format!("{label} claim is required")))
    }
}

fn decode_jwt_payload(raw: &str) -> Result<Option<Value>, AppContextError> {
    let mut parts = raw.split('.');
    let _header = parts.next();
    let Some(payload) = parts.next() else {
        return Ok(None);
    };
    if parts.next().is_none() {
        return Ok(None);
    }
    let decoded = base64url_decode(payload).ok_or_else(|| {
        AppContextError::invalid("invalid token payload: base64url decode failed".to_owned())
    })?;
    let value = serde_json::from_slice::<Value>(&decoded).map_err(|error| {
        AppContextError::invalid(format!("invalid token payload json: {error}"))
    })?;
    Ok(Some(value))
}

fn is_jwt_token(raw: &str) -> bool {
    let mut parts = raw.split('.');
    matches!(
        (parts.next(), parts.next(), parts.next()),
        (Some(_), Some(_), Some(_))
    ) && parts.next().is_none()
}

fn decode_jwt_header(raw: &str) -> Result<Value, AppContextError> {
    let header_segment = raw
        .split('.')
        .next()
        .ok_or_else(|| AppContextError::invalid("token header segment is required"))?;
    let decoded = base64url_decode(header_segment).ok_or_else(|| {
        AppContextError::invalid("invalid token header: base64url decode failed".to_owned())
    })?;
    serde_json::from_slice::<Value>(&decoded)
        .map_err(|error| AppContextError::invalid(format!("invalid token header json: {error}")))
}

fn jwt_signature_segment(raw: &str) -> Option<&str> {
    let mut parts = raw.split('.');
    parts.next();
    parts.next();
    parts.next()
}

fn tenant_signing_lookup_from_env() -> Option<EnvBootstrapTenantSigningKeyLookup> {
    let tenant_id = std::env::var(APP_CONTEXT_JWT_TENANT_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())?;
    let key_id = std::env::var(APP_CONTEXT_JWT_KEY_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| APP_CONTEXT_JWT_KEY_ID_DEFAULT.to_owned());
    let secret = resolve_secret_from_env_or_file(
        APP_CONTEXT_JWT_SIGNING_SECRET_ENV,
        APP_CONTEXT_JWT_SIGNING_SECRET_FILE_ENV,
    )?;
    Some(EnvBootstrapTenantSigningKeyLookup::new(
        tenant_id,
        key_id,
        secret.as_bytes(),
    ))
}

fn verify_tenant_bound_jwt(
    raw: &str,
    lookup: EnvBootstrapTenantSigningKeyLookup,
) -> Result<(), AppContextError> {
    let verifier = TenantBoundJwtVerifier::new(lookup);
    verifier
        .verify_and_decode_claims(raw)
        .map_err(|error| AppContextError::invalid(error.message))?;
    Ok(())
}

fn validate_jwt_token(raw: &str) -> Result<(), AppContextError> {
    let header = decode_jwt_header(raw)?;
    let algorithm = header
        .get("alg")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let signature = jwt_signature_segment(raw).unwrap_or("").trim();
    let unsigned_local_token = algorithm.eq_ignore_ascii_case("none") || signature == "local";
    let environment = resolve_web_environment_from_process_env();
    let dev_or_test = matches!(environment, WebEnvironment::Dev | WebEnvironment::Test);

    if unsigned_local_token {
        if !dev_or_test {
            return Err(AppContextError::invalid(
                "unsigned local JWT tokens are not allowed outside dev/test environments",
            ));
        }
        let payload = decode_jwt_payload(raw)?
            .ok_or_else(|| AppContextError::invalid("token payload must be present"))?;
        validate_jwt_time_claims(&payload)?;
        return Ok(());
    }

    if algorithm.eq_ignore_ascii_case("none") {
        return Err(AppContextError::invalid(
            "JWT alg=none tokens are not allowed in production-like environments",
        ));
    }

    if let Some(lookup) = tenant_signing_lookup_from_env() {
        verify_tenant_bound_jwt(raw, lookup)?;
        return Ok(());
    }

    if !dev_or_test {
        return Err(AppContextError::invalid(format!(
            "signed JWT verification requires {APP_CONTEXT_JWT_TENANT_ID_ENV} and {APP_CONTEXT_JWT_SIGNING_SECRET_ENV} (optional {APP_CONTEXT_JWT_KEY_ID_ENV}, default {APP_CONTEXT_JWT_KEY_ID_DEFAULT}) in production-like environments"
        )));
    }

    let payload = decode_jwt_payload(raw)?
        .ok_or_else(|| AppContextError::invalid("token payload must be present"))?;
    validate_jwt_time_claims(&payload)?;
    Ok(())
}

fn validate_jwt_time_claims(payload: &Value) -> Result<(), AppContextError> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0);
    if let Some(exp) = payload.get("exp") {
        let exp = parse_jwt_numeric_claim(exp, "exp")?;
        if now >= exp {
            return Err(AppContextError::invalid("token has expired"));
        }
    }
    if let Some(nbf) = payload.get("nbf") {
        let nbf = parse_jwt_numeric_claim(nbf, "nbf")?;
        if now < nbf {
            return Err(AppContextError::invalid("token is not yet valid"));
        }
    }
    Ok(())
}

fn parse_jwt_numeric_claim(value: &Value, claim_name: &str) -> Result<i64, AppContextError> {
    match value {
        Value::Number(number) => number.as_i64().ok_or_else(|| {
            AppContextError::invalid(format!("{claim_name} claim must be an integer"))
        }),
        Value::String(raw) => raw.trim().parse::<i64>().map_err(|error| {
            AppContextError::invalid(format!("{claim_name} claim must be an integer: {error}"))
        }),
        _ => Err(AppContextError::invalid(format!(
            "{claim_name} claim must be an integer"
        ))),
    }
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

fn encode_local_jwt_claims(claims: Value) -> String {
    let mut claims = claims;
    if let Some(object) = claims.as_object_mut() {
        object
            .entry("token_version")
            .or_insert(json!(sdkwork_web_core::stamp_token_version()));
    }
    let header = base64url_encode(r#"{"alg":"none","typ":"JWT"}"#.as_bytes());
    let payload = base64url_encode(claims.to_string().as_bytes());
    format!("{header}.{payload}.local")
}

fn parse_key_value_claims(raw: &str) -> BTreeMap<String, String> {
    raw.split(';')
        .filter_map(|part| {
            let (key, value) = part.split_once('=')?;
            let key = key.trim();
            let value = value.trim();
            if key.is_empty() || value.is_empty() {
                return None;
            }
            Some((key.to_owned(), value.to_owned()))
        })
        .collect()
}

fn claim_value_to_string(value: &Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::String(value) => Some(value.trim().to_owned()).filter(|value| !value.is_empty()),
        Value::Bool(value) => Some(value.to_string()),
        Value::Number(value) => Some(value.to_string()),
        Value::Array(items) => {
            let values = items
                .iter()
                .filter_map(claim_value_to_string)
                .collect::<Vec<_>>();
            if values.is_empty() {
                None
            } else {
                Some(values.join(","))
            }
        }
        Value::Object(_) => serde_json::to_string(value).ok(),
    }
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

fn parse_environment(value: Option<String>) -> WebEnvironment {
    match value
        .as_deref()
        .unwrap_or("prod")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "dev" | "development" => WebEnvironment::Dev,
        "test" | "testing" => WebEnvironment::Test,
        _ => WebEnvironment::Prod,
    }
}

/// Resolve the canonical SDKWork web environment from process env (`SDKWORK_IM_ENVIRONMENT`).
pub fn resolve_web_environment_from_process_env() -> WebEnvironment {
    parse_environment(std::env::var("SDKWORK_IM_ENVIRONMENT").ok())
}

fn ensure_local_dual_token_environment_for_unconfigured_process() {
    if std::env::var("SDKWORK_IM_ENVIRONMENT").is_ok() {
        return;
    }
    // Local dual-token helpers are used by integration tests and dev harnesses that do not
    // bootstrap SDKWORK_IM_ENVIRONMENT explicitly. Pin test mode only while the process still
    // relies on the implicit production default.
    unsafe {
        std::env::set_var("SDKWORK_IM_ENVIRONMENT", "test");
    }
}

/// Whether services may fall back to header-only AppContext resolution without IAM DB lookup.
pub fn allows_header_only_app_context_fallback() -> bool {
    matches!(
        resolve_web_environment_from_process_env(),
        WebEnvironment::Dev | WebEnvironment::Test
    )
}

fn parse_deployment_mode(value: Option<String>) -> WebDeploymentMode {
    match value
        .as_deref()
        .unwrap_or("saas")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "local" => WebDeploymentMode::Local,
        "private" | "private_cloud" => WebDeploymentMode::Private,
        _ => WebDeploymentMode::Saas,
    }
}

fn parse_auth_level(value: Option<String>) -> WebAuthLevel {
    match value
        .as_deref()
        .unwrap_or("password")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "anonymous" => WebAuthLevel::Anonymous,
        "mfa" => WebAuthLevel::Mfa,
        "system" => WebAuthLevel::System,
        "api_key" | "apikey" => WebAuthLevel::ApiKey,
        _ => WebAuthLevel::Password,
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

fn split_scope(value: &str) -> BTreeSet<String> {
    value
        .split(|ch: char| ch.is_whitespace() || ch == ',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn format_environment(value: &WebEnvironment) -> &'static str {
    match value {
        WebEnvironment::Dev => "dev",
        WebEnvironment::Test => "test",
        WebEnvironment::Prod => "prod",
    }
}

fn format_deployment_mode(value: &WebDeploymentMode) -> &'static str {
    match value {
        WebDeploymentMode::Saas => "saas",
        WebDeploymentMode::Local => "local",
        WebDeploymentMode::Private => "private",
    }
}

fn format_auth_level(value: &WebAuthLevel) -> &'static str {
    match value {
        WebAuthLevel::Anonymous => "anonymous",
        WebAuthLevel::Password => "password",
        WebAuthLevel::Mfa => "mfa",
        WebAuthLevel::System => "system",
        WebAuthLevel::ApiKey => "api_key",
    }
}

fn app_context_error_response(error: AppContextError) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        axum::Json(json!({
            "code": error.code(),
            "message": error.message(),
        })),
    )
        .into_response()
}
