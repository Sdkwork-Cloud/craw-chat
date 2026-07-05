#!/usr/bin/env node
/**
 * Align OpenAPI list parameters: canonical pageSize (PageSizeQuery) replaces limit wire.
 * Handlers still accept legacy `limit` via SdkWorkCursorListQuery alias.
 */
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const targets = [
  'apis/open-api/im/sdkwork-im-im.openapi.yaml',
  'sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml',
];

const PAGE_SIZE_PARAM = `    PageSizeQuery:
      name: pageSize
      in: query
      required: false
      description: Standard list page size per PAGINATION_SPEC.md and API_SPEC.md section 14.1.
      schema:
        type: integer
        format: int32
        minimum: 1
        maximum: 200
        default: 20`;

for (const relativePath of targets) {
  const filePath = path.join(repoRoot, relativePath);
  let yaml = fs.readFileSync(filePath, 'utf8');

  if (!yaml.includes('PageSizeQuery:')) {
    yaml = yaml.replace(
      /    LimitQuery:\n      name: limit\n/,
      `${PAGE_SIZE_PARAM}\n    LimitQuery:\n      name: limit\n      deprecated: true\n      description: Legacy alias for pageSize; use PageSizeQuery instead.\n`,
    );
  }

  yaml = yaml.replaceAll(
    "- $ref: '#/components/parameters/LimitQuery'",
    "- $ref: '#/components/parameters/PageSizeQuery'",
  );

  if (yaml.includes("name: limit\n      in: query\n      required: false\n      schema:")) {
    throw new Error(`${relativePath} still exposes undocumented limit parameters`);
  }

  fs.writeFileSync(filePath, yaml, 'utf8');
  process.stdout.write(`aligned list pageSize wire in ${relativePath}\n`);
}
