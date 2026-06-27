#!/usr/bin/env node
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadGeneratorYaml } from '../../workspace-sdk-generator-root-shared.mjs';
import { applySdkworkV3OpenApiStandard } from '../../workspace-openapi-v3-standard.mjs';

const prefix = 'sdkwork-im-sdk';

function fail(message) {
  console.error(`[${prefix}] ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  const parsed = { schemaUrl: '', output: '', timeoutMs: 30000 };
  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--schema-url') {
      parsed.schemaUrl = argv[index + 1] || '';
      index += 1;
      continue;
    }
    if (current === '--output') {
      parsed.output = argv[index + 1] || '';
      index += 1;
      continue;
    }
    if (current === '--timeout-ms') {
      parsed.timeoutMs = Number.parseInt(argv[index + 1] || '', 10);
      index += 1;
      continue;
    }
    fail(`Unknown argument: ${current}`);
  }
  if (!parsed.schemaUrl) fail('Missing required argument: --schema-url');
  if (!parsed.output) fail('Missing required argument: --output');
  if (!Number.isFinite(parsed.timeoutMs) || parsed.timeoutMs <= 0) fail('Invalid --timeout-ms value');
  return parsed;
}

function parseDocument(raw, yaml) {
  const trimmed = raw.trim();
  if (!trimmed) fail('Live OpenAPI schema was empty.');
  const document = trimmed.startsWith('{') || trimmed.startsWith('[') ? JSON.parse(trimmed) : yaml.load(raw);
  if (!document || typeof document !== 'object' || typeof document.openapi !== 'string' || !document.openapi.startsWith('3.')) {
    fail('Live OpenAPI schema must be an OpenAPI 3.x document.');
  }
  return document;
}

const args = parseArgs(process.argv.slice(2));
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const yaml = await loadGeneratorYaml(workspaceRoot);
const response = await fetch(args.schemaUrl, {
  headers: { accept: 'application/yaml, text/yaml, application/json, text/plain;q=0.9, */*;q=0.8' },
  signal: AbortSignal.timeout(args.timeoutMs),
}).catch((error) => fail(`Failed to fetch live OpenAPI schema from ${args.schemaUrl}: ${error.message}`));

if (!response.ok) fail(`Live OpenAPI schema request failed (${response.status} ${response.statusText}) for ${args.schemaUrl}`);

const document = parseDocument(await response.text(), yaml);
applySdkworkV3OpenApiStandard(document);
const nextContents = yaml.dump(document, { noRefs: true, sortKeys: false, lineWidth: 120 });
const outputPath = path.resolve(args.output);
if (!existsSync(outputPath) || readFileSync(outputPath, 'utf8') !== nextContents) {
  mkdirSync(path.dirname(outputPath), { recursive: true });
  writeFileSync(outputPath, nextContents, 'utf8');
}
process.stdout.write(outputPath);
