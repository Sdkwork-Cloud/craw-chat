#!/usr/bin/env node
import { execFileSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { migrateOpenApiDocument } from '../../../sdkwork-specs/tools/lib/migrate-openapi-legacy-envelope.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const files = [
  'apis/open-api/im/sdkwork-im-im.openapi.yaml',
  'apis/app-api/communication/sdkwork-im-app-api.openapi.yaml',
  'apis/backend-api/communication/sdkwork-im-backend-api.openapi.yaml',
];

function loadYaml(filePath) {
  return JSON.parse(
    execFileSync(
      'python',
      [
        '-c',
        'import json, yaml, sys; print(json.dumps(yaml.safe_load(open(sys.argv[1], encoding="utf-8"))))',
        filePath,
      ],
      { encoding: 'utf8' },
    ),
  );
}

function writeYaml(filePath, document) {
  execFileSync(
    'python',
    [
      '-c',
      'import json, sys, yaml; yaml.safe_dump(json.load(sys.stdin), open(sys.argv[1], "w", encoding="utf-8"), sort_keys=False, allow_unicode=True)',
      filePath,
    ],
    { input: JSON.stringify(document), encoding: 'utf8' },
  );
}

for (const relativePath of files) {
  const filePath = path.join(repoRoot, relativePath);
  const migrated = migrateOpenApiDocument(loadYaml(filePath));
  writeYaml(filePath, migrated);
  const reloaded = loadYaml(filePath);
  const samplePath = relativePath.includes('im-im')
    ? '/im/v3/api/spaces/{spaceId}/invites'
    : null;
  if (samplePath && reloaded.paths?.[samplePath]?.get) {
    const schema = reloaded.paths[samplePath].get.responses['200'].content['application/json'].schema;
    const wrapped = Boolean(schema?.allOf?.some((part) => String(part?.$ref || '').includes('SdkWorkApiResponse')));
    if (!wrapped) {
      throw new Error(`${relativePath} invite list response did not persist SdkWorkApiResponse envelope`);
    }
  }
  process.stdout.write(`migrated ${relativePath}\n`);
}
