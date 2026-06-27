#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const k8sRoot = path.join(repoRoot, 'deployments', 'kubernetes', 'cloud-split-services');

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
    `missing kubernetes manifest: deployments/kubernetes/cloud-split-services/${relativePath}`,
  );
}

const stagingProfile = path.join(repoRoot, 'configs', 'topology', 'cloud.split-services.staging.env');
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

console.log('sdkwork-im commercial deployment contract passed');
