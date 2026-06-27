import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import assert from 'node:assert/strict';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const appRoot = path.resolve(__dirname, '..');
const coreSdkRoot = path.join(
  appRoot,
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
);

function readSdkSource(fileName) {
  return readFileSync(path.join(coreSdkRoot, fileName), 'utf8');
}

const sessionSource = readSdkSource('session.ts');
const sdkClientFiles = [
  'appSdkClient.ts',
  'appbaseAppSdkClient.ts',
  'agentAppSdkClient.ts',
  'driveAppSdkClient.ts',
  'imSdkClient.ts',
];

assert.match(
  sessionSource,
  /export function getSdkworkChatGlobalTokenManager\(/u,
  'PC SDK runtime must expose one shared TokenManager for all authenticated SDK clients.',
);

assert.match(
  sessionSource,
  /export function syncSdkworkChatGlobalTokenManager\(/u,
  'Session persistence must synchronize the shared TokenManager after login, refresh, restore, and logout.',
);

assert.match(
  sessionSource,
  /syncSdkworkChatGlobalTokenManager\(normalizedSession\)/u,
  'persistAppSdkSessionTokens must sync normalized session state into the shared TokenManager.',
);

assert.match(
  sessionSource,
  /syncSdkworkChatGlobalTokenManager\(null\)/u,
  'clearAppSdkSessionTokens must clear the shared TokenManager.',
);

assert.match(
  sessionSource,
  /return\s+session\.authToken\s*&&\s*session\.accessToken\s*\?\s*session\s*:\s*null/u,
  'Session normalization must fail closed unless both authToken and accessToken are present.',
);

assert.match(
  sessionSource,
  /Sdkwork IM session requires authToken and accessToken\./u,
  'Session persistence must reject single-token sessions instead of accepting token fallbacks.',
);

assert.match(
  sessionSource,
  /transientAccessToken/u,
  'TokenManager must keep credential-entry bootstrap access tokens in memory without persisting a partial session.',
);

assert.match(
  sessionSource,
  /export function resolveAppSdkAccessToken[\s\S]*return\s+session\?\.accessToken;/u,
  'Access-Token resolution must never fall back to authToken.',
);

assert.match(
  sessionSource,
  /export function resolveAppSdkAuthToken[\s\S]*return\s+session\?\.authToken;/u,
  'Authorization token resolution must never fall back to accessToken.',
);

assert.match(
  sessionSource,
  /export function isAppSdkSessionExpired[\s\S]*Date\.now\(\)\s*>=\s*expiresAt/u,
  'Session storage must classify expired SDKWork app sessions before reporting authenticated state.',
);

assert.match(
  sessionSource,
  /isAppSdkSessionAuthenticated[\s\S]*Boolean\(session\?\.authToken\s*&&\s*session\?\.accessToken\)\s*&&\s*!isAppSdkSessionExpired\(session\)/u,
  'Authenticated app sessions require both SDKWork tokens and a non-expired session.',
);

for (const fileName of sdkClientFiles) {
  const source = readSdkSource(fileName);
  assert.match(
    source,
    /getSdkworkChatGlobalTokenManager/u,
    `${fileName} must inject the PC runtime shared TokenManager.`,
  );
  assert.doesNotMatch(
    source,
    /createSdkworkChatSessionTokenManager\(currentSession\)/u,
    `${fileName} must not create a per-client TokenManager snapshot.`,
  );
}

const appAuthRuntimeSource = readSdkSource('appAuthRuntime.ts');
assert.match(
  appAuthRuntimeSource,
  /resetDriveAppSdkClient/u,
  'Authenticated SDK reset must include the Drive app SDK client so media uploads cannot reuse a stale session.',
);
assert.match(
  appAuthRuntimeSource,
  /getDriveAppSdkClient/u,
  'Authenticated SDK inventory must include the Drive app SDK client so uploads share the IAM runtime TokenManager.',
);
assert.match(
  appAuthRuntimeSource,
  /resetSdkworkChatAuthenticatedSdkClients[\s\S]*resetDriveAppSdkClient\(\)/u,
  'Authenticated SDK reset must clear the Drive app SDK client alongside appbase, IM, app, AIoT, and agent clients.',
);
assert.match(
  appAuthRuntimeSource,
  /getAuthenticatedSdkClients[\s\S]*getDriveAppSdkClient\(\)/u,
  'Authenticated SDK inventory must bind the Drive app SDK client into the shared TokenManager closure.',
);

assert.doesNotMatch(
  appAuthRuntimeSource,
  /verificationCodeBypassEnabled/u,
  'Chat auth runtime development verification config must be prefill-only and must not expose bypass semantics.',
);

assert.match(
  appAuthRuntimeSource,
  /verificationCodePrefillEnabled/u,
  'Chat auth runtime must use the appbase verificationCodePrefillEnabled field for development verification-code hints.',
);

console.log('sdkwork im pc SDK runtime TokenManager contract passed.');
