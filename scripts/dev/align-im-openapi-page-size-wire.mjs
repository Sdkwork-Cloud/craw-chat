#!/usr/bin/env node
/**
 * Verify OpenAPI list parameters use canonical `page_size` HTTP wire name.
 *
 * `pageSize` remains valid for JSON bodies, response models, and language-level SDK options.
 * It is not a valid HTTP query parameter for pre-launch SDKWork APIs.
 */
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const targets = [
  'apis/open-api/im/sdkwork-im-im.openapi.yaml',
  'apis/app-api/communication/sdkwork-im-app-api.openapi.yaml',
  'apis/backend-api/communication/sdkwork-im-backend-api.openapi.yaml',
  'sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml',
  'sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.sdkgen.yaml',
  'sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.flutter.sdkgen.yaml',
  'sdks/sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.openapi.yaml',
  'sdks/sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.sdkgen.yaml',
  'sdks/sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.flutter.sdkgen.yaml',
  'sdks/sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.openapi.yaml',
  'sdks/sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.sdkgen.yaml',
  'sdks/sdkwork-im-sdk/openapi/im-spaces-paths.fragment.yaml',
];

let hasError = false;

const forbiddenPageSizeAliases = ['pageSize', 'limit', 'page_no', 'pageNo', 'per_page', 'size'];

function findQueryParameters(yaml) {
  const matches = [];
  const lines = yaml.split(/\r?\n/u);
  for (let index = 0; index < lines.length; index += 1) {
    const nameMatch = lines[index].match(/^\s*-?\s*name:\s*([A-Za-z0-9_]+)\s*$/u);
    if (!nameMatch) {
      continue;
    }
    const from = Math.max(0, index - 2);
    const to = Math.min(lines.length - 1, index + 8);
    for (let nearbyIndex = from; nearbyIndex <= to; nearbyIndex += 1) {
      if (/^\s*in:\s*query\s*$/u.test(lines[nearbyIndex])) {
        matches.push(nameMatch[1]);
        break;
      }
    }
  }
  return matches;
}

for (const relativePath of targets) {
  const filePath = path.join(repoRoot, relativePath);
  if (!fs.existsSync(filePath)) {
    continue;
  }
  const yaml = fs.readFileSync(filePath, 'utf8');
  const queryParameters = findQueryParameters(yaml);

  if (yaml.includes('LimitQuery')) {
    process.stderr.write(`ERROR: ${relativePath} still contains LimitQuery references\n`);
    hasError = true;
  }

  for (const forbiddenName of forbiddenPageSizeAliases) {
    if (queryParameters.includes(forbiddenName)) {
      process.stderr.write(`ERROR: ${relativePath} exposes forbidden pagination query parameter ${forbiddenName}\n`);
      hasError = true;
    }
  }

  if (yaml.includes('PageSizeQuery:') && !queryParameters.includes('page_size')) {
    process.stderr.write(`ERROR: ${relativePath} defines PageSizeQuery without page_size query wire name\n`);
    hasError = true;
  }

  if (!yaml.includes('PageSizeQuery:') && relativePath.includes('sdkwork-im-im.')) {
    process.stderr.write(`WARN: ${relativePath} does not contain PageSizeQuery definition\n`);
  }
}

if (hasError) {
  process.exit(1);
}

process.stdout.write('All OpenAPI list parameters aligned to page_size wire name\n');
