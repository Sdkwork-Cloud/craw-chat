#![allow(clippy::items_after_test_module)]

use std::collections::{BTreeMap, BTreeSet};

use sdkwork_im_api_registry::{HttpMethod, RouteProtocol};
use serde_json::{Map, Value, json};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteEntry {
    pub path: String,
    pub methods: Vec<HttpMethod>,
    pub protocol: RouteProtocol,
    pub websocket_subprotocols: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebsocketRouteMetadata {
    pub path: String,
    pub subprotocols: Vec<String>,
}

pub struct OpenApiServiceSpec<'a> {
    pub title: &'a str,
    pub version: &'a str,
    pub description: &'a str,
    pub openapi_path: &'a str,
    pub docs_path: &'a str,
}

pub fn extract_routes_from_function(
    source: &str,
    fn_name: &str,
    websocket_routes: &[WebsocketRouteMetadata],
    excluded_paths: &[&str],
) -> Result<Vec<RouteEntry>, String> {
    let signature = format!("fn {fn_name}");
    let start = source
        .find(&format!("pub {signature}"))
        .or_else(|| source.find(&signature))
        .ok_or_else(|| format!("could not find function `{fn_name}`"))?;
    let relative_open_brace = source[start..]
        .find('{')
        .ok_or_else(|| format!("could not find body start for `{fn_name}`"))?;
    let open_brace = start + relative_open_brace;
    let close_brace = find_matching_delimiter(source, open_brace, '{', '}')
        .ok_or_else(|| format!("could not find body end for `{fn_name}`"))?;
    let body = &source[open_brace + 1..close_brace];

    let websocket_lookup = websocket_routes
        .iter()
        .map(|route| (route.path.as_str(), route))
        .collect::<BTreeMap<_, _>>();
    let mut routes: BTreeMap<String, BTreeSet<HttpMethod>> = BTreeMap::new();
    let mut search_from = 0usize;

    while let Some(relative_route) = body[search_from..].find(".route(") {
        let route_start = search_from + relative_route + ".route".len();
        let route_end = find_matching_delimiter(body, route_start, '(', ')')
            .ok_or_else(|| format!("unbalanced route declaration in `{fn_name}`"))?;
        let route_call = &body[route_start + 1..route_end];
        let (path, handler_expr) = split_route_call(route_call)?;
        if excluded_paths.contains(&path) {
            search_from = route_end + 1;
            continue;
        }

        let methods = extract_methods(handler_expr);
        if methods.is_empty() {
            return Err(format!("could not infer methods for route `{path}`"));
        }

        routes.entry(path.to_owned()).or_default().extend(methods);
        search_from = route_end + 1;
    }

    Ok(routes
        .into_iter()
        .map(|(path, methods)| {
            let websocket_metadata = websocket_lookup.get(path.as_str());
            RouteEntry {
                protocol: if websocket_metadata.is_some() {
                    RouteProtocol::Websocket
                } else {
                    RouteProtocol::Http
                },
                websocket_subprotocols: websocket_metadata
                    .map(|route| route.subprotocols.clone())
                    .unwrap_or_default(),
                path,
                methods: methods.into_iter().collect(),
            }
        })
        .collect())
}

pub fn build_openapi_document<TagFn, SecurityFn, SummaryFn>(
    spec: &OpenApiServiceSpec<'_>,
    routes: &[RouteEntry],
    classify_tag: TagFn,
    classify_security: SecurityFn,
    summarize_operation: SummaryFn,
) -> Value
where
    TagFn: Fn(&str, HttpMethod) -> String,
    SecurityFn: Fn(&str, HttpMethod) -> bool,
    SummaryFn: Fn(&str, HttpMethod) -> String,
{
    let mut paths = Map::new();
    let mut tags = BTreeSet::new();
    let mut has_security = false;

    for route in routes {
        let mut operations = Map::new();
        for method in &route.methods {
            let tag = classify_tag(&route.path, *method);
            let secured = classify_security(&route.path, *method);
            let summary = summarize_operation(&route.path, *method);
            let mut operation = Map::new();

            tags.insert(tag.clone());
            has_security |= secured;

            operation.insert(
                "operationId".to_owned(),
                Value::String(operation_id(&route.path, *method)),
            );
            operation.insert("summary".to_owned(), Value::String(summary));
            operation.insert("tags".to_owned(), json!([tag]));
            operation.insert(
                "responses".to_owned(),
                if route.protocol == RouteProtocol::Websocket {
                    json!({
                        "101": {
                            "description": "WebSocket upgrade successful"
                        }
                    })
                } else {
                    json!({
                        "200": {
                            "description": "Successful response"
                        }
                    })
                },
            );

            operation.insert(
                "security".to_owned(),
                if secured {
                    json!([{ "AuthToken": [], "AccessToken": [] }])
                } else {
                    json!([])
                },
            );

            if route.protocol == RouteProtocol::Websocket {
                operation.insert(
                    "x-sdkwork-im-protocol".to_owned(),
                    Value::String("websocket".to_owned()),
                );
                if !route.websocket_subprotocols.is_empty() {
                    operation.insert(
                        "x-sdkwork-im-websocket-subprotocols".to_owned(),
                        Value::Array(
                            route
                                .websocket_subprotocols
                                .iter()
                                .cloned()
                                .map(Value::String)
                                .collect(),
                        ),
                    );
                }
            }

            operations.insert(method_name(*method).to_owned(), Value::Object(operation));
        }

        paths.insert(route.path.clone(), Value::Object(operations));
    }

    let mut document = Map::new();
    document.insert("openapi".to_owned(), Value::String("3.1.0".to_owned()));
    document.insert(
        "info".to_owned(),
        json!({
            "title": spec.title,
            "version": spec.version,
            "description": spec.description
        }),
    );
    document.insert("servers".to_owned(), json!([{ "url": "/" }]));
    document.insert(
        "tags".to_owned(),
        Value::Array(
            tags.into_iter()
                .map(|tag| {
                    json!({
                        "name": tag,
                        "description": format!("{} operations", humanize_label(&tag))
                    })
                })
                .collect(),
        ),
    );
    document.insert("paths".to_owned(), Value::Object(paths));

    if has_security {
        document.insert(
            "components".to_owned(),
            json!({
                "securitySchemes": {
                    "AuthToken": {
                        "type": "http",
                        "scheme": "bearer",
                        "bearerFormat": "JWT"
                    },
                    "AccessToken": {
                        "type": "apiKey",
                        "in": "header",
                        "name": "Access-Token"
                    }
                }
            }),
        );
    }

    Value::Object(document)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protected_operations_use_sdkwork_dual_token_security() {
        let spec = OpenApiServiceSpec {
            title: "Test API",
            version: "0.1.0",
            description: "Test OpenAPI document",
            openapi_path: "/openapi.json",
            docs_path: "/docs",
        };
        let routes = vec![RouteEntry {
            path: "/im/v3/api/chat/conversations".to_owned(),
            methods: vec![HttpMethod::Post],
            protocol: RouteProtocol::Http,
            websocket_subprotocols: Vec::new(),
        }];

        let document = build_openapi_document(
            &spec,
            &routes,
            |_path, _method| "chat".to_owned(),
            |_path, _method| true,
            |_path, _method| "Create conversation".to_owned(),
        );

        let security = document
            .pointer("/paths/~1im~1v3~1api~1chat~1conversations/post/security")
            .and_then(Value::as_array)
            .expect("protected operation should define security");
        assert!(
            security
                .iter()
                .any(|entry| entry.get("AuthToken").is_some()
                    && entry.get("AccessToken").is_some()),
            "protected operations must require SDKWork AuthToken plus AccessToken"
        );

        let schemes = document
            .pointer("/components/securitySchemes")
            .and_then(Value::as_object)
            .expect("security schemes should be present");
        assert_eq!(
            schemes
                .get("AuthToken")
                .and_then(|scheme| scheme.get("type"))
                .and_then(Value::as_str),
            Some("http")
        );
        assert_eq!(
            schemes
                .get("AccessToken")
                .and_then(|scheme| scheme.get("name"))
                .and_then(Value::as_str),
            Some("Access-Token")
        );
        assert!(
            schemes.get("bearerAuth").is_none(),
            "legacy bearerAuth scheme must not be emitted"
        );
    }

    #[test]
    fn operation_id_uses_dotted_lower_camel_resource_action_contract() {
        assert_eq!(
            operation_id("/app/v3/api/auth/sessions", HttpMethod::Post),
            "sessions.create"
        );
        assert_eq!(
            operation_id("/app/v3/api/auth/sessions/current", HttpMethod::Get),
            "sessions.current.retrieve"
        );
        assert_eq!(
            operation_id("/app/v3/api/auth/sessions/current", HttpMethod::Delete),
            "sessions.current.delete"
        );
        assert_eq!(
            operation_id(
                "/app/v3/api/auth/verification_codes/verify",
                HttpMethod::Post
            ),
            "verificationCodes.verify"
        );
        assert_eq!(
            operation_id("/backend/v3/api/iam/users", HttpMethod::Get),
            "users.list"
        );
        assert_eq!(
            operation_id("/backend/v3/api/iam/users/{userId}", HttpMethod::Get),
            "users.retrieve"
        );
        assert_eq!(
            operation_id("/backend/v3/api/iam/users/{userId}", HttpMethod::Patch),
            "users.update"
        );
        assert_eq!(
            operation_id("/app/v3/api/iam/organization_memberships", HttpMethod::Get),
            "organizationMemberships.list"
        );
        assert_eq!(
            operation_id(
                "/backend/v3/api/iam/organization_memberships",
                HttpMethod::Post
            ),
            "organizationMemberships.create"
        );
        assert_eq!(
            operation_id(
                "/backend/v3/api/iam/organization_memberships/{membershipId}",
                HttpMethod::Patch
            ),
            "organizationMemberships.update"
        );
        assert_eq!(
            operation_id(
                "/backend/v3/api/iam/roles/{roleId}/permissions/{permissionId}",
                HttpMethod::Delete
            ),
            "roles.permissions.delete"
        );
    }
}

pub fn render_docs_html(spec: &OpenApiServiceSpec<'_>) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{title}</title>
  <style>
    :root {{
      color-scheme: light;
      --bg: #f5f1e8;
      --panel: #fffdf7;
      --ink: #1f2933;
      --muted: #52606d;
      --border: #d9cbb6;
      --accent: #8d5a2b;
      --accent-soft: #f0dfcc;
      --get: #16794d;
      --post: #0b65c2;
      --put: #8b5cf6;
      --patch: #c17c00;
      --delete: #b83232;
      --options: #4b5563;
      --head: #374151;
    }}
    * {{ box-sizing: border-box; }}
    body {{
      margin: 0;
      font-family: "Segoe UI", "PingFang SC", "Microsoft YaHei", sans-serif;
      background:
        radial-gradient(circle at top right, rgba(141, 90, 43, 0.16), transparent 28%),
        linear-gradient(180deg, #fcfaf6 0%, var(--bg) 100%);
      color: var(--ink);
    }}
    main {{
      max-width: 1120px;
      margin: 0 auto;
      padding: 32px 20px 64px;
    }}
    .hero {{
      background: linear-gradient(135deg, rgba(255, 255, 255, 0.95), rgba(252, 245, 232, 0.92));
      border: 1px solid var(--border);
      border-radius: 24px;
      padding: 28px;
      box-shadow: 0 18px 50px rgba(91, 63, 35, 0.08);
    }}
    .eyebrow {{
      display: inline-block;
      padding: 6px 10px;
      border-radius: 999px;
      background: var(--accent-soft);
      color: var(--accent);
      font-size: 12px;
      font-weight: 700;
      letter-spacing: 0.08em;
      text-transform: uppercase;
    }}
    h1 {{
      margin: 14px 0 10px;
      font-size: clamp(28px, 4vw, 44px);
      line-height: 1.05;
    }}
    p {{
      margin: 0;
      line-height: 1.6;
      color: var(--muted);
    }}
    .toolbar {{
      display: flex;
      flex-wrap: wrap;
      gap: 12px;
      align-items: center;
      margin-top: 20px;
    }}
    .toolbar input {{
      flex: 1 1 280px;
      padding: 12px 14px;
      border-radius: 14px;
      border: 1px solid var(--border);
      background: rgba(255, 255, 255, 0.9);
      color: var(--ink);
      font-size: 14px;
    }}
    .toolbar a {{
      display: inline-flex;
      align-items: center;
      justify-content: center;
      min-height: 44px;
      padding: 0 16px;
      border-radius: 14px;
      background: var(--ink);
      color: white;
      text-decoration: none;
      font-weight: 600;
    }}
    .stats {{
      display: flex;
      flex-wrap: wrap;
      gap: 12px;
      margin-top: 20px;
    }}
    .stat {{
      min-width: 120px;
      padding: 12px 14px;
      border-radius: 16px;
      background: rgba(255, 255, 255, 0.82);
      border: 1px solid var(--border);
    }}
    .stat strong {{
      display: block;
      font-size: 22px;
      margin-bottom: 4px;
    }}
    .group {{
      margin-top: 28px;
      padding: 18px;
      border-radius: 20px;
      border: 1px solid var(--border);
      background: rgba(255, 253, 247, 0.92);
      box-shadow: 0 12px 30px rgba(91, 63, 35, 0.05);
    }}
    .group h2 {{
      margin: 0 0 14px;
      font-size: 20px;
      text-transform: capitalize;
    }}
    .routes {{
      display: grid;
      gap: 12px;
    }}
    .route {{
      padding: 14px 16px;
      border-radius: 16px;
      border: 1px solid rgba(82, 96, 109, 0.18);
      background: rgba(255, 255, 255, 0.9);
    }}
    .route-head {{
      display: flex;
      flex-wrap: wrap;
      gap: 10px;
      align-items: center;
    }}
    .method {{
      min-width: 76px;
      padding: 6px 10px;
      border-radius: 999px;
      color: white;
      text-align: center;
      font-size: 12px;
      font-weight: 700;
      letter-spacing: 0.08em;
    }}
    .method.get {{ background: var(--get); }}
    .method.post {{ background: var(--post); }}
    .method.put {{ background: var(--put); }}
    .method.patch {{ background: var(--patch); }}
    .method.delete {{ background: var(--delete); }}
    .method.options {{ background: var(--options); }}
    .method.head {{ background: var(--head); }}
    code {{
      font-family: "Cascadia Code", "Fira Code", Consolas, monospace;
      font-size: 14px;
    }}
    .summary {{
      margin-top: 8px;
      color: var(--muted);
      font-size: 14px;
    }}
    .meta {{
      display: flex;
      gap: 8px;
      flex-wrap: wrap;
      margin-top: 10px;
    }}
    .pill {{
      border-radius: 999px;
      padding: 5px 10px;
      font-size: 12px;
      color: var(--accent);
      background: var(--accent-soft);
    }}
    .empty {{
      margin-top: 24px;
      padding: 28px;
      text-align: center;
      border-radius: 18px;
      border: 1px dashed var(--border);
      color: var(--muted);
    }}
  </style>
</head>
<body>
  <main>
    <section class="hero">
      <span class="eyebrow">OpenAPI 3.1</span>
      <h1>{title}</h1>
      <p>{description}</p>
      <div class="toolbar">
        <input id="search" type="search" placeholder="Search by method, path, tag, or summary">
        <a href="{openapi_path}" target="_blank" rel="noreferrer">Open Raw JSON</a>
      </div>
      <div class="stats" id="stats"></div>
    </section>
    <div id="content"></div>
  </main>
  <script>
    const title = {title_json};
    const openapiPath = {openapi_path_json};
    const statsEl = document.getElementById('stats');
    const contentEl = document.getElementById('content');
    const searchEl = document.getElementById('search');

    function escapeHtml(value) {{
      return value
        .replaceAll('&', '&amp;')
        .replaceAll('<', '&lt;')
        .replaceAll('>', '&gt;')
        .replaceAll('"', '&quot;')
        .replaceAll("'", '&#39;');
    }}

    function renderStats(routes) {{
      const uniquePaths = new Set(routes.map((route) => route.path));
      const protectedCount = routes.filter((route) => route.protected).length;
      statsEl.innerHTML = `
        <div class="stat"><strong>${{routes.length}}</strong><span>operations</span></div>
        <div class="stat"><strong>${{uniquePaths.size}}</strong><span>paths</span></div>
        <div class="stat"><strong>${{protectedCount}}</strong><span>protected</span></div>
      `;
    }}

    function renderRoutes(routes) {{
      const query = searchEl.value.trim().toLowerCase();
      const filtered = routes.filter((route) => {{
        if (!query) return true;
        return [route.method, route.path, route.tag, route.summary, route.protected ? 'auth' : 'public']
          .join(' ')
          .toLowerCase()
          .includes(query);
      }});

      if (!filtered.length) {{
        contentEl.innerHTML = '<div class="empty">No routes match the current filter.</div>';
        return;
      }}

      const groups = new Map();
      for (const route of filtered) {{
        if (!groups.has(route.tag)) {{
          groups.set(route.tag, []);
        }}
        groups.get(route.tag).push(route);
      }}

      const html = Array.from(groups.entries()).map(([tag, items]) => {{
        const routesHtml = items.map((route) => `
          <article class="route">
            <div class="route-head">
              <span class="method ${{route.method.toLowerCase()}}">${{route.method}}</span>
              <code>${{escapeHtml(route.path)}}</code>
            </div>
            <div class="summary">${{escapeHtml(route.summary)}}</div>
            <div class="meta">
              <span class="pill">${{route.protected ? 'Bearer auth' : 'Public'}}</span>
              <span class="pill">${{escapeHtml(route.operationId)}}</span>
            </div>
          </article>
        `).join('');
        return `
          <section class="group">
            <h2>${{escapeHtml(tag)}}</h2>
            <div class="routes">${{routesHtml}}</div>
          </section>
        `;
      }}).join('');

      contentEl.innerHTML = html;
    }}

    fetch(openapiPath)
      .then((response) => {{
        if (!response.ok) {{
          throw new Error(`Failed to load OpenAPI document (${{response.status}})`);
        }}
        return response.json();
      }})
      .then((document) => {{
        document.title = title;
        const routes = [];
        for (const [path, operations] of Object.entries(document.paths || {{}})) {{
          for (const [method, operation] of Object.entries(operations || {{}})) {{
            routes.push({{
              path,
              method: method.toUpperCase(),
              summary: operation.summary || `${{method.toUpperCase()}} ${{path}}`,
              tag: (operation.tags && operation.tags[0]) || 'misc',
              operationId: operation.operationId || `${{method}}_${{path}}`,
              protected: Array.isArray(operation.security) && operation.security.length > 0
            }});
          }}
        }}
        routes.sort((left, right) =>
          left.tag.localeCompare(right.tag) ||
          left.path.localeCompare(right.path) ||
          left.method.localeCompare(right.method)
        );
        renderStats(routes);
        renderRoutes(routes);
        searchEl.addEventListener('input', () => renderRoutes(routes));
      }})
      .catch((error) => {{
        contentEl.innerHTML = `<div class="empty">${{escapeHtml(error.message)}}</div>`;
      }});
  </script>
</body>
</html>
"#,
        title = spec.title,
        description = spec.description,
        openapi_path = spec.openapi_path,
        title_json = serde_json::to_string(spec.title).expect("json title"),
        openapi_path_json = serde_json::to_string(spec.openapi_path).expect("json openapi path"),
    )
}

fn split_route_call(route_call: &str) -> Result<(&str, &str), String> {
    let bytes = route_call.as_bytes();
    let mut index = 0usize;

    while bytes.get(index).is_some_and(u8::is_ascii_whitespace) {
        index += 1;
    }

    if bytes.get(index) != Some(&b'"') {
        return Err("route path must start with a string literal".to_owned());
    }

    index += 1;
    let path_start = index;
    while let Some(byte) = bytes.get(index) {
        match *byte {
            b'\\' => index += 2,
            b'"' => break,
            _ => index += 1,
        }
    }
    let path_end = index;

    if bytes.get(index) != Some(&b'"') {
        return Err("route path string literal is not terminated".to_owned());
    }

    index += 1;
    while bytes.get(index).is_some_and(u8::is_ascii_whitespace) {
        index += 1;
    }

    if bytes.get(index) != Some(&b',') {
        return Err("route declaration missing comma separator".to_owned());
    }

    index += 1;
    Ok((
        &route_call[path_start..path_end],
        route_call[index..].trim(),
    ))
}

fn extract_methods(handler_expr: &str) -> BTreeSet<HttpMethod> {
    let mut methods = BTreeSet::new();

    for (needle, method) in [
        ("delete(", HttpMethod::Delete),
        ("get(", HttpMethod::Get),
        ("head(", HttpMethod::Head),
        ("options(", HttpMethod::Options),
        ("patch(", HttpMethod::Patch),
        ("post(", HttpMethod::Post),
        ("put(", HttpMethod::Put),
    ] {
        if handler_expr.contains(needle) {
            methods.insert(method);
        }
    }

    methods
}

fn find_matching_delimiter(
    source: &str,
    open_index: usize,
    open: char,
    close: char,
) -> Option<usize> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;

    for (offset, ch) in source[open_index..].char_indices() {
        let index = open_index + offset;

        if in_string {
            if escaped {
                escaped = false;
                continue;
            }

            match ch {
                '\\' => escaped = true,
                '"' => in_string = false,
                _ => {}
            }
            continue;
        }

        match ch {
            '"' => in_string = true,
            ch if ch == open => depth += 1,
            ch if ch == close => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }

    None
}

fn method_name(method: HttpMethod) -> &'static str {
    match method {
        HttpMethod::Delete => "delete",
        HttpMethod::Get => "get",
        HttpMethod::Head => "head",
        HttpMethod::Options => "options",
        HttpMethod::Patch => "patch",
        HttpMethod::Post => "post",
        HttpMethod::Put => "put",
    }
}

fn operation_id(path: &str, method: HttpMethod) -> String {
    let mut resources = operation_resource_segments(path);
    let action = operation_action(
        &mut resources,
        method,
        path_contains_parameter(path),
        path_ends_with_parameter(path),
    );
    if resources.is_empty() {
        resources.push("resource".to_owned());
    }
    format!("{}.{}", resources.join("."), action)
}

fn operation_resource_segments(path: &str) -> Vec<String> {
    let raw_segments = path
        .trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let scoped_segments = strip_api_prefix(raw_segments);

    let mut resources = scoped_segments
        .into_iter()
        .filter(|segment| !is_path_parameter(segment))
        .map(to_lower_camel_segment)
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();

    // API paths include an explicit surface domain segment that maps to the top-level
    // SDK namespace; remove it from operationId resources to avoid duplication.
    if resources.len() > 1 && has_standard_api_prefix(path) {
        resources.remove(0);
    }

    resources
}

fn strip_api_prefix(segments: Vec<&str>) -> Vec<&str> {
    if segments.len() >= 3
        && matches!(segments[0], "im" | "app" | "backend")
        && segments[1] == "v3"
        && segments[2] == "api"
    {
        segments[3..].to_vec()
    } else {
        segments
    }
}

fn has_standard_api_prefix(path: &str) -> bool {
    let normalized = path.trim_start_matches('/');
    normalized.starts_with("im/v3/api/")
        || normalized.starts_with("app/v3/api/")
        || normalized.starts_with("backend/v3/api/")
}

fn is_path_parameter(segment: &str) -> bool {
    segment.starts_with('{') && segment.ends_with('}')
}

fn path_ends_with_parameter(path: &str) -> bool {
    path.trim_matches('/')
        .split('/')
        .next_back()
        .is_some_and(is_path_parameter)
}

fn path_contains_parameter(path: &str) -> bool {
    path.trim_matches('/').split('/').any(is_path_parameter)
}

fn looks_like_collection_resource(segment: &str) -> bool {
    segment.ends_with('s') || segment.ends_with("List")
}

fn to_lower_camel_segment(value: &str) -> String {
    let mut output = String::new();
    let mut uppercase_next = false;

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            if output.is_empty() {
                output.push(ch.to_ascii_lowercase());
                uppercase_next = false;
            } else if uppercase_next {
                output.push(ch.to_ascii_uppercase());
                uppercase_next = false;
            } else {
                output.push(ch.to_ascii_lowercase());
            }
        } else {
            uppercase_next = !output.is_empty();
        }
    }

    output
}

fn operation_action(
    resources: &mut Vec<String>,
    method: HttpMethod,
    contains_parameter: bool,
    ends_with_parameter: bool,
) -> String {
    if resources.is_empty() {
        return match method {
            HttpMethod::Get => "retrieve".to_owned(),
            HttpMethod::Post => "create".to_owned(),
            HttpMethod::Put | HttpMethod::Patch => "update".to_owned(),
            HttpMethod::Delete => "delete".to_owned(),
            HttpMethod::Options => "options".to_owned(),
            HttpMethod::Head => "head".to_owned(),
        };
    }

    let command_suffixes = [
        "accept",
        "ack",
        "activate",
        "apply",
        "approve",
        "archive",
        "batchCreate",
        "batchDelete",
        "batchUpdate",
        "bind",
        "cancel",
        "claim",
        "decline",
        "deactivate",
        "diff",
        "disconnect",
        "drain",
        "establish",
        "migrate",
        "preview",
        "publish",
        "reclaim",
        "refresh",
        "reject",
        "release",
        "remove",
        "repair",
        "republish",
        "requeue",
        "restore",
        "resume",
        "revoke",
        "rollback",
        "submit",
        "sync",
        "takeover",
        "unpublish",
        "verify",
    ];
    let singleton_resources = ["current", "default", "healthz", "me", "readyz"];
    let singleton_verbs = ["diff"];

    match method {
        HttpMethod::Get => {
            if ends_with_parameter
                || (contains_parameter
                    && resources
                        .last()
                        .is_some_and(|segment| !looks_like_collection_resource(segment)))
            {
                "retrieve".to_owned()
            } else if resources
                .last()
                .is_some_and(|segment| singleton_verbs.contains(&segment.as_str()))
            {
                resources.pop().unwrap_or_else(|| "retrieve".to_owned())
            } else if resources
                .last()
                .is_some_and(|segment| singleton_resources.contains(&segment.as_str()))
            {
                "retrieve".to_owned()
            } else {
                "list".to_owned()
            }
        }
        HttpMethod::Post => {
            if resources
                .last()
                .is_some_and(|segment| command_suffixes.contains(&segment.as_str()))
            {
                resources.pop().unwrap_or_else(|| "create".to_owned())
            } else {
                "create".to_owned()
            }
        }
        HttpMethod::Put | HttpMethod::Patch => "update".to_owned(),
        HttpMethod::Delete => "delete".to_owned(),
        HttpMethod::Options => "options".to_owned(),
        HttpMethod::Head => "head".to_owned(),
    }
}

fn humanize_label(value: &str) -> String {
    value
        .replace(['_', '-'], " ")
        .split_whitespace()
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
