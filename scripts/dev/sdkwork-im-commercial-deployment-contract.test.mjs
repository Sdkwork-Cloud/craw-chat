#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const k8sRoot = path.join(repoRoot, 'deployments', 'kubernetes', 'cloud');

const requiredManifests = [
  'namespace.yaml',
  'ingress.yaml',
  'pod-disruption-budgets.yaml',
  'horizontal-pod-autoscalers.yaml',
  'im-gateway/deployment.yaml',
  'im-gateway/service.yaml',
  'session-gateway/deployment.yaml',
  'conversation-service/deployment.yaml',
  'governance-service/deployment.yaml',
  'notification-service/deployment.yaml',
  'projection-service/deployment.yaml',
  'media-service/deployment.yaml',
  'streaming-service/deployment.yaml',
];

for (const relativePath of requiredManifests) {
  assert.equal(
    fs.existsSync(path.join(k8sRoot, relativePath)),
    true,
    `missing kubernetes manifest: deployments/kubernetes/cloud/${relativePath}`,
  );
}

const stagingProfile = path.join(repoRoot, 'configs', 'topology', 'cloud.staging.env');
assert.equal(fs.existsSync(stagingProfile), true, 'missing staging topology profile');

const prometheusRules = path.join(repoRoot, 'deployments', 'observability', 'prometheus-rules.yaml');
assert.equal(fs.existsSync(prometheusRules), true, 'missing prometheus alert rules');

const otelCollector = path.join(repoRoot, 'deployments', 'observability', 'otel-collector.yaml');
assert.equal(fs.existsSync(otelCollector), true, 'missing otel collector manifest');

const observabilityRunbook = path.join(repoRoot, 'deployments', 'observability', 'README.md');
assert.equal(fs.existsSync(observabilityRunbook), true, 'missing observability runbook');

const customerOpsGuide = path.join(repoRoot, 'docs', 'product', 'compliance', 'CUSTOMER_OPERATIONS.md');
assert.equal(fs.existsSync(customerOpsGuide), true, 'missing customer operations guide');

const dataProtectionGuide = path.join(repoRoot, 'docs', 'product', 'compliance', 'DATA_PROTECTION.md');
assert.equal(fs.existsSync(dataProtectionGuide), true, 'missing data protection guide');

const dependabot = path.join(repoRoot, '.github', 'dependabot.yml');
assert.equal(fs.existsSync(dependabot), true, 'missing dependabot config');

const conversationRuntimeSource = fs.readFileSync(
  path.join(
    repoRoot,
    'services',
    'sdkwork-comms-conversation-service',
    'src',
    'runtime.rs',
  ),
  'utf8',
);
const conversationComponentSpec = JSON.parse(
  fs.readFileSync(
    path.join(
      repoRoot,
      'services',
      'sdkwork-comms-conversation-service',
      'specs',
      'component.spec.json',
    ),
    'utf8',
  ),
);
const conversationConfigMap = fs.readFileSync(
  path.join(k8sRoot, 'conversation-service', 'configmap.example.yaml'),
  'utf8',
);
const consolidatedConfigMaps = fs.readFileSync(
  path.join(k8sRoot, 'configmaps-and-secrets.yaml'),
  'utf8',
);
const conversationDeployment = fs.readFileSync(
  path.join(k8sRoot, 'conversation-service', 'deployment.yaml'),
  'utf8',
);
const localPostgresExample = fs.readFileSync(
  path.join(repoRoot, '.env.postgres.example'),
  'utf8',
);

const conversationCacheLimits = {
  SDKWORK_IM_CONVERSATION_MAX_IN_MEMORY: '10000',
  SDKWORK_IM_CONVERSATION_CACHE_MAX_BYTES: '536870912',
};
const declaredConversationConfigKeys = new Set(
  conversationComponentSpec.contracts.configKeys,
);

for (const [key, value] of Object.entries(conversationCacheLimits)) {
  assert.match(
    conversationRuntimeSource,
    new RegExp('"' + key + '"', 'u'),
    'conversation runtime must consume ' + key,
  );
  assert.equal(
    declaredConversationConfigKeys.has(key),
    true,
    'conversation component contract must declare ' + key,
  );
  assert.match(
    localPostgresExample,
    new RegExp('^' + key + '=' + value + '$', 'mu'),
    '.env.postgres.example must declare ' + key + '=' + value,
  );
  assert.match(
    conversationConfigMap,
    new RegExp('^  ' + key + ': "' + value + '"$', 'mu'),
    'conversation service ConfigMap must declare ' + key + '=' + value,
  );
  assert.match(
    consolidatedConfigMaps,
    new RegExp('^  ' + key + ': "' + value + '"$', 'mu'),
    'consolidated ConfigMap must declare ' + key + '=' + value,
  );
  assert.doesNotMatch(
    conversationDeployment,
    new RegExp('name:\\s*' + key, 'u'),
    'conversation Deployment must consume ' + key + ' from its ConfigMap without duplicating it',
  );
}

for (const profile of [
  'standalone.development',
  'standalone.staging',
  'standalone.production',
  'cloud.development',
  'cloud.staging',
  'cloud.production',
]) {
  const topology = fs.readFileSync(
    path.join(repoRoot, 'configs', 'topology', profile + '.env'),
    'utf8',
  );
  for (const [key, value] of Object.entries(conversationCacheLimits)) {
    assert.match(
      topology,
      new RegExp('^' + key + '=' + value + '$', 'mu'),
      profile + ' must declare ' + key + '=' + value,
    );
  }
}

for (const profile of ['standalone.production', 'cloud.staging', 'cloud.production']) {
  const topology = fs.readFileSync(
    path.join(repoRoot, 'configs', 'topology', profile + '.env'),
    'utf8',
  );
  for (const key of [
    'ENGINE',
    'HOST',
    'PORT',
    'NAME',
    'SCHEMA',
    'USERNAME',
    'PASSWORD_FILE',
    'SSL_MODE',
    'MAX_CONNECTIONS',
  ]) {
    assert.match(
      topology,
      new RegExp('^SDKWORK_IM_DATABASE_' + key + '=', 'mu'),
      profile + ' must declare canonical SDKWORK_IM_DATABASE_' + key,
    );
  }
  assert.doesNotMatch(
    topology,
    /^SDKWORK_CLAW_DATABASE_/mu,
    profile + ' must not depend on legacy SDKWORK_CLAW database aliases',
  );
}

const releaseStageSource = fs.readFileSync(
  path.join(repoRoot, 'scripts', 'release', 'stage-sdkwork-im-release-package.mjs'),
  'utf8',
);
assert.doesNotMatch(
  releaseStageSource,
  /SDKWORK_CLAW_DATABASE_(?:NAME|SCHEMA|USERNAME)/u,
  'release server environment must use canonical SDKWORK_IM database keys',
);

assert.match(
  conversationDeployment,
  /limits:[\s\S]*memory:\s*2Gi/u,
  'conversation container memory limit must leave headroom above the 512 MiB cache budget',
);

console.log('sdkwork-im commercial deployment contract passed');
