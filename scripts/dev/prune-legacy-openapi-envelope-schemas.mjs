#!/usr/bin/env node
import { copyFileSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { loadGeneratorYaml } from '../../sdks/workspace-sdk-generator-root-shared.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const yaml = await loadGeneratorYaml(repoRoot);

const inlineCommandResponse = {
  allOf: [
    { $ref: '#/components/schemas/SdkWorkApiResponse' },
    {
      type: 'object',
      required: ['data'],
      properties: {
        data: {
          type: 'object',
          additionalProperties: false,
          required: ['accepted'],
          properties: {
            accepted: { type: 'boolean', const: true },
            resourceId: { type: 'string' },
            status: { type: 'string' },
          },
        },
      },
    },
  ],
};

function collectSchemaRefs(value, refs = new Set()) {
  if (!value || typeof value !== 'object') {
    return refs;
  }
  if (Array.isArray(value)) {
    for (const item of value) {
      collectSchemaRefs(item, refs);
    }
    return refs;
  }
  if (typeof value.$ref === 'string') {
    const schemaName = value.$ref.match(/^#\/components\/schemas\/([^/]+)$/)?.[1];
    if (schemaName) {
      refs.add(schemaName);
    }
  }
  for (const child of Object.values(value)) {
    collectSchemaRefs(child, refs);
  }
  return refs;
}

function replaceLegacyCommandResponseRefs(document) {
  const replaceValue = (value) => {
    if (!value || typeof value !== 'object') {
      return;
    }
    if (Array.isArray(value)) {
      for (const item of value) {
        replaceValue(item);
      }
      return;
    }
    if (value.$ref === '#/components/schemas/SdkWorkCommandResponse') {
      delete value.$ref;
      Object.assign(value, structuredClone(inlineCommandResponse));
      return;
    }
    for (const child of Object.values(value)) {
      replaceValue(child);
    }
  };
  replaceValue(document);
}

function pruneUnreachableSchemas(document) {
  replaceLegacyCommandResponseRefs(document);
  const schemas = document.components?.schemas ?? {};
  const reachable = collectSchemaRefs(document.paths ?? {});
  let changed = true;
  while (changed) {
    changed = false;
    for (const schemaName of [...reachable]) {
      const before = reachable.size;
      collectSchemaRefs(schemas[schemaName], reachable);
      if (reachable.size !== before) {
        changed = true;
      }
    }
  }
  const removed = [];
  for (const schemaName of Object.keys(schemas)) {
    if (schemaName !== 'ProblemDetail' && !reachable.has(schemaName)) {
      delete schemas[schemaName];
      removed.push(schemaName);
    }
  }
  return removed;
}

const authorityPairs = [
  {
    apisPath: 'apis/open-api/im/sdkwork-im-im.openapi.yaml',
    sdkMirrorPath: 'sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml',
  },
  {
    apisPath: 'apis/app-api/communication/sdkwork-im-app-api.openapi.yaml',
    sdkMirrorPath: 'sdks/sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.openapi.yaml',
  },
  {
    apisPath: 'apis/backend-api/communication/sdkwork-im-backend-api.openapi.yaml',
    sdkMirrorPath: 'sdks/sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.openapi.yaml',
  },
];

for (const { apisPath, sdkMirrorPath } of authorityPairs) {
  const absoluteApisPath = path.join(repoRoot, apisPath);
  const document = yaml.load(readFileSync(absoluteApisPath, 'utf8'));
  const removed = pruneUnreachableSchemas(document);
  const nextContents = yaml.dump(document, { lineWidth: 120, noRefs: true, sortKeys: false });
  writeFileSync(absoluteApisPath, nextContents, 'utf8');
  copyFileSync(absoluteApisPath, path.join(repoRoot, sdkMirrorPath));
  process.stdout.write(`pruned ${removed.length} unreachable schemas in ${apisPath}\n`);
}
