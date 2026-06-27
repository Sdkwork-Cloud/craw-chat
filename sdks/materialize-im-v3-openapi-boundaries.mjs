#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { loadGeneratorYaml } from './workspace-sdk-generator-root-shared.mjs';
import {
  applyFlutterCompatibilityTransforms,
  cloneOpenApiJson,
  loadOpenApiDocument,
  writeOpenApiYamlDocument,
} from './workspace-openapi-source-shared.mjs';
import { applySdkworkV3OpenApiStandard } from './workspace-openapi-v3-standard.mjs';
import { mergeImSpacesOpenApiFragments } from './merge-im-spaces-openapi-fragments.mjs';
import { applyWebFrameworkOpenApiExtensions } from '../scripts/sdkwork-im-web-framework-openapi-extensions.mjs';

const sdkRoot = path.dirname(fileURLToPath(import.meta.url));
const imRoot = path.join(sdkRoot, 'sdkwork-im-sdk');
const backendRoot = path.join(sdkRoot, 'sdkwork-im-backend-sdk');
const appRoot = path.join(sdkRoot, 'sdkwork-im-app-sdk');

const imAuthorityPath = path.join(imRoot, 'openapi', 'sdkwork-im-im.openapi.yaml');
const imDerivedPath = path.join(imRoot, 'openapi', 'sdkwork-im-im.sdkgen.yaml');
const imFlutterDerivedPath = path.join(imRoot, 'openapi', 'sdkwork-im-im.flutter.sdkgen.yaml');
const backendAuthorityPath = path.join(backendRoot, 'openapi', 'sdkwork-im-backend-api.openapi.yaml');
const backendDerivedPath = path.join(backendRoot, 'openapi', 'sdkwork-im-backend-api.sdkgen.yaml');
const appAuthorityPath = path.join(appRoot, 'openapi', 'sdkwork-im-app-api.openapi.yaml');
const appDerivedPath = path.join(appRoot, 'openapi', 'sdkwork-im-app-api.sdkgen.yaml');
const appFlutterDerivedPath = path.join(appRoot, 'openapi', 'sdkwork-im-app-api.flutter.sdkgen.yaml');
const appbaseAppAuthorityPath = path.resolve(
  sdkRoot,
  '..',
  '..',
  'sdkwork-appbase',
  'sdks',
  'sdkwork-iam-app-sdk',
  'openapi',
  'sdkwork-iam-app-api.openapi.yaml',
);
const appbaseBackendAuthorityPath = path.resolve(
  sdkRoot,
  '..',
  '..',
  'sdkwork-appbase',
  'sdks',
  'sdkwork-iam-backend-sdk',
  'openapi',
  'sdkwork-iam-backend-api.openapi.yaml',
);

const backendPrefix = '/backend/v3/api';
const appPrefix = '/app/v3/api';
const imPrefix = '/im/v3/api';
const backendManagementGroups = new Set(['admin', 'audit', 'control', 'ops']);
const appForbiddenManagementGroups = new Set(['admin', 'audit', 'control', 'ops']);

function fail(message) {
  console.error(`[materialize-im-v3-openapi-boundaries] ${message}`);
  process.exit(1);
}

function stripPrefix(pathKey, prefix) {
  return pathKey.startsWith(`${prefix}/`) ? pathKey.slice(prefix.length + 1) : '';
}

function firstGroup(pathKey, prefix) {
  return stripPrefix(pathKey, prefix).split('/').filter(Boolean)[0] || '';
}

function secondGroup(pathKey, prefix) {
  return stripPrefix(pathKey, prefix).split('/').filter(Boolean)[1] || '';
}

function pathWithoutPrefix(pathKey, prefix) {
  const stripped = stripPrefix(pathKey, prefix);
  return stripped ? stripped.replace(/\{[^}]+\}/g, '{}') : '';
}

function collectRouteSet(document, prefix) {
  const routes = new Set();
  for (const pathKey of Object.keys(document.paths ?? {})) {
    if (pathKey.startsWith(`${prefix}/`)) {
      routes.add(pathWithoutPrefix(pathKey, prefix));
    }
  }
  return routes;
}

function rebasePath(pathKey, fromPrefix, toPrefix) {
  if (!pathKey.startsWith(`${fromPrefix}/`)) {
    fail(`Cannot rebase path outside ${fromPrefix}: ${pathKey}`);
  }
  return `${toPrefix}/${stripPrefix(pathKey, fromPrefix)}`;
}

function isImStandardPath(pathKey, prefix) {
  const route = pathWithoutPrefix(pathKey, prefix);
  return (
    route.startsWith('chat/')
    || route.startsWith('calls/')
    || route.startsWith('presence/')
    || route.startsWith('realtime/')
    || route.startsWith('social/')
    || route.startsWith('spaces/')
    || route === 'spaces'
    || route.startsWith('streams')
  );
}

function isDeviceCapabilityPath(pathKey, prefix) {
  const route = pathWithoutPrefix(pathKey, prefix);
  return (
    route.startsWith('device/')
    || route.startsWith('devices/')
  );
}

function isAiotOwnedPath(pathKey, prefix) {
  return pathWithoutPrefix(pathKey, prefix).startsWith('iot/');
}

function isRtcPath(pathKey, prefix) {
  return pathWithoutPrefix(pathKey, prefix).startsWith('rtc/');
}

function isDriveOwnedMediaLifecyclePath(pathKey, prefix) {
  const route = pathWithoutPrefix(pathKey, prefix);
  return (
    route.startsWith('media/uploads')
    || route.startsWith('media/{}/')
    || route === 'media/{}'
  );
}

function isBackendManagementPath(pathKey) {
  const group = firstGroup(pathKey, backendPrefix);
  if (backendManagementGroups.has(group)) {
    return true;
  }
  return group === 'automation' && secondGroup(pathKey, backendPrefix) === 'governance';
}

function isAppManagementPath(pathKey) {
  const group = firstGroup(pathKey, appPrefix);
  if (appForbiddenManagementGroups.has(group)) {
    return true;
  }
  return group === 'automation' && secondGroup(pathKey, appPrefix) === 'governance';
}

function collectRebasedPaths({
  sources,
  fromPrefix,
  toPrefix,
  shouldInclude,
  failOutsidePrefix = true,
}) {
  const paths = {};
  for (const source of sources) {
    for (const [pathKey, pathItem] of Object.entries(source.paths ?? {})) {
      if (!pathKey.startsWith(`${fromPrefix}/`)) {
        if (failOutsidePrefix) {
          fail(`OpenAPI authority contains a path outside ${fromPrefix}: ${pathKey}`);
        }
        continue;
      }
      if (!shouldInclude(pathKey)) {
        continue;
      }
      const nextPathKey = fromPrefix === toPrefix ? pathKey : rebasePath(pathKey, fromPrefix, toPrefix);
      paths[nextPathKey] = cloneOpenApiJson(pathItem);
    }
  }
  return paths;
}

function normalizeBackendOperationIds(document) {
  const opsProviderBindings = document.paths?.['/backend/v3/api/ops/provider_bindings']?.get;
  if (opsProviderBindings) {
    opsProviderBindings.operationId = 'ops.providerBindings.list';
  }

  const opsProviderBindingDrift = document.paths?.['/backend/v3/api/ops/provider_bindings/drift']?.get;
  if (opsProviderBindingDrift) {
    opsProviderBindingDrift.operationId = 'ops.providerBindings.drift.retrieve';
  }

  const controlProviderBindings = document.paths?.['/backend/v3/api/control/provider_bindings']?.get;
  if (controlProviderBindings) {
    controlProviderBindings.operationId = 'control.providerBindings.list';
  }

  const controlProviderBindingsCreate = document.paths?.['/backend/v3/api/control/provider_bindings']?.post;
  if (controlProviderBindingsCreate) {
    controlProviderBindingsCreate.operationId = 'control.providerBindings.create';
  }
}

function normalizeTags(document, allowedTagNames) {
  const existingTags = document.tags ?? [];
  const normalizedTags = [];
  const seen = new Set();
  for (const tag of existingTags) {
    if (!allowedTagNames.has(tag.name) || seen.has(tag.name)) {
      continue;
    }
    normalizedTags.push(cloneOpenApiJson(tag));
    seen.add(tag.name);
  }
  for (const tagName of allowedTagNames) {
    if (!seen.has(tagName)) {
      normalizedTags.push({ name: tagName });
      seen.add(tagName);
    }
  }
  document.tags = normalizedTags;
}

function annotateOwnerMetadata(document, { owner, apiAuthority }) {
  document['x-sdkwork-owner'] = owner;
  document['x-sdkwork-api-authority'] = apiAuthority;
  for (const pathItem of Object.values(document.paths ?? {})) {
    if (!pathItem || typeof pathItem !== 'object') {
      continue;
    }
    for (const [method, operation] of Object.entries(pathItem)) {
      if (!['delete', 'get', 'head', 'options', 'patch', 'post', 'put', 'trace'].includes(method.toLowerCase())) {
        continue;
      }
      if (!operation || typeof operation !== 'object') {
        continue;
      }
      operation['x-sdkwork-owner'] = owner;
      operation['x-sdkwork-api-authority'] = apiAuthority;
    }
  }
}

function removeSchemaProperties(schema, propertyNames) {
  if (!schema?.properties || typeof schema.properties !== 'object') {
    return;
  }
  for (const propertyName of propertyNames) {
    delete schema.properties[propertyName];
  }
}

function driveReferenceSchema() {
  return {
    additionalProperties: false,
    properties: {
      driveUri: { type: 'string' },
      spaceId: { type: 'string' },
      nodeId: { type: 'string' },
      nodeVersion: { type: 'string' },
    },
    required: ['driveUri', 'spaceId', 'nodeId'],
    type: 'object',
  };
}

function constStringSchema(value) {
  return {
    enum: [value],
    type: 'string',
  };
}

function normalizeContentPartSchema(schemas) {
  if (!schemas.ContentPart?.properties || typeof schemas.ContentPart.properties !== 'object') {
    return;
  }

  const baseProperties = schemas.ContentPart.properties;
  const textContentPart = {
    additionalProperties: false,
    properties: {
      kind: constStringSchema('text'),
      text: cloneOpenApiJson(baseProperties.text ?? { type: 'string' }),
    },
    required: ['kind', 'text'],
    type: 'object',
  };
  const dataContentPart = {
    additionalProperties: false,
    properties: {
      kind: constStringSchema('data'),
      schemaRef: cloneOpenApiJson(baseProperties.schemaRef ?? { type: 'string' }),
      encoding: cloneOpenApiJson(baseProperties.encoding ?? { type: 'string' }),
      payload: cloneOpenApiJson(baseProperties.payload ?? { type: 'string' }),
    },
    required: ['kind', 'schemaRef', 'encoding', 'payload'],
    type: 'object',
  };
  const mediaContentPart = {
    additionalProperties: false,
    properties: {
      kind: constStringSchema('media'),
      drive: { $ref: '#/components/schemas/DriveReference' },
      resource: { $ref: '#/components/schemas/MediaResource' },
      mediaRole: cloneOpenApiJson(baseProperties.mediaRole ?? { type: 'string' }),
    },
    required: ['kind', 'drive', 'resource'],
    type: 'object',
  };
  const signalContentPart = {
    additionalProperties: false,
    properties: {
      kind: constStringSchema('signal'),
      signalType: cloneOpenApiJson(baseProperties.signalType ?? { type: 'string' }),
      schemaRef: cloneOpenApiJson(baseProperties.schemaRef ?? { type: 'string' }),
      payload: cloneOpenApiJson(baseProperties.payload ?? { type: 'string' }),
    },
    required: ['kind', 'signalType', 'payload'],
    type: 'object',
  };
  const streamRefContentPart = {
    additionalProperties: false,
    properties: {
      kind: constStringSchema('stream_ref'),
      streamId: cloneOpenApiJson(baseProperties.streamId ?? { type: 'string' }),
      streamType: cloneOpenApiJson(baseProperties.streamType ?? { type: 'string' }),
      state: cloneOpenApiJson(baseProperties.state ?? { type: 'string' }),
    },
    required: ['kind', 'streamId', 'streamType', 'state'],
    type: 'object',
  };

  schemas.TextContentPart = textContentPart;
  schemas.DataContentPart = dataContentPart;
  schemas.MediaContentPart = mediaContentPart;
  schemas.SignalContentPart = signalContentPart;
  schemas.StreamRefContentPart = streamRefContentPart;
  schemas.ContentPart = {
    discriminator: {
      propertyName: 'kind',
    },
    oneOf: [
      { $ref: '#/components/schemas/TextContentPart' },
      { $ref: '#/components/schemas/DataContentPart' },
      { $ref: '#/components/schemas/MediaContentPart' },
      { $ref: '#/components/schemas/SignalContentPart' },
      { $ref: '#/components/schemas/StreamRefContentPart' },
    ],
  };
}

function normalizeDriveBackedMediaComponents(document) {
  const schemas = document.components?.schemas;
  if (!schemas || typeof schemas !== 'object') {
    return;
  }

  schemas.DriveReference = driveReferenceSchema();

  removeSchemaProperties(schemas.ContentPart, ['mediaAssetId']);
  if (schemas.ContentPart?.properties && typeof schemas.ContentPart.properties === 'object') {
    schemas.ContentPart.properties.drive = { $ref: '#/components/schemas/DriveReference' };
  }
  normalizeContentPartSchema(schemas);

  for (const schemaName of ['MediaResource', 'MediaRendition']) {
    removeSchemaProperties(schemas[schemaName], ['bucketId', 'objectKey', 'objectVersion']);
  }

  for (const schemaName of ['MediaSource']) {
    const schema = schemas[schemaName];
    if (!Array.isArray(schema?.enum)) {
      continue;
    }
    schema.enum = schema.enum
      .filter((value) => value !== 'object_storage')
      .map((value) => (value === 'object_storage' ? 'drive' : value));
    if (!schema.enum.includes('drive')) {
      schema.enum.unshift('drive');
    }
  }

  if (schemas.RtcRecordingArtifact && typeof schemas.RtcRecordingArtifact === 'object') {
    schemas.RtcRecordingArtifact = {
      additionalProperties: false,
      properties: {
        tenantId: { type: 'string' },
        rtcSessionId: { type: 'string' },
        drive: { $ref: '#/components/schemas/DriveReference' },
        resource: { $ref: '#/components/schemas/MediaResource' },
        mediaRole: { type: 'string' },
      },
      required: ['tenantId', 'rtcSessionId', 'drive', 'resource', 'mediaRole'],
      type: 'object',
    };
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
    const escapedGroup = groupName.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const componentName = value.$ref.match(new RegExp(`^#/components/${escapedGroup}/([^/]+)$`))?.[1];
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
      if (reachable.size !== before) {
        changed = true;
      }
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

function normalizeImAuthority(im) {
  const next = cloneOpenApiJson(im);
  next.info = {
    title: 'Sdkwork IM IM Standardized Development API',
    version: im.info?.version || '0.1.0',
    description:
      'IM standardized development OpenAPI contract for conversations, messages, realtime, media, streams, social IM flows, and communication spaces.',
  };
  next.paths = collectRebasedPaths({
    sources: [im],
    fromPrefix: imPrefix,
    toPrefix: imPrefix,
    shouldInclude: (pathKey) => isImStandardPath(pathKey, imPrefix),
  });

  const allowedTagNames = new Set();
  for (const pathKey of Object.keys(next.paths)) {
    const group = firstGroup(pathKey, imPrefix);
    if (group) {
      allowedTagNames.add(group);
    }
  }
  normalizeTags(next, allowedTagNames);
  normalizeDriveBackedMediaComponents(next);
  pruneUnreachableSchemas(next);
  pruneUnreachableParameters(next);
  return next;
}

function normalizeBackendAuthority(backend, dependencyBackendRouteSet) {
  const next = cloneOpenApiJson(backend);
  next.info = {
    title: 'Sdkwork IM Backend Management API',
    version: backend.info?.version || '0.1.0',
    description:
      'Backend management OpenAPI contract for operator, governance, control, and admin APIs under one SDK family.',
  };
  next.paths = {};

  for (const [pathKey, pathItem] of Object.entries(backend.paths ?? {})) {
    if (!pathKey.startsWith(`${backendPrefix}/`)) {
      fail(`Backend authority contains a path outside ${backendPrefix}: ${pathKey}`);
    }
    if (dependencyBackendRouteSet.has(pathWithoutPrefix(pathKey, backendPrefix))) {
      continue;
    }
    if (!isBackendManagementPath(pathKey)) {
      fail(`Backend authority contains non-management API path that belongs in ${appPrefix}: ${pathKey}`);
    }
    next.paths[pathKey] = cloneOpenApiJson(pathItem);
  }

  normalizeTags(next, new Set(['admin', 'audit', 'automation', 'control', 'ops']));
  normalizeBackendOperationIds(next);
  pruneUnreachableSchemas(next);
  pruneUnreachableParameters(next);
  return next;
}

function normalizeAppAuthority(app, im, dependencyAppRouteSet) {
  const next = cloneOpenApiJson(app);
  next.info = {
    title: app.info?.title || 'Sdkwork IM App API',
    version: app.info?.version || '0.1.0',
    description:
      'Owner-only app-development OpenAPI contract for Sdkwork IM-owned app-business and non-management HTTP APIs. Dependency capabilities, including sdkwork-appbase identity/session/IAM/QR auth, are consumed through sdkDependencies and dependency SDKs instead of being regenerated here.',
  };
  const appNativePaths = collectRebasedPaths({
    sources: [app],
    fromPrefix: appPrefix,
    toPrefix: appPrefix,
    shouldInclude: (pathKey) =>
      !isAppManagementPath(pathKey)
      && !isDeviceCapabilityPath(pathKey, appPrefix)
      && !isAiotOwnedPath(pathKey, appPrefix)
      && !isImStandardPath(pathKey, appPrefix)
      && !isRtcPath(pathKey, appPrefix)
      && !isDriveOwnedMediaLifecyclePath(pathKey, appPrefix)
      && !dependencyAppRouteSet.has(pathWithoutPrefix(pathKey, appPrefix)),
  });
  const appPathsFromImAuthority = collectRebasedPaths({
    sources: [im],
    fromPrefix: imPrefix,
    toPrefix: appPrefix,
    shouldInclude: (pathKey) =>
      !isDeviceCapabilityPath(pathKey, imPrefix)
      && !isAiotOwnedPath(pathKey, imPrefix)
      && !isImStandardPath(pathKey, imPrefix)
      && !isRtcPath(pathKey, imPrefix)
      && !isDriveOwnedMediaLifecyclePath(pathKey, imPrefix)
      && !dependencyAppRouteSet.has(pathWithoutPrefix(pathKey, imPrefix)),
  });
  next.paths = {
    ...appPathsFromImAuthority,
    ...appNativePaths,
  };

  const allowedTagNames = new Set();
  for (const pathKey of Object.keys(next.paths)) {
    if (!pathKey.startsWith(`${appPrefix}/`)) {
      fail(`App authority contains a path outside ${appPrefix}: ${pathKey}`);
    }
    if (isAppManagementPath(pathKey)) {
      fail(`App authority contains management API path that belongs in ${backendPrefix}: ${pathKey}`);
    }
    const group = firstGroup(pathKey, appPrefix);
    if (group) {
      allowedTagNames.add(group);
    }
  }

  normalizeTags(next, allowedTagNames);
  normalizeDriveBackedMediaComponents(next);
  pruneUnreachableSchemas(next);
  pruneUnreachableParameters(next);
  return next;
}

function sdkgenDerivedDocument(
  document,
  {
    describeRealtimeWebsocketExclusion = false,
    applyFlutterCompatibility = false,
    describeFlutterCompatibility = false,
  } = {},
) {
  const next = cloneOpenApiJson(document);
  let removedRealtimeWebsocket = false;
  for (const pathKey of Object.keys(next.paths ?? {})) {
    if (pathKey.endsWith('/realtime/ws')) {
      delete next.paths[pathKey];
      removedRealtimeWebsocket = true;
    }
  }
  if (removedRealtimeWebsocket && describeRealtimeWebsocketExclusion && next.info && typeof next.info === 'object') {
    const description = typeof next.info.description === 'string' ? next.info.description.trim() : '';
    const suffix =
      'Derived sdkgen input excludes the realtime websocket upgrade route. Websocket transport stays manual-owned.';
    next.info.description = description ? `${description}\n${suffix}` : suffix;
  }
  if (applyFlutterCompatibility) {
    applyFlutterCompatibilityTransforms(next, {
      describePrimitiveRefExpansion: describeFlutterCompatibility,
    });
  }
  pruneUnreachableSchemas(next);
  pruneUnreachableParameters(next);
  return next;
}

const yaml = await loadGeneratorYaml(sdkRoot);
const im = loadOpenApiDocument({ prefix: 'sdkwork-im-sdk', filePath: imAuthorityPath, yaml });
mergeImSpacesOpenApiFragments(im, yaml);
const backend = loadOpenApiDocument({ prefix: 'sdkwork-im-backend-sdk', filePath: backendAuthorityPath, yaml });
const app = loadOpenApiDocument({ prefix: 'sdkwork-im-app-sdk', filePath: appAuthorityPath, yaml });
const appbaseApp = loadOpenApiDocument({
  prefix: 'sdkwork-iam-app-sdk',
  filePath: appbaseAppAuthorityPath,
  yaml,
});
const appbaseBackend = loadOpenApiDocument({
  prefix: 'sdkwork-iam-backend-sdk',
  filePath: appbaseBackendAuthorityPath,
  yaml,
});
const appbaseAppRouteSet = collectRouteSet(appbaseApp, appPrefix);
const appbaseBackendRouteSet = collectRouteSet(appbaseBackend, backendPrefix);
const dependencyAppRouteSet = appbaseAppRouteSet;
const dependencyBackendRouteSet = appbaseBackendRouteSet;

const consolidatedIm = normalizeImAuthority(im);
applySdkworkV3OpenApiStandard(consolidatedIm);
applyWebFrameworkOpenApiExtensions(consolidatedIm, 'open-api');
annotateOwnerMetadata(consolidatedIm, { owner: 'sdkwork-im', apiAuthority: 'sdkwork-im.im' });
const consolidatedImSdkgen = sdkgenDerivedDocument(consolidatedIm, {
  describeRealtimeWebsocketExclusion: true,
});
const consolidatedImFlutter = sdkgenDerivedDocument(consolidatedIm, {
  describeRealtimeWebsocketExclusion: true,
  applyFlutterCompatibility: true,
  describeFlutterCompatibility: true,
});

const consolidatedBackend = normalizeBackendAuthority(backend, dependencyBackendRouteSet);
applySdkworkV3OpenApiStandard(consolidatedBackend);
applyWebFrameworkOpenApiExtensions(consolidatedBackend, 'backend-api');
annotateOwnerMetadata(consolidatedBackend, { owner: 'sdkwork-im', apiAuthority: 'sdkwork-im.backend' });

const consolidatedApp = normalizeAppAuthority(app, im, dependencyAppRouteSet);
applySdkworkV3OpenApiStandard(consolidatedApp);
applyWebFrameworkOpenApiExtensions(consolidatedApp, 'app-api');
annotateOwnerMetadata(consolidatedApp, { owner: 'sdkwork-im', apiAuthority: 'sdkwork-im.app' });
const consolidatedAppSdkgen = sdkgenDerivedDocument(consolidatedApp);
const consolidatedAppFlutter = sdkgenDerivedDocument(consolidatedApp, {
  applyFlutterCompatibility: true,
});

if (!consolidatedIm.paths['/im/v3/api/spaces']) {
  fail('IM authority is missing /im/v3/api/spaces.');
}
if (!consolidatedIm.paths['/im/v3/api/chat/conversations']) {
  fail('IM authority is missing /im/v3/api/chat/conversations.');
}
if (!consolidatedIm.paths['/im/v3/api/realtime/ws']) {
  fail('IM authority is missing /im/v3/api/realtime/ws.');
}
if (consolidatedIm.paths['/im/v3/api/automation/executions']) {
  fail('IM authority must not contain /im/v3/api/automation/executions.');
}
if (consolidatedIm.paths['/im/v3/api/portal/access']) {
  fail('IM authority must not contain /im/v3/api/portal/access.');
}
if (!consolidatedBackend.paths['/backend/v3/api/control/protocol_registry']) {
  fail('Backend authority is missing /backend/v3/api/control/protocol_registry.');
}
if (!consolidatedBackend.paths['/backend/v3/api/admin/api_keys']) {
  fail('Backend authority is missing /backend/v3/api/admin/api_keys.');
}
if (!consolidatedBackend.paths['/backend/v3/api/automation/governance']) {
  fail('Backend authority is missing /backend/v3/api/automation/governance.');
}
if (!consolidatedApp.paths['/app/v3/api/automation/executions']) {
  fail('App authority is missing /app/v3/api/automation/executions.');
}
for (const appPath of Object.keys(consolidatedApp.paths)) {
  if (appPath.startsWith('/app/v3/api/iot/')) {
    fail(`App authority must not contain AIoT route now owned by sdkwork-aiot: ${appPath}`);
  }
}
for (const appPath of Object.keys(consolidatedApp.paths)) {
  if (appPath.startsWith('/app/v3/api/rtc/')) {
    fail(`App authority must not contain retired RTC app-api signaling route now owned by IM calls: ${appPath}`);
  }
}
if (consolidatedApp.paths['/app/v3/api/chat/conversations']) {
  fail('App authority must not contain /app/v3/api/chat/conversations.');
}

writeOpenApiYamlDocument({ filePath: imAuthorityPath, document: consolidatedIm, yaml });
writeOpenApiYamlDocument({ filePath: imDerivedPath, document: consolidatedImSdkgen, yaml });
writeOpenApiYamlDocument({ filePath: imFlutterDerivedPath, document: consolidatedImFlutter, yaml });
writeOpenApiYamlDocument({ filePath: backendAuthorityPath, document: consolidatedBackend, yaml });
writeOpenApiYamlDocument({ filePath: backendDerivedPath, document: consolidatedBackend, yaml });
writeOpenApiYamlDocument({ filePath: appAuthorityPath, document: consolidatedApp, yaml });
writeOpenApiYamlDocument({ filePath: appDerivedPath, document: consolidatedAppSdkgen, yaml });
writeOpenApiYamlDocument({ filePath: appFlutterDerivedPath, document: consolidatedAppFlutter, yaml });

const imRepoRoot = path.join(sdkRoot, '..');
writeOpenApiYamlDocument({
  filePath: path.join(imRepoRoot, 'apis', 'open-api', 'im', 'sdkwork-im-im.openapi.yaml'),
  document: consolidatedIm,
  yaml,
});
writeOpenApiYamlDocument({
  filePath: path.join(imRepoRoot, 'apis', 'backend-api', 'communication', 'sdkwork-im-backend-api.openapi.yaml'),
  document: consolidatedBackend,
  yaml,
});
writeOpenApiYamlDocument({
  filePath: path.join(imRepoRoot, 'apis', 'app-api', 'communication', 'sdkwork-im-app-api.openapi.yaml'),
  document: consolidatedApp,
  yaml,
});

console.log('[materialize-im-v3-openapi-boundaries] OpenAPI SDK boundaries materialized.');
