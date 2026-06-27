#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadGeneratorYaml } from '../../workspace-sdk-generator-root-shared.mjs';
import {
  applyFlutterCompatibilityTransforms,
  cloneOpenApiJson,
  loadOpenApiDocument,
  parseOpenApiSourceArgs,
  writeOpenApiYamlDocument,
} from '../../workspace-openapi-source-shared.mjs';
import { applySdkworkV3OpenApiStandard } from '../../workspace-openapi-v3-standard.mjs';

const prefix = 'sdkwork-im-app-sdk';

function stripRealtimeWebsocketPath(document) {
  for (const pathKey of Object.keys(document.paths ?? {})) {
    if (pathKey.endsWith('/realtime/ws')) {
      delete document.paths[pathKey];
    }
  }
}

const args = parseOpenApiSourceArgs(process.argv.slice(2), {
  prefix,
  allowPreferDerived: true,
  allowTargetLanguage: true,
});
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const yaml = await loadGeneratorYaml(workspaceRoot);
const derived = cloneOpenApiJson(loadOpenApiDocument({ prefix, filePath: path.resolve(args.base), yaml }));
applySdkworkV3OpenApiStandard(derived);
stripRealtimeWebsocketPath(derived);
if (args.targetLanguage === 'flutter') {
  applyFlutterCompatibilityTransforms(derived);
}
writeOpenApiYamlDocument({ filePath: path.resolve(args.derived), document: derived, yaml });
process.stdout.write(args.preferDerived ? path.resolve(args.derived) : path.resolve(args.base));
