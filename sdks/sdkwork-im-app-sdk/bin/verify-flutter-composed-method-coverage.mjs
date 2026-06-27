#!/usr/bin/env node
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const PREFIX = 'sdkwork-im-app-sdk';
const APP_MODULES = [
  'portal',
  'notifications',
  'automation',
  'provider',
];

function fail(message) {
  throw new Error(message);
}

function readSource(filePath) {
  return readFileSync(filePath, 'utf8').replace(/^\uFEFF/, '');
}

function extractGeneratedMethodNames(source) {
  const names = new Set();
  for (const match of source.matchAll(/Future<[^\n]+>\s+([A-Za-z0-9_]+)\s*\(/g)) {
    names.add(match[1]);
  }
  return [...names].sort();
}

function extractComposedCallTargets(source, moduleName) {
  const names = new Set();
  const pattern = new RegExp(
    `context\\.transportClient\\.${moduleName}\\.([A-Za-z0-9_]+)\\s*\\(`,
    'g',
  );
  for (const match of source.matchAll(pattern)) {
    names.add(match[1]);
  }
  return [...names].sort();
}

function assertMethodCoverage(moduleName, generatedMethods, composedCallTargets) {
  const covered = new Set(composedCallTargets);
  const missing = generatedMethods.filter((methodName) => !covered.has(methodName));
  if (missing.length > 0) {
    fail(
      `[${PREFIX}] Flutter composed ${moduleName} module is missing generated method passthroughs: ${missing.join(', ')}`,
    );
  }
}

export function verifyFlutterComposedMethodCoverage(workspaceRoot) {
  const root = workspaceRoot ?? path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

  for (const moduleName of APP_MODULES) {
    const generatedSource = readSource(
      path.join(
        root,
        'sdkwork-im-app-sdk-flutter',
        'generated',
        'server-openapi',
        'lib',
        'src',
        'api',
        `${moduleName}.dart`,
      ),
    );
    const composedSource = readSource(
      path.join(
        root,
        'sdkwork-im-app-sdk-flutter',
        'composed',
        'lib',
        'src',
        `${moduleName}_module.dart`,
      ),
    );

    const generatedMethods = extractGeneratedMethodNames(generatedSource);
    const composedCallTargets = extractComposedCallTargets(composedSource, moduleName);
    assertMethodCoverage(moduleName, generatedMethods, composedCallTargets);
  }

  const rtcModuleSource = readSource(
    path.join(
      root,
      'sdkwork-im-app-sdk-flutter',
      'composed',
      'lib',
      'src',
      'rtc_module.dart',
    ),
  );
  if (/context\.transportClient\.rtc\b/.test(rtcModuleSource)) {
    fail(`[${PREFIX}] Flutter composed rtc module must consume rtc_sdk, not generated app transport rtc.`);
  }
  if (!/package:rtc_sdk\/rtc_sdk\.dart/.test(rtcModuleSource)) {
    fail(`[${PREFIX}] Flutter composed rtc module must import rtc_sdk.`);
  }
}

const invokedPath = process.argv[1] ? pathToFileURL(path.resolve(process.argv[1])).href : null;
const isCliEntry = invokedPath === import.meta.url;

if (isCliEntry) {
  try {
    verifyFlutterComposedMethodCoverage();
    console.log(`[${PREFIX}] Flutter composed method coverage verification passed.`);
  } catch (error) {
    console.error(`[${PREFIX}] ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  }
}
