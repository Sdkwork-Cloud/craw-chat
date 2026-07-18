import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function readText(...segments) {
  return readFileSync(path.join(repoRoot, ...segments), 'utf8');
}

const productRuntimeText = readText('crates', 'sdkwork-api-product-runtime', 'src', 'lib.rs');
const gatewayAssemblyText = readText('crates', 'sdkwork-im-gateway-assembly', 'src', 'bootstrap.rs');
const cloudGatewayRuntimeText = readText('services', 'sdkwork-im-cloud-gateway', 'src', 'runtime.rs');
const portalSnapshotsText = readText('crates', 'im-portal-snapshots', 'src', 'snapshots.rs');
const portalHandlersText = readText('services', 'portal-service', 'src', 'handlers.rs');
const securityServiceText = readText(
  'apps',
  'sdkwork-im-pc',
  'packages',
  'sdkwork-im-console-security',
  'src',
  'services',
  'SecurityService.ts',
);
const dashboardServiceText = readText(
  'apps',
  'sdkwork-im-pc',
  'packages',
  'sdkwork-im-console-dashboard',
  'src',
  'services',
  'DashboardService.ts',
);
const infraStatusServiceText = readText(
  'apps',
  'sdkwork-im-pc',
  'packages',
  'sdkwork-im-admin-infrastructure',
  'src',
  'services',
  'InfraStatusService.ts',
);
const devGatewayConfigText = readText(
  'etc',
  'sdkwork-api-cloud-gateway.sdkwork-im.development.toml',
);

assert.doesNotMatch(
  productRuntimeText,
  /fn portal_snapshot_json/u,
  'product-runtime must not retain the static portal_snapshot_json stub',
);
assert.match(
  productRuntimeText,
  /build_portal_snapshot_for_section/u,
  'product-runtime fallback must reuse im-portal-snapshots builders',
);
assert.match(
  gatewayAssemblyText,
  /sdkwork_routes_im_portal_app_api::gateway_mount/u,
  'gateway assembly must mount portal-service routes',
);
assert.match(
  cloudGatewayRuntimeText,
  /fn should_delegate_to_product_runtime\(_path: &str\) -> bool \{\s*false\s*\}/u,
  'cloud gateway must not delegate portal traffic to product-runtime stub',
);
assert.match(
  portalSnapshotsText,
  /"governance"\s*=>[\s\S]*unavailable_availability\("audit",\s*"audit sample was not supplied"\)/u,
  'portal governance must expose typed unavailable state when audit data is unavailable',
);
assert.doesNotMatch(
  portalSnapshotsText,
  /health_score|return -1;/u,
  'portal governance must not publish a synthetic health score sentinel',
);
assert.match(
  portalHandlersText,
  /finish_api_json/u,
  'portal handlers must serialize SdkWorkApiResponse through finish_api_json',
);
assert.match(
  portalHandlersText,
  /SdkWorkResourceData/u,
  'portal handlers must wrap payloads in SdkWorkResourceData.item',
);
assert.match(
  securityServiceText,
  /healthScore:\s*number \| null/u,
  'console security service must allow null health score when audit data is missing',
);
assert.match(
  dashboardServiceText,
  /state:\s*snapshot\.availability\.state[\s\S]*metrics:\s*metrics\s*\?[\s\S]*:\s*\[\]/u,
  'console dashboard must gate activity trends on portal metric availability',
);
assert.match(
  readText(
    'apps',
    'sdkwork-im-pc',
    'packages',
    'sdkwork-im-console-dashboard',
    'src',
    'ConsoleDashboard.tsx',
  ),
  /暂无可验证运行指标/u,
  'console dashboard UI must render empty activity state instead of fake zero bars',
);
assert.match(
  infraStatusServiceText,
  /realtimeWindowHealth/u,
  'admin infra status must expose realtime window health instead of redisHitRate',
);
assert.match(
  devGatewayConfigText,
  /\/app\/v3\/api\/portal/u,
  'cloud gateway development config must register portal dependency surface',
);
assert.match(
  productRuntimeText,
  /portal_envelope_json/u,
  'product-runtime portal fallback must emit SdkWorkApiResponse { code, data.item, traceId }',
);
assert.match(
  readText('services', 'portal-service', 'src', 'openapi.rs'),
  /Sdkwork IM Portal Service API/u,
  'portal-service must export live OpenAPI for operational verification',
);

console.log('sdkwork-im portal alignment standard passed');
