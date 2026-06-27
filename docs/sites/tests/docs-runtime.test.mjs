import assert from 'node:assert/strict';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

import * as viteRuntimeLib from '../../../scripts/dev/vite-runtime-lib.mjs';

const docsRoot = path.resolve(import.meta.dirname, '..');
const currentWorkspaceRoot = path.resolve(docsRoot, '..', '..');
const removedRustCompatClientPattern = new RegExp(['Sdkwork', 'Im', 'Client'].join(''));
const retiredSdkPattern = (...parts) => new RegExp(parts.join(''));

function listMarkdownFiles(root) {
  const files = [];
  if (!existsSync(root)) {
    return files;
  }
  for (const entry of readdirSync(root)) {
    const fullPath = path.join(root, entry);
    const stat = statSync(fullPath);
    if (stat.isDirectory()) {
      files.push(...listMarkdownFiles(fullPath));
    } else if (entry.endsWith('.md')) {
      files.push(fullPath);
    }
  }
  return files;
}

function resolveCanonicalDocsRoot(workspaceRoot) {
  const isWorktreeCheckout = path.basename(path.dirname(workspaceRoot)) === '.worktrees';
  const canonicalRepoRoot = isWorktreeCheckout
    ? path.resolve(workspaceRoot, '..', '..')
    : workspaceRoot;

  return path.join(canonicalRepoRoot, 'docs', 'sites');
}

test('workspace donor roots include the canonical docs site when local docs node_modules are empty', () => {
  const donorRoots = viteRuntimeLib.resolveWorkspaceDonorRoots(docsRoot);
  const canonicalDocsRoot = resolveCanonicalDocsRoot(currentWorkspaceRoot);
  const canonicalIsCurrentDocsRoot = canonicalDocsRoot === docsRoot;

  if (canonicalIsCurrentDocsRoot) {
    assert.equal(
      donorRoots.includes(canonicalDocsRoot),
      false,
      'resolveWorkspaceDonorRoots should not duplicate the current docs root as its own donor',
    );
    return;
  }

  assert.equal(
    existsSync(path.join(canonicalDocsRoot, 'node_modules', 'vitepress', 'package.json')),
    true,
    'test setup should include a canonical docs site donor with vitepress installed',
  );
  assert.equal(
    donorRoots.includes(canonicalDocsRoot),
    true,
    'docs/sites should discover the canonical docs site as a donor root',
  );
});

test('api reference docs use SDKWork dual-token and AppContext security terminology', () => {
  const apiReferenceRoot = path.join(docsRoot, 'api-reference');
  const forbiddenPatterns = [
    /Security<\/strong><span>Bearer token<\/span>/,
    /Shared bearer/i,
    /trusted-header/i,
    /trusted headers/i,
    /missing_authorization/,
    /invalid_token/,
  ];

  for (const filePath of listMarkdownFiles(apiReferenceRoot)) {
    const relativePath = path.relative(currentWorkspaceRoot, filePath);
    const source = readFileSync(filePath, 'utf8');
    for (const forbidden of forbiddenPatterns) {
      assert.doesNotMatch(
        source,
        forbidden,
        `${relativePath} must describe SDKWork dual-token validation plus AppContext projection, not legacy bearer/trusted-header auth`,
      );
    }
  }
});

test('api reference generation scripts preserve AppContext security terminology', () => {
  const scriptPaths = [
    path.join(docsRoot, 'scripts', 'generate-operation-pages.mjs'),
    path.join(docsRoot, 'scripts', 'standardize-api-docs.mjs'),
    path.join(docsRoot, 'scripts', 'verify-api-docs.mjs'),
  ];
  const forbiddenPatterns = [
    /Bearer token/,
    /Shared bearer/i,
    /trusted-header/i,
    /trusted headers/i,
    /missing_authorization/,
    /invalid_token/,
    /Authentication failed\./,
  ];

  for (const filePath of scriptPaths) {
    const relativePath = path.relative(currentWorkspaceRoot, filePath);
    const source = readFileSync(filePath, 'utf8');
    for (const forbidden of forbiddenPatterns) {
      assert.doesNotMatch(
        source,
        forbidden,
        `${relativePath} must generate SDKWork dual-token and AppContext terminology`,
      );
    }
  }
});

test('sdk docs preserve SDKWork credential and Sdkwork IM client-route terminology', () => {
  const checkedRoots = [
    path.join(docsRoot, 'sdk'),
    path.join(docsRoot, 'api-reference'),
    path.join(docsRoot, 'reference'),
    path.join(currentWorkspaceRoot, 'sdks', 'sdkwork-im-sdk', 'docs'),
  ];
  const forbiddenPatterns = [
    /bearer-token/i,
    /appbase bearer/i,
    /appbase-issued bearer/i,
    /\bquery bearer\b/i,
    /\bheader bearer\b/i,
    /Public auth is bearer-token only/i,
    /\bsession resume\b/i,
    /\.session\(\)/,
    /ResumeSessionRequest/,
  ];

  for (const checkedRoot of checkedRoots) {
    for (const filePath of listMarkdownFiles(checkedRoot)) {
      const relativePath = path.relative(currentWorkspaceRoot, filePath);
      const source = readFileSync(filePath, 'utf8');
      for (const forbidden of forbiddenPatterns) {
        assert.doesNotMatch(
          source,
          forbidden,
          `${relativePath} must use SDKWork appbase credential wording and Sdkwork IM client-route naming`,
        );
      }
    }
  }
});

test('media API reference documents sdkwork-drive delegation and standardized MediaResource usage', () => {
  const checkedFiles = [
    path.join(docsRoot, 'api-reference', 'im', 'media.md'),
    path.join(docsRoot, 'api-reference', 'im', 'messages.md'),
  ];
  const forbiddenPatterns = [
    /mediaAssetId/,
    /MediaAsset/,
    /MediaUploadMutationResponse/,
    /MediaUploadSession/,
    /CreateUploadRequest/,
    /CompleteUploadRequest/,
    /bucketId/,
    /objectKey/,
    /storageProvider/,
    /\/im\/v3\/api\/media\/uploads/,
    /download_url/,
    /presigned upload/i,
    /object_storage/,
  ];

  for (const filePath of checkedFiles) {
    const relativePath = path.relative(currentWorkspaceRoot, filePath);
    const source = readFileSync(filePath, 'utf8');

    for (const forbidden of forbiddenPatterns) {
      assert.doesNotMatch(
        source,
        forbidden,
        `${relativePath} must document Drive-backed media references instead of the removed IM upload lifecycle`,
      );
    }

    assert.match(source, /sdkwork-drive/, `${relativePath} must name sdkwork-drive as the file authority`);
    assert.match(
      source,
      /drive:\/\/spaces\/\{spaceId\}\/nodes\/\{nodeId\}/,
      `${relativePath} must show the canonical Drive URI shape`,
    );
    assert.match(source, /DriveReference/, `${relativePath} must document DriveReference`);
    assert.match(source, /ContentPart\.drive/, `${relativePath} must document ContentPart.drive`);
    assert.match(source, /MediaResource/, `${relativePath} must document MediaResource as the usage structure`);
  }
});

test('media operation reference pages are not generated for removed IM media lifecycle routes', () => {
  const operationsMediaRoot = path.join(docsRoot, 'api-reference', 'operations', 'im', 'media');

  assert.equal(
    existsSync(operationsMediaRoot),
    false,
    'removed /im/v3/api/media lifecycle routes must not keep operation reference pages',
  );
});

test('authority openapi contract exposes Drive-backed media references without upload mutations', () => {
  const workspaceRoot = path.resolve(docsRoot, '..', '..');
  const openapiPath = path.join(
    workspaceRoot,
    'sdks',
    'sdkwork-im-sdk',
    'openapi',
    'sdkwork-im-im.openapi.yaml',
  );
  const openapi = readFileSync(openapiPath, 'utf8');

  assert.doesNotMatch(openapi, /operationId:\s*uploads\.create/);
  assert.doesNotMatch(openapi, /operationId:\s*uploads\.complete/);
  assert.doesNotMatch(openapi, /MediaUploadMutationResponse:/);
  assert.doesNotMatch(openapi, /MediaUploadSession:/);
  assert.doesNotMatch(openapi, /mediaAssetId:/);
  assert.doesNotMatch(openapi, /bucketId:/);
  assert.doesNotMatch(openapi, /objectKey:/);
  assert.match(openapi, /DriveReference:/);
  assert.match(openapi, /drive:/);
  assert.match(openapi, /MediaResource:/);
});

test('sdk docs describe Drive-backed media message references instead of IM upload helpers', () => {
  const checkedFiles = [
    path.join(docsRoot, 'sdk', 'flutter-sdk.md'),
    path.join(docsRoot, 'sdk', 'typescript-sdk.md'),
  ];
  const forbiddenPatterns = [
    /sdk\.media\.upload\(/,
    /sdk\.upload\(/,
    /sdk\.uploadAndSendMessage\(/,
    /ImUploadedMediaAsset/,
    /ImMediaUploadSession/,
    /MediaUploadMutationResponse/,
    /MediaUploadSession/,
    /presigned upload/i,
    /mediaAssetId/,
    /object_storage/,
  ];

  for (const filePath of checkedFiles) {
    const relativePath = path.relative(currentWorkspaceRoot, filePath);
    const source = readFileSync(filePath, 'utf8');

    for (const forbidden of forbiddenPatterns) {
      assert.doesNotMatch(
        source,
        forbidden,
        `${relativePath} must route file lifecycle work to sdkwork-drive and keep IM SDK media usage reference-only`,
      );
    }

    assert.match(source, /sdkwork-drive/);
    assert.match(source, /DriveReference/);
    assert.match(source, /ContentPart\.drive/);
    assert.match(source, /createImageMessage/);
  }
});

test('language support doc links to the dedicated TypeScript and Flutter SDK references', () => {
  const languageSupportDoc = readFileSync(path.join(docsRoot, 'sdk', 'language-support.md'), 'utf8');

  assert.match(languageSupportDoc, /\[TypeScript SDK\]\(\/sdk\/typescript-sdk\)/);
  assert.match(languageSupportDoc, /\[Flutter SDK\]\(\/sdk\/flutter-sdk\)/);
  assert.match(languageSupportDoc, /\[App API SDK\]\(\/sdk\/app-sdk\)/);
});

test('app sdk overview documents assembly metadata and verified workspace semantics', () => {
  const appSdkDoc = readFileSync(path.join(docsRoot, 'sdk', 'app-sdk.md'), 'utf8');

  assert.match(appSdkDoc, /\.sdkwork-assembly\.json/);
  assert.match(appSdkDoc, /manifestPath/);
  assert.match(appSdkDoc, /generatedAt/);
  assert.match(appSdkDoc, /generated[\s\S]*composed/i);
  assert.match(appSdkDoc, /verify-sdk\.mjs/);
});

test('backend sdk overview documents control and admin as backend modules', () => {
  const backendSdkDoc = readFileSync(path.join(docsRoot, 'sdk', 'backend-sdk.md'), 'utf8');

  assert.match(backendSdkDoc, /sdkwork-im-backend-sdk/);
  assert.match(backendSdkDoc, /SdkworkBackendClient/);
  assert.match(backendSdkDoc, /\/backend\/v3\/api\/control\/\*/);
  assert.match(backendSdkDoc, /\/backend\/v3\/api\/admin\/\*/);
  assert.match(backendSdkDoc, /Do not introduce a new admin SDK family/);
  assert.doesNotMatch(backendSdkDoc, retiredSdkPattern('sdkwork', '-control', '-plane', '-sdk'));
  assert.doesNotMatch(backendSdkDoc, retiredSdkPattern('sdkwork', '-im', '-admin', '-sdk'));
});

test('rtc sdk overview documents independent provider runtime ownership', () => {
  const rtcSdkDoc = readFileSync(path.join(docsRoot, 'sdk', 'rtc-sdk.md'), 'utf8');

  assert.match(rtcSdkDoc, /sdkwork-rtc-sdk/);
  assert.match(rtcSdkDoc, /not generated from OpenAPI/);
  assert.match(rtcSdkDoc, /provider package/);
  assert.match(rtcSdkDoc, /native driver/);
  assert.match(rtcSdkDoc, /verify-sdk\.mjs/);
});

test('language support doc links to the current app, backend, and RTC SDK references', () => {
  const languageSupportDoc = readFileSync(path.join(docsRoot, 'sdk', 'language-support.md'), 'utf8');

  assert.match(languageSupportDoc, /\[App API SDK\]\(\/sdk\/app-sdk\)/);
  assert.match(languageSupportDoc, /\[Backend SDK\]\(\/sdk\/backend-sdk\)/);
  assert.match(languageSupportDoc, /\[RTC SDK\]\(\/sdk\/rtc-sdk\)/);
  assert.doesNotMatch(languageSupportDoc, retiredSdkPattern('control', '-plane', '-sdk'));
  assert.doesNotMatch(languageSupportDoc, retiredSdkPattern('im', '-admin', '-sdk'));
});

test('control-plane overview documents backend sdk alignment and social domains', () => {
  const controlPlaneDoc = readFileSync(
    path.join(docsRoot, 'api-reference', 'control-plane-api.md'),
    'utf8',
  );

  assert.doesNotMatch(
    controlPlaneDoc,
    /does not yet include a checked-in admin OpenAPI authority file/i,
  );
  assert.match(controlPlaneDoc, /sdkwork-im-backend-sdk/);
  assert.match(controlPlaneDoc, /\/sdk\/backend-sdk/);
  assert.match(controlPlaneDoc, /\/api-reference\/control-plane\/social/);
  assert.match(controlPlaneDoc, /\/api-reference\/control-plane\/social-runtime/);
  assert.doesNotMatch(controlPlaneDoc, retiredSdkPattern('sdkwork', '-control', '-plane', '-sdk'));
});

test('api reference groups follow the im app backend authority split', () => {
  const sidebarSource = readFileSync(
    path.join(docsRoot, '.vitepress', 'api-reference-sidebar.mjs'),
    'utf8',
  );
  const apiIndexDoc = readFileSync(path.join(docsRoot, 'api-reference', 'index.md'), 'utf8');
  const appApiDoc = readFileSync(path.join(docsRoot, 'api-reference', 'app-api.md'), 'utf8');
  const portalDoc = readFileSync(
    path.join(docsRoot, 'api-reference', 'app', 'portal-access.md'),
    'utf8',
  );

  assert.match(sidebarSource, /text:\s*"IM Standard API"/);
  assert.match(sidebarSource, /text:\s*"App API"/);
  assert.match(sidebarSource, /text:\s*"Backend API"/);
  assert.doesNotMatch(sidebarSource, /text:\s*"Platform API"/);
  assert.doesNotMatch(sidebarSource, /text:\s*"IoT API"/);

  assert.match(apiIndexDoc, /IM Standard API/);
  assert.match(apiIndexDoc, /App API/);
  assert.match(apiIndexDoc, /Backend API/);
  assert.doesNotMatch(apiIndexDoc, /Open Platform API overview/);
  assert.doesNotMatch(apiIndexDoc, /Open IoT API overview/);

  assert.match(appApiDoc, /\/app\/v3\/api\/\*/);
  assert.match(appApiDoc, /sdkwork-im-app-sdk/);
  assert.match(appApiDoc, /Portal Access/);
  assert.match(appApiDoc, /Notifications/);
  assert.match(appApiDoc, /Automation/);
  assert.match(appApiDoc, /Provider Health/);
  assert.doesNotMatch(appApiDoc, /Device Twin/);
  assert.doesNotMatch(appApiDoc, /IoT Protocol/);
  assert.doesNotMatch(appApiDoc, /Conversation Runtime/);
  assert.doesNotMatch(appApiDoc, /Media and Streams/);
  assert.doesNotMatch(appApiDoc, /Realtime Presence/);

  assert.match(portalDoc, /\/app\/v3\/api\/portal\/access/);
  assert.match(portalDoc, /sdkwork-im-app-sdk/);
  assert.doesNotMatch(portalDoc, /@sdkwork\/im-sdk/);
  assert.doesNotMatch(portalDoc, /\/im\/v3\/api\/portal\/access/);
});

test('cli docs describe boundary materialization and current SDK verification commands', () => {
  const cliDoc = readFileSync(path.join(docsRoot, 'reference', 'cli-and-scripts.md'), 'utf8');

  assert.match(cliDoc, /materialize-im-v3-openapi-boundaries\.mjs/);
  assert.match(cliDoc, /sdkwork-im-app-sdk/);
  assert.match(cliDoc, /sdkwork-im-backend-sdk/);
  assert.match(cliDoc, /sdkwork-rtc-sdk/);
  assert.match(cliDoc, /\.sdkwork-assembly\.json/);
  assert.doesNotMatch(cliDoc, retiredSdkPattern('sdkwork', '-control', '-plane', '-sdk'));
  assert.doesNotMatch(cliDoc, retiredSdkPattern('sdkwork', '-im', '-admin', '-sdk'));
});

test('cli docs describe app sdk verification and assembly commands', () => {
  const cliDoc = readFileSync(path.join(docsRoot, 'reference', 'cli-and-scripts.md'), 'utf8');

  assert.match(cliDoc, /sdkwork-im-sdk/);
  assert.match(cliDoc, /sdkwork-im-im\.sdkgen\.yaml/);
  assert.match(cliDoc, /sdkwork-im-im\.flutter\.sdkgen\.yaml/);
  assert.match(cliDoc, /node \.\\sdks\\sdkwork-im-sdk\\bin\\verify-sdk\.mjs/);
  assert.match(cliDoc, /\.sdkwork-assembly\.json/);
});

test('typescript sdk guide documents package contract, assembly metadata, and maintainer workflow', () => {
  const typescriptDoc = readFileSync(path.join(docsRoot, 'sdk', 'typescript-sdk.md'), 'utf8');

  assert.match(typescriptDoc, /Current Delivery Reality/);
  assert.match(typescriptDoc, /Package Contract/);
  assert.match(typescriptDoc, /Local Workspace Workflow/);
  assert.match(typescriptDoc, /What To Read Next/);
  assert.match(typescriptDoc, /@sdkwork\/im-sdk/);
  assert.doesNotMatch(typescriptDoc, /@sdkwork-internal\/im-sdk-generated/);
  assert.doesNotMatch(typescriptDoc, /@sdkwork\/im-sdk-generated/);
  assert.match(typescriptDoc, /ImSdkClient/);
  assert.match(typescriptDoc, /generated\/server-openapi/);
  assert.match(typescriptDoc, /composed/);
  assert.match(typescriptDoc, /\.sdkwork-assembly\.json/);
  assert.match(typescriptDoc, /manifestPath/);
  assert.match(typescriptDoc, /generatedAt/);
  assert.match(typescriptDoc, /verify-sdk\.mjs/);
  assert.match(typescriptDoc, /verify-typescript-workspace\.mjs/);
  assert.match(typescriptDoc, /\/sdk\/app-sdk/);
  assert.match(typescriptDoc, /\/sdk\/language-support/);
  assert.match(typescriptDoc, /\/api-reference\/im\/media/);
});

test('sdk docs verifier forbids internal and unsupported generated TypeScript package identities', () => {
  const verifierSource = readFileSync(
    path.join(docsRoot, 'scripts', 'verify-sdk-docs.mjs'),
    'utf8',
  );

  assert.match(verifierSource, /@sdkwork-internal\/im-sdk-generated/);
  assert.match(verifierSource, /@sdkwork\/im-sdk-generated/);
});

test('flutter sdk guide documents current parity, assembly metadata, and local workspace workflow', () => {
  const flutterDoc = readFileSync(path.join(docsRoot, 'sdk', 'flutter-sdk.md'), 'utf8');

  assert.match(flutterDoc, /Current Delivery Reality/);
  assert.match(flutterDoc, /Package Contract/);
  assert.match(flutterDoc, /Current Surface Reality/);
  assert.match(flutterDoc, /Current Parity Gap/);
  assert.match(flutterDoc, /Local Workspace Workflow/);
  assert.match(flutterDoc, /What To Read Next/);
  assert.match(flutterDoc, /im_sdk_composed/);
  assert.match(flutterDoc, /im_sdk_generated/);
  assert.match(flutterDoc, /ImSdkComposedClient/);
  assert.match(flutterDoc, /ImBuilders/);
  assert.match(flutterDoc, /generated\/server-openapi/);
  assert.match(flutterDoc, /pubspec_overrides\.yaml/);
  assert.doesNotMatch(flutterDoc, /generatedConfig/);
  assert.match(flutterDoc, /\.sdkwork-assembly\.json/);
  assert.match(flutterDoc, /manifestPath/);
  assert.match(flutterDoc, /generatedAt/);
  assert.match(flutterDoc, /verify-sdk\.mjs --with-dart/);
  assert.match(flutterDoc, /\/sdk\/app-sdk/);
  assert.match(flutterDoc, /\/sdk\/language-support/);
  assert.match(flutterDoc, /\/api-reference\/im\/media/);
});

test('rust quick start teaches the shipped ImSdkClient entrypoint', () => {
  const rustQuickStartDoc = readFileSync(path.join(docsRoot, 'sdk', 'rust-quick-start.md'), 'utf8');

  assert.match(rustQuickStartDoc, /use im_sdk::ImSdkClient;/);
  assert.match(rustQuickStartDoc, /ImSdkClient::new_with_base_url/);
  assert.doesNotMatch(rustQuickStartDoc, removedRustCompatClientPattern);
});

test('language support guide explains workspace boundaries, official package names, and release semantics', () => {
  const languageSupportDoc = readFileSync(path.join(docsRoot, 'sdk', 'language-support.md'), 'utf8');

  assert.match(languageSupportDoc, /Current Verified Baseline/);
  assert.match(languageSupportDoc, /How To Use This Page/);
  assert.match(languageSupportDoc, /repo contract/i);
  assert.match(languageSupportDoc, /generated\/server-openapi/);
  assert.match(languageSupportDoc, /composed/);
  assert.match(languageSupportDoc, /\.sdkwork-assembly\.json/);
  assert.match(languageSupportDoc, /@sdkwork\/im-sdk/);
  assert.match(languageSupportDoc, /im_sdk_composed/);
  assert.match(languageSupportDoc, /sdkwork-im-app-sdk/);
  assert.match(languageSupportDoc, /sdkwork-im-backend-sdk/);
  assert.match(languageSupportDoc, /sdkwork-rtc-sdk/);
  assert.match(languageSupportDoc, /verify-sdk\.mjs/);
  assert.match(languageSupportDoc, /\/sdk\/typescript-sdk/);
  assert.match(languageSupportDoc, /\/sdk\/flutter-sdk/);
  assert.match(languageSupportDoc, /\/sdk\/backend-sdk/);
  assert.match(languageSupportDoc, /\/sdk\/rtc-sdk/);
  assert.match(languageSupportDoc, /\/sdk\/app-sdk/);
});

test('backend sdk overview does not document generatedConfig as public create surface', () => {
  const backendSdkDoc = readFileSync(path.join(docsRoot, 'sdk', 'backend-sdk.md'), 'utf8');

  assert.doesNotMatch(backendSdkDoc, /generatedConfig/);
});

