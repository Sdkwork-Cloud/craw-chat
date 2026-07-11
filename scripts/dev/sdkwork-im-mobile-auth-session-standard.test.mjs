import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const root = process.cwd();

function source(path) {
  return readFileSync(resolve(root, path), 'utf8');
}

function assertNoPattern(path, pattern, message) {
  const content = source(path);
  assert.equal(pattern.test(content), false, `${path}: ${message}`);
}

function assertPattern(path, pattern, message) {
  const content = source(path);
  assert.equal(pattern.test(content), true, `${path}: ${message}`);
}

const h5Bridge =
  'apps/sdkwork-im-h5/packages/sdkwork-im-h5-core/src/session/appbaseAuthBridge.ts';
const h5AppSession =
  'apps/sdkwork-im-h5/packages/sdkwork-im-h5-core/src/session/appSession.ts';
const h5IamSession =
  'apps/sdkwork-im-h5/packages/sdkwork-im-h5-core/src/session/iamSession.ts';
const h5IamRuntime = 'apps/sdkwork-im-h5/src/bootstrap/iamRuntime.ts';
const flutterBridge =
  'apps/sdkwork-im-flutter-mobile/packages/sdkwork_im_flutter_mobile_core/lib/src/session/appbase_auth_bridge.dart';
const flutterSession =
  'apps/sdkwork-im-flutter-mobile/packages/sdkwork_im_flutter_mobile_core/lib/src/session/app_session.dart';
const flutterAppAuth = 'apps/sdkwork-im-flutter-mobile/lib/bootstrap/app_auth.dart';
const flutterSdkClients = 'apps/sdkwork-im-flutter-mobile/lib/bootstrap/sdk_clients.dart';
const flutterAuthGate = 'apps/sdkwork-im-flutter-mobile/lib/app_auth_gate.dart';

for (const path of [h5Bridge, flutterBridge]) {
  assertNoPattern(
    path,
    /x-sdkwork-(?:tenant|organization|user|actor|session|app|environment|deployment|auth|data|permission|device|context)|\bactorId\b|\bactor_id\b/u,
    'mobile appbase callback parsing must not accept legacy AppContext projection aliases',
  );
}

assertNoPattern(
  h5Bridge,
  /\|\|\s*accessToken/u,
  'H5 appbase callback must reject missing authToken instead of reusing accessToken',
);
assertPattern(
  h5Bridge,
  /if\s*\(\s*!accessToken\s*\|\|\s*!authToken\s*\)\s*\{\s*return null;/su,
  'H5 appbase callback must require both accessToken and authToken',
);
assertNoPattern(
  h5IamSession,
  /authToken\s*\?\?\s*accessToken|accessToken\s*\|\|\s*authToken/u,
  'H5 stored session hydration must require a complete dual-token pair',
);
assertNoPattern(
  h5IamRuntime,
  /appbase-callback|callbackSession\.(?:tenantId|organizationId|userId)/u,
  'H5 bootstrap must not synthesize AppContext from callback query or defaults',
);
assertNoPattern(
  h5AppSession,
  /dev-access-token|dev-auth-token|tenantId:\s*"100001"|userId:\s*"user"/u,
  'H5 default app session must not carry fake credentials or default identity scope',
);

assertNoPattern(
  flutterBridge,
  /authToken\.isEmpty\s*\?\s*accessToken\s*:\s*authToken/u,
  'Flutter appbase callback must reject missing authToken instead of reusing accessToken',
);
assertPattern(
  flutterBridge,
  /if\s*\(\s*accessToken\.isEmpty\s*\|\|\s*authToken\.isEmpty\s*\)\s*\{\s*return null;/su,
  'Flutter appbase callback must require both accessToken and authToken',
);
assertNoPattern(
  flutterSession,
  /authToken\s*=\s*json\['authToken'\]\?\.toString\(\)\.trim\(\)\s*\?\?\s*accessToken/u,
  'Flutter stored session model must not synthesize authToken from accessToken',
);
assertPattern(
  flutterAppAuth,
  /session\.isComplete/u,
  'Flutter secure storage bootstrap must require complete dual-token and context state',
);
assertNoPattern(
  flutterSdkClients,
  /authToken:\s*activeSession\?\.authToken\s*\?\?\s*activeSession\?\.accessToken/u,
  'Flutter SDK bootstrap must not pass accessToken as authToken',
);
assertNoPattern(
  flutterAuthGate,
  /authToken:\s*_authTokenController\.text\.trim\(\)\.isEmpty\s*\?\s*_accessTokenController\.text\.trim\(\)/su,
  'Flutter dev credential form must reject missing authToken instead of reusing accessToken',
);
assertNoPattern(
  flutterAuthGate,
  /(?:tenantId|organizationId|userId):\s*_[A-Za-z]+Controller\.text\.trim\(\)\.isEmpty\s*\?\s*defaultAppSession\./su,
  'Flutter dev credential form must reject missing AppContext fields instead of using defaults',
);
assertPattern(
  flutterAuthGate,
  /nextSession\.isComplete/u,
  'Flutter dev credential form must require complete session state before saving',
);

console.log('mobile auth session standard check passed');
