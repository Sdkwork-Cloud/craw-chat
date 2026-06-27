#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { mergeImSpacesOpenApiFragments } from '../sdks/merge-im-spaces-openapi-fragments.mjs';
import { loadGeneratorYaml } from '../sdks/workspace-sdk-generator-root-shared.mjs';
import {
  loadOpenApiDocument,
  writeOpenApiYamlDocument,
} from '../sdks/workspace-openapi-source-shared.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.join(scriptDir, '..');
const sdkRoot = path.join(repoRoot, 'sdks');
const authorityPath = path.join(
  sdkRoot,
  'sdkwork-im-sdk',
  'openapi',
  'sdkwork-im-im.openapi.yaml',
);

const yaml = await loadGeneratorYaml(sdkRoot);
const authority = loadOpenApiDocument({
  prefix: 'merge-im-spaces-openapi-fragment',
  filePath: authorityPath,
  yaml,
});

mergeImSpacesOpenApiFragments(authority, yaml);
writeOpenApiYamlDocument({ filePath: authorityPath, document: authority, yaml });
console.log('[merge-im-spaces-openapi-fragment] merged spaces paths into IM authority.');
