import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');
const iamRepoRoot = path.resolve(repoRoot, '..', 'sdkwork-iam');

function read(relativePath, root = repoRoot) {
  return fs.readFileSync(path.join(root, relativePath), 'utf8');
}

const bootstrapSource = read('crates/sdkwork-im-iam-application-bootstrap/src/lib.rs');
const bootstrapCargo = read('crates/sdkwork-im-iam-application-bootstrap/Cargo.toml');
const sharedBootstrapSource = read(
  'crates/sdkwork-iam-embedded-application-bootstrap/src/runtime.rs',
  iamRepoRoot,
);
const sharedManifestSource = read(
  'crates/sdkwork-iam-embedded-application-bootstrap/src/manifest.rs',
  iamRepoRoot,
);
const standaloneGatewayMain = read('services/sdkwork-im-standalone-gateway/src/main.rs');
const standaloneGatewayCargo = read('services/sdkwork-im-standalone-gateway/Cargo.toml');
const topologySource = read('scripts/lib/im-topology.mjs');
const signingSecretsSource = read(
  'crates/sdkwork-iam-web-adapter/src/signing_secrets.rs',
  iamRepoRoot,
);
const bootstrapLibSource = read('crates/sdkwork-iam-bootstrap/src/lib.rs', iamRepoRoot);
const tenantSigningKeySource = read(
  'crates/sdkwork-iam-bootstrap/src/tenant_signing_key.rs',
  iamRepoRoot,
);
const imPcDevSource = read('scripts/lib/im-pc-dev.mjs');
const workspaceCargo = read('Cargo.toml');
const iamAdapterSource = read(
  'crates/sdkwork-iam-web-adapter/src/application_registry.rs',
  iamRepoRoot,
);
const databaseHostSource = read('crates/sdkwork-iam-database-host/src/lib.rs', iamRepoRoot);

assert.match(
  bootstrapSource,
  /sdkwork_iam_embedded_application_bootstrap::ensure_tenant_applications_on_pool/u,
  'IM adapter must delegate tenant application provisioning to the shared embedded bootstrap crate.',
);

assert.match(
  bootstrapCargo,
  /sdkwork_iam_embedded_application_bootstrap/u,
  'IM IAM application bootstrap crate must depend on sdkwork-iam-embedded-application-bootstrap.',
);

assert.doesNotMatch(
  bootstrapSource,
  /ensure_tenant_application_runtime/u,
  'IM adapter must not duplicate ensure_tenant_application_runtime; use the shared crate.',
);

assert.match(
  bootstrapSource,
  /IM_PC_RUNTIME_APP_ID:\s*&str\s*=\s*"sdkwork-im-pc"/u,
  'IM IAM application bootstrap must register the PC runtime appId used by sdkwork-im-pc auth runtime.',
);

assert.match(
  bootstrapSource,
  /IM_H5_RUNTIME_APP_ID:\s*&str\s*=\s*"sdkwork-im-h5"/u,
  'IM IAM application bootstrap must register the H5 runtime appId used by sdkwork-im-h5 auth runtime.',
);

assert.match(
  bootstrapSource,
  /IM_FLUTTER_MOBILE_RUNTIME_APP_ID:\s*&str\s*=\s*"sdkwork-im-flutter-mobile"/u,
  'IM IAM application bootstrap must register the Flutter mobile runtime appId used by sdkwork-im-flutter-mobile auth runtime.',
);

assert.match(
  bootstrapSource,
  /ensure_tenant_application_from_app_root_with_env_and_fallback\(/u,
  'IM IAM application bootstrap must provision tenant applications with a repository-root fallback instead of silently skipping when SDKWORK_*_APP_ROOT env vars are absent.',
);

assert.doesNotMatch(
  bootstrapSource,
  /ensure_tenant_application_from_app_root_with_env\(/u,
  'IM IAM application bootstrap must not use ensure_tenant_application_from_app_root_with_env, which silently skips provisioning when app root env vars are unset.',
);

assert.match(
  bootstrapSource,
  /resolve_im_repo_root/u,
  'IM IAM application bootstrap must resolve a fallback IM repository app root when env vars are absent.',
);

assert.match(
  topologySource,
  /SDKWORK_IAM_APP_ROOT:\s*IAM_REPO_ROOT/u,
  'Dev topology must export SDKWORK_IAM_APP_ROOT at the sdkwork-iam repository root for IMF catalog materialization.',
);

assert.match(
  imPcDevSource,
  /IAM_APPLICATION_BOOTSTRAP_ENV/u,
  'Managed sdkwork-api-cloud-gateway dev process must inherit IAM application bootstrap env.',
);

assert.match(
  bootstrapLibSource,
  /ensure_postgres_tenant_signing_key/u,
  'IAM bootstrap must provision tenant signing keys in PostgreSQL when default subjects are seeded.',
);

assert.match(
  signingSecretsSource,
  /load_postgres_active_tenant_signing_key/u,
  'IAM signing secrets must re-export database-backed tenant signing key loading.',
);

assert.match(
  tenantSigningKeySource,
  /tenant_primary_signing_kid/u,
  'Tenant signing keys must use a stable per-tenant kid for concurrent-safe provisioning.',
);

assert.match(
  tenantSigningKeySource,
  /ON CONFLICT \(tenant_id, kid\) DO NOTHING/u,
  'Tenant signing key provisioning must be insert-only and concurrency-safe.',
);

assert.doesNotMatch(
  topologySource,
  /resolveIamSigningMasterSecretDevEnv/u,
  'IM dev topology must not inject SDKWORK_IAM_TENANT_SIGNING_MASTER_SECRET env overrides.',
);

assert.match(
  bootstrapSource,
  /sdkwork_im_pc_dev/u,
  'IM PC runtime binding test must assert shared instance_key rules (sdkwork_im_pc_dev).',
);

assert.match(
  bootstrapSource,
  /sdkwork_im_h5_prod/u,
  'IM H5 runtime binding test must assert shared instance_key rules (sdkwork_im_h5_prod).',
);

assert.match(
  bootstrapSource,
  /sdkwork_im_flutter_mobile_dev/u,
  'IM Flutter mobile runtime binding test must assert shared instance_key rules (sdkwork_im_flutter_mobile_dev).',
);

assert.match(
  sharedBootstrapSource,
  /upsert_postgres_default_subject/u,
  'Shared embedded bootstrap must seed the default IAM tenant subject before provisioning the app.',
);

assert.match(
  sharedBootstrapSource,
  /postgres_url_with_search_path/u,
  'Shared embedded bootstrap must align postgres search_path with the unified IAM schema before provisioning.',
);

assert.match(
  sharedManifestSource,
  /tenant_application_instance_key/u,
  'Shared embedded bootstrap must derive instance_key through IAM adapter rules.',
);

assert.match(
  iamAdapterSource,
  /reconcile_postgres_tenant_application_org_template_rows/u,
  'IAM adapter must reconcile duplicate tenant-application org-template rows before enforcing uniqueness.',
);

assert.match(
  iamAdapterSource,
  /upsert_tenant_application_row/u,
  'IAM adapter must upsert product tenant applications instead of blind insert.',
);

assert.match(
  iamAdapterSource,
  /ON CONFLICT \(id\) DO UPDATE/u,
  'IAM adapter must upsert tenant application rows by stable id.',
);

assert.match(
  iamAdapterSource,
  /resolve_tenant_application_primary_domain/u,
  'IAM adapter must resolve unique primaryDomain per tenant application before upsert.',
);

assert.match(
  databaseHostSource,
  /ensure_tenant_application_from_app_root_if_configured/u,
  'IAM database host must invoke embedded tenant application bootstrap after IAM migrations when app root is configured.',
);

assert.match(
  topologySource,
  /SDKWORK_APP_ROOT:\s*REPO_ROOT/u,
  'Dev topology must inject SDKWORK_APP_ROOT for embedded IAM bootstrap.',
);

assert.match(
  standaloneGatewayMain,
  /sdkwork_iam_database_host::bootstrap_iam_database_from_env\(\)/u,
  'Standalone gateway must bootstrap IAM schema before tenant application bootstrap.',
);

assert.match(
  standaloneGatewayMain,
  /ensure_im_tenant_application_runtime_from_env/u,
  'Standalone gateway must provision the IM PC IAM tenant application on startup.',
);

assert.match(
  standaloneGatewayCargo,
  /sdkwork_im_iam_application_bootstrap/u,
  'Standalone gateway must depend on the IM IAM application bootstrap crate.',
);

assert.match(
  workspaceCargo,
  /sdkwork-im-iam-application-bootstrap/u,
  'Workspace must include the IM IAM application bootstrap crate.',
);

console.log('sdkwork-im IAM application bootstrap standard passed.');
