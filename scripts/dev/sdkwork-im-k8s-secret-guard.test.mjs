#!/usr/bin/env node
/**
 * Kubernetes Secret template placeholder guard.
 *
 * Validates `deployments/kubernetes/cloud/configmaps-and-secrets.yaml` enforces:
 *   - No legacy `CHANGE_ME` placeholder remains anywhere in the template.
 *   - The canonical `<REPLACE_WITH_ACTUAL_VALUE>` placeholder is documented by
 *     a WARNING header comment so operators know to substitute real credentials
 *     (or use `kubectl create secret generic`) before applying to production.
 *   - Placeholder values only appear inside `Secret` resources (sensitive
 *     credentials), never inside `ConfigMap` resources (non-sensitive runtime
 *     configuration that is shared in plaintext).
 *
 * This guards against reintroducing the ambiguous `CHANGE_ME` placeholder and
 * against leaking credential placeholders into non-sensitive ConfigMaps.
 */
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const templatePath = path.join(
  repoRoot,
  'deployments',
  'kubernetes',
  'cloud',
  'configmaps-and-secrets.yaml',
);

assert.ok(
  fs.existsSync(templatePath),
  `K8s secret template not found at ${templatePath}`,
);

const text = fs.readFileSync(templatePath, 'utf8');
const violations = [];

// 1. Legacy `CHANGE_ME` placeholder must not appear anywhere in the template.
if (text.includes('CHANGE_ME')) {
  const lines = text.split('\n');
  lines.forEach((line, idx) => {
    if (line.includes('CHANGE_ME')) {
      violations.push(
        `line ${idx + 1}: legacy placeholder "CHANGE_ME" must be replaced with "<REPLACE_WITH_ACTUAL_VALUE>": ${line.trim()}`,
      );
    }
  });
}

// 2. The WARNING header must document the placeholder convention.
const expectedWarning =
  'WARNING: This template contains placeholder values. Before applying to production,';
assert.ok(
  text.includes(expectedWarning),
  'configmaps-and-secrets.yaml must include the WARNING header documenting the <REPLACE_WITH_ACTUAL_VALUE> placeholder convention',
);

// 3. `<REPLACE_WITH_ACTUAL_VALUE>` placeholders must only live inside Secret
//    resources, never inside ConfigMap resources (which are non-sensitive).
const YAML = (await import('yaml')).default;
const docs = YAML.parseAllDocuments(text).map((doc) => doc.toJS());

assert.ok(
  Array.isArray(docs) && docs.length > 0,
  'configmaps-and-secrets.yaml must contain at least one YAML document',
);

for (const doc of docs) {
  if (!doc || typeof doc !== 'object') continue;
  const kind = doc.kind;
  if (kind === 'ConfigMap') {
    const data = doc.data || {};
    const cmName = doc.metadata && doc.metadata.name ? doc.metadata.name : '<unknown>';
    for (const [key, value] of Object.entries(data)) {
      if (typeof value === 'string' && value.includes('<REPLACE_WITH_ACTUAL_VALUE>')) {
        violations.push(
          `ConfigMap "${cmName}" field "${key}" must not contain a credential placeholder; only Secret resources may carry <REPLACE_WITH_ACTUAL_VALUE>`,
        );
      }
    }
  }
}

assert.equal(
  violations.length,
  0,
  `k8s secret guard violations:\n${violations.join('\n')}`,
);

// 4. Confirm at least one Secret carries the canonical placeholder so the
//    template is not silently stripped of its credential fields.
const secrets = docs.filter((doc) => doc && doc.kind === 'Secret');
assert.ok(secrets.length > 0, 'configmaps-and-secrets.yaml must define at least one Secret');
const sharedSecret = secrets.find(
  (doc) => doc.metadata && doc.metadata.name === 'sdkwork-im-shared-secrets',
);
assert.ok(
  sharedSecret && sharedSecret.stringData,
  'sdkwork-im-shared-secrets Secret with stringData must exist',
);

const placeholderFields = Object.entries(sharedSecret.stringData).filter(([, value]) =>
  typeof value === 'string' && value.includes('<REPLACE_WITH_ACTUAL_VALUE>'),
);
assert.ok(
  placeholderFields.length > 0,
  'sdkwork-im-shared-secrets must carry at least one <REPLACE_WITH_ACTUAL_VALUE> placeholder field',
);

console.log('sdkwork-im k8s secret guard passed');
