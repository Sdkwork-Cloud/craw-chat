#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadGeneratorYaml } from '../../workspace-sdk-generator-root-shared.mjs';
import {
  cloneOpenApiJson,
  loadOpenApiDocument,
  parseOpenApiSourceArgs,
  writeOpenApiYamlDocument,
} from '../../workspace-openapi-source-shared.mjs';
import { applySdkworkV3OpenApiStandard } from '../../workspace-openapi-v3-standard.mjs';

const prefix = 'sdkwork-im-backend-sdk';
const args = parseOpenApiSourceArgs(process.argv.slice(2), {
  prefix,
  allowPreferDerived: true,
});
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const yaml = await loadGeneratorYaml(workspaceRoot);
const derived = cloneOpenApiJson(loadOpenApiDocument({ prefix, filePath: path.resolve(args.base), yaml }));
applySdkworkV3OpenApiStandard(derived);
writeOpenApiYamlDocument({ filePath: path.resolve(args.derived), document: derived, yaml });
process.stdout.write(args.preferDerived ? path.resolve(args.derived) : path.resolve(args.base));
