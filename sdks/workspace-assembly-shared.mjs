import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';

function fail(prefix, message) {
  console.error(`[${prefix}] ${message}`);
  process.exit(1);
}

export function parseAssemblyArgs(argv, { prefix }) {
  const parsed = {
    languages: [],
  };

  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--language') {
      const value = (argv[index + 1] || '').trim().toLowerCase();
      if (!value) {
        fail(prefix, 'Missing value for --language');
      }
      parsed.languages.push(value);
      index += 1;
      continue;
    }

    fail(prefix, `Unknown argument: ${current}`);
  }

  return parsed;
}

export function readJson(filePath) {
  return JSON.parse(readFileSync(filePath, 'utf8'));
}

export function readYaml(filePath, yaml) {
  return yaml.load(readFileSync(filePath, 'utf8'));
}

export function readAuthorityMeta(authorityPath, yaml) {
  const document = readYaml(authorityPath, yaml);
  return {
    title: document?.info?.title || '',
    apiVersion: document?.info?.version || '',
    openapiVersion: document?.openapi || '',
  };
}

export function renderPackageAssembly({
  workspaceRoot,
  workspaceName,
  packageConfig,
  yaml,
}) {
  const workspacePath = path.join(workspaceRoot, workspaceName);
  const manifestPath = path.join(workspacePath, ...packageConfig.manifest);
  if (!existsSync(manifestPath)) {
    return null;
  }

  return {
    layer: packageConfig.layer,
    packagePath: path.join(workspaceName, ...packageConfig.path).replaceAll('\\', '/'),
    manifestPath: path.relative(workspaceRoot, manifestPath).replaceAll('\\', '/'),
    ...packageConfig.readManifest(manifestPath, yaml),
  };
}

export function renderLanguageAssembly({
  workspaceRoot,
  language,
  yaml,
  languageConfigs,
  prefix,
}) {
  const config = languageConfigs[language];
  if (!config) {
    fail(prefix, `Unsupported language for assembly: ${language}`);
  }

  const workspacePath = path.join(workspaceRoot, config.workspace);
  const generatedManifestPath = path.join(workspacePath, ...config.packages[0].manifest);
  if (!existsSync(generatedManifestPath)) {
    fail(prefix, `Missing generated manifest for ${language}: ${generatedManifestPath}`);
  }

  const packages = config.packages
    .map((packageConfig) =>
      renderPackageAssembly({
        workspaceRoot,
        workspaceName: config.workspace,
        packageConfig,
        yaml,
      }),
    )
    .filter(Boolean);
  const generatedPackage = packages.find((entry) => entry.layer === 'generated');
  if (!generatedPackage) {
    fail(prefix, `Missing generated package assembly for ${language}`);
  }

  return {
    language,
    workspace: config.workspace,
    generatedPath: generatedPackage.packagePath,
    manifestPath: generatedPackage.manifestPath,
    name: generatedPackage.name,
    version: generatedPackage.version,
    description: generatedPackage.description,
    entrypoints: generatedPackage.entrypoints,
    packages,
  };
}

export function detectAssemblyLanguages({
  requestedLanguages,
  workspaceRoot,
  generatedManifestPaths,
  supportedLanguages = ['typescript', 'flutter'],
  defaultLanguages = ['typescript', 'flutter'],
}) {
  const languageSet = new Set(requestedLanguages);
  for (const language of supportedLanguages) {
    const manifestPath = generatedManifestPaths[language];
    if (manifestPath && existsSync(manifestPath)) {
      languageSet.add(language);
    }
  }
  return languageSet.size > 0 ? [...languageSet] : [...defaultLanguages];
}

function toComparableAssembly(assembly, comparableKeys) {
  return Object.fromEntries(
    comparableKeys.map((key) => [key, assembly[key]]),
  );
}

export function writeStableAssembly({
  assemblyPath,
  assemblyBase,
  comparableKeys,
}) {
  mkdirSync(path.dirname(assemblyPath), { recursive: true });

  let currentAssembly = null;
  if (existsSync(assemblyPath)) {
    currentAssembly = readJson(assemblyPath);
  }

  const currentComparable = currentAssembly
    ? toComparableAssembly(currentAssembly, comparableKeys)
    : null;
  const nextComparable = toComparableAssembly(assemblyBase, comparableKeys);
  const nextAssemblyObject = {
    ...assemblyBase,
    generatedAt:
      currentComparable && JSON.stringify(currentComparable) === JSON.stringify(nextComparable)
        ? currentAssembly.generatedAt
        : new Date().toISOString(),
  };

  const nextAssembly = `${JSON.stringify(nextAssemblyObject, null, 2)}\n`;
  if (!existsSync(assemblyPath) || readFileSync(assemblyPath, 'utf8') !== nextAssembly) {
    writeFileSync(assemblyPath, nextAssembly, 'utf8');
  }

  return assemblyPath;
}
