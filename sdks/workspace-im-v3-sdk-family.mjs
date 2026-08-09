import { spawnSync } from 'node:child_process';
import { existsSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadGeneratorYaml, resolveGeneratorRoot } from './workspace-sdk-generator-root-shared.mjs';
import { loadOpenApiDocument, primitiveComponentSchemaNames } from './workspace-openapi-source-shared.mjs';

export const officialLanguages = [
  'typescript',
  'flutter',
  'rust',
  'java',
  'csharp',
  'swift',
  'kotlin',
  'go',
  'python',
];

const manifestByLanguage = {
  typescript: 'package.json',
  flutter: 'pubspec.yaml',
  rust: 'Cargo.toml',
  java: 'pom.xml',
  csharp: null,
  swift: 'Package.swift',
  kotlin: 'build.gradle.kts',
  go: 'go.mod',
  python: 'pyproject.toml',
};

const languageDisplayName = {
  typescript: 'TypeScript',
  flutter: 'Flutter',
  rust: 'Rust',
  java: 'Java',
  csharp: 'C#',
  swift: 'Swift',
  kotlin: 'Kotlin',
  go: 'Go',
  python: 'Python',
};

const textExtensions = new Set([
  '.bat',
  '.cs',
  '.csproj',
  '.dart',
  '.gradle',
  '.go',
  '.java',
  '.json',
  '.kt',
  '.kts',
  '.md',
  '.mjs',
  '.ps1',
  '.py',
  '.rs',
  '.sh',
  '.swift',
  '.toml',
  '.ts',
  '.txt',
  '.xml',
  '.yaml',
  '.yml',
]);

function fail(prefix, message) {
  console.error(`[${prefix}] ${message}`);
  process.exit(1);
}

function run(prefix, command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: options.cwd,
    encoding: options.capture ? 'utf8' : undefined,
    stdio: options.capture ? ['ignore', 'pipe', 'pipe'] : 'inherit',
    shell: false,
  });
  if (result.error) {
    fail(prefix, `${options.step || command} failed to start: ${result.error.message}`);
  }
  if (typeof result.status === 'number' && result.status !== 0) {
    if (options.capture && result.stderr) {
      process.stderr.write(result.stderr);
    }
    fail(prefix, `${options.step || command} failed with exit code ${result.status}`);
  }
  if (result.signal) {
    fail(prefix, `${options.step || command} terminated with signal ${result.signal}`);
  }
  return options.capture ? String(result.stdout || '').trim() : '';
}

function parseLanguages(prefix, values) {
  const normalized = [];
  for (const value of values) {
    for (const segment of String(value || '').split(',')) {
      const language = segment.trim().toLowerCase();
      if (language) {
        normalized.push(language);
      }
    }
  }
  const languages = normalized.length > 0 ? normalized : officialLanguages;
  for (const language of languages) {
    if (!officialLanguages.includes(language)) {
      fail(prefix, `Unsupported language: ${language}`);
    }
  }
  return [...new Set(languages)];
}

function parseGenerateArgs(prefix, argv) {
  const parsed = {
    languages: [],
    fixedSdkVersion: '',
    baseUrl: 'http://127.0.0.1:18089',
    schemaUrl: '',
    refreshLive: false,
  };
  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--language') {
      parsed.languages.push(argv[index + 1] || '');
      index += 1;
      continue;
    }
    if (current === '--fixed-sdk-version') {
      parsed.fixedSdkVersion = argv[index + 1] || '';
      index += 1;
      continue;
    }
    if (current === '--base-url') {
      parsed.baseUrl = argv[index + 1] || '';
      index += 1;
      continue;
    }
    if (current === '--schema-url') {
      parsed.schemaUrl = argv[index + 1] || '';
      index += 1;
      continue;
    }
    if (current === '--refresh-live') {
      parsed.refreshLive = true;
      continue;
    }
    fail(prefix, `Unknown argument: ${current}`);
  }
  return parsed;
}

function parseVerifyArgs(prefix, argv) {
  const parsed = { languages: [] };
  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--language') {
      parsed.languages.push(argv[index + 1] || '');
      index += 1;
      continue;
    }
    fail(prefix, `Unknown argument: ${current}`);
  }
  return parsed;
}

function readText(filePath) {
  return readFileSync(filePath, 'utf8').replace(/^\uFEFF/, '');
}

function readJson(filePath) {
  return JSON.parse(readText(filePath));
}

function relativePath(root, filePath) {
  return path.relative(root, filePath).replace(/\\/g, '/');
}

function generatedDescriptionFor(config, language) {
  if (!config.generatedApiLabel) {
    return null;
  }
  return `Generator-owned ${languageDisplayName[language] || language} transport SDK for the ${config.generatedApiLabel}.`;
}

function normalizeSdkManifestMetadata(root, config) {
  const manifestPath = sdkManifestPath(root);
  if (!existsSync(manifestPath)) {
    return;
  }
  const manifest = readJson(manifestPath);
  let changed = false;
  if (config.sdkOwner && manifest.sdkOwner !== config.sdkOwner) {
    manifest.sdkOwner = config.sdkOwner;
    changed = true;
  }
  if (config.apiAuthority && manifest.apiAuthority !== config.apiAuthority) {
    manifest.apiAuthority = config.apiAuthority;
    changed = true;
  }
  if (manifest.sdkFamily !== config.sdkName) {
    manifest.sdkFamily = config.sdkName;
    changed = true;
  }
  if (manifest.sdkName !== config.sdkName) {
    manifest.sdkName = config.sdkName;
    changed = true;
  }
  const consumerPackageName = typeScriptConsumerPackageName(config);
  const transportPackageName = languagePackageName(config, 'typescript');
  if (manifest.packageName !== consumerPackageName) {
    manifest.packageName = consumerPackageName;
    changed = true;
  }
  if (manifest.transportPackageName !== transportPackageName) {
    manifest.transportPackageName = transportPackageName;
    changed = true;
  }
  if (config.generatedApiLabel) {
    for (const entry of manifest.languages ?? []) {
      const expected = generatedDescriptionFor(config, entry.language);
      if (expected && entry.description !== expected) {
        entry.description = expected;
        changed = true;
      }
      if (entry.language === 'typescript') {
        if (entry.name !== transportPackageName) {
          entry.name = transportPackageName;
          changed = true;
        }
        if (entry.consumerPackageName !== consumerPackageName) {
          entry.consumerPackageName = consumerPackageName;
          changed = true;
        }
        if (entry.transportPackageName !== transportPackageName) {
          entry.transportPackageName = transportPackageName;
          changed = true;
        }
      }
    }
  }
  if (Array.isArray(config.sdkDependencies) && !deepEqualJson(manifest.sdkDependencies, config.sdkDependencies)) {
    manifest.sdkDependencies = config.sdkDependencies;
    changed = true;
  }
  if (changed) {
    writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, 'utf8');
  }
}

function buildGeneratedLanguage(root, config, language) {
  if (language !== 'typescript') {
    return;
  }
  const outputDir = languageOutputDir(root, config, language);
  const buildScriptPath = path.join(outputDir, 'custom', 'build-runtime.mjs');
  if (!existsSync(buildScriptPath)) {
    fail(config.sdkName, `typescript generated package is missing ${path.relative(root, buildScriptPath)}`);
  }
  run(config.sdkName, 'node', [buildScriptPath], {
    cwd: outputDir,
    step: 'typescript:custom/build-runtime.mjs',
  });
}

function writeJsonIfChanged(filePath, value) {
  const next = `${JSON.stringify(value, null, 2)}\n`;
  if (!existsSync(filePath) || readFileSync(filePath, 'utf8') !== next) {
    writeFileSync(filePath, next, 'utf8');
  }
}

function normalizeGeneratedPackageMetadata(root, config, language) {
  if (language !== 'typescript') {
    return;
  }

  const outputDir = languageOutputDir(root, config, language);
  const expectedPackageName = languagePackageName(config, language);
  const expectedConsumerPackageName = typeScriptConsumerPackageName(config);
  const packageJsonPath = path.join(outputDir, 'package.json');
  if (existsSync(packageJsonPath)) {
    const packageJson = readJson(packageJsonPath);
    if (packageJson.name !== expectedPackageName) {
      packageJson.name = expectedPackageName;
      writeJsonIfChanged(packageJsonPath, packageJson);
    }
  }

  const packageLockPath = path.join(outputDir, 'package-lock.json');
  if (existsSync(packageLockPath)) {
    const packageLock = readJson(packageLockPath);
    if (packageLock.name !== expectedPackageName) {
      packageLock.name = expectedPackageName;
    }
    if (packageLock.packages?.['']?.name !== expectedPackageName) {
      packageLock.packages[''].name = expectedPackageName;
    }
    writeJsonIfChanged(packageLockPath, packageLock);
  }

  const sdkMetadataPath = path.join(outputDir, 'sdkwork-sdk.json');
  if (existsSync(sdkMetadataPath)) {
    const sdkMetadata = readJson(sdkMetadataPath);
    if (sdkMetadata.packageName !== expectedPackageName) {
      sdkMetadata.packageName = expectedPackageName;
    }
    if (sdkMetadata.transportPackageName !== expectedPackageName) {
      sdkMetadata.transportPackageName = expectedPackageName;
    }
    if (sdkMetadata.consumerPackageName !== expectedConsumerPackageName) {
      sdkMetadata.consumerPackageName = expectedConsumerPackageName;
    }
    writeJsonIfChanged(sdkMetadataPath, sdkMetadata);
  }
}

function languageOutputDir(root, config, language) {
  return path.join(root, `${config.sdkName}-${language}`, 'generated', 'server-openapi');
}

function prepareGeneratedOutput(root, config, language) {
  const prepareScript = path.join(root, 'bin', 'prepare-generated-output.mjs');
  if (!existsSync(prepareScript)) {
    return;
  }

  run(config.sdkName, 'node', [
    prepareScript,
    '--language',
    language,
  ], { cwd: root, step: `prepare-generated-output:${language}` });
}

function manifestNameFor(config, language) {
  if (language === 'csharp') {
    return `${config.namespaces.csharp}.csproj`;
  }
  return manifestByLanguage[language];
}

function languagePackageName(config, language) {
  return config.packages[language];
}

function sdkManifestPath(root) {
  return path.join(root, 'sdk-manifest.json');
}

function typeScriptConsumerPackageName(config) {
  return config.consumerPackages?.typescript ?? `@sdkwork/${config.sdkName.replace(/^sdkwork-/u, '')}`;
}

function generatorPackageName(config, language) {
  if (language === 'typescript') {
    return typeScriptConsumerPackageName(config);
  }
  return languagePackageName(config, language);
}

function stableJson(value) {
  if (Array.isArray(value)) {
    return value.map((entry) => stableJson(entry));
  }
  if (!value || typeof value !== 'object') {
    return value;
  }
  return Object.fromEntries(
    Object.keys(value)
      .sort()
      .map((key) => [key, stableJson(value[key])]),
  );
}

function deepEqualJson(left, right) {
  return JSON.stringify(stableJson(left)) === JSON.stringify(stableJson(right));
}

function inputForLanguage(root, config, language) {
  if (language === 'flutter' && config.flutterDerivedSpec) {
    return path.join(root, config.flutterDerivedSpec);
  }
  return path.join(root, config.derivedSpec);
}

function generatorArgs(root, config, language, resolvedVersion, baseUrl) {
  const args = [
    'generate',
    '--input',
    inputForLanguage(root, config, language),
    '--output',
    languageOutputDir(root, config, language),
    '--name',
    config.sdkName,
    '--type',
    config.sdkType,
    '--language',
    language,
    '--base-url',
    baseUrl,
    '--api-prefix',
    config.apiPrefix,
    '--fixed-sdk-version',
    resolvedVersion,
    '--sdk-root',
    root,
    '--sdk-name',
    config.sdkName,
    '--package-name',
    generatorPackageName(config, language),
    '--standard-profile',
    'sdkwork-v3',
  ];
  if (language === 'csharp') {
    args.push('--namespace', config.namespaces.csharp);
  }
  if (config.primaryClient) {
    args.push('--client-name', config.primaryClient);
  }
  if (config.legacyClient) {
    args.push('--legacy-client-name', config.legacyClient);
  }
  return args;
}

async function readAuthority(root, config) {
  const yaml = await loadGeneratorYaml(root);
  return loadOpenApiDocument({
    prefix: config.sdkName,
    filePath: path.join(root, config.authoritySpec),
    yaml,
  });
}

function verifyOpenApiDocument(config, document, sourceLabel, failures) {
  if (!String(document.openapi || '').startsWith('3.')) {
    failures.push(`${sourceLabel} must be OpenAPI 3.x`);
  }
  const paths = Object.keys(document.paths ?? {});
  if (paths.length === 0) {
    failures.push(`${sourceLabel} must define paths`);
  }
  for (const pathKey of paths) {
    if (!pathKey.startsWith(config.apiPrefix)) {
      failures.push(`${sourceLabel} path must start with ${config.apiPrefix}: ${pathKey}`);
    }
    for (const forbidden of config.forbiddenPathParts) {
      if (pathKey.includes(forbidden)) {
        failures.push(`${sourceLabel} path must not contain ${forbidden}: ${pathKey}`);
      }
    }
  }
  for (const required of config.requiredPaths) {
    if (!paths.includes(required)) {
      failures.push(`${sourceLabel} must include required path ${required}`);
    }
  }
  const securitySchemes = document.components?.securitySchemes ?? {};
  if (
    securitySchemes.ApiKey?.type !== 'apiKey'
    || securitySchemes.ApiKey?.in !== 'header'
    || securitySchemes.ApiKey?.name !== 'X-API-Key'
  ) {
    failures.push(`${sourceLabel} must define components.securitySchemes.ApiKey as X-API-Key header`);
  }
  if (securitySchemes.AuthToken?.type !== 'http' || securitySchemes.AuthToken?.scheme !== 'bearer') {
    failures.push(`${sourceLabel} must define components.securitySchemes.AuthToken as HTTP bearer`);
  }
  if (
    securitySchemes.AccessToken?.type !== 'apiKey'
    || securitySchemes.AccessToken?.in !== 'header'
    || securitySchemes.AccessToken?.name !== 'Access-Token'
  ) {
    failures.push(`${sourceLabel} must define components.securitySchemes.AccessToken as Access-Token header`);
  }
  if (!document.components?.schemas?.ProblemDetail) {
    failures.push(`${sourceLabel} must define components.schemas.ProblemDetail`);
  }

  for (const [pathKey, pathItem] of Object.entries(document.paths ?? {})) {
    for (const [method, operation] of Object.entries(pathItem ?? {})) {
      const normalizedMethod = method.toLowerCase();
      if (!['get', 'put', 'post', 'delete', 'patch', 'options', 'head', 'trace'].includes(normalizedMethod)) {
        continue;
      }
      if (!operation.operationId || !String(operation.operationId).includes('.')) {
        failures.push(`${sourceLabel} ${normalizedMethod.toUpperCase()} ${pathKey} must use dotted operationId`);
      }
      if (!Array.isArray(operation.tags) || operation.tags.length === 0) {
        failures.push(`${sourceLabel} ${normalizedMethod.toUpperCase()} ${pathKey} must declare tags`);
      }
      if (!operation.summary) {
        failures.push(`${sourceLabel} ${normalizedMethod.toUpperCase()} ${pathKey} must declare summary`);
      }
      const security = operation.security;
      const anonymous = Array.isArray(security) && security.length === 0;
      const apiKeyRequirement = (entry) => entry
        && Object.keys(entry).length === 1
        && 'ApiKey' in entry;
      const dualTokenRequirement = (entry) => entry
        && Object.keys(entry).length === 2
        && 'AuthToken' in entry
        && 'AccessToken' in entry;
      const apiKeyOrDualToken = Array.isArray(security)
        && security.length === 2
        && security.some(apiKeyRequirement)
        && security.some(dualTokenRequirement)
        && security.every((entry) => apiKeyRequirement(entry) || dualTokenRequirement(entry));
      if (!anonymous && operation['x-sdkwork-auth-mode'] !== 'api-key-or-dual-token') {
        failures.push(`${sourceLabel} ${normalizedMethod.toUpperCase()} ${pathKey} must use x-sdkwork-auth-mode api-key-or-dual-token`);
      }
      if (!anonymous && !apiKeyOrDualToken) {
        failures.push(`${sourceLabel} ${normalizedMethod.toUpperCase()} ${pathKey} must use separate ApiKey and combined AuthToken plus AccessToken security alternatives`);
      }
      for (const [status, response] of Object.entries(operation.responses ?? {})) {
        const statusNumber = Number.parseInt(status, 10);
        if (Number.isFinite(statusNumber) && statusNumber >= 400) {
          if (!response?.content?.['application/problem+json']) {
            failures.push(`${sourceLabel} ${normalizedMethod.toUpperCase()} ${pathKey} response ${status} must include application/problem+json`);
          }
        }
      }
    }
  }
}

function dependencyPackageValues(dependency) {
  return Object.values(dependency?.packageByLanguage ?? {})
    .filter((value) => typeof value === 'string' && value.trim().length > 0)
    .map((value) => value.trim());
}

function forbiddenGeneratedDependencyPackages(config) {
  const packages = new Set();
  for (const dependency of config.sdkDependencies ?? []) {
    if (dependency?.generatedTransportImportPolicy !== 'forbidden') {
      continue;
    }
    for (const packageName of dependencyPackageValues(dependency)) {
      packages.add(packageName);
    }
  }
  return [...packages].sort((left, right) => right.length - left.length || left.localeCompare(right));
}

function verifySdkDependencyShape(dependency, index, failures) {
  const label = `sdkDependencies[${index}]`;
  if (!dependency || typeof dependency !== 'object' || Array.isArray(dependency)) {
    failures.push(`${label} must be an object`);
    return;
  }
  if (!dependency.workspace || typeof dependency.workspace !== 'string') {
    failures.push(`${label}.workspace is required`);
  }
  if (!dependency.role || typeof dependency.role !== 'string') {
    failures.push(`${label}.role is required`);
  }
  if (dependency.required !== true) {
    failures.push(`${label}.required must be true`);
  }
  if (dependency.dependencyMode !== 'consumer-sdk') {
    failures.push(`${label}.dependencyMode must be consumer-sdk`);
  }
  if (dependency.generatedTransportImportPolicy !== 'forbidden') {
    failures.push(`${label}.generatedTransportImportPolicy must be forbidden`);
  }
  if (
    dependency.apiPrefix !== null
    && (typeof dependency.apiPrefix !== 'string' || !dependency.apiPrefix.startsWith('/'))
  ) {
    failures.push(`${label}.apiPrefix must be an absolute API prefix or null`);
  }
  if (!dependency.packageByLanguage || typeof dependency.packageByLanguage !== 'object') {
    failures.push(`${label}.packageByLanguage is required`);
    return;
  }
  for (const language of officialLanguages) {
    if (!dependency.packageByLanguage[language]) {
      failures.push(`${label}.packageByLanguage must declare ${language}`);
    }
  }
}

function verifySdkDependencies(root, config, failures) {
  const manifestPath = sdkManifestPath(root);
  const componentSpecPath = path.join(root, 'specs/component.spec.json');
  const readmePath = path.join(root, 'README.md');
  const manifest = existsSync(manifestPath) ? readJson(manifestPath) : {};
  const componentSpec = existsSync(componentSpecPath) ? readJson(componentSpecPath) : {};
  const readmeSource = existsSync(readmePath) ? readText(readmePath) : '';
  const configuredDependencies = config.sdkDependencies;

  if (configuredDependencies == null) {
    if (Array.isArray(manifest.sdkDependencies) && manifest.sdkDependencies.length > 0) {
      failures.push('sdk-manifest.json must not declare sdkDependencies without sdk-family-config.mjs');
    }
    if (Array.isArray(componentSpec.contracts?.sdkDependencies) && componentSpec.contracts.sdkDependencies.length > 0) {
      failures.push('specs/component.spec.json must not declare contracts.sdkDependencies without sdk-family-config.mjs');
    }
    return;
  }
  if (!Array.isArray(configuredDependencies)) {
    failures.push('sdk-family-config.mjs sdkDependencies must be an array');
    return;
  }
  if (!Array.isArray(manifest.sdkDependencies)) {
    failures.push('sdk-manifest.json must declare sdkDependencies');
  } else if (!deepEqualJson(manifest.sdkDependencies, configuredDependencies)) {
    failures.push('sdk-manifest.json sdkDependencies must match sdk-family-config.mjs sdkDependencies');
  }
  if (!existsSync(componentSpecPath)) {
    failures.push('specs/component.spec.json is required when sdkDependencies are declared');
  } else if (!Array.isArray(componentSpec.contracts?.sdkDependencies)) {
    failures.push('specs/component.spec.json must declare contracts.sdkDependencies');
  } else if (!deepEqualJson(componentSpec.contracts?.sdkDependencies, configuredDependencies)) {
    failures.push('specs/component.spec.json contracts.sdkDependencies must match sdk-family-config.mjs sdkDependencies');
  }

  const seenWorkspaces = new Set();
  for (const [index, dependency] of configuredDependencies.entries()) {
    verifySdkDependencyShape(dependency, index, failures);
    if (!dependency || typeof dependency !== 'object') {
      continue;
    }
    if (seenWorkspaces.has(dependency.workspace)) {
      failures.push(`sdkDependencies must not duplicate workspace ${dependency.workspace}`);
    }
    seenWorkspaces.add(dependency.workspace);

    const requiredReadmeMarkers = [
      dependency.workspace,
      dependency.role,
      dependency.dependencyMode,
      dependency.generatedTransportImportPolicy,
      ...(dependency.apiPrefix ? [dependency.apiPrefix] : []),
      ...dependencyPackageValues(dependency),
    ];
    for (const marker of requiredReadmeMarkers) {
      if (!readmeSource.includes(marker)) {
        failures.push(`README.md must mention SDK dependency marker ${marker}`);
      }
    }
  }
}

function verifyReadme(root, config, failures) {
  const readmePath = path.join(root, 'README.md');
  if (!existsSync(readmePath)) {
    failures.push('README.md is required');
    return;
  }
  const source = readText(readmePath);
  for (const marker of [config.sdkName, config.apiPrefix, config.schemaUrl]) {
    if (!source.includes(marker)) {
      failures.push(`README.md must mention ${marker}`);
    }
  }
}

function verifySdkManifest(root, config, languages, failures) {
  const manifestPath = sdkManifestPath(root);
  if (!existsSync(manifestPath)) {
    failures.push('sdk-manifest.json is required');
    return;
  }
  const manifest = readJson(manifestPath);
  if (manifest.sdkFamily !== config.sdkName || manifest.sdkName !== config.sdkName || manifest.workspace !== config.sdkName) {
    failures.push('sdk-manifest.json SDK family fields must match SDK name');
  }
  if (config.sdkOwner && manifest.sdkOwner !== config.sdkOwner) {
    failures.push(`sdk-manifest.json sdkOwner must be ${config.sdkOwner}`);
  }
  if (config.apiAuthority && manifest.apiAuthority !== config.apiAuthority) {
    failures.push(`sdk-manifest.json apiAuthority must be ${config.apiAuthority}`);
  }
  if (manifest.discoverySurface?.apiPrefix !== config.apiPrefix) {
    failures.push('sdk-manifest.json discoverySurface.apiPrefix must match API prefix');
  }
  if (manifest.discoverySurface?.sdkTarget !== config.sdkTarget) {
    failures.push('sdk-manifest.json discoverySurface.sdkTarget must match SDK target');
  }
  if (manifest.generationInputSpec !== config.derivedSpec) {
    failures.push(`sdk-manifest.json generationInputSpec must be ${config.derivedSpec}`);
  }
  const expectedConsumerPackageName = typeScriptConsumerPackageName(config);
  const expectedTransportPackageName = languagePackageName(config, 'typescript');
  if (manifest.packageName !== expectedConsumerPackageName) {
    failures.push(`sdk-manifest.json packageName must be ${expectedConsumerPackageName}`);
  }
  if (manifest.transportPackageName !== expectedTransportPackageName) {
    failures.push(`sdk-manifest.json transportPackageName must be ${expectedTransportPackageName}`);
  }
  const entries = new Map((manifest.languages ?? []).map((entry) => [entry.language, entry]));
  for (const language of languages) {
    const entry = entries.get(language);
    if (!entry) {
      failures.push(`sdk-manifest.json must list language ${language}`);
      continue;
    }
    const expectedWorkspace = `${config.sdkName}-${language}`;
    const expectedPath = `${expectedWorkspace}/generated/server-openapi`;
    if (entry.workspace !== expectedWorkspace) {
      failures.push(`sdk-manifest.json ${language}.workspace must be ${expectedWorkspace}`);
    }
    if (entry.generatedPath !== expectedPath) {
      failures.push(`sdk-manifest.json ${language}.generatedPath must be ${expectedPath}`);
    }
    if (language === 'typescript') {
      if (entry.name !== expectedTransportPackageName) {
        failures.push(`sdk-manifest.json typescript.name must be ${expectedTransportPackageName}`);
      }
      if (entry.consumerPackageName !== expectedConsumerPackageName) {
        failures.push(`sdk-manifest.json typescript.consumerPackageName must be ${expectedConsumerPackageName}`);
      }
      if (entry.transportPackageName !== expectedTransportPackageName) {
        failures.push(`sdk-manifest.json typescript.transportPackageName must be ${expectedTransportPackageName}`);
      }
    }
    const expectedDescription = generatedDescriptionFor(config, language);
    if (expectedDescription && entry.description !== expectedDescription) {
      failures.push(`sdk-manifest.json ${language}.description must be ${expectedDescription}`);
    }
  }
}

function* walkFiles(root, relative = '') {
  const current = path.join(root, relative);
  for (const entry of readdirSync(current, { withFileTypes: true })) {
    const childRelative = path.join(relative, entry.name);
    const childPath = path.join(root, childRelative);
    if (entry.isDirectory()) {
      if (entry.name === 'node_modules' || entry.name === 'dist' || entry.name === '.git' || entry.name === '.sdkwork') {
        continue;
      }
      yield* walkFiles(root, childRelative);
      continue;
    }
    if (entry.isFile() && textExtensions.has(path.extname(entry.name).toLowerCase())) {
      yield childPath;
    }
  }
}

function verifyGeneratedLanguage(root, config, language, failures) {
  const outputDir = languageOutputDir(root, config, language);
  if (!existsSync(outputDir)) {
    failures.push(`${language} generated output is missing: ${outputDir}`);
    return;
  }
  const manifestName = manifestNameFor(config, language);
  const manifestPath = path.join(outputDir, manifestName);
  if (!existsSync(manifestPath)) {
    failures.push(`${language} generated manifest is missing: ${manifestPath}`);
  }
  if (language === 'typescript') {
    const sdkMetadataPath = path.join(outputDir, 'sdkwork-sdk.json');
    const sdkSourcePath = path.join(outputDir, 'src', 'sdk.ts');
    const httpClientSourcePath = path.join(outputDir, 'src', 'http', 'client.ts');
    const commonTypesSourcePath = path.join(outputDir, 'src', 'types', 'common.ts');
    const generatedReadmePath = path.join(outputDir, 'README.md');
    if (!existsSync(sdkMetadataPath)) {
      failures.push('typescript sdkwork-sdk.json is required');
    } else {
      const metadata = readJson(sdkMetadataPath);
      if (metadata.sdkType !== config.sdkType) {
        failures.push(`typescript sdkwork-sdk.json sdkType must be ${config.sdkType}`);
      }
      if (metadata.packageName !== languagePackageName(config, language)) {
        failures.push(`typescript sdkwork-sdk.json packageName must be ${languagePackageName(config, language)}`);
      }
      if (metadata.transportPackageName !== languagePackageName(config, language)) {
        failures.push(`typescript sdkwork-sdk.json transportPackageName must be ${languagePackageName(config, language)}`);
      }
      if (metadata.consumerPackageName !== typeScriptConsumerPackageName(config)) {
        failures.push(`typescript sdkwork-sdk.json consumerPackageName must be ${typeScriptConsumerPackageName(config)}`);
      }
    }
    if (existsSync(manifestPath)) {
      const manifest = readJson(manifestPath);
      if (manifest.name !== languagePackageName(config, language)) {
        failures.push(`typescript package.json name must be ${languagePackageName(config, language)}`);
      }
    }
    if (!existsSync(sdkSourcePath) || !readText(sdkSourcePath).includes(`class ${config.primaryClient}`)) {
      failures.push(`typescript SDK must export ${config.primaryClient}`);
    }
    for (const [relativePath, filePath] of [
      ['src/sdk.ts', sdkSourcePath],
      ['src/http/client.ts', httpClientSourcePath],
      ['src/types/common.ts', commonTypesSourcePath],
      ['README.md', generatedReadmePath],
    ]) {
      if (!existsSync(filePath)) {
        failures.push(`typescript generated ${relativePath} is required`);
        continue;
      }
    }
    for (const relativePath of [
      'dist/index.js',
      'dist/index.cjs',
      'dist/index.d.ts',
      'dist/sdk.d.ts',
      'dist/http/client.d.ts',
      'dist/types/common.d.ts',
    ]) {
      const filePath = path.join(outputDir, relativePath);
      if (!existsSync(filePath)) {
        failures.push(`typescript generated ${relativePath} is required`);
        continue;
      }
    }
  }
  for (const filePath of walkFiles(outputDir)) {
    const source = readText(filePath);
    const relative = relativePath(root, filePath);
    for (const forbidden of forbiddenGeneratedDependencyPackages(config)) {
      if (source.includes(forbidden)) {
        failures.push(`${relative} must not import or declare forbidden SDK dependency package ${forbidden}`);
      }
    }
    for (const forbidden of config.forbiddenGeneratedText) {
      if (source.includes(forbidden)) {
        failures.push(`${relative} must not contain ${forbidden}`);
      }
    }
  }
}

export async function runGenerateSdkFamily(config, argv) {
  const root = path.resolve(path.dirname(fileURLToPath(config.importMetaUrl)), '..');
  const args = parseGenerateArgs(config.sdkName, argv);
  const languages = parseLanguages(config.sdkName, args.languages);
  const generatorRoot = resolveGeneratorRoot(root);
  const generatorScript = path.join(generatorRoot, 'bin', 'sdkgen.js');
  const resolveVersionScript = path.join(generatorRoot, 'bin', 'resolve-sdk-version.js');

  if (args.refreshLive) {
    const refreshArgs = [
      path.join(root, 'bin', 'refresh-live-openapi-source.mjs'),
      '--schema-url',
      args.schemaUrl || new URL(config.schemaUrl, args.baseUrl).toString(),
      '--output',
      path.join(root, config.authoritySpec),
    ];
    run(config.sdkName, 'node', refreshArgs, { cwd: root, step: 'refresh-live-openapi-source' });
  }

  run(config.sdkName, 'node', [
    path.join(root, 'bin', 'prepare-openapi-source.mjs'),
    '--base',
    path.join(root, config.authoritySpec),
    '--derived',
    path.join(root, config.derivedSpec),
    '--prefer-derived',
  ], { cwd: root, step: 'prepare-openapi-source' });

  if (config.flutterDerivedSpec) {
    run(config.sdkName, 'node', [
      path.join(root, 'bin', 'prepare-openapi-source.mjs'),
      '--base',
      path.join(root, config.authoritySpec),
      '--derived',
      path.join(root, config.flutterDerivedSpec),
      '--prefer-derived',
      '--target-language',
      'flutter',
    ], { cwd: root, step: 'prepare-openapi-source:flutter' });
  }

  const authority = await readAuthority(root, config);
  const fixedSdkVersion = args.fixedSdkVersion || authority.info?.version || '0.1.0';
  const resolvedVersion = existsSync(resolveVersionScript)
    ? run(config.sdkName, 'node', [
        resolveVersionScript,
        '--sdk-root',
        root,
        '--sdk-name',
        config.sdkName,
        '--sdk-type',
        config.sdkType,
        '--fixed-sdk-version',
        fixedSdkVersion,
        '--package-name',
        typeScriptConsumerPackageName(config),
        '--no-sync-published-version',
      ], { cwd: root, step: 'resolve-sdk-version', capture: true })
    : fixedSdkVersion;

  for (const language of languages) {
    prepareGeneratedOutput(root, config, language);
    run(config.sdkName, 'node', [
      generatorScript,
      ...generatorArgs(root, config, language, resolvedVersion, args.baseUrl),
    ], { cwd: root, step: `sdkgen:${language}` });
    normalizeGeneratedPackageMetadata(root, config, language);
    buildGeneratedLanguage(root, config, language);
    const manifestPath = path.join(languageOutputDir(root, config, language), manifestNameFor(config, language));
    if (!existsSync(manifestPath)) {
      fail(config.sdkName, `sdkgen:${language} did not materialize ${manifestPath}`);
    }
  }

  normalizeSdkManifestMetadata(root, config);
  await runVerifySdkFamily(config, ['--language', languages.join(',')]);
}

export async function runVerifySdkFamily(config, argv) {
  const root = path.resolve(path.dirname(fileURLToPath(config.importMetaUrl)), '..');
  const args = parseVerifyArgs(config.sdkName, argv);
  const languages = parseLanguages(config.sdkName, args.languages);
  const yaml = await loadGeneratorYaml(root);
  const failures = [];
  const authority = loadOpenApiDocument({
    prefix: config.sdkName,
    filePath: path.join(root, config.authoritySpec),
    yaml,
  });
  verifyOpenApiDocument(config, authority, config.authoritySpec, failures);
  const derived = loadOpenApiDocument({
    prefix: config.sdkName,
    filePath: path.join(root, config.derivedSpec),
    yaml,
  });
  verifyOpenApiDocument(config, derived, config.derivedSpec, failures);
  if (config.flutterDerivedSpec) {
    const flutterDerived = loadOpenApiDocument({
      prefix: config.sdkName,
      filePath: path.join(root, config.flutterDerivedSpec),
      yaml,
    });
    verifyOpenApiDocument(config, flutterDerived, config.flutterDerivedSpec, failures);
    if (Object.keys(flutterDerived.paths ?? {}).some((pathKey) => pathKey.endsWith('/realtime/ws'))) {
      failures.push(`${config.flutterDerivedSpec} must not expose websocket transport to generated Flutter HTTP SDK`);
    }
    const primitiveSchemas = primitiveComponentSchemaNames(flutterDerived);
    if (primitiveSchemas.length > 0) {
      failures.push(
        `${config.flutterDerivedSpec} must inline primitive component refs before Flutter sdkgen; found ${primitiveSchemas.join(', ')}`,
      );
    }
  }
  verifyReadme(root, config, failures);
  verifySdkManifest(root, config, languages, failures);
  verifySdkDependencies(root, config, failures);
  for (const language of languages) {
    verifyGeneratedLanguage(root, config, language, failures);
  }
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(`[${config.sdkName}] ${failure}`);
    }
    process.exit(1);
  }
  console.log(`[${config.sdkName}] SDK family verification passed.`);
}
