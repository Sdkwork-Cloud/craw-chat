use anyhow::{bail, Context, Result};
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, Method, StatusCode, Uri},
    response::{Redirect, Response},
    routing::{any, get},
    Router,
};
use bytes::Bytes;
use rand::random;
use reqwest::Client;
use sdkwork_api_config::{StandaloneConfig, StandaloneConfigLoader};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Component, Path as StdPath, PathBuf},
    sync::{Arc, OnceLock},
};
use tokio::{fs, net::TcpListener, sync::oneshot};
use url::{Host, Url};

mod admin_sandbox;
mod sandbox_policy;
mod web_client_routing;

use admin_sandbox::{handle_admin_sandbox_request, SharedAdminSandboxState};
use im_portal_snapshots::{build_portal_snapshot_for_section, build_portal_workspace_view};
use ops_service::OpsRuntime;
use sandbox_policy::ensure_admin_sandbox_allowed;
use web_client_routing::{select_available_web_client, WebClient};

const JSON_CONTENT_TYPE: &str = "application/json; charset=utf-8";
const BACKEND_ADMIN_API_PREFIX: &str = "/backend/v3/api/admin";
const ADMIN_BACKEND_NOT_CONFIGURED_MESSAGE: &str = "Admin backend proxy target is not configured. Set SDKWORK_ADMIN_PROXY_TARGET to a backend that serves /backend/v3/api/admin.";
const PC_PRODUCT_API_UPSTREAM_ENV: &str = "SDKWORK_IM_PC_API_UPSTREAM";
const CACHE_CONTROL_HEADER: &str = "cache-control";
const CONTENT_SECURITY_POLICY_HEADER: &str = "content-security-policy";
const CROSS_ORIGIN_RESOURCE_POLICY_HEADER: &str = "cross-origin-resource-policy";
const PERMISSIONS_POLICY_HEADER: &str = "permissions-policy";
const REFERRER_POLICY_HEADER: &str = "referrer-policy";
const X_CONTENT_TYPE_OPTIONS_HEADER: &str = "x-content-type-options";
const X_FRAME_OPTIONS_HEADER: &str = "x-frame-options";
const DEFAULT_PERMISSIONS_POLICY: &str = "accelerometer=(), camera=(), geolocation=(), gyroscope=(), magnetometer=(), microphone=(), payment=(), usb=()";
const LOCAL_APP_MODULES: &[&str] = &[
    "chat",
    "workspace",
    "orders",
    "shop",
    "calendar",
    "notary",
    "knowledge",
    "enterprise",
    "devices",
    "community",
    "voice",
    "agent",
    "course",
    "contacts",
    "favorites",
    "mail",
    "approval",
    "report",
    "attendance",
    "drive",
    "videogen",
    "imagegen",
    "voicegen",
    "musicgen",
    "writing",
];
const PORTAL_SNAPSHOT_SECTIONS: &[&str] = &[
    "access",
    "automation",
    "conversations",
    "dashboard",
    "governance",
    "home",
    "media",
    "realtime",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductSiteDir {
    pub pc_site_dir: PathBuf,
    pub h5_site_dir: Option<PathBuf>,
}

impl ProductSiteDir {
    pub fn new(pc_site_dir: impl Into<PathBuf>) -> Self {
        Self {
            pc_site_dir: pc_site_dir.into(),
            h5_site_dir: None,
        }
    }

    pub fn with_h5_site_dir(mut self, h5_site_dir: impl Into<PathBuf>) -> Self {
        self.h5_site_dir = Some(h5_site_dir.into());
        self
    }
}

pub fn resolve_product_site_dir_from_env(repo_root: impl AsRef<StdPath>) -> Result<ProductSiteDir> {
    let repo_root = repo_root.as_ref();
    let admin_site_dir =
        resolve_site_dir_from_env(&["SDKWORK_IM_ADMIN_SITE_DIR", "SDKWORK_ADMIN_SITE_DIR"]);
    let portal_site_dir =
        resolve_site_dir_from_env(&["SDKWORK_IM_PORTAL_SITE_DIR", "SDKWORK_PORTAL_SITE_DIR"]);
    let configured_site_dir = select_shared_product_site_dir(admin_site_dir, portal_site_dir)?;
    let pc_site_dir = configured_site_dir
        .unwrap_or_else(|| repo_root.join("apps").join("sdkwork-im-pc").join("dist"));
    let h5_site_dir = resolve_site_dir_from_env(&["SDKWORK_IM_H5_SITE_DIR"])
        .unwrap_or_else(|| repo_root.join("apps").join("sdkwork-im-h5").join("dist"));

    Ok(ProductSiteDir::new(pc_site_dir).with_h5_site_dir(h5_site_dir))
}

fn select_shared_product_site_dir(
    admin_site_dir: Option<PathBuf>,
    portal_site_dir: Option<PathBuf>,
) -> Result<Option<PathBuf>> {
    match (admin_site_dir, portal_site_dir) {
        (Some(admin), Some(portal)) if !site_dirs_are_equivalent(&admin, &portal) => {
            bail!(
                "SDKWORK_IM_ADMIN_SITE_DIR and SDKWORK_IM_PORTAL_SITE_DIR must reference the same shared apps/sdkwork-im-pc renderer build"
            )
        }
        (Some(site_dir), Some(_)) | (Some(site_dir), None) | (None, Some(site_dir)) => {
            Ok(Some(site_dir))
        }
        (None, None) => Ok(None),
    }
}

fn site_dirs_are_equivalent(left: &StdPath, right: &StdPath) -> bool {
    if left == right {
        return true;
    }

    match (std::fs::canonicalize(left), std::fs::canonicalize(right)) {
        (Ok(left), Ok(right)) => left == right,
        _ => false,
    }
}

fn site_index_html_exists(site_dir: &StdPath) -> bool {
    site_dir.join("index.html").is_file()
}

fn resolve_site_dir_from_env(env_names: &[&str]) -> Option<PathBuf> {
    env_names.iter().find_map(|env_name| {
        std::env::var(env_name)
            .ok()
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouterProductRuntimeOptions {
    pub site_dir: ProductSiteDir,
    pub include_portal_api_routes: bool,
}

impl RouterProductRuntimeOptions {
    pub fn desktop(site_dir: ProductSiteDir) -> Self {
        Self {
            site_dir,
            include_portal_api_routes: true,
        }
    }

    pub fn desktop_for_api_assembly_host(site_dir: ProductSiteDir) -> Self {
        Self {
            site_dir,
            include_portal_api_routes: false,
        }
    }
}

#[derive(Debug)]
pub struct RouterProductRuntime {
    base_url: String,
    shutdown_tx: Option<oneshot::Sender<()>>,
    _site_dir: ProductSiteDir,
}

#[derive(Clone)]
struct RuntimeProxyState {
    client: Client,
    admin_proxy_target: String,
    admin_sandbox: Option<SharedAdminSandboxState>,
    pc_product_api_upstream: String,
    portal_api_base_url: String,
    site_dir: ProductSiteDir,
}

enum ResolvedSiteAsset {
    StaticFile(PathBuf),
    SpaShell(PathBuf),
}

impl RouterProductRuntime {
    pub async fn start(
        _loader: StandaloneConfigLoader,
        config: StandaloneConfig,
        options: RouterProductRuntimeOptions,
    ) -> Result<Self> {
        let listener = TcpListener::bind(resolve_runtime_bind_addr(
            config.runtime_bind_addr.as_str(),
        )?)
        .await
        .context("failed to bind local desktop runtime listener")?;
        let local_addr = listener
            .local_addr()
            .context("failed to resolve local desktop runtime listener address")?;
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let site_dir = options.site_dir.clone();
        let app = build_product_runtime_router(config, options).await?;

        tokio::spawn(async move {
            let _ = axum::serve(listener, app)
                .with_graceful_shutdown(async move {
                    let _ = shutdown_rx.await;
                })
                .await;
        });

        Ok(Self {
            base_url: format!("http://{local_addr}"),
            shutdown_tx: Some(shutdown_tx),
            _site_dir: site_dir,
        })
    }

    pub fn public_base_url(&self) -> Option<&str> {
        Some(self.base_url.as_str())
    }
}

async fn validate_product_site_dir(site_dir: ProductSiteDir) -> Result<()> {
    let pc_result = validate_site_dir(site_dir.pc_site_dir.as_path(), "PC renderer").await;
    let h5_result = match site_dir.h5_site_dir.as_deref() {
        Some(h5_site_dir) => validate_site_dir(h5_site_dir, "H5 renderer").await,
        None => Err(anyhow::anyhow!("H5 renderer is not configured")),
    };
    if pc_result.is_err() && h5_result.is_err() {
        anyhow::bail!(
            "at least one browser renderer must contain index.html; PC: {}; H5: {}",
            pc_result.expect_err("PC renderer result should be an error"),
            h5_result.expect_err("H5 renderer result should be an error")
        );
    }
    Ok(())
}

pub async fn build_product_runtime_router(
    config: StandaloneConfig,
    options: RouterProductRuntimeOptions,
) -> Result<Router> {
    ensure_admin_sandbox_allowed(
        config.admin_sandbox_enabled,
        im_app_context::is_production_like_im_environment(),
    )?;
    validate_product_site_dir(options.site_dir.clone()).await?;
    let include_portal_api_routes = options.include_portal_api_routes;
    let site_dir = options.site_dir;
    let state = build_runtime_proxy_state(config, site_dir.clone());

    let mut router = Router::new()
        .route(BACKEND_ADMIN_API_PREFIX, any(proxy_admin_request))
        .route(
            format!("{BACKEND_ADMIN_API_PREFIX}/{{*path}}").as_str(),
            any(proxy_admin_request),
        )
        .route("/api/config/modules", get(get_local_app_modules))
        .route("/api/agent/{*path}", any(proxy_pc_product_api_request));
    if include_portal_api_routes {
        router = router
            .route("/app/v3/api/portal/workspace", get(get_portal_workspace))
            .route("/app/v3/api/portal/{section}", get(get_portal_snapshot));
    }

    Ok(router
        .route("/app/v3/api", any(api_not_found))
        .route("/app/v3/api/{*path}", any(api_not_found))
        .route("/api", any(api_not_found))
        .route("/api/{*path}", any(api_not_found))
        .route("/admin", get(redirect_admin_root))
        .route("/admin/", get(serve_admin_site))
        .route("/admin/{*path}", get(serve_admin_site))
        .route("/", get(serve_web_site))
        .route("/{*path}", get(serve_web_site))
        .with_state(state))
}

fn build_runtime_proxy_state(
    config: StandaloneConfig,
    site_dir: ProductSiteDir,
) -> RuntimeProxyState {
    let admin_proxy_target = trim_trailing_slash(config.admin_proxy_target);
    let admin_sandbox = if admin_proxy_target.trim().is_empty() && config.admin_sandbox_enabled {
        let state = match config.admin_sandbox_storage_file {
            Some(storage_file) => SharedAdminSandboxState::seeded_with_storage_file(storage_file),
            None => SharedAdminSandboxState::seeded(),
        };
        eprintln!(
            "warning: SDKWORK_ADMIN_SANDBOX is enabled. Admin sandbox consumes sdkwork-appbase bearer tokens and does not provide sdkwork-im login endpoints."
        );
        Some(state)
    } else {
        None
    };

    RuntimeProxyState {
        client: Client::new(),
        admin_proxy_target,
        admin_sandbox,
        pc_product_api_upstream: resolve_pc_product_api_upstream(),
        portal_api_base_url: config.portal_api_base_url,
        site_dir,
    }
}

async fn validate_site_dir(site_dir: &StdPath, site_name: &str) -> Result<()> {
    let metadata = fs::metadata(site_dir).await.with_context(|| {
        format!(
            "desktop runtime {site_name} site directory is missing: {}",
            site_dir.display()
        )
    })?;
    if !metadata.is_dir() {
        anyhow::bail!(
            "desktop runtime {site_name} site directory is not a directory: {}",
            site_dir.display()
        );
    }

    let index_path = site_dir.join("index.html");
    let index_metadata = fs::metadata(index_path.as_path()).await.with_context(|| {
        format!(
            "desktop runtime {site_name} site is missing index.html: {}",
            index_path.display()
        )
    })?;
    if !index_metadata.is_file() {
        anyhow::bail!(
            "desktop runtime {site_name} site index.html is not a file: {}",
            index_path.display()
        );
    }

    Ok(())
}

impl Drop for RouterProductRuntime {
    fn drop(&mut self) {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(());
        }
    }
}

fn resolve_runtime_bind_addr(value: &str) -> Result<SocketAddr> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0));
    }

    trimmed
        .parse()
        .with_context(|| format!("invalid desktop runtime bind address: {trimmed}"))
}

fn trim_trailing_slash(value: String) -> String {
    value.trim().trim_end_matches('/').to_owned()
}

fn resolve_pc_product_api_upstream() -> String {
    std::env::var(PC_PRODUCT_API_UPSTREAM_ENV)
        .ok()
        .map(|value| value.trim().trim_end_matches('/').to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_default()
}

fn admin_proxy_path_and_query(uri: &Uri) -> String {
    uri.path_and_query()
        .map(|value| value.as_str())
        .unwrap_or(BACKEND_ADMIN_API_PREFIX)
        .to_owned()
}

async fn api_not_found() -> Response {
    json_error_response(StatusCode::NOT_FOUND, "Runtime route not found.")
}

async fn get_local_app_modules() -> Response {
    let modules = LOCAL_APP_MODULES
        .iter()
        .map(|module| format!("\"{}\"", escape_json_string(module)))
        .collect::<Vec<_>>()
        .join(",");
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .body(Body::from(format!("{{\"modules\":[{modules}]}}")))
        .expect("local modules response should build")
}

async fn get_portal_snapshot(Path(section): Path<String>) -> Response {
    let section = section.trim();
    if !PORTAL_SNAPSHOT_SECTIONS.contains(&section) {
        return json_error_response(StatusCode::NOT_FOUND, "Portal snapshot route not found.");
    }

    let ops = portal_ops_runtime();
    let snapshot = build_portal_snapshot_for_section(section, ops, None, None)
        .and_then(|value| serde_json::to_value(value).ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "meta": {
                    "section": section,
                    "opsStatus": "unknown",
                },
                "availability": {
                    "state": "unavailable",
                    "source": "local-product-runtime",
                    "complete": false,
                    "reason": "portal snapshot is unavailable",
                },
            })
        });
    json_response(StatusCode::OK, portal_envelope_json(snapshot))
}

async fn get_portal_workspace() -> Response {
    let workspace = build_portal_workspace_view();
    json_response(
        StatusCode::OK,
        portal_envelope_json(
            serde_json::to_value(workspace).unwrap_or_else(|_| serde_json::json!({})),
        ),
    )
}

fn portal_ops_runtime() -> Arc<OpsRuntime> {
    static OPS: OnceLock<Arc<OpsRuntime>> = OnceLock::new();
    OPS.get_or_init(|| Arc::new(OpsRuntime::default())).clone()
}

fn portal_envelope_json(item: serde_json::Value) -> String {
    let trace_id = format!("{:032x}", random::<u128>());
    serde_json::json!({
        "code": 0,
        "data": { "item": item },
        "traceId": trace_id,
    })
    .to_string()
}

async fn redirect_admin_root() -> Redirect {
    Redirect::permanent("/admin/")
}

async fn serve_admin_site(
    State(state): State<RuntimeProxyState>,
    headers: HeaderMap,
    uri: Uri,
) -> Response {
    let request_path = uri.path().strip_prefix("/admin").unwrap_or("/");
    let Some((_, site_dir)) = select_request_site_dir(&state.site_dir, &headers) else {
        return text_response(
            StatusCode::SERVICE_UNAVAILABLE,
            "No browser renderer is available",
        );
    };
    let mut response = serve_site_request(site_dir, request_path).await;
    apply_user_agent_vary(response.headers_mut());
    response
}

async fn serve_web_site(
    State(state): State<RuntimeProxyState>,
    headers: HeaderMap,
    uri: Uri,
) -> Response {
    let Some((client, site_dir)) = select_request_site_dir(&state.site_dir, &headers) else {
        return text_response(
            StatusCode::SERVICE_UNAVAILABLE,
            "No browser renderer is available",
        );
    };
    let mut response = match resolve_site_request_asset(site_dir, uri.path()).await {
        Ok(ResolvedSiteAsset::StaticFile(path)) => serve_site_file(path.as_path()).await,
        Ok(ResolvedSiteAsset::SpaShell(path)) if client == WebClient::Pc => {
            serve_pc_shell(path.as_path(), state.portal_api_base_url.as_str()).await
        }
        Ok(ResolvedSiteAsset::SpaShell(path)) => serve_site_file(path.as_path()).await,
        Err(response) => response,
    };
    apply_user_agent_vary(response.headers_mut());
    response
}

fn select_request_site_dir<'a>(
    site_dir: &'a ProductSiteDir,
    headers: &HeaderMap,
) -> Option<(WebClient, &'a StdPath)> {
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|value| value.to_str().ok());
    let pc_available = site_index_html_exists(site_dir.pc_site_dir.as_path());
    let h5_available = site_dir
        .h5_site_dir
        .as_deref()
        .is_some_and(site_index_html_exists);
    let client = select_available_web_client(user_agent, pc_available, h5_available)?;
    let selected_dir = match client {
        WebClient::Pc => site_dir.pc_site_dir.as_path(),
        WebClient::H5 => site_dir.h5_site_dir.as_deref()?,
    };
    Some((client, selected_dir))
}

fn apply_user_agent_vary(headers: &mut HeaderMap) {
    headers.insert(header::VARY, HeaderValue::from_static("user-agent"));
}

async fn serve_site_request(site_dir: &StdPath, request_path: &str) -> Response {
    match resolve_site_request_asset(site_dir, request_path).await {
        Ok(ResolvedSiteAsset::StaticFile(path) | ResolvedSiteAsset::SpaShell(path)) => {
            serve_site_file(path.as_path()).await
        }
        Err(response) => response,
    }
}

async fn resolve_site_request_asset(
    site_dir: &StdPath,
    request_path: &str,
) -> Result<ResolvedSiteAsset, Response> {
    let Some(relative_path) = sanitize_site_relative_path(request_path) else {
        return Err(text_response(StatusCode::NOT_FOUND, "Not Found"));
    };

    if relative_path.as_os_str().is_empty() {
        return Ok(ResolvedSiteAsset::SpaShell(site_dir.join("index.html")));
    }

    let candidate = site_dir.join(&relative_path);
    let top_level_index = relative_path == StdPath::new("index.html");
    match fs::metadata(&candidate).await {
        Ok(metadata) if metadata.is_file() => {
            return Ok(if top_level_index {
                ResolvedSiteAsset::SpaShell(candidate)
            } else {
                ResolvedSiteAsset::StaticFile(candidate)
            });
        }
        Ok(metadata) if metadata.is_dir() => {
            let nested_index = candidate.join("index.html");
            return Ok(ResolvedSiteAsset::StaticFile(nested_index));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(text_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to inspect runtime site asset: {error}"),
            ));
        }
    }

    if request_looks_like_static_asset(relative_path.as_path()) {
        return Err(text_response(StatusCode::NOT_FOUND, "Not Found"));
    }

    Ok(ResolvedSiteAsset::SpaShell(site_dir.join("index.html")))
}

fn sanitize_site_relative_path(request_path: &str) -> Option<PathBuf> {
    let trimmed = request_path.trim_start_matches('/');
    let mut normalized = PathBuf::new();

    for component in StdPath::new(trimmed).components() {
        match component {
            Component::Normal(part) => normalized.push(part),
            Component::CurDir => {}
            Component::RootDir => {}
            Component::ParentDir | Component::Prefix(_) => return None,
        }
    }

    Some(normalized)
}

fn request_looks_like_static_asset(relative_path: &StdPath) -> bool {
    relative_path.extension().is_some()
}

async fn serve_site_file(path: &StdPath) -> Response {
    match fs::read(path).await {
        Ok(body) => {
            let content_type = mime_guess::from_path(path)
                .first_or_octet_stream()
                .essence_str()
                .to_owned();
            let mut response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, content_type)
                .body(Body::from(body))
                .expect("runtime site file response should build");
            let is_html = response
                .headers()
                .get(header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .map(|value| value.starts_with("text/html"))
                .unwrap_or(false);
            if is_html {
                apply_html_shell_headers(response.headers_mut(), None);
            } else {
                apply_site_security_headers(response.headers_mut());
            }
            response
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            text_response(StatusCode::NOT_FOUND, "Not Found")
        }
        Err(error) => text_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read runtime site asset: {error}"),
        ),
    }
}

async fn serve_pc_shell(path: &StdPath, portal_api_base_url: &str) -> Response {
    match fs::read_to_string(path).await {
        Ok(html) => {
            let script_nonce = create_script_nonce();
            let injected = inject_portal_api_base_url(
                html.as_str(),
                portal_api_base_url,
                script_nonce.as_str(),
            );
            let mut response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                .body(Body::from(injected))
                .expect("PC shell response should build");
            apply_html_shell_headers(
                response.headers_mut(),
                Some(HtmlShellSecurityPolicy::for_portal_shell(
                    portal_api_base_url,
                    script_nonce.as_str(),
                )),
            );
            response
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            text_response(StatusCode::NOT_FOUND, "Not Found")
        }
        Err(error) => text_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read runtime PC shell: {error}"),
        ),
    }
}

fn inject_portal_api_base_url(html: &str, portal_api_base_url: &str, script_nonce: &str) -> String {
    let html = apply_nonce_to_inline_portal_scripts(html, script_nonce);
    let serialized_url = serde_json::to_string(portal_api_base_url)
        .expect("portal api base url should serialize into javascript");
    let script = format!(
        "<script nonce=\"{script_nonce}\">window.__SDKWORK_IM_PORTAL_API_BASE_URL__ = {serialized_url};</script>"
    );

    if let Some(head_close_index) = html.find("</head>") {
        let mut injected = String::with_capacity(html.len() + script.len());
        injected.push_str(&html[..head_close_index]);
        injected.push_str(script.as_str());
        injected.push_str(&html[head_close_index..]);
        return injected;
    }

    format!("{script}{html}")
}

fn apply_nonce_to_inline_portal_scripts(html: &str, script_nonce: &str) -> String {
    let mut result = String::with_capacity(html.len() + 64);
    let mut cursor = 0;

    while let Some(relative_start) = html[cursor..].find("<script") {
        let start = cursor + relative_start;
        result.push_str(&html[cursor..start]);

        let Some(relative_end) = html[start..].find('>') else {
            result.push_str(&html[start..]);
            return result;
        };
        let end = start + relative_end + 1;
        let opening_tag = &html[start..end];

        if script_tag_requires_runtime_nonce(opening_tag) {
            let tag_without_close = &opening_tag[..opening_tag.len() - 1];
            result.push_str(tag_without_close);
            result.push_str(format!(" nonce=\"{script_nonce}\">").as_str());
        } else {
            result.push_str(opening_tag);
        }

        cursor = end;
    }

    result.push_str(&html[cursor..]);
    result
}

fn script_tag_requires_runtime_nonce(opening_tag: &str) -> bool {
    let normalized = opening_tag.to_ascii_lowercase();
    let is_importmap = normalized.contains(r#"type="importmap""#)
        || normalized.contains("type='importmap'")
        || normalized.contains("type=importmap");

    is_importmap && !normalized.contains(" src=") && !normalized.contains(" nonce=")
}

struct HtmlShellSecurityPolicy {
    connect_src: String,
    script_nonce: Option<String>,
}

impl HtmlShellSecurityPolicy {
    fn default_shell() -> Self {
        Self {
            connect_src: "'self'".to_owned(),
            script_nonce: None,
        }
    }

    fn for_portal_shell(portal_api_base_url: &str, script_nonce: &str) -> Self {
        Self {
            connect_src: resolve_connect_src(portal_api_base_url),
            script_nonce: Some(script_nonce.to_owned()),
        }
    }
}

fn create_script_nonce() -> String {
    format!("{:032x}", random::<u128>())
}

fn apply_site_security_headers(headers: &mut HeaderMap) {
    headers.insert(
        X_CONTENT_TYPE_OPTIONS_HEADER,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        REFERRER_POLICY_HEADER,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(X_FRAME_OPTIONS_HEADER, HeaderValue::from_static("DENY"));
    headers.insert(
        PERMISSIONS_POLICY_HEADER,
        HeaderValue::from_static(DEFAULT_PERMISSIONS_POLICY),
    );
    headers.insert(
        CROSS_ORIGIN_RESOURCE_POLICY_HEADER,
        HeaderValue::from_static("same-origin"),
    );
}

fn apply_html_shell_headers(headers: &mut HeaderMap, policy: Option<HtmlShellSecurityPolicy>) {
    let policy = policy.unwrap_or_else(HtmlShellSecurityPolicy::default_shell);
    apply_site_security_headers(headers);
    headers.insert(CACHE_CONTROL_HEADER, HeaderValue::from_static("no-store"));
    headers.insert(
        CONTENT_SECURITY_POLICY_HEADER,
        HeaderValue::from_str(create_html_content_security_policy(policy).as_str())
            .expect("html shell content security policy should be valid"),
    );
}

fn create_html_content_security_policy(policy: HtmlShellSecurityPolicy) -> String {
    let script_src = match policy.script_nonce.as_deref() {
        Some(script_nonce) => format!("'self' 'nonce-{script_nonce}'"),
        None => "'self'".to_owned(),
    };

    format!(
        "default-src 'self'; base-uri 'self'; connect-src {}; font-src 'self' data:; form-action 'self'; frame-ancestors 'none'; img-src 'self' data: blob:; object-src 'none'; script-src {}; style-src 'self' 'unsafe-inline'",
        policy.connect_src, script_src
    )
}

fn resolve_connect_src(portal_api_base_url: &str) -> String {
    let mut sources = vec!["'self'".to_owned()];

    if let Ok(url) = Url::parse(portal_api_base_url) {
        if matches!(url.scheme(), "http" | "https") {
            let origin = url.origin().ascii_serialization();
            push_unique_source(&mut sources, origin);
            if let Some(websocket_origin) = websocket_origin_for_url(&url) {
                push_unique_source(&mut sources, websocket_origin);
            }
        }
    }

    sources.join(" ")
}

fn push_unique_source(sources: &mut Vec<String>, value: String) {
    if !sources.iter().any(|existing| existing == &value) {
        sources.push(value);
    }
}

fn websocket_origin_for_url(url: &Url) -> Option<String> {
    let websocket_scheme = match url.scheme() {
        "http" => "ws",
        "https" => "wss",
        _ => return None,
    };
    let host = match url.host()? {
        Host::Domain(value) => value.to_owned(),
        Host::Ipv4(value) => value.to_string(),
        Host::Ipv6(value) => format!("[{value}]"),
    };

    match url.port() {
        Some(port) => Some(format!("{websocket_scheme}://{host}:{port}")),
        None => Some(format!("{websocket_scheme}://{host}")),
    }
}

async fn proxy_admin_request(
    State(state): State<RuntimeProxyState>,
    method: Method,
    headers: HeaderMap,
    uri: Uri,
    body: Bytes,
) -> Response {
    if let Some(admin_sandbox) = &state.admin_sandbox {
        return handle_admin_sandbox_request(admin_sandbox, method, headers, uri, body).await;
    }

    if state.admin_proxy_target.trim().is_empty() {
        return json_error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            ADMIN_BACKEND_NOT_CONFIGURED_MESSAGE,
        );
    }

    let upstream_url = format!(
        "{}{}",
        state.admin_proxy_target,
        admin_proxy_path_and_query(&uri),
    );
    let mut request_builder = state.client.request(method, upstream_url);

    for (name, value) in headers.iter() {
        if *name == header::HOST || *name == header::CONTENT_LENGTH || *name == header::CONNECTION {
            continue;
        }
        request_builder = request_builder.header(name, value);
    }

    match request_builder.body(body).send().await {
        Ok(upstream_response) => build_proxy_response(upstream_response).await,
        Err(error) => json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("desktop admin proxy request failed: {error}").as_str(),
        ),
    }
}

async fn proxy_pc_product_api_request(
    State(state): State<RuntimeProxyState>,
    method: Method,
    headers: HeaderMap,
    uri: Uri,
    body: Bytes,
) -> Response {
    if state.pc_product_api_upstream.trim().is_empty() {
        return json_error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            format!(
                "PC product API upstream is not configured. Set {PC_PRODUCT_API_UPSTREAM_ENV} to a backend that serves /api/agent/*."
            )
            .as_str(),
        );
    }

    let upstream_url = format!(
        "{}{}",
        state.pc_product_api_upstream,
        uri.path_and_query()
            .map(|value| value.as_str())
            .unwrap_or("/api/agent"),
    );
    let mut request_builder = state.client.request(method, upstream_url);

    for (name, value) in headers.iter() {
        if *name == header::HOST || *name == header::CONTENT_LENGTH || *name == header::CONNECTION {
            continue;
        }
        request_builder = request_builder.header(name, value);
    }

    match request_builder.body(body).send().await {
        Ok(upstream_response) => build_proxy_response(upstream_response).await,
        Err(error) => json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("PC product API proxy request failed: {error}").as_str(),
        ),
    }
}

fn json_response(status: StatusCode, body: impl Into<String>) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .body(Body::from(body.into()))
        .expect("json runtime response should build")
}

fn text_response(status: StatusCode, message: impl Into<String>) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(Body::from(message.into()))
        .expect("text runtime response should build")
}

fn json_error_response(status: StatusCode, message: &str) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .body(Body::from(format!(
            "{{\"error\":{{\"message\":\"{}\"}},\"status\":{}}}",
            escape_json_string(message),
            status.as_u16()
        )))
        .expect("json proxy error response should build")
}

fn escape_json_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

async fn build_proxy_response(upstream_response: reqwest::Response) -> Response {
    let status = upstream_response.status();
    let headers = upstream_response.headers().clone();
    let body = upstream_response.bytes().await.unwrap_or_default();
    let mut response_builder = Response::builder().status(status);

    for (name, value) in headers.iter() {
        if *name == header::TRANSFER_ENCODING || *name == header::CONNECTION {
            continue;
        }
        response_builder = response_builder.header(name, value);
    }

    response_builder
        .body(Body::from(body))
        .expect("proxied admin response should build")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use std::{
        fs,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    struct TestSiteDir {
        path: PathBuf,
    }

    impl TestSiteDir {
        fn new(label: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after unix epoch")
                .as_nanos();
            let path =
                std::env::temp_dir().join(format!("sdkwork-api-product-runtime-{label}-{unique}"));
            fs::create_dir_all(&path).expect("test site dir should be creatable");
            Self { path }
        }

        fn path(&self) -> &Path {
            self.path.as_path()
        }

        fn write(&self, relative_path: &str, body: &str) {
            let file_path = self.path.join(relative_path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).expect("test file parent dir should be creatable");
            }
            fs::write(file_path, body).expect("test site file should be writable");
        }
    }

    impl Drop for TestSiteDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    async fn start_runtime(site_dir: ProductSiteDir) -> RouterProductRuntime {
        RouterProductRuntime::start(
            StandaloneConfigLoader,
            StandaloneConfig {
                runtime_bind_addr: "127.0.0.1:0".into(),
                admin_proxy_target: String::new(),
                portal_api_base_url: "http://127.0.0.1:18079".into(),
                admin_sandbox_enabled: false,
                admin_sandbox_storage_file: None,
            },
            RouterProductRuntimeOptions::desktop(site_dir),
        )
        .await
        .expect("desktop product runtime should start")
    }

    async fn fetch_response(base_url: &str, path: &str) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{base_url}{path}"))
            .send()
            .await
            .expect("runtime request should succeed")
    }

    async fn fetch_response_with_user_agent(
        base_url: &str,
        path: &str,
        user_agent: &str,
    ) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{base_url}{path}"))
            .header(header::USER_AGENT, user_agent)
            .send()
            .await
            .expect("runtime request should succeed")
    }

    fn response_header(response: &reqwest::Response, name: &str) -> Option<String> {
        response
            .headers()
            .get(name)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned)
    }

    async fn parse_json_response(
        response: reqwest::Response,
        description: &str,
    ) -> serde_json::Value {
        let body = response
            .text()
            .await
            .unwrap_or_else(|error| panic!("{description} body should be readable: {error}"));
        serde_json::from_str(body.as_str()).unwrap_or_else(|error| {
            panic!("{description} should be valid JSON: {error}; body: {body}")
        })
    }

    #[tokio::test]
    async fn proxy_admin_request_returns_structured_503_when_backend_target_is_missing() {
        let response = proxy_admin_request(
            State(RuntimeProxyState {
                client: Client::new(),
                admin_proxy_target: String::new(),
                admin_sandbox: None,
                pc_product_api_upstream: String::new(),
                portal_api_base_url: "http://127.0.0.1:18079".into(),
                site_dir: ProductSiteDir::new("."),
            }),
            Method::GET,
            HeaderMap::new(),
            Uri::from_static("/backend/v3/api/admin/storage/config"),
            Bytes::new(),
        )
        .await;

        let status = response.status();
        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body should be readable");
        let body_text = String::from_utf8(body.to_vec()).expect("response body should be utf8");

        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(
            content_type.as_deref(),
            Some("application/json; charset=utf-8")
        );
        assert!(body_text.contains("SDKWORK_ADMIN_PROXY_TARGET"));
        assert!(body_text.contains("/backend/v3/api/admin"));
    }

    #[tokio::test]
    async fn api_assembly_host_options_do_not_register_portal_api_routes() {
        let pc_site_dir = TestSiteDir::new("assembly-host-pc");
        pc_site_dir.write("index.html", "<!doctype html><title>pc-shell</title>");

        let product_router = build_product_runtime_router(
            StandaloneConfig {
                runtime_bind_addr: "127.0.0.1:0".into(),
                admin_proxy_target: String::new(),
                portal_api_base_url: "http://127.0.0.1:18079".into(),
                admin_sandbox_enabled: false,
                admin_sandbox_storage_file: None,
            },
            RouterProductRuntimeOptions::desktop_for_api_assembly_host(ProductSiteDir::new(
                pc_site_dir.path().to_path_buf(),
            )),
        )
        .await
        .expect("assembly host product router should build");

        let _merged = product_router.merge(Router::new().route(
            "/app/v3/api/portal/workspace",
            get(|| async { StatusCode::OK }),
        ));
    }

    #[tokio::test]
    async fn router_runtime_serves_one_pc_renderer_for_root_and_admin_spa_routes() {
        let pc_site_dir = TestSiteDir::new("pc-site");
        pc_site_dir.write("index.html", "<!doctype html><title>pc-shell</title>");
        pc_site_dir.write("assets/pc.js", "console.log('pc-asset');");

        let runtime = start_runtime(ProductSiteDir::new(pc_site_dir.path().to_path_buf())).await;
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();

        let admin_index = fetch_response(base_url.as_str(), "/admin/").await;
        assert_eq!(admin_index.status(), StatusCode::OK);
        assert!(admin_index
            .text()
            .await
            .expect("admin index body should be readable")
            .contains("pc-shell"));

        let admin_route = fetch_response(base_url.as_str(), "/admin/operators/shift").await;
        assert_eq!(admin_route.status(), StatusCode::OK);
        assert!(admin_route
            .text()
            .await
            .expect("admin route body should be readable")
            .contains("pc-shell"));

        let admin_asset = fetch_response(base_url.as_str(), "/admin/assets/pc.js").await;
        assert_eq!(admin_asset.status(), StatusCode::OK);
        assert_eq!(
            admin_asset
                .text()
                .await
                .expect("admin asset body should be readable"),
            "console.log('pc-asset');"
        );

        let portal_index = fetch_response(base_url.as_str(), "/").await;
        assert_eq!(portal_index.status(), StatusCode::OK);
        assert!(portal_index
            .text()
            .await
            .expect("PC index body should be readable")
            .contains("pc-shell"));

        let portal_route = fetch_response(base_url.as_str(), "/workspace/inbox").await;
        assert_eq!(portal_route.status(), StatusCode::OK);
        assert!(portal_route
            .text()
            .await
            .expect("PC route body should be readable")
            .contains("pc-shell"));

        let pc_asset = fetch_response(base_url.as_str(), "/assets/pc.js").await;
        assert_eq!(pc_asset.status(), StatusCode::OK);
        assert_eq!(
            pc_asset
                .text()
                .await
                .expect("PC asset body should be readable"),
            "console.log('pc-asset');"
        );
    }

    #[tokio::test]
    async fn router_runtime_selects_h5_for_mobile_and_pc_for_desktop_on_one_origin() {
        let pc_site_dir = TestSiteDir::new("adaptive-pc");
        pc_site_dir.write("index.html", "<!doctype html><title>pc-shell</title>");
        pc_site_dir.write("assets/client.js", "console.log('pc');");
        let h5_site_dir = TestSiteDir::new("adaptive-h5");
        h5_site_dir.write("index.html", "<!doctype html><title>h5-shell</title>");
        h5_site_dir.write("assets/client.js", "console.log('h5');");

        let runtime = start_runtime(
            ProductSiteDir::new(pc_site_dir.path().to_path_buf())
                .with_h5_site_dir(h5_site_dir.path().to_path_buf()),
        )
        .await;
        tokio::task::yield_now().await;
        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url");

        let desktop = fetch_response_with_user_agent(base_url, "/", "Windows NT 10.0").await;
        assert_eq!(desktop.status(), StatusCode::OK);
        assert_eq!(
            response_header(&desktop, header::VARY.as_str()).as_deref(),
            Some("user-agent")
        );
        assert!(desktop
            .text()
            .await
            .expect("PC body should be readable")
            .contains("pc-shell"));

        let mobile = fetch_response_with_user_agent(base_url, "/", "iPhone Mobile").await;
        assert_eq!(mobile.status(), StatusCode::OK);
        assert_eq!(
            response_header(&mobile, header::VARY.as_str()).as_deref(),
            Some("user-agent")
        );
        assert!(mobile
            .text()
            .await
            .expect("H5 body should be readable")
            .contains("h5-shell"));

        let mobile_asset =
            fetch_response_with_user_agent(base_url, "/assets/client.js", "Android Mobile").await;
        assert_eq!(
            mobile_asset
                .text()
                .await
                .expect("H5 asset should be readable"),
            "console.log('h5');"
        );
    }

    #[tokio::test]
    async fn router_runtime_falls_back_when_either_renderer_is_missing() {
        let missing_pc = TestSiteDir::new("missing-pc");
        let h5_site_dir = TestSiteDir::new("fallback-h5");
        h5_site_dir.write("index.html", "<!doctype html><title>h5-fallback</title>");
        let h5_runtime = start_runtime(
            ProductSiteDir::new(missing_pc.path().to_path_buf())
                .with_h5_site_dir(h5_site_dir.path().to_path_buf()),
        )
        .await;
        let h5_base_url = h5_runtime
            .public_base_url()
            .expect("H5 fallback runtime should expose a URL");
        let desktop = fetch_response_with_user_agent(h5_base_url, "/", "Windows NT 10.0").await;
        assert!(desktop
            .text()
            .await
            .expect("H5 fallback should be readable")
            .contains("h5-fallback"));

        let pc_site_dir = TestSiteDir::new("fallback-pc");
        pc_site_dir.write("index.html", "<!doctype html><title>pc-fallback</title>");
        let pc_runtime = start_runtime(ProductSiteDir::new(pc_site_dir.path().to_path_buf())).await;
        let pc_base_url = pc_runtime
            .public_base_url()
            .expect("PC fallback runtime should expose a URL");
        let mobile = fetch_response_with_user_agent(pc_base_url, "/", "iPhone Mobile").await;
        assert!(mobile
            .text()
            .await
            .expect("PC fallback should be readable")
            .contains("pc-fallback"));
    }

    #[tokio::test]
    async fn router_runtime_injects_portal_api_base_url_into_pc_shell() {
        let pc_site_dir = TestSiteDir::new("pc-injection");
        pc_site_dir.write(
            "index.html",
            "<!doctype html><html><head><title>pc-shell</title></head><body>pc</body></html>",
        );

        let runtime = RouterProductRuntime::start(
            StandaloneConfigLoader,
            StandaloneConfig {
                runtime_bind_addr: "127.0.0.1:0".into(),
                admin_proxy_target: String::new(),
                portal_api_base_url: "https://portal-api.example.com/runtime-edge".into(),
                admin_sandbox_enabled: false,
                admin_sandbox_storage_file: None,
            },
            RouterProductRuntimeOptions::desktop(ProductSiteDir::new(
                pc_site_dir.path().to_path_buf(),
            )),
        )
        .await
        .expect("desktop product runtime should start");
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();

        let portal_index = fetch_response(base_url.as_str(), "/").await;
        assert_eq!(portal_index.status(), StatusCode::OK);
        let content_security_policy =
            response_header(&portal_index, CONTENT_SECURITY_POLICY_HEADER)
                .expect("PC shell should include a content security policy");
        assert!(content_security_policy.contains("https://portal-api.example.com"));
        assert!(content_security_policy.contains("wss://portal-api.example.com"));
        assert_eq!(
            response_header(&portal_index, CACHE_CONTROL_HEADER).as_deref(),
            Some("no-store")
        );
        assert_eq!(
            response_header(&portal_index, X_CONTENT_TYPE_OPTIONS_HEADER).as_deref(),
            Some("nosniff")
        );
        assert_eq!(
            response_header(&portal_index, REFERRER_POLICY_HEADER).as_deref(),
            Some("strict-origin-when-cross-origin")
        );
        assert_eq!(
            response_header(&portal_index, X_FRAME_OPTIONS_HEADER).as_deref(),
            Some("DENY")
        );
        assert_eq!(
            response_header(&portal_index, PERMISSIONS_POLICY_HEADER).as_deref(),
            Some(DEFAULT_PERMISSIONS_POLICY)
        );
        assert_eq!(
            response_header(&portal_index, CROSS_ORIGIN_RESOURCE_POLICY_HEADER).as_deref(),
            Some("same-origin")
        );
        let body = portal_index
            .text()
            .await
            .expect("portal index body should be readable");
        assert!(body.contains("__SDKWORK_IM_PORTAL_API_BASE_URL__"));
        assert!(body.contains("https://portal-api.example.com/runtime-edge"));
        let nonce_start = body
            .find("script nonce=\"")
            .expect("PC shell should inject a nonce-backed script")
            + "script nonce=\"".len();
        let nonce_end = body[nonce_start..]
            .find('"')
            .map(|offset| nonce_start + offset)
            .expect("PC shell nonce should terminate");
        let nonce = &body[nonce_start..nonce_end];
        assert!(content_security_policy.contains(format!("'nonce-{nonce}'").as_str()));
        assert!(content_security_policy.contains("script-src 'self' 'nonce-"));
        assert!(!content_security_policy.contains("script-src 'self' 'unsafe-inline'"));
    }

    #[test]
    fn portal_shell_injection_applies_runtime_nonce_to_inline_importmap_scripts() {
        let html = r#"<!doctype html><html><head><script type="importmap">{ "imports": { "@sdkwork/sdk-common": "/__vendor__/sdkwork-sdk-common/index.js" } }</script></head><body>portal</body></html>"#;

        let injected = inject_portal_api_base_url(
            html,
            "https://portal-api.example.com/runtime-edge",
            "nonce123",
        );

        assert!(injected.contains(r#"<script type="importmap" nonce="nonce123">"#));
    }

    #[tokio::test]
    async fn router_runtime_applies_security_headers_to_admin_shells_and_static_assets() {
        let pc_site_dir = TestSiteDir::new("pc-security");
        pc_site_dir.write(
            "index.html",
            "<!doctype html><html><head><title>pc-shell</title></head><body>pc</body></html>",
        );
        pc_site_dir.write("assets/pc.js", "console.log('pc-asset');");

        let runtime = start_runtime(ProductSiteDir::new(pc_site_dir.path().to_path_buf())).await;
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();

        let admin_shell = fetch_response(base_url.as_str(), "/admin/").await;
        assert_eq!(admin_shell.status(), StatusCode::OK);
        let admin_csp = response_header(&admin_shell, CONTENT_SECURITY_POLICY_HEADER)
            .expect("admin shell should emit a content security policy");
        assert!(admin_csp.contains("connect-src 'self'"));
        assert!(!admin_csp.contains("nonce-"));
        assert_eq!(
            response_header(&admin_shell, CACHE_CONTROL_HEADER).as_deref(),
            Some("no-store")
        );

        let pc_asset = fetch_response(base_url.as_str(), "/assets/pc.js").await;
        assert_eq!(pc_asset.status(), StatusCode::OK);
        assert_eq!(
            response_header(&pc_asset, X_CONTENT_TYPE_OPTIONS_HEADER).as_deref(),
            Some("nosniff")
        );
        assert_eq!(
            response_header(&pc_asset, REFERRER_POLICY_HEADER).as_deref(),
            Some("strict-origin-when-cross-origin")
        );
        assert_eq!(
            response_header(&pc_asset, X_FRAME_OPTIONS_HEADER).as_deref(),
            Some("DENY")
        );
        assert_eq!(
            response_header(&pc_asset, PERMISSIONS_POLICY_HEADER).as_deref(),
            Some(DEFAULT_PERMISSIONS_POLICY)
        );
        assert_eq!(
            response_header(&pc_asset, CROSS_ORIGIN_RESOURCE_POLICY_HEADER).as_deref(),
            Some("same-origin")
        );
        assert_eq!(
            response_header(&pc_asset, CONTENT_SECURITY_POLICY_HEADER),
            None
        );
        assert_eq!(response_header(&pc_asset, CACHE_CONTROL_HEADER), None);
    }

    #[tokio::test]
    async fn router_runtime_refuses_to_start_without_pc_renderer_index_html() {
        let pc_site_dir = TestSiteDir::new("pc-missing-index");

        let error = RouterProductRuntime::start(
            StandaloneConfigLoader,
            StandaloneConfig {
                runtime_bind_addr: "127.0.0.1:0".into(),
                admin_proxy_target: String::new(),
                portal_api_base_url: "http://127.0.0.1:18079".into(),
                admin_sandbox_enabled: false,
                admin_sandbox_storage_file: None,
            },
            RouterProductRuntimeOptions::desktop(ProductSiteDir::new(
                pc_site_dir.path().to_path_buf(),
            )),
        )
        .await
        .expect_err("runtime should fail fast when the PC renderer index is missing");

        assert!(error.to_string().contains("PC renderer"));
        assert!(error.to_string().contains("index.html"));
    }

    #[tokio::test]
    async fn router_runtime_keeps_api_and_missing_assets_outside_spa_fallback() {
        let pc_site_dir = TestSiteDir::new("pc-api-guard");
        pc_site_dir.write("index.html", "<!doctype html><title>pc-shell</title>");

        let runtime = start_runtime(ProductSiteDir::new(pc_site_dir.path().to_path_buf())).await;
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();

        let missing_admin_asset =
            fetch_response(base_url.as_str(), "/admin/assets/missing.js").await;
        assert_eq!(missing_admin_asset.status(), StatusCode::NOT_FOUND);
        assert!(!missing_admin_asset
            .text()
            .await
            .expect("missing admin asset body should be readable")
            .contains("pc-shell"));

        let missing_portal_asset = fetch_response(base_url.as_str(), "/assets/missing.js").await;
        assert_eq!(missing_portal_asset.status(), StatusCode::NOT_FOUND);
        assert!(!missing_portal_asset
            .text()
            .await
            .expect("missing PC asset body should be readable")
            .contains("pc-shell"));

        let unknown_api = fetch_response(base_url.as_str(), "/api/runtime-health").await;
        assert_eq!(unknown_api.status(), StatusCode::NOT_FOUND);
        assert!(!unknown_api
            .text()
            .await
            .expect("unknown api body should be readable")
            .contains("pc-shell"));

        let modules_api = fetch_response(base_url.as_str(), "/api/config/modules").await;
        assert_eq!(modules_api.status(), StatusCode::OK);
        let modules_body = modules_api
            .text()
            .await
            .expect("modules api body should be readable");
        assert!(modules_body.contains("\"chat\""));
        assert!(modules_body.contains("\"knowledge\""));

        let agent_api = reqwest::Client::new()
            .post(format!("{base_url}/api/agent/doc"))
            .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
            .body(r#"{"action":"summarize","content":"hello"}"#)
            .send()
            .await
            .expect("agent api request should complete");
        assert_eq!(agent_api.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert!(agent_api
            .text()
            .await
            .expect("agent api body should be readable")
            .contains("SDKWORK_IM_PC_API_UPSTREAM"));

        let admin_api =
            fetch_response(base_url.as_str(), "/backend/v3/api/admin/storage/config").await;
        assert_eq!(admin_api.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert!(admin_api
            .text()
            .await
            .expect("admin api body should be readable")
            .contains("SDKWORK_ADMIN_PROXY_TARGET"));
    }

    #[tokio::test]
    async fn router_runtime_serves_sdkwork_app_portal_home_snapshot() {
        let pc_site_dir = TestSiteDir::new("portal-home-pc");
        pc_site_dir.write("index.html", "<!doctype html><title>pc-shell</title>");

        let runtime = start_runtime(ProductSiteDir::new(pc_site_dir.path().to_path_buf())).await;
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();

        let response = fetch_response(base_url.as_str(), "/app/v3/api/portal/home").await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response_header(&response, header::CONTENT_TYPE.as_str()).as_deref(),
            Some(JSON_CONTENT_TYPE),
        );
        let value = parse_json_response(response, "portal home snapshot").await;
        assert_eq!(value.get("code").and_then(|value| value.as_i64()), Some(0));
        assert!(
            value
                .get("traceId")
                .and_then(|value| value.as_str())
                .is_some(),
            "portal home snapshot must include SdkWorkApiResponse traceId"
        );
        let item = value
            .get("data")
            .and_then(|data| data.get("item"))
            .expect("portal home snapshot should use SdkWorkApiResponse data.item");
        assert_eq!(
            item.pointer("/meta/section")
                .and_then(|value| value.as_str()),
            Some("home"),
        );
        assert_eq!(
            item.pointer("/availability/state")
                .and_then(|value| value.as_str()),
            Some("unavailable"),
        );
        assert_eq!(
            item.pointer("/availability/complete")
                .and_then(|value| value.as_bool()),
            Some(false),
        );
        assert!(
            item.get("enabledModules").is_none(),
            "portal home must not fabricate a local module catalogue when no authority is wired"
        );
        assert!(
            item.get("organizationDirectory")
                .is_none_or(serde_json::Value::is_null),
            "portal home snapshot must not embed legacy organization directory data; IAM directory endpoints own that surface"
        );
    }

    #[tokio::test]
    async fn router_runtime_does_not_expose_appbase_owned_iam_routes() {
        let pc_site_dir = TestSiteDir::new("appbase-owned-iam-pc");
        pc_site_dir.write("index.html", "<!doctype html><title>pc-shell</title>");

        let runtime = start_runtime(ProductSiteDir::new(pc_site_dir.path().to_path_buf())).await;
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();
        let client = reqwest::Client::new();

        for (method, path, body) in [
            (
                reqwest::Method::POST,
                "/app/v3/api/auth/sessions",
                serde_json::json!({
                    "username": "dev-bootstrap@sdkwork-iam.local",
                    "password": "wrong-password",
                }),
            ),
            (
                reqwest::Method::POST,
                "/app/v3/api/auth/registrations",
                serde_json::json!({
                    "username": "new-user@sdkwork-iam.local",
                    "password": "dev123456",
                }),
            ),
            (
                reqwest::Method::POST,
                "/app/v3/api/auth/sessions/refresh",
                serde_json::json!({ "refreshToken": "local-refresh-fake" }),
            ),
            (
                reqwest::Method::POST,
                "/app/v3/api/oauth/device_authorizations",
                serde_json::json!({}),
            ),
            (
                reqwest::Method::POST,
                "/app/v3/api/oauth/device_authorizations/session-1/password_completions",
                serde_json::json!({
                    "username": "dev-bootstrap@sdkwork-iam.local",
                    "password": "wrong-password",
                }),
            ),
        ] {
            let response = client
                .request(method, format!("{base_url}{path}"))
                .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
                .body(body.to_string())
                .send()
                .await
                .expect("appbase-owned POST request should return response");
            assert_eq!(
                response.status(),
                StatusCode::NOT_FOUND,
                "{path} must not be served by sdkwork-api-product-runtime; sdkwork-appbase owns IAM login/session validation"
            );
        }

        for path in [
            "/app/v3/api/auth/sessions/current",
            "/app/v3/api/iam/users/current",
            "/app/v3/api/system/iam/runtime",
            "/app/v3/api/system/iam/verification_policy",
            "/app/v3/api/oauth/device_authorizations/session-1",
        ] {
            let response = fetch_response(base_url.as_str(), path).await;
            assert_eq!(
                response.status(),
                StatusCode::NOT_FOUND,
                "{path} must not be served by sdkwork-api-product-runtime; sdkwork-appbase owns IAM login/session validation"
            );
        }
    }

    #[tokio::test]
    async fn router_runtime_keeps_appbase_iam_directory_routes_unowned() {
        let pc_site_dir = TestSiteDir::new("iam-directory-pc");
        pc_site_dir.write("index.html", "<!doctype html><title>pc-shell</title>");

        let runtime = start_runtime(ProductSiteDir::new(pc_site_dir.path().to_path_buf())).await;
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();

        for path in [
            "/app/v3/api/iam/organizations",
            "/app/v3/api/iam/organizations/tree",
            "/app/v3/api/iam/organization_memberships?organizationId=sdkwork-local-org",
            "/app/v3/api/iam/departments?organizationId=sdkwork-local-org",
            "/app/v3/api/iam/departments/tree?organizationId=sdkwork-local-org",
            "/app/v3/api/iam/department_assignments?departmentId=dept-product",
            "/app/v3/api/iam/positions?departmentId=dept-product",
            "/app/v3/api/iam/position_assignments?departmentAssignmentId=assignment-dev-product",
            "/app/v3/api/iam/role_bindings?scopeKind=department&scopeId=dept-product",
        ] {
            let response = fetch_response(base_url.as_str(), path).await;
            assert_eq!(
                response.status(),
                StatusCode::NOT_FOUND,
                "{path} must not be served by sdkwork-api-product-runtime; sdkwork-appbase owns IAM directory routes"
            );
        }
    }

    #[test]
    fn resolve_product_site_dir_uses_the_standard_pc_dist_without_runtime_fallbacks() {
        let repo_root = std::env::temp_dir().join(format!(
            "sdkwork-api-product-runtime-dev-fallback-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after unix epoch")
                .as_nanos()
        ));
        let pc_dist = repo_root.join("apps").join("sdkwork-im-pc").join("dist");
        fs::create_dir_all(&pc_dist).expect("PC renderer dist should be creatable");
        fs::write(pc_dist.join("index.html"), "<!doctype html>")
            .expect("PC renderer index should be writable");

        unsafe {
            std::env::remove_var("SDKWORK_IM_ADMIN_SITE_DIR");
            std::env::remove_var("SDKWORK_IM_PORTAL_SITE_DIR");
            std::env::remove_var("SDKWORK_ADMIN_SITE_DIR");
            std::env::remove_var("SDKWORK_PORTAL_SITE_DIR");
        }

        let resolved = resolve_product_site_dir_from_env(&repo_root)
            .expect("standard PC renderer dist should resolve");
        assert_eq!(resolved.pc_site_dir, pc_dist);

        let _ = fs::remove_dir_all(&repo_root);
    }

    #[test]
    fn divergent_product_site_dirs_are_rejected() {
        let error = select_shared_product_site_dir(
            Some(PathBuf::from("admin-dist")),
            Some(PathBuf::from("portal-dist")),
        )
        .expect_err("separate frontend builds must not satisfy the shared PC renderer boundary");

        assert!(error
            .to_string()
            .contains("same shared apps/sdkwork-im-pc renderer"));
    }

    #[test]
    fn standalone_config_tracks_admin_sandbox_mode() {
        let config_source = include_str!("../../sdkwork-api-config/src/lib.rs");

        assert!(config_source.contains("SDKWORK_ADMIN_SANDBOX"));
        assert!(config_source.contains("SDKWORK_ADMIN_SANDBOX_STORAGE_FILE"));
    }
}
