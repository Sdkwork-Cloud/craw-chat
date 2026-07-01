#!/usr/bin/env node
/**
 * One-shot bootstrap for IM HTTP route crates per WEB_BACKEND_SPEC.md.
 * Run: node scripts/dev/bootstrap-im-route-crates.mjs
 */
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { IM_ROUTE_CRATES } from './sdkwork-im-web-backend-route-crates.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

const SERVICE_DEPS = {
  'social-service': { key: 'social-service', path: '../../services/social-service' },
  'space-service': { key: 'space-service', path: '../../services/space-service' },
  'sdkwork-comms-conversation-service': {
    key: 'sdkwork-comms-conversation-service',
    path: '../../services/sdkwork-comms-conversation-service',
  },
  'session-gateway': { key: 'session-gateway', path: '../../services/session-gateway' },
  'media-service': { key: 'media-service', path: '../../services/media-service' },
  'automation-service': { key: 'automation-service', path: '../../services/automation-service' },
  'notification-service': { key: 'notification-service', path: '../../services/notification-service' },
  'streaming-service': { key: 'streaming-service', path: '../../services/streaming-service' },
  'audit-service': { key: 'audit-service', path: '../../services/audit-service' },
  'ops-service': { key: 'ops-service', path: '../../services/ops-service' },
  'governance-service': { key: 'governance-service', path: '../../services/governance-service' },
  'projection-service': { key: 'projection-service', path: '../../services/projection-service' },
};

function capabilityFromPackage(packageName) {
  const m = packageName.match(/^sdkwork-routes-im-(.+)$/);
  return m ? m[1].replace(/-/g, '_') : packageName;
}

function readme(entry) {
  return `# ${entry.packageName}

## Purpose

HTTP route crate for SDKWork IM \`${entry.apiSurface}\` surface at \`${entry.pathPrefix}\`.

## Owner

SDKWork IM maintainers.

## Allowed Content

- Path constants (\`paths.rs\`)
- Route manifest metadata (\`manifest.rs\`)
- Axum route mounting (\`routes.rs\`)
- IM web-framework wrapping (\`web_bootstrap.rs\`)

## Forbidden Content

- Business logic, persistence, or provider clients
- Raw HTTP credential parsing outside \`sdkwork-web-framework\`
- Generated SDK imports for the same API authority

## Related Specs

- \`../../sdkwork-specs/WEB_BACKEND_SPEC.md\`
- \`../../sdkwork-specs/WEB_FRAMEWORK_SPEC.md\`
- \`../../sdkwork-specs/API_SPEC.md\`

## Verification

\`\`\`bash
cargo check -p ${entry.packageName}
node scripts/dev/sdkwork-im-web-backend-standard.test.mjs
\`\`\`
`;
}

function webBootstrap(entry) {
  return `use axum::Router;
use sdkwork_im_web_bootstrap::wrap_im_service_router_with_manifest;

use crate::manifest::route_manifest;

/// Wrap the ${entry.packageName} router with the canonical SDKWork interceptor
/// pipeline (\`WebFrameworkLayer\`) and the actual [\`HttpRouteManifest\`]
/// declared by \`manifest::route_manifest()\`.
///
/// Passing the real route table (instead of an empty manifest) enables
/// \`IamAuthorizationPolicy\` enforcement, route-level HTTP metrics
/// dimensions, and OpenAPI metadata consistency per \`API_SPEC.md\` §4.5,
/// §14, and §15.
pub fn wrap_router(router: Router) -> Router {
    wrap_im_service_router_with_manifest(router, route_manifest())
}
`;
}

function manifest(entry) {
  const cap = capabilityFromPackage(entry.packageName).replace(/_open_api$|_backend_api$|_app_api$/, '');
  const tag = cap.replace(/_/g, '');
  return `use sdkwork_web_contract::{HttpMethod, HttpRoute};
use sdkwork_web_core::HttpRouteManifest;

use crate::paths;

/// API surface: ${entry.apiSurface}
pub const API_SURFACE: &str = "${entry.apiSurface}";

pub const ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        paths::PREFIX,
        "${tag}",
        "${tag}.prefix",
    ),
];

pub fn route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(ROUTES)
}
`;
}

function paths(entry) {
  return `pub const PREFIX: &str = "${entry.pathPrefix}";
`;
}

function cargoToml(entry) {
  const dep = SERVICE_DEPS[entry.serviceCrate];
  return `[package]
name = "${entry.packageName}"
version.workspace = true
edition.workspace = true
license.workspace = true

[lib]
name = "${entry.libName}"
path = "src/lib.rs"

[dependencies]
axum.workspace = true
sdkwork_web_contract.workspace = true
sdkwork_web_core.workspace = true
sdkwork_im_web_bootstrap.workspace = true
${dep.key} = { path = "${dep.path}" }
`;
}

function libRs(entry) {
  const lines = [
    'mod manifest;',
    'mod paths;',
    'mod routes;',
    'mod web_bootstrap;',
    '',
    'pub use manifest::{route_manifest, API_SURFACE};',
    'pub use paths::PREFIX;',
    '',
  ];

  if (entry.packageName === 'sdkwork-routes-im-social-open-api') {
    lines.push(
      'use std::sync::Arc;',
      '',
      'use axum::Router;',
      'use social_service::postgres::PostgresAppState;',
      'use social_service::runtime::SocialRuntime;',
      '',
      'pub fn build_supplemental_public_app(state: PostgresAppState) -> Router {',
      '    web_bootstrap::wrap_router(routes::build_supplemental_router(state))',
      '}',
      '',
      'pub fn build_public_app_with_postgres_extension(',
      '    social_runtime: Arc<SocialRuntime>,',
      '    postgres_state: Option<PostgresAppState>,',
      ') -> Router {',
      '    let router = routes::build_control_merge_router(social_runtime);',
      '    match postgres_state {',
      '        Some(state) => router.merge(routes::build_supplemental_router(state)),',
      '        None => router,',
      '    }',
      '    web_bootstrap::wrap_router(router)',
      '}',
    );
  } else if (entry.packageName === 'sdkwork-routes-im-social-backend-api') {
    lines.push(
      'use std::sync::Arc;',
      '',
      'use axum::Router;',
      'use social_service::runtime::SocialRuntime;',
      '',
      'pub fn build_control_public_app(social_runtime: Arc<SocialRuntime>) -> Router {',
      '    web_bootstrap::wrap_router(routes::build_control_router(social_runtime))',
      '}',
    );
  } else if (entry.packageName === 'sdkwork-routes-im-chat-open-api') {
    lines.push(
      'use axum::Router;',
      'use conversation_runtime::http::{',
      '    build_default_app_with_principal_directory, PrincipalDirectory,',
      '};',
      'use std::sync::Arc;',
      '',
      'pub fn build_public_app() -> Router {',
      '    web_bootstrap::wrap_router(routes::build_api_router_from_default_app())',
      '}',
      '',
      'pub fn build_public_app_with_allow_all_principals() -> Router {',
      '    build_public_app()',
      '}',
      '',
      'pub fn build_public_app_with_principal_directory(',
      '    principal_directory: Arc<dyn PrincipalDirectory>,',
      ') -> Router {',
      '    web_bootstrap::wrap_router(',
      '        routes::build_api_router_from_app(',
      '            build_default_app_with_principal_directory(principal_directory),',
      '        ),',
      '    )',
      '}',
    );
  } else if (
    entry.serviceCrate === 'projection-service' ||
    entry.serviceCrate === 'space-service'
  ) {
    const stateType =
      entry.serviceCrate === 'space-service' ? 'space_service::http::AppState' : '';
    const arg = entry.serviceCrate === 'space-service' ? 'state: space_service::http::AppState' : '';
    const call =
      entry.serviceCrate === 'space-service'
        ? 'routes::build_api_router(state)'
        : 'routes::build_api_router()';
    lines.push(
      'use axum::Router;',
      '',
      `pub fn ${entry.buildFn}(${arg}) -> Router {`,
      `    web_bootstrap::wrap_router(${call})`,
      '}',
    );
  } else {
    lines.push(
      'use axum::Router;',
      '',
      `pub fn ${entry.buildFn}() -> Router {`,
      '    web_bootstrap::wrap_router(routes::build_api_router())',
      '}',
    );
  }

  return lines.join('\n') + '\n';
}

function routesPlaceholder(entry) {
  return `// Route mounting is expanded in service migration; .route( calls import handlers from ${entry.serviceCrate}.
use axum::Router;

pub fn build_api_router() -> Router {
    Router::new()
        .route(crate::paths::PREFIX, axum::routing::get(|| async { "ok" }))
}
`;
}

for (const entry of IM_ROUTE_CRATES) {
  const crateRoot = path.join(repoRoot, entry.crateDir);
  const srcDir = path.join(crateRoot, 'src');
  fs.mkdirSync(srcDir, { recursive: true });
  fs.writeFileSync(path.join(crateRoot, 'Cargo.toml'), cargoToml(entry));
  fs.writeFileSync(path.join(crateRoot, 'README.md'), readme(entry));
  fs.writeFileSync(path.join(srcDir, 'paths.rs'), paths(entry));
  fs.writeFileSync(path.join(srcDir, 'manifest.rs'), manifest(entry));
  fs.writeFileSync(path.join(srcDir, 'web_bootstrap.rs'), webBootstrap(entry));
  fs.writeFileSync(path.join(srcDir, 'lib.rs'), libRs(entry));
  if (!fs.existsSync(path.join(srcDir, 'routes.rs'))) {
    fs.writeFileSync(path.join(srcDir, 'routes.rs'), routesPlaceholder(entry));
  }
}

process.stdout.write(`Bootstrapped ${IM_ROUTE_CRATES.length} route crates\n`);
