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
  ['catalog', 'Catalog'],
  ['order', 'Order'],
  ['shop', 'Shop'],
  ['community', 'Community'],
  ['course', 'Course'],
  ['drive', 'Drive'],
  ['knowledgebase', 'Knowledgebase'],
  ['mail', 'Mail'],
];

for (const [domain, pascal] of domainSdkClients) {
  assert.match(
    appAuthRuntimeSource,
    new RegExp(`get${pascal}AppSdkClient`, 'u'),
    `Auth runtime must import the ${domain} app SDK client.`,
  );
  assert.match(
    appAuthRuntimeSource,
    new RegExp(`reset${pascal}AppSdkClient`, 'u'),
    `Auth runtime must import the ${domain} app SDK reset hook.`,
  );
  assert.match(
    functionBody(appAuthRuntimeSource, 'resetSdkworkChatAuthenticatedSdkClients'),
    new RegExp(`reset${pascal}AppSdkClient\\(\\)`, 'u'),
    `Session reset must reset the ${domain} app SDK client.`,
  );
  assert.match(
    functionBody(appAuthRuntimeSource, 'getAuthenticatedSdkClients'),
    new RegExp(`get${pascal}AppSdkClient\\(\\)`, 'u'),
    `Auth runtime sdkClients inventory must include the ${domain} app SDK client.`,
  );
}

console.log('domain app SDK auth runtime contract checks passed');
