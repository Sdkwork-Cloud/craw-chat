#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadGeneratorYaml } from '../../workspace-sdk-generator-root-shared.mjs';
import {
  applyFlutterCompatibilityTransforms,
  cloneOpenApiJson,
  loadOpenApiDocument,
  parseOpenApiSourceArgs,
  writeOpenApiYamlDocument,
} from '../../workspace-openapi-source-shared.mjs';
import { applySdkworkV3OpenApiStandard } from '../../workspace-openapi-v3-standard.mjs';

const prefix = 'sdkwork-im-sdk';

function stripRealtimeWebsocketPath(document) {
  for (const pathKey of Object.keys(document.paths ?? {})) {
    if (pathKey.endsWith('/realtime/ws')) {
      delete document.paths[pathKey];
    }
  }
}

function collectComponentRefs(value, groupName, refs = new Set()) {
  if (!value || typeof value !== 'object') {
    return refs;
  }
  if (Array.isArray(value)) {
    for (const item of value) {
      collectComponentRefs(item, groupName, refs);
    }
    return refs;
  }
  if (typeof value.$ref === 'string') {
    const componentName = value.$ref.match(new RegExp(`^#/components/${groupName}/([^/]+)$`))?.[1];
    if (componentName) {
      refs.add(componentName);
    }
  }
  for (const child of Object.values(value)) {
    collectComponentRefs(child, groupName, refs);
  }
  return refs;
}

function collectSchemaRefs(value, refs = new Set()) {
  return collectComponentRefs(value, 'schemas', refs);
}

function pruneUnreachableSchemas(document) {
  const schemas = document.components?.schemas;
  if (!schemas || typeof schemas !== 'object') {
    return;
  }
  const reachable = collectSchemaRefs(document.paths ?? {});
  let changed = true;
  while (changed) {
    changed = false;
    for (const schemaName of [...reachable]) {
      const before = reachable.size;
      collectSchemaRefs(schemas[schemaName], reachable);
      changed = changed || before !== reachable.size;
    }
  }
  for (const schemaName of Object.keys(schemas)) {
    if (schemaName !== 'ProblemDetail' && !reachable.has(schemaName)) {
      delete schemas[schemaName];
    }
  }
}

function pruneUnreachableParameters(document) {
  const parameters = document.components?.parameters;
  if (!parameters || typeof parameters !== 'object') {
    return;
  }
  const reachable = collectComponentRefs(document.paths ?? {}, 'parameters');
  for (const parameterName of Object.keys(parameters)) {
    if (!reachable.has(parameterName)) {
      delete parameters[parameterName];
    }
  }
}

const args = parseOpenApiSourceArgs(process.argv.slice(2), {
  prefix,
  allowPreferDerived: true,
  allowTargetLanguage: true,
});
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const yaml = await loadGeneratorYaml(workspaceRoot);
const derived = cloneOpenApiJson(loadOpenApiDocument({ prefix, filePath: path.resolve(args.base), yaml }));
applySdkworkV3OpenApiStandard(derived);
stripRealtimeWebsocketPath(derived);
if (args.targetLanguage === 'flutter') {
  applyFlutterCompatibilityTransforms(derived);
}
pruneUnreachableSchemas(derived);
pruneUnreachableParameters(derived);
writeOpenApiYamlDocument({ filePath: path.resolve(args.derived), document: derived, yaml });
process.stdout.write(args.preferDerived ? path.resolve(args.derived) : path.resolve(args.base));
