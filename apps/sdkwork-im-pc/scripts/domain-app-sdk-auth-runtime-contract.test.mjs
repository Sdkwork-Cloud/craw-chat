#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';

const appRoot = path.resolve(import.meta.dirname, '..');

function readText(...segments) {
  return fs.readFileSync(path.join(appRoot, ...segments), 'utf8');
}

function functionBody(source, functionName) {
  const start = source.indexOf(`function ${functionName}`);
  assert.ok(start >= 0, `Expected function ${functionName} in appAuthRuntime.ts`);
  const braceStart = source.indexOf('{', start);
  let depth = 0;
  for (let index = braceStart; index < source.length; index += 1) {
    const char = source[index];
    if (char === '{') {
      depth += 1;
    } else if (char === '}') {
      depth -= 1;
      if (depth === 0) {
        return source.slice(start, index + 1);
      }
    }
  }
  throw new Error(`Could not extract function body for ${functionName}`);
}

const appAuthRuntimeSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'appAuthRuntime.ts',
);

const domainSdkClients = [
  { domain: 'catalog', getter: 'getImHostedCatalogAppSdkClient', resetter: 'resetCommercePcIntegration' },
  { domain: 'order', getter: 'getImHostedOrderAppSdkClient', resetter: 'resetCommercePcIntegration' },
  { domain: 'shop', getter: 'getImHostedShopAppSdkClient', resetter: 'resetCommercePcIntegration' },
  { domain: 'community', getter: 'getCommunityAppSdkClient', resetter: 'resetCommunityPcIntegration' },
  { domain: 'course', getter: 'getCourseAppSdkClient', resetter: 'resetCoursePcIntegration' },
  { domain: 'drive', getter: 'getDriveAppSdkClient', resetter: 'resetDriveAppSdkClient' },
  { domain: 'knowledgebase', getter: 'getKnowledgebaseAppSdkClient', resetter: 'resetKnowledgebaseAppSdkClient' },
  { domain: 'mail', getter: null, resetter: 'resetMailPcIntegration' },
];

for (const { domain, getter, resetter } of domainSdkClients) {
  if (getter) {
    assert.match(
      appAuthRuntimeSource,
      new RegExp(getter, 'u'),
      `Auth runtime must import the ${domain} app SDK client accessor (${getter}).`,
    );
    assert.match(
      functionBody(appAuthRuntimeSource, 'getAuthenticatedSdkClients'),
      new RegExp(`${getter}\\(\\)`, 'u'),
      `Auth runtime sdkClients inventory must include the ${domain} app SDK client.`,
    );
  }
  assert.match(
    appAuthRuntimeSource,
    new RegExp(resetter, 'u'),
    `Auth runtime must import the ${domain} app SDK reset hook (${resetter}).`,
  );
  assert.match(
    functionBody(appAuthRuntimeSource, 'resetSdkworkChatAuthenticatedSdkClients'),
    new RegExp(`${resetter}\\(\\)`, 'u'),
    `Session reset must reset the ${domain} app SDK client.`,
  );
}

console.log('domain app SDK auth runtime contract checks passed');

// The login session hook must idempotently trigger the system-agent welcome
// message (server deduplicates); losing this wiring silently drops the
// welcome conversation for new accounts.
assert.match(
  appAuthRuntimeSource,
  /ensurePcWelcomeMessage/u,
  'Auth runtime must import the PC welcome ensure hook.',
);
const sessionHookSource = appAuthRuntimeSource.slice(
  appAuthRuntimeSource.indexOf('onSessionChanged'),
);
assert.match(
  sessionHookSource,
  /ensurePcWelcomeMessage\(\)/u,
  'Auth runtime onSessionChanged must invoke ensurePcWelcomeMessage().',
);
console.log('PC welcome session hook contract checks passed');
