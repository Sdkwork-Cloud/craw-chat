#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';

const appRoot = path.resolve(import.meta.dirname, '..');
const repoRoot = path.resolve(appRoot, '..', '..');

function readText(...segments) {
  return fs.readFileSync(path.join(appRoot, ...segments), 'utf8');
}

function readJson(...segments) {
  return JSON.parse(readText(...segments));
}

function readRepoText(...segments) {
  return fs.readFileSync(path.join(repoRoot, ...segments), 'utf8');
}

function readRepoJson(...segments) {
  return JSON.parse(readRepoText(...segments));
}

function functionBody(source, functionName) {
  const match = new RegExp(`function\\s+${functionName}\\s*\\(`, 'u').exec(source);
  assert.ok(match, `Expected ${functionName} in source.`);

  const openBraceIndex = source.indexOf('{', match.index);
  assert.notEqual(openBraceIndex, -1, `Expected ${functionName} body.`);

  let depth = 0;
  for (let index = openBraceIndex; index < source.length; index += 1) {
    const character = source[index];
    if (character === '{') {
      depth += 1;
    } else if (character === '}') {
      depth -= 1;
      if (depth === 0) {
        return source.slice(openBraceIndex, index + 1);
      }
    }
  }

  throw new Error(`Could not find closing brace for ${functionName}.`);
}

const packageJson = readJson('package.json');
const corePackageJson = readJson('packages', 'sdkwork-im-pc-core', 'package.json');
const shellPackageJson = readJson('packages', 'sdkwork-im-pc-shell', 'package.json');
const tsconfigApp = readJson('tsconfig.app.json');
const pnpmWorkspaceSource = readRepoText('pnpm-workspace.yaml');
const viteConfigSource = readText('vite.config.ts');
const releaseSources = readRepoJson('config', 'shared-sdk-release-sources.json');
const workflow = readRepoJson('sdkwork.workflow.json');
const appAuthRuntimeSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'appAuthRuntime.ts',
);
const voiceClientSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'voiceAppSdkClient.ts',
);
const voicePcIntegrationSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'voicePcIntegration.ts',
);
const voiceBootstrapSource = readText('src', 'bootstrap', 'voicePc.ts');
const shellLoadersSource = readText(
  'packages',
  'sdkwork-im-pc-shell',
  'src',
  'capabilityModuleLoaders.ts',
);
const gatewayConfigSource = readRepoText('crates', 'sdkwork-im-cloud-gateway-config', 'src', 'lib.rs');
const gatewaySource = readRepoText('services', 'sdkwork-im-cloud-gateway', 'src', 'lib.rs');
const devRunnerSource = readRepoText('scripts', 'lib', 'im-pc-dev.mjs');
const sharedSdkGitSource = readRepoText('scripts', 'dev', 'prepare-shared-sdk-git-sources.mjs');
const voicePcRoot = path.resolve(repoRoot, '..', 'sdkwork-voice', 'apps', 'sdkwork-voice-pc');
const voiceMarketViewSource = fs.readFileSync(
  path.join(voicePcRoot, 'packages', 'sdkwork-voice-pc-market', 'src', 'VoiceMarketView.tsx'),
  'utf8',
);
const voiceMarketSdkSource = fs.readFileSync(
  path.join(voicePcRoot, 'packages', 'sdkwork-voice-pc-market', 'src', 'services', 'voiceMarketSdk.ts'),
  'utf8',
);
const voiceCoreCatalogSource = fs.readFileSync(
  path.join(voicePcRoot, 'packages', 'sdkwork-voice-pc-core', 'src', 'voiceAudioAssetCatalog.ts'),
  'utf8',
);

assert.equal(
  packageJson.scripts?.['test:voice-app-sdk-integration'],
  'node scripts/voice-app-sdk-integration-contract.test.mjs',
  'Chat PC must expose a dedicated voice app SDK integration contract script.',
);

assert.equal(
  corePackageJson.dependencies?.['@sdkwork/voice-app-sdk'],
  'workspace:*',
  '@sdkwork/im-pc-core must consume sdkwork-voice through the workspace app SDK package.',
);

assert.equal(
  shellPackageJson.dependencies?.['@sdkwork/voice-pc-market'],
  'workspace:*',
  '@sdkwork/im-pc-shell must consume the sdkwork-voice-pc-market embed package through workspace:*.',
);

assert.equal(
  shellPackageJson.dependencies?.['@sdkwork/voice-pc-speech'],
  'workspace:*',
  '@sdkwork/im-pc-shell must consume the sdkwork-voice-pc-speech embed package through workspace:*.',
);

assert.match(
  pnpmWorkspaceSource,
  /sdkwork-voice-pc-market/u,
  'pnpm-workspace.yaml must include the sdkwork-voice-pc-market package.',
);

assert.equal(
  fs.existsSync(path.join(appRoot, 'packages', 'sdkwork-im-pc-voice')),
  false,
  'apps/sdkwork-im-pc must not keep a local sdkwork-im-pc-voice package.',
);

assert.equal(
  fs.existsSync(path.join(appRoot, 'packages', 'sdkwork-im-pc-voice-gen')),
  false,
  'apps/sdkwork-im-pc must not keep a local sdkwork-im-pc-voice-gen package.',
);

assert.match(
  releaseSources.sources?.['sdkwork-voice']?.repoUrl ?? '',
  /^(?:https:\/\/github\.com\/|git@github\.com:)Sdkwork-Cloud\/sdkwork-voice\.git$/u,
  'Shared SDK release config must materialize sdkwork-voice from the canonical git repository.',
);

assert.equal(
  releaseSources.sources?.['sdkwork-voice']?.ref,
  workflow.dependencies?.find((dependency) => dependency.id === 'sdkwork-voice')?.ref,
  'Shared SDK release config must use the same pinned sdkwork-voice ref as sdkwork.workflow.json.',
);

assert.match(
  sharedSdkGitSource,
  /id:\s*['"]sdkwork-voice['"][\s\S]*sdkwork-voice-pc-market[\\/]package\.json/u,
  'Shared SDK git materializer must know how to prepare the sdkwork-voice-pc-market embed package.',
);

assert.match(
  devRunnerSource,
  /SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL[\s\S]*explicitVoiceAppApiUpstream[\s\S]*SDKWORK_IM_VOICE_APP_API_UPSTREAM/u,
  'PC dev runner must default sdkwork-voice traffic through the shared gateway root while preserving explicit Voice split upstream overrides.',
);

assert.match(
  gatewayConfigSource,
  /sdkwork-voice-app-api[\s\S]*SDKWORK_IM_VOICE_APP_API_UPSTREAM[\s\S]*SDKWORK_VOICE_APP_API_UPSTREAM[\s\S]*SDKWORK_VOICE_APP_API_BASE_URL/u,
  'Gateway config must expose deterministic sdkwork-voice app-api upstream environment keys.',
);

assert.match(
  gatewaySource,
  /sdkwork-voice-app-api[\s\S]*\/app\/v3\/api\/voice\/\{\*path\}[\s\S]*SdkworkVoiceAppSdk/u,
  'Web gateway must route sdkwork-voice app-api paths to the Voice app SDK upstream.',
);

assert.match(
  voiceClientSource,
  /createClient/u,
  'Core voice client must use the sdkwork-voice generated app SDK factory.',
);

assert.match(
  voiceClientSource,
  /tokenManager:\s*getSdkworkChatGlobalTokenManager\(\)/u,
  'Core voice client must share the Sdkwork IM global token manager.',
);

assert.doesNotMatch(
  voiceClientSource,
  /fetch\(|axios|Authorization|Access-Token/u,
  'Core voice client must not assemble raw HTTP or auth headers.',
);

assert.match(
  voicePcIntegrationSource,
  /bootstrapVoicePcForIm/u,
  'IM core must expose voice PC integration bootstrap.',
);

assert.match(
  voicePcIntegrationSource,
  /configureVoicePcRuntime/u,
  'IM core must configure sdkwork-voice-pc runtime through the integration module.',
);

assert.match(
  voicePcIntegrationSource,
  /rebootstrapVoicePcRuntimeForIm/u,
  'IM core must re-bootstrap voice PC runtime after session changes.',
);

assert.match(
  functionBody(appAuthRuntimeSource, 'createSdkworkChatIamRuntime'),
  /rebootstrapVoicePcRuntimeForIm\(\)/u,
  'IM auth runtime must re-bootstrap voice PC runtime after session changes.',
);

assert.match(
  voiceBootstrapSource,
  /bootstrapVoicePcForIm/u,
  'IM app bootstrap must wire sdkwork-voice-pc host adapters.',
);

assert.match(
  shellLoadersSource,
  /import\('@sdkwork\/voice-pc-market'\)/u,
  'IM shell must lazy-load the sdkwork-voice-pc-market capability package.',
);

assert.match(
  shellLoadersSource,
  /import\('@sdkwork\/voice-pc-speech'\)/u,
  'IM shell must lazy-load the sdkwork-voice-pc-speech capability package.',
);

assert.match(
  viteConfigSource,
  /@sdkwork\/voice-app-sdk/u,
  'Vite config must alias @sdkwork/voice-app-sdk to the generated transport SDK.',
);

assert.ok(
  tsconfigApp.compilerOptions?.paths?.['@sdkwork/utils'],
  'tsconfig.app must map @sdkwork/utils so composed SDK clients typecheck.',
);

assert.match(
  voiceCoreCatalogSource,
  /audioAssets\.list/u,
  'Voice PC core catalog must list audio assets through the generated voice app SDK.',
);

assert.match(
  voiceMarketSdkSource,
  /listVoiceAudioAssetOptions/u,
  'Voice market SDK path must consume the shared voice audio asset catalog helper.',
);

assert.match(
  voiceMarketViewSource,
  /isVoiceMarketPilotEnabled/u,
  'Voice market embed must remain pilot-gated until backend integration ships.',
);

console.log('voice-app-sdk-integration-contract.test.mjs: ok');
