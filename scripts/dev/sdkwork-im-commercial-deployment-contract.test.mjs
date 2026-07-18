#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseAllDocuments } from 'yaml';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const k8sRoot = path.join(repoRoot, 'deployments', 'kubernetes', 'cloud');

const requiredManifests = [
  'namespace.yaml',
  'ingress.yaml',
  'pod-disruption-budgets.yaml',
  'horizontal-pod-autoscalers.yaml',
  'additional-service-deployments.yaml',
  'image-inventory.json',
  'image-lock.schema.json',
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

const stagingProfile = path.join(repoRoot, 'etc', 'topology', 'cloud.staging.env');
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

const activeCloudServices = [
  'im-gateway',
  'session-gateway',
  'conversation-service',
  'governance-service',
  'notification-service',
  'projection-service',
  'media-service',
  'streaming-service',
  'audit-service',
  'automation-service',
  'social-service',
  'space-service',
  'ops-service',
];
const imageInventory = JSON.parse(
  fs.readFileSync(path.join(k8sRoot, 'image-inventory.json'), 'utf8'),
);
assert.equal(imageInventory.schemaVersion, 1);
assert.deepEqual(Object.keys(imageInventory.images).sort(), [...activeCloudServices].sort());
const cloudBuildInventory = JSON.parse(
  fs.readFileSync(path.join(repoRoot, 'deployments', 'docker', 'cloud-service-builds.json'), 'utf8'),
);
assert.equal(cloudBuildInventory.schemaVersion, 1);
assert.deepEqual(Object.keys(cloudBuildInventory.services).sort(), [...activeCloudServices].sort());
const cloudDockerfile = fs.readFileSync(
  path.join(repoRoot, 'deployments', 'docker', 'sdkwork-im-cloud-service.Dockerfile'),
  'utf8',
);
assert.match(cloudDockerfile, /^ARG RUNTIME_IMAGE$/mu);
assert.match(cloudDockerfile, /^FROM \$\{RUNTIME_IMAGE\}/mu);
assert.doesNotMatch(cloudDockerfile, /cargo fetch|\|\| true|:latest/u);

const deploymentManifestPaths = [
  'additional-service-deployments.yaml',
  'im-gateway/deployment.yaml',
  'session-gateway/deployment.yaml',
  'conversation-service/deployment.yaml',
  'governance-service/deployment.yaml',
  'notification-service/deployment.yaml',
  'projection-service/deployment.yaml',
  'media-service/deployment.yaml',
  'streaming-service/deployment.yaml',
];
const deploymentResources = deploymentManifestPaths.flatMap((relativePath) =>
  parseAllDocuments(fs.readFileSync(path.join(k8sRoot, relativePath), 'utf8'))
    .map((document) => {
      assert.equal(document.errors.length, 0, `${relativePath} must be valid unambiguous YAML`);
      return document.toJS();
    })
    .filter((resource) => resource?.kind === 'Deployment'),
);
assert.deepEqual(
  deploymentResources.map((resource) => resource.metadata.name).sort(),
  [...activeCloudServices].sort(),
);
for (const deployment of deploymentResources) {
  const service = deployment.metadata.name;
  assert.equal(deployment.spec.strategy?.type, 'RollingUpdate', `${service} must use RollingUpdate`);
  assert.equal(
    deployment.spec.strategy?.rollingUpdate?.maxUnavailable,
    0,
    `${service} must keep all current replicas available during rollout`,
  );
  assert.equal(deployment.spec.strategy?.rollingUpdate?.maxSurge, 1);
  assert.ok(deployment.spec.minReadySeconds >= 10, `${service} must prove readiness before promotion`);
  assert.ok(deployment.spec.progressDeadlineSeconds >= 300);

  const podSpec = deployment.spec.template.spec;
  assert.ok(
    podSpec.terminationGracePeriodSeconds >= 60,
    `${service} must have a bounded graceful termination window`,
  );
  const spreadByKey = new Map(
    (podSpec.topologySpreadConstraints ?? []).map((constraint) => [
      constraint.topologyKey,
      constraint,
    ]),
  );
  assert.equal(spreadByKey.get('kubernetes.io/hostname')?.whenUnsatisfiable, 'DoNotSchedule');
  assert.equal(spreadByKey.get('topology.kubernetes.io/zone')?.whenUnsatisfiable, 'ScheduleAnyway');
  for (const constraint of spreadByKey.values()) {
    assert.equal(constraint.maxSkew, 1);
    assert.equal(
      constraint.labelSelector?.matchLabels?.['app.kubernetes.io/name'],
      service,
      `${service} topology selector must match the pod label`,
    );
  }

  assert.equal(podSpec.containers.length, 1);
  const container = podSpec.containers[0];
  assert.equal(
    container.image,
    `${imageInventory.images[service]}:latest`,
    `${service} template repository must match the governed image inventory`,
  );
  assert.ok(container.readinessProbe, `${service} must define readinessProbe`);
  assert.ok(container.livenessProbe, `${service} must define livenessProbe`);
  assert.ok(container.resources?.requests?.memory, `${service} must reserve memory`);
  assert.ok(container.resources?.limits?.memory, `${service} must cap memory`);
}
const hpaManifest = fs.readFileSync(path.join(k8sRoot, 'horizontal-pod-autoscalers.yaml'), 'utf8');
const pdbManifest = fs.readFileSync(path.join(k8sRoot, 'pod-disruption-budgets.yaml'), 'utf8');
for (const service of activeCloudServices) {
  if (service !== 'im-gateway') {
    assert.match(hpaManifest, new RegExp('name:\\s*' + service + '(?:-hpa)?(?:\\s|$)', 'u'));
  }
  assert.match(pdbManifest, new RegExp('name:\\s*' + service + '-pdb(?:\\s|$)', 'u'));
}
const consolidatedDeployments = fs.readFileSync(
  path.join(k8sRoot, 'additional-service-deployments.yaml'),
  'utf8',
);
const consolidatedRuntimeConfig = fs.readFileSync(
  path.join(k8sRoot, 'configmaps-and-secrets.yaml'),
  'utf8',
);
for (const retiredService of ['contact-service', 'interaction-service']) {
  assert.doesNotMatch(consolidatedDeployments, new RegExp(retiredService, 'u'));
  assert.doesNotMatch(consolidatedRuntimeConfig, new RegExp(retiredService, 'u'));
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
    path.join(repoRoot, 'etc', 'topology', profile + '.env'),
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
    path.join(repoRoot, 'etc', 'topology', profile + '.env'),
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
