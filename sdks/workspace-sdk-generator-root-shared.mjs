import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const canonicalGeneratorRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..', 'sdkwork-sdk-generator');

function normalizeCandidate(candidate) {
  return path.resolve(candidate);
}

function* generatorRootCandidates(workspaceRoot) {
  if (process.env.SDKWORK_GENERATOR_ROOT) {
    yield normalizeCandidate(process.env.SDKWORK_GENERATOR_ROOT);
  }

  yield canonicalGeneratorRoot;

  let current = path.resolve(workspaceRoot);
  while (true) {
    yield path.join(current, 'sdk', 'sdkwork-sdk-generator');
    const parent = path.dirname(current);
    if (parent === current) {
      break;
    }
    current = parent;
  }
}

export function resolveGeneratorRoot(workspaceRoot) {
  const tried = [];
  for (const candidate of generatorRootCandidates(workspaceRoot)) {
    const normalizedCandidate = normalizeCandidate(candidate);
    if (tried.includes(normalizedCandidate)) {
      continue;
    }
    tried.push(normalizedCandidate);
    if (existsSync(path.join(normalizedCandidate, 'bin', 'sdkgen.js'))
      && existsSync(path.join(normalizedCandidate, 'node_modules', 'js-yaml', 'dist', 'js-yaml.mjs'))) {
      return normalizedCandidate;
    }
  }

  throw new Error(
    `Unable to locate sdkwork-sdk-generator from ${workspaceRoot}. Tried: ${tried.join(', ')}`,
  );
}

export function resolveGeneratorJsYamlPath(workspaceRoot) {
  return path.join(
    resolveGeneratorRoot(workspaceRoot),
    'node_modules',
    'js-yaml',
    'dist',
    'js-yaml.mjs',
  );
}

export async function loadGeneratorYaml(workspaceRoot) {
  const yamlModule = await import(pathToFileURL(resolveGeneratorJsYamlPath(workspaceRoot)).href);
  return yamlModule.default;
}
