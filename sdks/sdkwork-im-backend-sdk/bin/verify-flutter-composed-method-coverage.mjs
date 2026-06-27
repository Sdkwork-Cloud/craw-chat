#!/usr/bin/env node
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const PREFIX = 'sdkwork-im-backend-sdk';

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

function extractComposedMethodNames(source) {
  const names = new Set();
  for (const match of source.matchAll(/Future<[^\n]+>\s+([A-Za-z0-9_]+)\s*\(/g)) {
    names.add(match[1]);
  }
  return [...names].sort();
}

function assertMethodCoverage(label, generatedMethods, composedMethodNames) {
  const covered = new Set(composedMethodNames);
  const missing = generatedMethods.filter((methodName) => !covered.has(methodName));
  if (missing.length > 0) {
    fail(
      `[${PREFIX}] Flutter composed ${label} module is missing generated method passthroughs: ${missing.join(', ')}`,
    );
  }
}

export function verifyFlutterComposedMethodCoverage(workspaceRoot) {
  const root = workspaceRoot ?? path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

  const generatedControlSource = readSource(
    path.join(
      root,
      'sdkwork-im-backend-sdk-flutter',
      'generated',
      'server-openapi',
      'lib',
      'src',
      'api',
      'control.dart',
    ),
  );
  const generatedAdminSource = readSource(
    path.join(
      root,
      'sdkwork-im-backend-sdk-flutter',
      'generated',
      'server-openapi',
      'lib',
      'src',
      'api',
      'admin.dart',
    ),
  );
  const composedControlSource = readSource(
    path.join(
      root,
      'sdkwork-im-backend-sdk-flutter',
      'composed',
      'lib',
      'src',
      'control_module.dart',
    ),
  );
  const composedAdminSource = readSource(
    path.join(
      root,
      'sdkwork-im-backend-sdk-flutter',
      'composed',
      'lib',
      'src',
      'admin_module.dart',
    ),
  );

  const generatedControlMethods = extractGeneratedMethodNames(generatedControlSource);
  const generatedAdminMethods = extractGeneratedMethodNames(generatedAdminSource);
  const composedControlMethods = extractComposedMethodNames(composedControlSource);
  const composedAdminMethods = extractComposedMethodNames(composedAdminSource);

  assertMethodCoverage('control', generatedControlMethods, composedControlMethods);
  assertMethodCoverage('admin', generatedAdminMethods, composedAdminMethods);
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
