#!/usr/bin/env node
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { loadGeneratorYaml } from '../../workspace-sdk-generator-root-shared.mjs';
import { loadOpenApiDocument } from '../../workspace-openapi-source-shared.mjs';

const HTTP_METHODS = new Set(['get', 'put', 'post', 'delete', 'patch', 'options', 'head', 'trace']);
const TYPESCRIPT_API_EXCLUDED_FILES = new Set(['index.ts', 'base.ts', 'paths.ts']);
const FLUTTER_API_EXCLUDED_FILES = new Set(['api.dart', 'paths.dart', 'response_helpers.dart']);

function fail(message) {
  throw new Error(message);
}

function readText(filePath) {
  return readFileSync(filePath, 'utf8').replace(/^\uFEFF/, '');
}

function collectOperationMap(document) {
  const operations = new Map();
  for (const [pathKey, pathItem] of Object.entries(document.paths ?? {})) {
    for (const [method, operation] of Object.entries(pathItem ?? {})) {
      const normalizedMethod = method.toLowerCase();
      if (!HTTP_METHODS.has(normalizedMethod)) {
        continue;
      }
      operations.set(`${normalizedMethod} ${pathKey}`, String(operation?.operationId ?? ''));
    }
  }
  return operations;
}

function collectApiModules(apiDir, extension, excludedFiles) {
  if (!existsSync(apiDir)) {
    fail(`API directory not found: ${apiDir}`);
  }
  return readdirSync(apiDir, { withFileTypes: true })
    .filter((entry) => entry.isFile() && entry.name.endsWith(extension) && !excludedFiles.has(entry.name))
    .map((entry) => canonicalModuleName(entry.name.slice(0, -extension.length)))
    .sort();
}

function extractTypeScriptClientModules(source) {
  const modules = new Set();
  for (const match of source.matchAll(/public readonly ([A-Za-z0-9_]+): [A-Za-z0-9_]+Api;/g)) {
    modules.add(canonicalModuleName(match[1]));
  }
  return [...modules].sort();
}

function extractFlutterClientModules(source) {
  const modules = new Set();
  for (const match of source.matchAll(/late final ([A-Za-z0-9_]+)Api ([A-Za-z0-9_]+);/g)) {
    modules.add(canonicalModuleName(match[2]));
  }
  return [...modules].sort();
}

function canonicalModuleName(name) {
  return String(name || '').toLowerCase().replace(/[-_]+/g, '');
}

function assertEqualSets(left, right, label) {
  const leftOnly = left.filter((entry) => !right.includes(entry));
  const rightOnly = right.filter((entry) => !left.includes(entry));
  if (leftOnly.length > 0 || rightOnly.length > 0) {
    fail(
      `${label} mismatch. leftOnly=${leftOnly.join(', ') || 'none'}; rightOnly=${rightOnly.join(', ') || 'none'}`,
    );
  }
}

export async function verifyFlutterTypeScriptParity(workspaceRoot) {
  const root = workspaceRoot ?? path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
  const yaml = await loadGeneratorYaml(path.resolve(root, '..'));
  const typescriptSpec = loadOpenApiDocument({
    prefix: 'sdkwork-im-app-sdk',
    filePath: path.join(root, 'openapi', 'sdkwork-im-app-api.sdkgen.yaml'),
    yaml,
  });
  const flutterSpec = loadOpenApiDocument({
    prefix: 'sdkwork-im-app-sdk',
    filePath: path.join(root, 'openapi', 'sdkwork-im-app-api.flutter.sdkgen.yaml'),
    yaml,
  });

  const typescriptOperations = collectOperationMap(typescriptSpec);
  const flutterOperations = collectOperationMap(flutterSpec);
  assertEqualSets(
    [...typescriptOperations.keys()].sort(),
    [...flutterOperations.keys()].sort(),
    'app OpenAPI operation route',
  );

  for (const [operationKey, typescriptOperationId] of typescriptOperations.entries()) {
    const flutterOperationId = flutterOperations.get(operationKey);
    if (flutterOperationId !== typescriptOperationId) {
      fail(
        `app OpenAPI operationId mismatch for ${operationKey}: typescript=${typescriptOperationId} flutter=${flutterOperationId}`,
      );
    }
  }

  const typescriptApiModules = collectApiModules(
    path.join(root, 'sdkwork-im-app-sdk-typescript', 'generated', 'server-openapi', 'src', 'api'),
    '.ts',
    TYPESCRIPT_API_EXCLUDED_FILES,
  );
  const flutterApiModules = collectApiModules(
    path.join(root, 'sdkwork-im-app-sdk-flutter', 'generated', 'server-openapi', 'lib', 'src', 'api'),
    '.dart',
    FLUTTER_API_EXCLUDED_FILES,
  );
  assertEqualSets(typescriptApiModules, flutterApiModules, 'app API module');

  const typescriptClientSource = readText(
    path.join(root, 'sdkwork-im-app-sdk-typescript', 'generated', 'server-openapi', 'src', 'sdk.ts'),
  );
  const flutterClientSource = readText(
    path.join(root, 'sdkwork-im-app-sdk-flutter', 'generated', 'server-openapi', 'lib', 'app_client.dart'),
  );
  const typescriptClientModules = extractTypeScriptClientModules(typescriptClientSource);
  const flutterClientModules = extractFlutterClientModules(flutterClientSource);
  assertEqualSets(typescriptClientModules, flutterClientModules, 'app client module');
}

const invokedPath = process.argv[1] ? pathToFileURL(path.resolve(process.argv[1])).href : null;
const isCliEntry = invokedPath === import.meta.url;

if (isCliEntry) {
  const workspaceRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
  try {
    await verifyFlutterTypeScriptParity(workspaceRoot);
    console.log('[sdkwork-im-app-sdk] Flutter/TypeScript parity verification passed.');
  } catch (error) {
    console.error(
      `[sdkwork-im-app-sdk] ${error instanceof Error ? error.message : String(error)}`,
    );
    process.exit(1);
  }
}
