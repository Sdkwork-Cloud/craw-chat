import assert from 'node:assert/strict';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { loadGeneratorYaml } from '../workspace-sdk-generator-root-shared.mjs';
import { loadOpenApiDocument } from '../workspace-openapi-source-shared.mjs';

const testDir = path.dirname(fileURLToPath(import.meta.url));
const sdkRoot = path.resolve(testDir, '..');
const repoRoot = path.resolve(sdkRoot, '..');
const rtcSdkRoot = path.resolve(repoRoot, '..', 'sdkwork-rtc', 'sdks', 'sdkwork-rtc-sdk');

function read(relativePath) {
  return readFileSync(path.join(sdkRoot, relativePath), 'utf8');
}

function readRepo(relativePath) {
  return readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function readRtcSdk(relativePath) {
  return readFileSync(path.join(rtcSdkRoot, relativePath), 'utf8');
}

function marker(...parts) {
  return parts.join('');
}

function forbiddenPattern(...terms) {
  return new RegExp(terms.map((term) => term.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')).join('|'));
}

function toPosixPath(value) {
  return value.replaceAll('\\', '/');
}

const scannedTextExtensions = new Set([
  '.cs',
  '.dart',
  '.go',
  '.gradle',
  '.java',
  '.js',
  '.json',
  '.kt',
  '.kts',
  '.md',
  '.mjs',
  '.py',
  '.rs',
  '.swift',
  '.toml',
  '.ts',
  '.yaml',
  '.yml',
]);

function collectTextFiles(rootPath) {
  if (!existsSync(rootPath)) {
    return [];
  }
  const stats = statSync(rootPath);
  if (stats.isFile()) {
    return scannedTextExtensions.has(path.extname(rootPath)) ? [rootPath] : [];
  }
  if (!stats.isDirectory()) {
    return [];
  }
  const files = [];
  const visit = (targetPath) => {
    const targetStats = statSync(targetPath);
    if (targetStats.isDirectory()) {
      for (const entry of readdirSync(targetPath)) {
        if (
          entry === 'node_modules'
          || entry === 'dist'
          || entry === 'build'
          || entry === '.dart_tool'
          || entry === '.sdkwork'
          || entry === 'manual-backups'
          || entry === 'tmp'
          || entry === 'locks'
        ) {
          continue;
        }
        visit(path.join(targetPath, entry));
      }
      return;
    }
    if (targetStats.isFile() && scannedTextExtensions.has(path.extname(targetPath))) {
      files.push(targetPath);
    }
  };
  visit(rootPath);
  return files.sort();
}

function assertNoActiveAppBusinessSdkSurfaceInImFamily() {
  const scanTargets = [
    'sdkwork-im-sdk/bin/normalize-generated-auth-surface.mjs',
    'sdkwork-im-sdk/bin/verify-auth-surface-alignment.mjs',
    'sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md',
    'sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed',
    'sdkwork-im-sdk/sdkwork-im-sdk-typescript/generated/server-openapi',
    'sdkwork-im-sdk/sdkwork-im-sdk-flutter/README.md',
    'sdkwork-im-sdk/sdkwork-im-sdk-flutter/composed',
    'sdkwork-im-sdk/sdkwork-im-sdk-flutter/generated/server-openapi',
    'sdkwork-im-sdk/sdkwork-im-sdk-rust/generated/server-openapi',
    'sdkwork-im-sdk/sdkwork-im-sdk-java/generated/server-openapi',
    'sdkwork-im-sdk/sdkwork-im-sdk-csharp/generated/server-openapi',
    'sdkwork-im-sdk/sdkwork-im-sdk-swift/generated/server-openapi',
    'sdkwork-im-sdk/sdkwork-im-sdk-kotlin/generated/server-openapi',
    'sdkwork-im-sdk/sdkwork-im-sdk-go/generated/server-openapi',
    'sdkwork-im-sdk/sdkwork-im-sdk-python/generated/server-openapi',
  ];
  const forbiddenAppBusinessSurfacePatterns = [
    {
      pattern: /(?:\/|\\\/)portal(?:\/|\\\/)/,
      reason: 'active /portal/* routes belong in sdkwork-im-app-sdk, not sdkwork-im-sdk',
    },
    {
      pattern: /(?:\/|\\\/)automation(?:\/|\\\/)/,
      reason: 'active /automation/* routes belong in sdkwork-im-app-sdk, not sdkwork-im-sdk',
    },
    {
      pattern: /(?:\/|\\\/)notifications(?:\/|\\\/)/,
      reason: 'active /notifications/* routes belong in sdkwork-im-app-sdk, not sdkwork-im-sdk',
    },
    {
      pattern: /(?:\/|\\\/)iot(?:\/|\\\/)/,
      reason: 'active /iot/* routes belong in sdkwork-im-app-sdk, not sdkwork-im-sdk',
    },
    {
      pattern: /(?:\/|\\\/)principal(?:\/|\\\/)/,
      reason: 'active /principal/* routes belong in sdkwork-im-app-sdk, not sdkwork-im-sdk',
    },
    {
      pattern: /provider_health|provider_callbacks/,
      reason: 'provider health and callbacks belong in sdkwork-im-app-sdk, not sdkwork-im-sdk',
    },
    {
      pattern: /\/twin\b|\bDeviceTwin\b|\bdevicesTwin\b|\bgetDeviceTwin\b|\bupdateDeviceTwin/,
      reason: 'device twin APIs belong in sdkwork-im-app-sdk, not sdkwork-im-sdk',
    },
    {
      pattern: /\bPortalApi\b|\bcreatePortalApi\b/,
      reason: 'generated PortalApi belongs in sdkwork-im-app-sdk',
    },
    {
      pattern: /\bAutomationApi\b|\bcreateAutomationApi\b/,
      reason: 'generated AutomationApi belongs in sdkwork-im-app-sdk',
    },
    {
      pattern: /\bNotificationApi\b|\bcreateNotificationApi\b/,
      reason: 'generated NotificationApi belongs in sdkwork-im-app-sdk',
    },
    {
      pattern: /\bImPortalModule\b/,
      reason: 'composed portal module belongs in app SDK wrappers, not IM SDK',
    },
    {
      pattern: /\bclient\.(?:portal|automation|notification)\b|\bsdk\.portal\b/,
      reason: 'public app-business client surface belongs in sdkwork-im-app-sdk',
    },
    {
      pattern: /\bclient\.deviceSessions\b|\bsdk\.deviceSessions\b|\bdeviceSessions\b/,
      reason: 'retired IM device session APIs must not be exposed by sdkwork-im-sdk',
    },
    {
      pattern: /\breadonly\s+portal\b|\blate\s+final\s+\w*Portal\w*\s+portal\b/,
      reason: 'public portal property belongs in sdkwork-im-app-sdk',
    },
    {
      pattern: /\bPortalSnapshot\b|\bPortalUserView\b|\bPortalWorkspaceView\b|\bDeviceTwin\b/,
      reason: 'app-api DTOs must not be generated by sdkwork-im-sdk',
    },
  ];
  const violations = [];
  for (const scanTarget of scanTargets) {
    const targetPath = path.join(sdkRoot, ...scanTarget.split('/'));
    for (const filePath of collectTextFiles(targetPath)) {
      const source = readFileSync(filePath, 'utf8');
      for (const { pattern, reason } of forbiddenAppBusinessSurfacePatterns) {
        const match = pattern.exec(source);
        if (match) {
          violations.push(`${toPosixPath(path.relative(sdkRoot, filePath))}: ${reason}; matched ${match[0]}`);
        }
      }
    }
  }
  assert.deepEqual(
    violations,
    [],
    'sdkwork-im-sdk must not expose active app-business API routes, generated APIs, DTOs, or composed app-business surfaces.',
  );
}

const sharedFamilySource = read('workspace-im-v3-sdk-family.mjs');
const sharedOpenApiStandardSource = read('workspace-openapi-v3-standard.mjs');
const sharedOpenApiSource = read('workspace-openapi-source-shared.mjs');
const appConfigSource = read('sdkwork-im-app-sdk/bin/sdk-family-config.mjs');
const backendConfigSource = read('sdkwork-im-backend-sdk/bin/sdk-family-config.mjs');
const imConfigSource = read('sdkwork-im-sdk/bin/sdk-family-config.mjs');
const appGenerateSource = read('sdkwork-im-app-sdk/bin/generate-sdk.mjs');
const backendGenerateSource = read('sdkwork-im-backend-sdk/bin/generate-sdk.mjs');
const imGenerateSource = read('sdkwork-im-sdk/bin/generate-sdk.mjs');
const appVerifySource = read('sdkwork-im-app-sdk/bin/verify-sdk.mjs');
const backendVerifySource = read('sdkwork-im-backend-sdk/bin/verify-sdk.mjs');
const imVerifySource = read('sdkwork-im-sdk/bin/verify-sdk.mjs');
const appPrepareSource = read('sdkwork-im-app-sdk/bin/prepare-openapi-source.mjs');
const backendPrepareSource = read('sdkwork-im-backend-sdk/bin/prepare-openapi-source.mjs');
const appRefreshSource = read('sdkwork-im-app-sdk/bin/refresh-live-openapi-source.mjs');
const backendRefreshSource = read('sdkwork-im-backend-sdk/bin/refresh-live-openapi-source.mjs');
const imPrepareSource = read('sdkwork-im-sdk/bin/prepare-openapi-source.mjs');
const imRefreshSource = read('sdkwork-im-sdk/bin/refresh-live-openapi-source.mjs');
const imGeneratePowerShellSource = read('sdkwork-im-sdk/bin/generate-sdk.ps1');
const imGenerateShellSource = read('sdkwork-im-sdk/bin/generate-sdk.sh');
const sdkWorkspaceIndexSource = read('README.md');
const appReadmeSource = read('sdkwork-im-app-sdk/README.md');
const appTypeScriptSrcSdkSource = read('sdkwork-im-app-sdk/sdkwork-im-app-sdk-typescript/generated/server-openapi/src/sdk.ts');
const appTypeScriptSrcIndexSource = read('sdkwork-im-app-sdk/sdkwork-im-app-sdk-typescript/generated/server-openapi/src/index.ts');
const appTypeScriptDistSdkSource = read('sdkwork-im-app-sdk/sdkwork-im-app-sdk-typescript/generated/server-openapi/dist/sdk.d.ts');
const appTypeScriptDistIndexSource = read('sdkwork-im-app-sdk/sdkwork-im-app-sdk-typescript/generated/server-openapi/dist/index.d.ts');
const appTypeScriptDistRuntimeSource = read('sdkwork-im-app-sdk/sdkwork-im-app-sdk-typescript/generated/server-openapi/dist/index.js');
const appFlutterClientSource = read('sdkwork-im-app-sdk/sdkwork-im-app-sdk-flutter/generated/server-openapi/lib/app_client.dart');
const appRustClientSource = read('sdkwork-im-app-sdk/sdkwork-im-app-sdk-rust/generated/server-openapi/src/client.rs');
const appRustLibSource = read('sdkwork-im-app-sdk/sdkwork-im-app-sdk-rust/generated/server-openapi/src/lib.rs');
const appAssembly = JSON.parse(read('sdkwork-im-app-sdk/.sdkwork-assembly.json'));
const appComponentSpec = JSON.parse(read('sdkwork-im-app-sdk/specs/component.spec.json'));
const appComponentSpecSource = read('sdkwork-im-app-sdk/specs/README.md');
const backendAssembly = JSON.parse(read('sdkwork-im-backend-sdk/.sdkwork-assembly.json'));
const backendComponentSpec = JSON.parse(read('sdkwork-im-backend-sdk/specs/component.spec.json'));
const backendComponentSpecSource = read('sdkwork-im-backend-sdk/specs/README.md');
const backendReadmeSource = read('sdkwork-im-backend-sdk/README.md');
const backendTypeScriptSrcSdkSource = read('sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-typescript/generated/server-openapi/src/sdk.ts');
const backendTypeScriptSrcIndexSource = read('sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-typescript/generated/server-openapi/src/index.ts');
const backendTypeScriptDistSdkSource = read('sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-typescript/generated/server-openapi/dist/sdk.d.ts');
const backendTypeScriptDistIndexSource = read('sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-typescript/generated/server-openapi/dist/index.d.ts');
const backendTypeScriptDistRuntimeSource = read('sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-typescript/generated/server-openapi/dist/index.js');
const backendFlutterClientSource = read('sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-flutter/generated/server-openapi/lib/backend_client.dart');
const imTypeScriptGeneratedPackage = JSON.parse(read('sdkwork-im-sdk/sdkwork-im-sdk-typescript/generated/server-openapi/package.json'));
const rtcReadmeSource = readRtcSdk('README.md');
const boundaryMaterializerSource = read('materialize-im-v3-openapi-boundaries.mjs');
const yaml = await loadGeneratorYaml(sdkRoot);
const imAuthority = loadOpenApiDocument({
  prefix: 'sdkwork-im-sdk',
  filePath: path.join(sdkRoot, 'sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml'),
  yaml,
});
const imDerived = loadOpenApiDocument({
  prefix: 'sdkwork-im-sdk',
  filePath: path.join(sdkRoot, 'sdkwork-im-sdk/openapi/sdkwork-im-im.sdkgen.yaml'),
  yaml,
});
const imFlutterDerived = loadOpenApiDocument({
  prefix: 'sdkwork-im-sdk',
  filePath: path.join(sdkRoot, 'sdkwork-im-sdk/openapi/sdkwork-im-im.flutter.sdkgen.yaml'),
  yaml,
});
const appAuthority = loadOpenApiDocument({
  prefix: 'sdkwork-im-app-sdk',
  filePath: path.join(sdkRoot, 'sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.openapi.yaml'),
  yaml,
});
const appDerived = loadOpenApiDocument({
  prefix: 'sdkwork-im-app-sdk',
  filePath: path.join(sdkRoot, 'sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.sdkgen.yaml'),
  yaml,
});
const appFlutterDerived = loadOpenApiDocument({
  prefix: 'sdkwork-im-app-sdk',
  filePath: path.join(sdkRoot, 'sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.flutter.sdkgen.yaml'),
  yaml,
});
const backendAuthority = loadOpenApiDocument({
  prefix: 'sdkwork-im-backend-sdk',
  filePath: path.join(sdkRoot, 'sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.openapi.yaml'),
  yaml,
});
const appbaseBackendAuthority = loadOpenApiDocument({
  prefix: 'sdkwork-iam-backend-sdk',
  filePath: path.resolve(
    sdkRoot,
    '..',
    '..',
    'sdkwork-iam',
    'sdks',
    'sdkwork-iam-backend-sdk',
    'openapi',
    'sdkwork-iam-backend-api.openapi.yaml',
  ),
  yaml,
});

function pathKeys(document) {
  return Object.keys(document.paths ?? {}).sort();
}

function relativeRoute(pathKey, prefix) {
  assert.ok(pathKey.startsWith(`${prefix}/`), `${pathKey} must start with ${prefix}/.`);
  return pathKey.slice(prefix.length + 1);
}

function firstRouteGroup(pathKey, prefix) {
  return relativeRoute(pathKey, prefix).split('/').filter(Boolean)[0] || '';
}

function routeWithoutPrefix(pathKey, prefix) {
  return relativeRoute(pathKey, prefix).replace(/\{[^}]+\}/g, '{}');
}

function assertAllPathsUsePrefix(label, document, prefix) {
  for (const pathKey of pathKeys(document)) {
    assert.ok(pathKey.startsWith(`${prefix}/`), `${label} path ${pathKey} must use ${prefix}.`);
  }
}

function assertNoPathsUsePrefix(label, document, forbiddenPrefix) {
  for (const pathKey of pathKeys(document)) {
    assert.ok(
      !pathKey.startsWith(`${forbiddenPrefix}/`),
      `${label} path ${pathKey} must not use ${forbiddenPrefix}.`,
    );
  }
}

function isImStandardRoute(route) {
  return (
    route.startsWith('chat/')
    || route.startsWith('media/uploads')
    || route.startsWith('media/{}/')
    || route === 'media/{}'
    || route.startsWith('presence/')
    || route.startsWith('realtime/')
    || route === 'calls/sessions'
    || route === 'calls/sessions/{}'
    || route.startsWith('calls/sessions/{}/')
    || route.startsWith('social/')
    || route.startsWith('spaces/')
    || route === 'spaces'
    || route.startsWith('streams')
  );
}

function assertDocumentHasOnlyImStandardRoutes(label, document, prefix) {
  for (const pathKey of pathKeys(document)) {
    const route = routeWithoutPrefix(pathKey, prefix);
    assert.ok(
      isImStandardRoute(route),
      `${label} route ${pathKey} is not IM standardized development API and belongs in app-api or backend-api.`,
    );
  }
}

function assertDocumentHasNoImStandardRoutes(label, document, prefix) {
  for (const pathKey of pathKeys(document)) {
    const route = routeWithoutPrefix(pathKey, prefix);
    assert.ok(
      !isImStandardRoute(route),
      `${label} route ${pathKey} duplicates the IM standardized development API and must not be exposed by app-api.`,
    );
  }
}

const appbaseOwnedAppRoutes = [
  'auth/password_reset_requests',
  'auth/password_resets',
  'auth/registrations',
  'auth/sessions',
  'auth/sessions/refresh',
  'auth/sessions/current',
  'iam/users/current',
  'iam/organizations',
  'iam/organizations/tree',
  'iam/organization_memberships',
  'iam/departments',
  'iam/departments/tree',
  'iam/department_assignments',
  'iam/positions',
  'iam/position_assignments',
  'iam/role_bindings',
  'system/iam/runtime',
  'system/iam/verification_policy',
  'oauth/authorization_urls',
  'oauth/device_authorizations',
  'oauth/device_authorizations/{}',
  'oauth/device_authorizations/{}/scans',
  'oauth/device_authorizations/{}/password_completions',
  'oauth/sessions',
];

function assertDocumentHasNoAppbaseOwnedAppRoutes(label, document, prefix) {
  const routes = new Set(pathKeys(document).map((pathKey) => routeWithoutPrefix(pathKey, prefix)));
  const overlaps = appbaseOwnedAppRoutes.filter((route) => routes.has(route));
  assert.deepEqual(
    overlaps,
    [],
    `${label} must not regenerate sdkwork-appbase-owned app-api routes; consume sdkwork-iam-app-sdk instead.`,
  );
}

function assertNoSemanticOverlap(leftLabel, leftDocument, leftPrefix, rightLabel, rightDocument, rightPrefix) {
  const leftRoutes = new Set(pathKeys(leftDocument).map((pathKey) => routeWithoutPrefix(pathKey, leftPrefix)));
  const overlaps = pathKeys(rightDocument)
    .map((pathKey) => routeWithoutPrefix(pathKey, rightPrefix))
    .filter((route) => leftRoutes.has(route));
  assert.deepEqual(
    [...new Set(overlaps)].sort(),
    [],
    `${leftLabel} and ${rightLabel} must not expose cloned semantic routes under different prefixes.`,
  );
}

function operationEntries(document) {
  const entries = [];
  for (const [pathKey, pathItem] of Object.entries(document.paths ?? {})) {
    for (const [method, operation] of Object.entries(pathItem ?? {})) {
      if (!['get', 'post', 'put', 'patch', 'delete', 'head', 'options', 'trace'].includes(method)) {
        continue;
      }
      entries.push({ pathKey, method, operation });
    }
  }
  return entries;
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

function assertSchemasArePathReachable(label, document) {
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
  const unreachable = Object.keys(schemas)
    .filter((schemaName) => schemaName !== 'ProblemDetail' && !reachable.has(schemaName))
    .sort();
  assert.deepEqual(unreachable, [], `${label} must prune unreachable component schemas.`);
}

function assertGeneratedTransportDoesNotImportSdkDependencies(label, workspace, dependencyPackages) {
  const packageMarkers = [...new Set(dependencyPackages)].sort();
  const generatedRoots = readdirSync(path.join(sdkRoot, workspace), { withFileTypes: true })
    .filter((entry) => entry.isDirectory() && entry.name.startsWith(`${workspace}-`))
    .map((entry) => path.join(sdkRoot, workspace, entry.name, 'generated', 'server-openapi'))
    .filter((entryPath) => existsSync(entryPath));
  assert.notEqual(generatedRoots.length, 0, `${label} must have generated transport outputs to scan.`);

  const violations = [];
  for (const generatedRoot of generatedRoots) {
    for (const filePath of collectTextFiles(generatedRoot)) {
      const source = readFileSync(filePath, 'utf8');
      for (const dependencyPackage of packageMarkers) {
        if (source.includes(dependencyPackage)) {
          violations.push(`${toPosixPath(path.relative(sdkRoot, filePath))}: generated transport must not import or declare ${dependencyPackage}`);
        }
      }
    }
  }
  assert.deepEqual(violations, [], `${label} generated transport must not import or declare SDK family dependencies.`);
}

function isPrimitiveComponentSchema(schema) {
  return Boolean(
    schema
      && typeof schema === 'object'
      && !Array.isArray(schema)
      && (['string', 'integer', 'number', 'boolean'].includes(schema.type)
        || (schema.type === 'object' && schema.additionalProperties && !schema.properties)),
  );
}

function assertFlutterDerivedExpandsPrimitiveComponentRefs(label, document) {
  const primitiveSchemas = Object.entries(document.components?.schemas ?? {})
    .filter(([, schema]) => isPrimitiveComponentSchema(schema))
    .map(([schemaName]) => schemaName)
    .sort();
  assert.deepEqual(
    primitiveSchemas,
    [],
    `${label} must inline primitive component refs before Flutter sdkgen instead of generating empty primitive wrapper models.`,
  );
}

const retiredSdkMarkers = [
  marker('sdkwork', '-control', '-plane', '-sdk'),
  marker('sdkwork', '-im', '-admin', '-sdk'),
  marker('@sdkwork', '/control', '-plane', '-sdk'),
  marker('@sdkwork', '/im', '-admin', '-sdk'),
  marker('control', '_plane', '_sdk'),
  marker('im', '_admin', '_sdk'),
  marker('/sdk', '/control', '-plane', '-sdk'),
  marker('/sdk', '/im', '-admin', '-sdk'),
  marker('Control', '-Plane', ' SDK'),
  marker('IM', ' Admin', ' SDK'),
];

for (const verifierSourcePath of [
  'docs/sites/tests/docs-runtime.test.mjs',
  'docs/sites/sdk/verify-sdk-site-docs.mjs',
  'docs/sites/scripts/verify-api-docs.mjs',
  'docs/sites/scripts/verify-sdk-docs.mjs',
  'docs/sites/scripts/verify-docs-site.mjs',
  'sdks/test/verify-im-v3-sdk-family-contract.test.mjs',
]) {
  const verifierSource = readRepo(verifierSourcePath);
  for (const retiredMarker of retiredSdkMarkers) {
    assert.doesNotMatch(
      verifierSource,
      new RegExp(retiredMarker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
      `${verifierSourcePath} must construct retired SDK marker ${retiredMarker} instead of embedding it literally.`,
    );
  }
}

for (const retiredWorkspace of [
  marker('sdkwork', '-control', '-plane', '-sdk'),
  marker('sdkwork', '-im', '-admin', '-sdk'),
]) {
  assert.equal(
    existsSync(path.join(sdkRoot, retiredWorkspace)),
    false,
    `retired SDK workspace ${retiredWorkspace} must not exist under sdks/.`,
  );
  assert.doesNotMatch(
    boundaryMaterializerSource,
    new RegExp(retiredWorkspace.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `boundary materializer must not read retired SDK workspace ${retiredWorkspace}.`,
  );
}

for (const marker of [
  "'typescript'",
  "'flutter'",
  "'rust'",
  "'java'",
  "'csharp'",
  "'swift'",
  "'kotlin'",
  "'go'",
  "'python'",
]) {
  assert.match(
    sharedFamilySource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `shared app/backend SDK family must support official language ${marker}.`,
  );
}

for (const marker of [
  "'--type'",
  'config.sdkType',
  "'--fixed-sdk-version'",
  "'--standard-profile'",
  "'sdkwork-v3'",
  'config.primaryClient',
  'custom/build-runtime.mjs',
  'dist/index.js',
  "'.java'",
  'forbiddenGeneratedAuthSurfaceText',
  'set_api_key',
  'SetApiKey',
  'API_KEY_HEADER',
  'defaultApiKeyHeader',
  'AuthToken',
  'AccessToken',
  'application/problem+json',
  'ProblemDetail',
  'forbiddenPathParts',
  'forbiddenGeneratedText',
  'sdkDependencies',
  'generatedTransportImportPolicy',
  'verifySdkDependencies',
  'forbiddenGeneratedDependencyPackages',
  'specs/component.spec.json',
  'componentSpec.contracts?.sdkDependencies',
]) {
  assert.match(
    sharedFamilySource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `shared app/backend SDK family must enforce ${marker}.`,
  );
}

assert.doesNotMatch(
  sharedFamilySource,
  /--requested-version/,
  'shared app/backend SDK family must not pass --requested-version to the generator version resolver.',
);

for (const marker of [
  'AuthToken',
  'type: \'http\'',
  'scheme: \'bearer\'',
  'AccessToken',
  'type: \'apiKey\'',
  'name: \'Access-Token\'',
  'document.security',
  'ProblemDetail',
  'application/problem+json',
]) {
  assert.match(
    sharedOpenApiStandardSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `sdkwork-v3 OpenAPI standardizer must enforce ${marker}.`,
  );
}

for (const marker of [
  'applyFlutterCompatibilityTransforms',
  'primitiveComponentSchemaNames',
  'isPrimitiveComponentSchema',
  'inlinePrimitiveComponentRefs',
  'describePrimitiveRefExpansion',
]) {
  assert.match(
    sharedOpenApiSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `shared OpenAPI source helpers must own Flutter primitive-ref compatibility marker ${marker}.`,
  );
}

for (const [label, source] of [
  ['IM prepare-openapi-source', imPrepareSource],
  ['app prepare-openapi-source', appPrepareSource],
  ['boundary materializer', boundaryMaterializerSource],
]) {
  assert.match(
    source,
    /applyFlutterCompatibilityTransforms/,
    `${label} must reuse the shared Flutter primitive-ref compatibility transform.`,
  );
}

for (const marker of [
  'appbaseAppAuthorityPath',
  'appbaseBackendAuthorityPath',
  'sdkwork-iam-app-sdk',
  'sdkwork-iam-backend-sdk',
  'appbaseAppRouteSet',
  'appbaseBackendRouteSet',
  'dependencyAppRouteSet',
  'dependencyBackendRouteSet',
]) {
  assert.match(
    boundaryMaterializerSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `boundary materializer must include dependency authority exclusion marker ${marker}.`,
  );
}

for (const [label, document, prefix] of [
  ['IM authority', imAuthority, '/im/v3/api'],
  ['IM derived', imDerived, '/im/v3/api'],
  ['IM Flutter derived', imFlutterDerived, '/im/v3/api'],
]) {
  assertAllPathsUsePrefix(label, document, prefix);
  assertNoPathsUsePrefix(label, document, '/app/v3/api');
  assertNoPathsUsePrefix(label, document, '/backend/v3/api');
  assertDocumentHasOnlyImStandardRoutes(label, document, prefix);
  assertSchemasArePathReachable(label, document);
}

assertNoActiveAppBusinessSdkSurfaceInImFamily();

for (const [label, document, prefix] of [
  ['app authority', appAuthority, '/app/v3/api'],
  ['app derived', appDerived, '/app/v3/api'],
  ['app Flutter derived', appFlutterDerived, '/app/v3/api'],
]) {
  const description = String(document.info?.description ?? '');
  assert.match(
    description,
    /Sdkwork IM-owned/i,
    `${label} info.description must state that the generated input is Sdkwork IM-owned.`,
  );
  assert.match(
    description,
    /owner-only/i,
    `${label} info.description must state that the generated input is owner-only.`,
  );
  assert.match(
    description,
    /sdkDependencies/i,
    `${label} info.description must state that dependency capabilities are consumed through sdkDependencies.`,
  );
  assert.ok(
    !/appbase API/i.test(description),
    `${label} info.description must not describe appbase APIs as part of the sdkwork-im SDK input.`,
  );
  assertAllPathsUsePrefix(label, document, prefix);
  assertNoPathsUsePrefix(label, document, '/im/v3/api');
  assertNoPathsUsePrefix(label, document, '/backend/v3/api');
  assertDocumentHasNoImStandardRoutes(label, document, prefix);
  assertDocumentHasNoAppbaseOwnedAppRoutes(label, document, prefix);
  for (const pathKey of pathKeys(document)) {
    assert.ok(
      !new Set(['admin', 'audit', 'control', 'ops']).has(firstRouteGroup(pathKey, prefix)),
      `app-api path ${pathKey} is a management route and belongs in backend-api.`,
    );
  }
  assertSchemasArePathReachable(label, document);
}

assertNoSemanticOverlap(
  'IM authority',
  imAuthority,
  '/im/v3/api',
  'app authority',
  appAuthority,
  '/app/v3/api',
);
assert.ok(
  !pathKeys(appFlutterDerived).some((pathKey) => pathKey.endsWith('/realtime/ws')),
  'app Flutter sdkgen input must remove websocket upgrade routes.',
);

assertAllPathsUsePrefix('backend authority', backendAuthority, '/backend/v3/api');
assertNoPathsUsePrefix('backend authority', backendAuthority, '/im/v3/api');
assertNoPathsUsePrefix('backend authority', backendAuthority, '/app/v3/api');
assertSchemasArePathReachable('backend authority', backendAuthority);
assertNoSemanticOverlap(
  'appbase backend authority',
  appbaseBackendAuthority,
  '/backend/v3/api',
  'backend authority',
  backendAuthority,
  '/backend/v3/api',
);
for (const pathKey of pathKeys(backendAuthority)) {
  const group = firstRouteGroup(pathKey, '/backend/v3/api');
  assert.ok(
    group === 'automation' || new Set(['admin', 'audit', 'control', 'ops']).has(group),
    `backend-api path ${pathKey} must be management/admin/control/operator scoped.`,
  );
}
for (const [label, document] of [
  ['IM authority', imAuthority],
  ['app authority', appAuthority],
  ['backend authority', backendAuthority],
]) {
  for (const { pathKey, method, operation } of operationEntries(document)) {
    assert.ok(
      !String(operation?.operationId ?? '').startsWith('admin.'),
      `${label} ${method.toUpperCase()} ${pathKey} must not expose a retired admin SDK operation namespace.`,
    );
    assert.ok(
      !String(operation?.operationId ?? '').startsWith('controlPlane.'),
      `${label} ${method.toUpperCase()} ${pathKey} must not expose a retired control-plane SDK operation namespace.`,
    );
  }
}

assert.match(appConfigSource, /sdkType:\s*'app'/, 'app SDK family config must use app sdkType.');
assert.match(appConfigSource, /sdkTarget:\s*'app'/, 'app SDK family config must use app sdkTarget.');
assert.match(appConfigSource, /legacyClient:\s*'SdkworkAppClient'/, 'app SDK family must keep SdkworkAppClient only as a compatibility alias.');
assert.match(appConfigSource, /primaryClient:\s*'SdkworkImAppClient'/, 'app SDK family must verify product-scoped SdkworkImAppClient.');
assert.match(
  appConfigSource,
  /generatedApiLabel:\s*'Sdkwork IM app-development API'/,
  'app SDK family config must label generated app-development transport packages.',
);
assert.match(appConfigSource, /apiPrefix:\s*'\/app\/v3\/api'/, 'app SDK family must target /app/v3/api.');
assert.match(appConfigSource, /schemaUrl:\s*'\/app\/v3\/openapi\.json'/, 'app SDK family must target /app/v3/openapi.json.');
for (const marker of [
  'sdkDependencies',
  'sdkwork-iam-app-sdk',
  'appbase-identity-and-session-capability',
  'sdkwork-im-sdk',
  'standardized-im-capability',
  'sdkwork-rtc-sdk',
  'provider-standard-rtc-runtime',
  'consumer-sdk',
  'generatedTransportImportPolicy',
  'forbidden',
  '@sdkwork/iam-app-sdk',
  '@sdkwork/im-sdk',
  '@sdkwork/rtc-sdk',
  'sdkwork-iam-app-sdk',
  'im_sdk',
  'rtc_sdk',
]) {
  assert.match(
    appConfigSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `app SDK config must declare dependency contract marker ${marker}.`,
  );
  assert.match(
    appReadmeSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `app SDK README must document dependency contract marker ${marker}.`,
  );
}
assert.deepEqual(
  appAssembly.sdkDependencies?.map((dependency) => dependency.workspace).sort(),
  ['sdkwork-aiot-app-sdk', 'sdkwork-iam-app-sdk', 'sdkwork-im-sdk', 'sdkwork-rtc-sdk'],
  'app SDK assembly must declare appbase, AIoT, IM, and RTC SDK dependencies.',
);
assert.deepEqual(
  appComponentSpec.contracts?.sdkDependencies?.map((dependency) => dependency.workspace).sort(),
  ['sdkwork-aiot-app-sdk', 'sdkwork-iam-app-sdk', 'sdkwork-im-sdk', 'sdkwork-rtc-sdk'],
  'app SDK component spec must declare appbase, AIoT, IM, and RTC SDK dependencies.',
);
for (const dependency of appAssembly.sdkDependencies ?? []) {
  assert.equal(dependency.required, true, `${dependency.workspace} dependency must be required.`);
  assert.equal(dependency.dependencyMode, 'consumer-sdk', `${dependency.workspace} dependency must use consumer-sdk mode.`);
  assert.equal(
    dependency.generatedTransportImportPolicy,
    'forbidden',
    `${dependency.workspace} dependency must be forbidden in generated app transport.`,
  );
}
assert.deepEqual(
  appComponentSpec.contracts?.sdkDependencies,
  appAssembly.sdkDependencies,
  'app SDK component spec sdkDependencies must match .sdkwork-assembly.json.',
);
for (const marker of [
  'sdkDependencies',
  'sdkwork-iam-app-sdk',
  'sdkwork-aiot-app-sdk',
  'sdkwork-im-sdk',
  'sdkwork-rtc-sdk',
  'consumer-sdk',
  'generatedTransportImportPolicy',
  'forbidden',
]) {
  assert.match(
    appComponentSpecSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `app SDK component specs README must document dependency contract marker ${marker}.`,
  );
}
assertGeneratedTransportDoesNotImportSdkDependencies(
  'app SDK',
  'sdkwork-im-app-sdk',
  appAssembly.sdkDependencies.flatMap((dependency) => Object.values(dependency.packageByLanguage ?? {})),
);
assertFlutterDerivedExpandsPrimitiveComponentRefs('app SDK Flutter derived OpenAPI', appFlutterDerived);
for (const appRequiredPath of [
  '/app/v3/api/portal/access',
  '/app/v3/api/notifications/requests',
  '/app/v3/api/automation/executions',
  '/app/v3/api/media/provider_health',
  '/app/v3/api/principal/profiles/provider_health',
]) {
  assert.match(
    appConfigSource,
    new RegExp(appRequiredPath.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `app SDK config must require non-management API path ${appRequiredPath}.`,
  );
}
for (const imStandardPath of [
  '/app/v3/api/device/sessions/resume',
  '/app/v3/api/devices/{deviceId}/twin',
  '/app/v3/api/iot/devices/{deviceId}/twin',
  '/app/v3/api/iot/devices/{deviceId}/commands',
  '/app/v3/api/iot/devices/{deviceId}/events',
  '/app/v3/api/iot/protocol/uplink',
  '/app/v3/api/chat/conversations',
  '/app/v3/api/chat/messages/{messageId}/edit',
  '/app/v3/api/social/friend_requests',
  '/app/v3/api/media/uploads',
  marker('/app/v3/api', '/calls/sessions'),
  marker('/app/v3/api', '/rtc/sessions'),
  marker('/app/v3/api', '/rtc/provider_health'),
  marker('/app/v3/api', '/rtc/provider_callbacks'),
  '/app/v3/api/streams',
]) {
  assert.doesNotMatch(
    appConfigSource,
    new RegExp(imStandardPath.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `app SDK config must not require IM standard API path ${imStandardPath}.`,
  );
}
for (const appbaseOwnedPath of appbaseOwnedAppRoutes.map((route) => `/app/v3/api/${route.replaceAll('{}', '[^/]+')}`)) {
  assert.doesNotMatch(
    appConfigSource,
    new RegExp(appbaseOwnedPath.replace(/[.*+?^${}()|[\]\\]/g, '\\$&').replaceAll('\\[\\^/\\]\\+', '[^/]+')),
    `app SDK config must not require sdkwork-appbase-owned API path ${appbaseOwnedPath}.`,
  );
}
assert.doesNotMatch(
  appConfigSource,
  forbiddenPattern(
    marker('chat', '-runtime'),
    marker('device', '-sessions'),
    marker('/api', '/v1'),
    marker('/auth', '/login'),
    marker('/auth', '/me'),
    marker('/portal', '/auth'),
  ),
  'app SDK config must not keep legacy path debt.',
);
assert.doesNotMatch(appConfigSource, /SdkworkBackendClient/, 'app SDK config must reject backend client bleed-through.');
for (const [label, source] of [
  ['app TypeScript source sdk', appTypeScriptSrcSdkSource],
  ['app TypeScript source index', appTypeScriptSrcIndexSource],
  ['app TypeScript dist sdk declarations', appTypeScriptDistSdkSource],
  ['app TypeScript dist index declarations', appTypeScriptDistIndexSource],
  ['app TypeScript dist runtime bundle', appTypeScriptDistRuntimeSource],
  ['app Flutter generated client', appFlutterClientSource],
  ['app Rust generated client', appRustClientSource],
  ['app Rust crate root', appRustLibSource],
]) {
  assert.doesNotMatch(
    source,
    /oauthAuthorization|OAuthAuthorization|deviceAuthorization|DeviceAuthorization|deviceAuthorizations|DeviceAuthorizations|passwordReset|PasswordReset|verificationCode|VerificationCode|qrAuth|QrAuth|IamUser|IamOrganization|IamDepartment|IamPosition|IamRoleBinding|AuthSession|CreateAuthSession|RefreshAuthSession|UpdateCurrentSession/,
    `${label} must not regenerate sdkwork-appbase-owned auth, IAM, session, verification, or OAuth device authorization surface.`,
  );
}
for (const [label, source] of [
  ['app TypeScript source sdk', appTypeScriptSrcSdkSource],
  ['app TypeScript source index', appTypeScriptSrcIndexSource],
  ['app TypeScript dist sdk declarations', appTypeScriptDistSdkSource],
  ['app TypeScript dist index declarations', appTypeScriptDistIndexSource],
  ['app TypeScript dist runtime bundle', appTypeScriptDistRuntimeSource],
]) {
  assert.match(source, /SdkworkImAppClient/, `${label} must publish the product-scoped SdkworkImAppClient.`);
  assert.match(source, /SdkworkAppClient/, `${label} may publish SdkworkAppClient only as a compatibility alias.`);
}
for (const [label, source] of [
  ['app TypeScript source sdk', appTypeScriptSrcSdkSource],
  ['app TypeScript source index', appTypeScriptSrcIndexSource],
  ['app TypeScript dist sdk declarations', appTypeScriptDistSdkSource],
  ['app TypeScript dist index declarations', appTypeScriptDistIndexSource],
]) {
  assert.doesNotMatch(
    source,
    /SdkworkImAppClient\s+as\s+SdkworkImAppClient|SdkworkImAppClient,\s*SdkworkImAppClient/,
    `${label} must not publish a self-alias or duplicate product client export.`,
  );
}
assert.match(
  appTypeScriptSrcSdkSource,
  /export \{ SdkworkImAppClient as SdkworkAppClient \};/,
  'app TypeScript source SDK must publish SdkworkAppClient as a compatibility alias.',
);
assert.match(
  appTypeScriptSrcIndexSource,
  /export \{ SdkworkImAppClient, SdkworkAppClient, createClient \} from '\.\/sdk';/,
  'app TypeScript source index must re-export the primary app client and compatibility alias once.',
);
assert.doesNotMatch(
  appTypeScriptDistSdkSource,
  /export declare class SdkworkAppClient/,
  'app TypeScript dist declarations must not publish SdkworkAppClient as the primary class.',
);
assert.doesNotMatch(
  appTypeScriptDistRuntimeSource,
  /class SdkworkAppClient\b/,
  'app TypeScript dist runtime must not publish SdkworkAppClient as the primary runtime class.',
);
assert.match(
  appFlutterClientSource,
  /class SdkworkImAppClient\b/,
  'app Flutter generated client must publish the product-scoped SdkworkImAppClient.',
);
assert.match(
  appFlutterClientSource,
  /typedef SdkworkAppClient = SdkworkImAppClient;/,
  'app Flutter generated client must keep SdkworkAppClient only as a compatibility alias.',
);
assert.doesNotMatch(
  appFlutterClientSource,
  /class SdkworkAppClient\b|setApiKey|apiKeyHeader|apiKeyAsBearer/,
  'app Flutter generated client must not expose the legacy class or API-key auth surface.',
);
assert.match(
  appRustClientSource,
  /pub struct SdkworkImAppClient\b/,
  'app Rust generated client must publish the product-scoped SdkworkImAppClient.',
);
assert.match(
  appRustClientSource,
  /pub type SdkworkAppClient\s*=\s*SdkworkImAppClient;/,
  'app Rust generated client must keep SdkworkAppClient only as a compatibility alias.',
);
assert.doesNotMatch(
  appRustClientSource,
  /pub type SdkworkImAppClient\s*=\s*SdkworkImAppClient;/,
  'app Rust generated client must not publish a recursive primary client alias.',
);
assert.match(
  appRustLibSource,
  /pub use client::\{\s*SdkworkAppClient,\s*SdkworkImAppClient\s*\};|pub use client::\{\s*SdkworkImAppClient,\s*SdkworkAppClient\s*\};/,
  'app Rust generated crate root must re-export both the primary client and compatibility alias.',
);

assert.match(backendConfigSource, /sdkType:\s*'backend'/, 'backend SDK family config must use backend sdkType.');
assert.match(backendConfigSource, /sdkTarget:\s*'backend'/, 'backend SDK family config must use backend sdkTarget.');
assert.match(backendConfigSource, /legacyClient:\s*'SdkworkBackendClient'/, 'backend SDK family must keep SdkworkBackendClient only as a compatibility alias.');
assert.match(backendConfigSource, /primaryClient:\s*'SdkworkImBackendClient'/, 'backend SDK family must verify product-scoped SdkworkImBackendClient.');
assert.match(
  backendConfigSource,
  /generatedApiLabel:\s*'Sdkwork IM backend\/operator API'/,
  'backend SDK family config must label generated backend/operator transport packages.',
);
assert.match(backendConfigSource, /apiPrefix:\s*'\/backend\/v3\/api'/, 'backend SDK family must target /backend/v3/api.');
assert.match(backendConfigSource, /schemaUrl:\s*'\/backend\/v3\/openapi\.json'/, 'backend SDK family must target /backend/v3/openapi.json.');
for (const marker of [
  'sdkDependencies',
  'sdkwork-iam-backend-sdk',
  'appbase-backend-management-capability',
  'consumer-sdk',
  'generatedTransportImportPolicy',
  'forbidden',
  '@sdkwork/iam-backend-sdk',
  'sdkwork_iam_backend_sdk',
  'SDKWork.Appbase.BackendSdk',
]) {
  assert.match(
    backendConfigSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `backend SDK config must declare dependency contract marker ${marker}.`,
  );
  assert.match(
    backendReadmeSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `backend SDK README must document dependency contract marker ${marker}.`,
  );
}
assert.deepEqual(
  backendAssembly.sdkDependencies?.map((dependency) => dependency.workspace).sort(),
  ['sdkwork-iam-backend-sdk'],
  'backend SDK assembly must declare appbase backend SDK dependency.',
);
assert.deepEqual(
  backendComponentSpec.contracts?.sdkDependencies?.map((dependency) => dependency.workspace).sort(),
  ['sdkwork-iam-backend-sdk'],
  'backend SDK component spec must declare appbase backend SDK dependency.',
);
for (const dependency of backendAssembly.sdkDependencies ?? []) {
  assert.equal(dependency.required, true, `${dependency.workspace} dependency must be required.`);
  assert.equal(dependency.dependencyMode, 'consumer-sdk', `${dependency.workspace} dependency must use consumer-sdk mode.`);
  assert.equal(
    dependency.generatedTransportImportPolicy,
    'forbidden',
    `${dependency.workspace} dependency must be forbidden in generated backend transport.`,
  );
}
assert.deepEqual(
  backendComponentSpec.contracts?.sdkDependencies,
  backendAssembly.sdkDependencies,
  'backend SDK component spec sdkDependencies must match .sdkwork-assembly.json.',
);
for (const marker of [
  'sdkDependencies',
  'sdkwork-iam-backend-sdk',
  'consumer-sdk',
  'generatedTransportImportPolicy',
  'forbidden',
]) {
  assert.match(
    backendComponentSpecSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `backend SDK component specs README must document dependency contract marker ${marker}.`,
  );
}
assertGeneratedTransportDoesNotImportSdkDependencies(
  'backend SDK',
  'sdkwork-im-backend-sdk',
  backendAssembly.sdkDependencies.flatMap((dependency) => Object.values(dependency.packageByLanguage ?? {})),
);
for (const backendRequiredPath of [
  '/backend/v3/api/ops/health',
  '/backend/v3/api/audit/records',
  '/backend/v3/api/automation/governance',
  '/backend/v3/api/control/protocol_registry',
  '/backend/v3/api/admin/api_keys',
]) {
  assert.match(
    backendConfigSource,
    new RegExp(backendRequiredPath.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `backend SDK config must require consolidated backend path ${backendRequiredPath}.`,
  );
}
assert.doesNotMatch(
  backendConfigSource,
  /marker\('\/api', '\/(?:admin|control)'\)/,
  'backend SDK config must not reject admin or control routes; both are backend modules.',
);
assert.doesNotMatch(
  backendConfigSource,
  forbiddenPattern(
    marker('chat', '-runtime'),
    marker('device', '-sessions'),
    marker('/api', '/v1'),
    marker('/auth', '/login'),
    marker('/auth', '/me'),
    marker('/portal', '/auth'),
  ),
  'backend SDK config must not keep legacy path debt.',
);
assert.doesNotMatch(backendConfigSource, /SdkworkAppClient/, 'backend SDK config must reject app client bleed-through.');
for (const [label, source] of [
  ['backend TypeScript source sdk', backendTypeScriptSrcSdkSource],
  ['backend TypeScript source index', backendTypeScriptSrcIndexSource],
  ['backend TypeScript dist sdk declarations', backendTypeScriptDistSdkSource],
  ['backend TypeScript dist index declarations', backendTypeScriptDistIndexSource],
  ['backend TypeScript dist runtime bundle', backendTypeScriptDistRuntimeSource],
]) {
  assert.match(source, /SdkworkImBackendClient/, `${label} must publish the product-scoped SdkworkImBackendClient.`);
  assert.match(source, /SdkworkBackendClient/, `${label} may publish SdkworkBackendClient only as a compatibility alias.`);
}
for (const [label, source] of [
  ['backend TypeScript source sdk', backendTypeScriptSrcSdkSource],
  ['backend TypeScript source index', backendTypeScriptSrcIndexSource],
  ['backend TypeScript dist sdk declarations', backendTypeScriptDistSdkSource],
  ['backend TypeScript dist index declarations', backendTypeScriptDistIndexSource],
]) {
  assert.doesNotMatch(
    source,
    /SdkworkImBackendClient\s+as\s+SdkworkImBackendClient|SdkworkImBackendClient,\s*SdkworkImBackendClient/,
    `${label} must not publish a self-alias or duplicate product client export.`,
  );
}
assert.match(
  backendTypeScriptSrcSdkSource,
  /export \{ SdkworkImBackendClient as SdkworkBackendClient \};/,
  'backend TypeScript source SDK must publish SdkworkBackendClient as a compatibility alias.',
);
assert.match(
  backendTypeScriptSrcIndexSource,
  /export \{ SdkworkImBackendClient, SdkworkBackendClient, createClient \} from '\.\/sdk';/,
  'backend TypeScript source index must re-export the primary backend client and compatibility alias once.',
);
assert.doesNotMatch(
  backendTypeScriptDistSdkSource,
  /export declare class SdkworkBackendClient/,
  'backend TypeScript dist declarations must not publish SdkworkBackendClient as the primary class.',
);
assert.doesNotMatch(
  backendTypeScriptDistRuntimeSource,
  /class SdkworkBackendClient\b/,
  'backend TypeScript dist runtime must not publish SdkworkBackendClient as the primary runtime class.',
);
assert.match(
  backendFlutterClientSource,
  /class SdkworkImBackendClient\b/,
  'backend Flutter generated client must publish the product-scoped SdkworkImBackendClient.',
);
assert.match(
  backendFlutterClientSource,
  /typedef SdkworkBackendClient = SdkworkImBackendClient;/,
  'backend Flutter generated client must keep SdkworkBackendClient only as a compatibility alias.',
);
assert.doesNotMatch(
  backendFlutterClientSource,
  /class SdkworkBackendClient\b|setApiKey|apiKeyHeader|apiKeyAsBearer/,
  'backend Flutter generated client must not expose the legacy class or API-key auth surface.',
);
for (const nonManagementBackendPath of [
  '/backend/v3/api/media/provider_health',
  '/backend/v3/api/iot/protocol/uplink',
  marker('/backend/v3/api', '/rtc/provider_health'),
]) {
  assert.doesNotMatch(
    backendConfigSource,
    new RegExp(nonManagementBackendPath.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `backend SDK config must not keep non-management API path ${nonManagementBackendPath}.`,
  );
}

assert.match(imConfigSource, /sdkType:\s*'im'/, 'IM SDK family config must use im sdkType.');
assert.match(imConfigSource, /sdkTarget:\s*'im'/, 'IM SDK family config must use im sdkTarget.');
assert.match(imConfigSource, /primaryClient:\s*'SdkworkImClient'/, 'IM SDK family must verify SdkworkImClient.');
assert.match(
  imConfigSource,
  /generatedApiLabel:\s*'Sdkwork IM IM standardized development API'/,
  'IM SDK family config must label generated IM standardized development transport packages.',
);
assert.match(imConfigSource, /apiPrefix:\s*'\/im\/v3\/api'/, 'IM SDK family must target /im/v3/api.');
assert.match(imConfigSource, /schemaUrl:\s*'\/im\/v3\/openapi\.json'/, 'IM SDK family must target /im/v3/openapi.json.');
for (const imRequiredPath of [
  '/im/v3/api/chat/conversations',
  '/im/v3/api/chat/rooms',
  '/im/v3/api/chat/messages/{messageId}/edit',
  '/im/v3/api/social/friend_requests',
  '/im/v3/api/calls/sessions',
  '/im/v3/api/streams',
]) {
  assert.match(
    imConfigSource,
    new RegExp(imRequiredPath.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `IM SDK config must require standardized API path ${imRequiredPath}.`,
  );
}
assertFlutterDerivedExpandsPrimitiveComponentRefs('IM SDK Flutter derived OpenAPI', imFlutterDerived);
for (const nonImPath of [
  '/im/v3/api/device/sessions/resume',
  '/im/v3/api/device/sessions/disconnect',
  '/im/v3/api/devices/register',
  '/im/v3/api/devices/{deviceId}/sync_feed',
  '/im/v3/api/portal/access',
  '/im/v3/api/devices/{deviceId}/twin',
  '/im/v3/api/notifications/requests',
  '/im/v3/api/automation/executions',
  '/im/v3/api/media/provider_health',
  '/im/v3/api/media/uploads',
  '/im/v3/api/media/{mediaAssetId}',
  '/im/v3/api/principal/profiles/provider_health',
  '/im/v3/api/iot/protocol/uplink',
  marker('/im/v3/api', '/rtc/sessions'),
]) {
  assert.doesNotMatch(
    imConfigSource,
    new RegExp(nonImPath.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `IM SDK config must not require app-business API path ${nonImPath}.`,
  );
}
assert.doesNotMatch(imConfigSource, /SdkworkAppClient/, 'IM SDK config must reject app client bleed-through.');
assert.doesNotMatch(imConfigSource, /SdkworkBackendClient/, 'IM SDK config must reject backend client bleed-through.');

for (const [label, document] of [
  ['IM authority', imAuthority],
  ['IM derived', imDerived],
  ['IM Flutter derived', imFlutterDerived],
]) {
  const operationIds = operationEntries(document).map(({ operation }) => String(operation?.operationId ?? ''));
  for (const forbiddenOperationId of [
    'device.sessions.resume',
    'device.sessions.disconnect',
    'device.registrations.create',
    'device.syncFeed.retrieve',
  ]) {
    assert.ok(
      !operationIds.includes(forbiddenOperationId),
      `${label} must not expose retired IM device operation ${forbiddenOperationId}.`,
    );
  }
  assert.ok(
    !operationIds.some((operationId) => /device(?:Sessions|\.sessions|\.registrations|\.syncFeed)|DeviceSessions/u.test(operationId)),
    `${label} must not expose an IM device SDK operation namespace.`,
  );
}

for (const [label, source] of [
  ['app generate', appGenerateSource],
  ['backend generate', backendGenerateSource],
  ['IM generate', imGenerateSource],
]) {
  assert.match(source, /runGenerateSdkFamily/, `${label} entrypoint must delegate to shared SDK family generation.`);
  assert.match(source, /sdkFamilyConfig/, `${label} entrypoint must use its local SDK family config.`);
}

assert.match(
  imGenerateSource,
  /materialize-local-openapi-seed\.mjs/,
  'IM generate entrypoint must materialize the local OpenAPI seed before boundary normalization.',
);
assert.match(
  imGenerateSource,
  /sync-openapi-authority-mirror\.mjs/,
  'IM generate entrypoint must sync apis/open-api authority into the SDK mirror before boundary normalization.',
);
assert.match(
  imGenerateSource,
  /materialize-im-v3-openapi-boundaries\.mjs/,
  'IM generate entrypoint must normalize OpenAPI SDK boundaries before generator execution.',
);
assert.match(
  imGenerateSource,
  /assemble-sdk\.mjs/,
  'IM generate entrypoint must assemble layered TypeScript/Flutter workspaces after generator output is verified.',
);
assert.match(
  imGenerateSource,
  /assemble-single-package\.mjs/,
  'IM generate entrypoint must assemble the TypeScript root single-package after TypeScript generator output changes.',
);
assert.match(
  imGenerateSource,
  /shouldAssembleTypeScriptRoot/,
  'IM generate entrypoint must only run TypeScript root single-package assembly when TypeScript is selected or all languages are generated.',
);
assert.doesNotMatch(
  imGenerateSource,
  /normalize-typescript-generated-package-manifest\.mjs/,
  'IM generate entrypoint must not post-process the TypeScript generated package manifest.',
);
assert.doesNotMatch(
  imGenerateSource,
  /normalize-generated-auth-surface/,
  'IM generate entrypoint must not call the retired generated auth surface normalizer.',
);
assert.doesNotMatch(
  sharedFamilySource,
  /function\s+normalizeGeneratedLanguage\b|function\s+normalizeGeneratedTypeScriptAuthSurface\b|function\s+normalize(?:Java|Kotlin|Rust|Go|Python|Flutter|Swift|Csharp)AuthSurface\b/,
  'Shared IM SDK family generation must not retain generated-output post-processing normalizers.',
);
assert.equal(
  imTypeScriptGeneratedPackage.name,
  '@sdkwork/im-sdk-generated',
  'IM TypeScript generated package name must be owned by the generator output.',
);
assert.equal(
  imTypeScriptGeneratedPackage.private,
  true,
  'IM TypeScript generated package manifest must mark generated transport private without Sdkwork IM post-processing.',
);
assert.equal(
  imTypeScriptGeneratedPackage.description,
  'Generator-owned TypeScript transport SDK for sdkwork-im-sdk.',
  'IM TypeScript generated package description must come from the canonical generator.',
);

for (const [label, source] of [
  ['app verify', appVerifySource],
  ['backend verify', backendVerifySource],
  ['IM verify', imVerifySource],
]) {
  assert.match(source, /runVerifySdkFamily/, `${label} entrypoint must delegate to shared SDK family verification.`);
  assert.match(source, /sdkFamilyConfig/, `${label} entrypoint must use its local SDK family config.`);
}

for (const [label, source] of [
  ['app prepare', appPrepareSource],
  ['backend prepare', backendPrepareSource],
  ['im prepare', imPrepareSource],
  ['app refresh', appRefreshSource],
  ['backend refresh', backendRefreshSource],
  ['im refresh', imRefreshSource],
]) {
  assert.match(
    source,
    /applySdkworkV3OpenApiStandard/,
    `${label} must apply the shared sdkwork-v3 OpenAPI standardizer.`,
  );
}

for (const [label, source] of [
  ['im PowerShell generate', imGeneratePowerShellSource],
  ['im shell generate', imGenerateShellSource],
]) {
  assert.match(source, /fixed-sdk-version/, `${label} must use fixed sdk version resolution.`);
  assert.doesNotMatch(source, /requested-version|RequestedVersion/, `${label} must not keep requested-version debt.`);
}

for (const marker of [
  'sdkwork-im-sdk',
  'sdkwork-im-app-sdk',
  'sdkwork-im-backend-sdk',
  'sdkwork-rtc-sdk',
  '/im/v3/api',
  '/app/v3/api',
  '/backend/v3/api',
  'depends on',
]) {
  assert.match(
    sdkWorkspaceIndexSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `SDK workspace index must document ${marker}.`,
  );
}

for (const retiredPublicFamily of retiredSdkMarkers.slice(0, 4)) {
  assert.doesNotMatch(
    sdkWorkspaceIndexSource,
    new RegExp(retiredPublicFamily.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `SDK workspace index must not publish retired SDK family ${retiredPublicFamily}.`,
  );
}

assert.match(
  sdkWorkspaceIndexSource,
  /\/backend\/v3\/api\/control\/\*/,
  'SDK workspace index must document control routes as backend SDK modules.',
);
assert.match(
  sdkWorkspaceIndexSource,
  /\/backend\/v3\/api\/admin\/\*/,
  'SDK workspace index must document admin routes as backend SDK modules.',
);
assert.match(
  rtcReadmeSource,
  /not a route-generated SDK workspace|not an OpenAPI-generated HTTP SDK family|does not[\s\S]*OpenAPI-generated transport problem/i,
  'RTC SDK README must keep RTC independent from OpenAPI-generated HTTP SDK families.',
);

assert.doesNotMatch(
  sdkWorkspaceIndexSource,
  /Generator-ready/,
  'SDK workspace index must be tightened to list generated app/backend SDK languages instead of Generator-ready placeholders.',
);

console.log('im v3 sdk family contract test passed');
