#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';

function read(rootDir, relativePath) {
  return readFileSync(path.join(rootDir, relativePath), 'utf8');
}

function expectIncludes(failures, source, expected, message) {
  if (!source.includes(expected)) {
    failures.push(message);
  }
}

function expectExcludes(failures, source, unexpected, message) {
  if (source.includes(unexpected)) {
    failures.push(message);
  }
}

function marker(...parts) {
  return parts.join('');
}

export function verifySdkSiteDocs(options = {}) {
  const rootDir = options.rootDir || path.resolve(import.meta.dirname, '..', '..', '..');
  const failures = [];

  const docsPackage = JSON.parse(read(rootDir, 'docs/sites/package.json'));
  const vitepressConfigSource = read(rootDir, 'docs/sites/.vitepress/config.mjs');
  const siteHomeSource = read(rootDir, 'docs/sites/index.md');
  const sdkIndexSource = read(rootDir, 'docs/sites/sdk/index.md');
  const appSource = read(rootDir, 'docs/sites/sdk/app-sdk.md');
  const backendSource = read(rootDir, 'docs/sites/sdk/backend-sdk.md');
  const rtcSource = read(rootDir, 'docs/sites/sdk/rtc-sdk.md');
  const languageSupportSource = read(rootDir, 'docs/sites/sdk/language-support.md');
  const gettingStartedIndexSource = read(rootDir, 'docs/sites/getting-started/index.md');
  const capabilitiesSource = read(rootDir, 'docs/sites/features/capabilities.md');
  const cliReferenceSource = read(rootDir, 'docs/sites/reference/cli-and-scripts.md');
  const apiReferenceIndexSource = read(rootDir, 'docs/sites/api-reference/index.md');
  const appApiSource = read(rootDir, 'docs/sites/api-reference/app-api.md');
  const backendApiSource = read(rootDir, 'docs/sites/api-reference/backend-api.md');
  const controlPlaneApiSource = read(rootDir, 'docs/sites/api-reference/control-plane-api.md');
  const releaseCatalog = JSON.parse(
    read(rootDir, 'artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json'),
  );

  for (const [scriptName, taskName] of [
    ['docs:generate', 'generate'],
    ['docs:dev', 'dev'],
    ['docs:build', 'build'],
    ['docs:preview', 'preview'],
    ['docs:verify', 'verify'],
  ]) {
    const scriptValue = String(docsPackage.scripts?.[scriptName] || '');
    if (!scriptValue.includes(`./scripts/run-docs-task.mjs ${taskName}`)) {
      failures.push(`docs/sites/package.json must route ${scriptName} through scripts/run-docs-task.mjs ${taskName}.`);
    }
    if (!scriptValue.includes('npm_node_execpath')) {
      failures.push(`docs/sites/package.json must keep the npm_node_execpath fallback for ${scriptName}.`);
    }
  }

  for (const removedPage of [
    marker('docs/sites/sdk', '/control', '-plane', '-sdk.md'),
    marker('docs/sites/sdk', '/control', '-plane', '-typescript', '-sdk.md'),
    marker('docs/sites/sdk', '/control', '-plane', '-flutter', '-sdk.md'),
    marker('docs/sites/sdk', '/im', '-admin', '-sdk.md'),
    'docs/sites/sdk/management-sdk.md',
  ]) {
    if (existsSync(path.join(rootDir, removedPage))) {
      failures.push(`${removedPage} must not exist as a current public SDK page.`);
    }
  }

  for (const requiredEntry of [
    '{ text: "IM Standard SDK", link: "/sdk/typescript-sdk" }',
    '{ text: "App API SDK", link: "/sdk/app-sdk" }',
    '{ text: "Backend SDK", link: "/sdk/backend-sdk" }',
    '{ text: "RTC SDK", link: "/sdk/rtc-sdk" }',
  ]) {
    expectIncludes(
      failures,
      vitepressConfigSource,
      requiredEntry,
      `docs/sites/.vitepress/config.mjs must include ${requiredEntry}.`,
    );
  }

  const currentDocs = [
    ['docs/sites/index.md', siteHomeSource],
    ['docs/sites/sdk/index.md', sdkIndexSource],
    ['docs/sites/sdk/app-sdk.md', appSource],
    ['docs/sites/sdk/backend-sdk.md', backendSource],
    ['docs/sites/sdk/rtc-sdk.md', rtcSource],
    ['docs/sites/sdk/language-support.md', languageSupportSource],
    ['docs/sites/getting-started/index.md', gettingStartedIndexSource],
    ['docs/sites/features/capabilities.md', capabilitiesSource],
    ['docs/sites/reference/cli-and-scripts.md', cliReferenceSource],
    ['docs/sites/api-reference/index.md', apiReferenceIndexSource],
    ['docs/sites/api-reference/app-api.md', appApiSource],
    ['docs/sites/api-reference/backend-api.md', backendApiSource],
    ['docs/sites/api-reference/control-plane-api.md', controlPlaneApiSource],
  ];

  const retiredSdkMarkers = [
    marker('sdkwork', '-control', '-plane', '-sdk'),
    marker('sdkwork', '-im', '-admin', '-sdk'),
    marker('@sdkwork', '/control', '-plane', '-sdk'),
    marker('@sdkwork', '/im', '-admin', '-sdk'),
    marker('/sdk', '/control', '-plane', '-sdk'),
    marker('/sdk', '/control', '-plane', '-typescript', '-sdk'),
    marker('/sdk', '/control', '-plane', '-flutter', '-sdk'),
    marker('/sdk', '/im', '-admin', '-sdk'),
    marker('Control', '-Plane', ' SDK'),
    marker('IM', ' Admin', ' SDK'),
    marker('control', '_plane', '_sdk'),
    marker('im', '_admin', '_sdk'),
    'sdkwork-sdkwork-im-sdk-management',
    '/sdk/management-sdk',
  ];

  for (const [label, source] of currentDocs) {
    for (const forbidden of retiredSdkMarkers) {
      expectExcludes(failures, source, forbidden, `${label} must not publish retired SDK marker ${forbidden}.`);
    }
  }

  for (const requiredEntry of [
    '`sdkwork-im-sdk`',
    '`sdkwork-im-app-sdk`',
    '`sdkwork-im-backend-sdk`',
    '`sdkwork-rtc-sdk`',
    '/im/v3/api',
    '/app/v3/api',
    '/backend/v3/api',
    '/backend/v3/api/control/*',
    '/backend/v3/api/admin/*',
    '[Backend SDK](/sdk/backend-sdk)',
    '[RTC SDK](/sdk/rtc-sdk)',
  ]) {
    expectIncludes(
      failures,
      sdkIndexSource,
      requiredEntry,
      `docs/sites/sdk/index.md must document ${requiredEntry}.`,
    );
  }

  for (const requiredEntry of [
    'sdkwork-im-app-sdk',
    '/app/v3/api',
    'SdkworkAppClient',
    'must not contain backend, admin, or control routes',
  ]) {
    expectIncludes(failures, appSource, requiredEntry, `docs/sites/sdk/app-sdk.md must include ${requiredEntry}.`);
  }

  for (const requiredEntry of [
    'sdkwork-im-backend-sdk',
    '/backend/v3/api',
    'SdkworkBackendClient',
    '/backend/v3/api/control/*',
    '/backend/v3/api/admin/*',
    'Do not introduce a new admin SDK family',
  ]) {
    expectIncludes(
      failures,
      backendSource,
      requiredEntry,
      `docs/sites/sdk/backend-sdk.md must include ${requiredEntry}.`,
    );
  }

  for (const requiredEntry of [
    'sdkwork-rtc-sdk',
    'not generated from OpenAPI',
    'provider package',
    'native driver',
    'node ../../../../sdkwork-rtc\\sdks\\sdkwork-rtc-sdk\\bin\\verify-sdk.mjs',
  ]) {
    expectIncludes(failures, rtcSource, requiredEntry, `docs/sites/sdk/rtc-sdk.md must include ${requiredEntry}.`);
  }

  for (const requiredEntry of [
    '`sdkwork-im-sdk` maps to `/im/v3/api`',
    '`sdkwork-im-app-sdk` maps to `/app/v3/api`',
    '`sdkwork-im-backend-sdk` maps to `/backend/v3/api`',
    '`sdkwork-rtc-sdk` maps to provider-runtime integration',
    '/backend/v3/api/control/*',
    '/backend/v3/api/admin/*',
  ]) {
    expectIncludes(
      failures,
      apiReferenceIndexSource,
      requiredEntry,
      `docs/sites/api-reference/index.md must include ${requiredEntry}.`,
    );
  }

  for (const requiredEntry of [
    'sdkwork-im-backend-sdk',
    'control modules',
    'sdks/sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.openapi.yaml',
  ]) {
    expectIncludes(
      failures,
      controlPlaneApiSource,
      requiredEntry,
      `docs/sites/api-reference/control-plane-api.md must include ${requiredEntry}.`,
    );
  }

  for (const requiredEntry of [
    'sdkwork-im-app-sdk',
    '/app/v3/api/*',
  ]) {
    expectIncludes(
      failures,
      appApiSource,
      requiredEntry,
      `docs/sites/api-reference/app-api.md must include ${requiredEntry}.`,
    );
  }
  for (const forbiddenEntry of ['Device Twin', 'IoT Protocol']) {
    expectExcludes(
      failures,
      appApiSource,
      forbiddenEntry,
      `docs/sites/api-reference/app-api.md must not include retired AIoT-owned domain ${forbiddenEntry}.`,
    );
  }

  for (const requiredEntry of [
    'sdkwork-im-backend-sdk',
    '/backend/v3/api/*',
    '/backend/v3/api/control/*',
    '/backend/v3/api/admin/*',
  ]) {
    expectIncludes(
      failures,
      backendApiSource,
      requiredEntry,
      `docs/sites/api-reference/backend-api.md must include ${requiredEntry}.`,
    );
  }

  for (const requiredEntry of [
    'materialize-im-v3-openapi-boundaries.mjs',
    'sdks\\sdkwork-im-app-sdk\\bin\\verify-sdk.mjs',
    'sdks\\sdkwork-im-backend-sdk\\bin\\verify-sdk.mjs',
    'sdks\\sdkwork-rtc-sdk\\bin\\verify-sdk.mjs',
  ]) {
    expectIncludes(
      failures,
      cliReferenceSource,
      requiredEntry,
      `docs/sites/reference/cli-and-scripts.md must include ${requiredEntry}.`,
    );
  }

  const releaseArtifacts = Array.isArray(releaseCatalog.sdkArtifacts) ? releaseCatalog.sdkArtifacts : [];
  const expectedAudiences = new Set(['im', 'app', 'backend', 'rtc']);
  const actualAudiences = new Set(releaseArtifacts.map((artifact) => artifact.audience));
  for (const expectedAudience of expectedAudiences) {
    if (!actualAudiences.has(expectedAudience)) {
      failures.push(`sdk-release-catalog.json must include ${expectedAudience} SDK artifacts.`);
    }
  }
  for (const retiredAudience of ['admin', 'im-admin']) {
    if (actualAudiences.has(retiredAudience)) {
      failures.push(`sdk-release-catalog.json must not include retired ${retiredAudience} artifacts.`);
    }
  }
  for (const artifact of releaseArtifacts) {
    for (const retiredMarker of [
      marker('sdkwork', '-control', '-plane', '-sdk'),
      marker('sdkwork', '-im', '-admin', '-sdk'),
    ]) {
      if (JSON.stringify(artifact).includes(retiredMarker)) {
        failures.push(`sdk-release-catalog.json artifact ${artifact.id} must not reference ${retiredMarker}.`);
      }
    }
  }

  return failures;
}

const isCli = process.argv[1] && path.resolve(process.argv[1]) === import.meta.filename;

if (isCli) {
  const failures = verifySdkSiteDocs();
  if (failures.length > 0) {
    console.error('[docs/sites/sdk] SDK site docs verification failed:');
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log('[docs/sites/sdk] SDK site docs verification passed.');
}
