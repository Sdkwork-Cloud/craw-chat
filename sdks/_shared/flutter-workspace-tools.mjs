import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import path from 'node:path';

function toPosixPath(value) {
  return value.replaceAll('\\', '/');
}

function writeFile(targetPath, source) {
  mkdirSync(path.dirname(targetPath), { recursive: true });
  const normalizedSource = source.endsWith('\n') ? source : `${source}\n`;
  if (!existsSync(targetPath) || readFileSync(targetPath, 'utf8') !== normalizedSource) {
    writeFileSync(targetPath, normalizedSource, 'utf8');
  }
}

function readJson(targetPath) {
  return JSON.parse(readFileSync(targetPath, 'utf8'));
}

function readText(targetPath) {
  return readFileSync(targetPath, 'utf8');
}

function readYamlScalar(targetPath, key) {
  const source = readText(targetPath);
  const match = source.match(new RegExp(`^${key}:\\s*(.+)$`, 'm'));
  return match ? match[1].trim().replace(/^['"]|['"]$/g, '') : '';
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function readOverridePath(source, packageName) {
  const match = source.match(
    new RegExp(`^\\s{2}${escapeRegExp(packageName)}:\\s*\\r?\\n\\s{4}path:\\s*(.+)$`, 'm'),
  );
  return match ? match[1].trim().replace(/^['"]|['"]$/g, '') : '';
}

function pascalCase(value) {
  const parts = String(value)
    .replace(/([a-z0-9])([A-Z])/g, '$1 $2')
    .split(/[^A-Za-z0-9]+/)
    .filter(Boolean);

  if (parts.length === 0) {
    return 'Value';
  }

  return parts
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join('');
}

function camelCase(value) {
  const pascal = pascalCase(value);
  return pascal.charAt(0).toLowerCase() + pascal.slice(1);
}

function extractPathParams(routePath) {
  return [...String(routePath).matchAll(/\{([^}]+)\}/g)].map((match) => match[1]);
}

function renderInterpolatedPath(routePath) {
  return String(routePath).replace(/\{([^}]+)\}/g, (_, rawParamName) => {
    const paramName = camelCase(rawParamName);
    return `\${Uri.encodeComponent(String(${paramName}))}`;
  });
}

function resolveCommonFlutterRoot(workspaceRoot) {
  let current = workspaceRoot;
  while (true) {
    const candidate = path.join(
      current,
      'sdk',
      'sdkwork-sdk-commons',
      'sdkwork-sdk-common-flutter',
    );
    if (existsSync(candidate)) {
      return candidate;
    }

    const parent = path.dirname(current);
    if (parent === current) {
      break;
    }
    current = parent;
  }

  return path.resolve(
    workspaceRoot,
    '..',
    '..',
    '..',
    '..',
    '..',
    'sdk',
    'sdkwork-sdk-commons',
    'sdkwork-sdk-common-flutter',
  );
}

function loadSurfaceModel(workspaceRoot, derivedSpecRelativePath) {
  const derivedPath = path.join(workspaceRoot, derivedSpecRelativePath);
  const derived = readJson(derivedPath);
  const surface = derived?.['x-sdkwork-sdk-surface'];
  if (!surface || typeof surface !== 'object') {
    throw new Error(`Missing x-sdkwork-sdk-surface metadata in ${derivedPath}`);
  }

  const groups = (Array.isArray(surface.surfaceGroups) ? surface.surfaceGroups : [])
    .map((entry) => entry?.operationGroup)
    .filter((value) => typeof value === 'string' && value.length > 0);
  const uniqueGroups = [...new Set(groups)];

  const operationBindings = (Array.isArray(surface.operationBindings) ? surface.operationBindings : [])
    .filter((entry) => entry?.protocol === 'http')
    .sort((left, right) => {
      const leftKey = `${left.operationGroup}:${left.path}:${left.method}:${left.operationId}`;
      const rightKey = `${right.operationGroup}:${right.path}:${right.method}:${right.operationId}`;
      return leftKey.localeCompare(rightKey);
    });

  return {
    derived,
    surface,
    groups: uniqueGroups,
    operationBindings,
  };
}

function renderGeneratedPubspec(config) {
  return [
    `name: ${config.generatedPackageName}`,
    `description: ${config.generatedPackageDescription}`,
    'version: 0.1.0',
    '',
    'environment:',
    "  sdk: '>=3.0.0 <4.0.0'",
    '',
    'dependencies:',
    '  sdkwork_common_flutter: ^1.0.0',
    '',
    'dev_dependencies:',
    '  test: ^1.24.0',
    '  lints: ^3.0.0',
  ].join('\n');
}

function renderGeneratedPubspecOverrides(commonFlutterRelativePath) {
  return [
    'dependency_overrides:',
    '  sdkwork_common_flutter:',
    `    path: ${commonFlutterRelativePath}`,
  ].join('\n');
}

function renderGeneratedReadme(config, groups) {
  const groupLines = groups
    .map((groupName) => `- \`client.${groupName}\` - ${groupName} API`)
    .join('\n');

  return `# ${config.generatedPackageName}

${config.generatedPackageDescription}.

## Package Role

This package is the generator-owned Flutter transport layer for the checked-in
${config.familyLabel} contract. Use it when you need direct access to grouped HTTP operations and public
transport types.

For business-facing ${config.consumerLabel} integrations, prefer the composed Flutter package
\`${config.composedPackageName}\`, which wraps this transport package with the higher-level
\`${config.composedClientClassName}\` facade.

## Installation

Add to \`pubspec.yaml\`:

\`\`\`yaml
dependencies:
  ${config.generatedPackageName}: ^0.1.0
\`\`\`

## Quick Start

\`\`\`dart
import 'package:${config.generatedPackageName}/${config.generatedLibraryFile}';

final client = SdkworkBackendClient.withBaseUrl(
  baseUrl: '${config.defaultBaseUrl}',
  authToken: 'your-auth-token',
);

${config.quickStartInvocation}
\`\`\`

## Authentication Modes

Choose one authentication mode per client instance.

### Bearer Token

\`\`\`dart
final client = SdkworkBackendClient.withBaseUrl(
  baseUrl: '${config.defaultBaseUrl}',
);

client.setAuthToken('your-auth-token');
\`\`\`

### API Key

\`\`\`dart
final client = SdkworkBackendClient.withBaseUrl(
  baseUrl: '${config.defaultBaseUrl}',
);

client.setApiKey('your-api-key');
\`\`\`

### Dual Token

\`\`\`dart
final client = SdkworkBackendClient.withBaseUrl(
  baseUrl: '${config.defaultBaseUrl}',
);

client.setAuthToken('your-auth-token');
client.setAccessToken('your-access-token');
\`\`\`

## Endpoint Targeting

${config.endpointTargeting.map((line) => `- ${line}`).join('\n')}

## Surface Groups

${groupLines}

## Package Boundary

- Use only the package root entrypoint:
  \`package:${config.generatedPackageName}/${config.generatedLibraryFile}\`.
- Do not import generated \`lib/src/\` paths from downstream code.
- Keep business orchestration in the composed Flutter package
  \`package:${config.composedPackageName}/${config.composedLibraryFile}\`.

## Regeneration Contract

- Generated files live under \`generated/server-openapi\`.
- Hand-written orchestration belongs under \`composed\`.
- Refresh the authority contract through the root workspace wrappers, then rerun the local
  materializer rather than editing generated transport files by hand.
`;
}

function renderGeneratedSdkworkConfig(config) {
  return JSON.stringify({
    schemaVersion: 1,
    name: config.workspaceName,
    version: '0.1.0',
    language: 'flutter',
    sdkType: 'backend',
    packageName: config.generatedPackageName,
    generator: '@sdkwork/sdk-generator',
    capabilities: {
      supportsGeneratedTests: false,
      supportsReadme: true,
      supportsCustomScaffold: true,
      supportsPublishWorkflow: true,
      hasDistinctBuildStep: false,
    },
    generation: {
      readme: true,
      tests: false,
    },
    ownership: {
      generatedOwnership: 'generated',
      scaffoldOwnership: 'scaffold',
      scaffoldRoots: ['custom/'],
      stateRoots: ['.sdkwork/'],
    },
  }, null, 2);
}

function renderGeneratedLibrary(config) {
  return [
    `export 'backend_client.dart';`,
    "export 'src/models.dart';",
    "export 'src/api/api.dart';",
  ].join('\n');
}

function renderGeneratedModels() {
  return [
    'typedef JsonMap = Map<String, dynamic>;',
    'typedef JsonValue = dynamic;',
  ].join('\n');
}

function renderGeneratedHttpClient() {
  return [
    "import 'package:sdkwork_common_flutter/sdkwork_common_flutter.dart';",
    '',
    'class HttpClient extends BaseHttpClient {',
    '  HttpClient({',
    '    required SdkConfig config,',
    '  }) : super(config);',
    '}',
  ].join('\n');
}

function renderGeneratedPaths() {
  return [
    "const String backendApiPrefix = '';",
    '',
    'String backendApiPath(String path) {',
    '  if (path.isEmpty) {',
    '    return backendApiPrefix;',
    '  }',
    "  if (path.startsWith('http://') || path.startsWith('https://')) {",
    '    return path;',
    '  }',
    "  final normalizedPrefixRaw = backendApiPrefix.trim();",
    "  final normalizedPrefix = normalizedPrefixRaw.isEmpty",
    "      ? ''",
    "      : '/${normalizedPrefixRaw.replaceAll(RegExp(r'^/+|/+$'), '')}';",
    "  final normalizedPath = path.startsWith('/') ? path : '/$path';",
    "  if (normalizedPrefix.isEmpty || normalizedPrefix == '/') {",
    '    return normalizedPath;',
    '  }',
    '  if (normalizedPath == normalizedPrefix ||',
    "      normalizedPath.startsWith('$normalizedPrefix/')) {",
    '    return normalizedPath;',
    '  }',
    "  return '$normalizedPrefix$normalizedPath';",
    '}',
  ].join('\n');
}

function renderGeneratedApiIndex(groups) {
  return [
    "export 'paths.dart';",
    ...groups.map((groupName) => `export '${groupName}.dart';`),
  ].join('\n');
}

function removeStaleGeneratedApiFiles(apiRoot, groups) {
  if (!existsSync(apiRoot)) {
    return;
  }

  const currentFileNames = new Set([
    'api.dart',
    'paths.dart',
    ...groups.map((groupName) => `${groupName}.dart`),
  ]);

  for (const entry of readdirSync(apiRoot, { withFileTypes: true })) {
    if (!entry.isFile() || !entry.name.endsWith('.dart')) {
      continue;
    }
    if (!currentFileNames.has(entry.name)) {
      rmSync(path.join(apiRoot, entry.name), { force: true });
    }
  }
}

function renderGeneratedOperation(operationBinding) {
  const methodName = camelCase(operationBinding.operationId);
  const httpMethod = String(operationBinding.method || 'get').toLowerCase();
  const summary = String(operationBinding.summary || operationBinding.operationId || '').trim();
  const pathParams = extractPathParams(operationBinding.path);
  const signatureLines = [];

  for (const rawParamName of pathParams) {
    signatureLines.push(`    Object ${camelCase(rawParamName)},`);
  }

  signatureLines.push('    {');
  if (httpMethod !== 'get' && httpMethod !== 'delete') {
    signatureLines.push('      dynamic body,');
  }
  signatureLines.push('      Map<String, dynamic>? params,');
  signatureLines.push('      Map<String, String>? headers,');
  if (httpMethod !== 'get' && httpMethod !== 'delete') {
    signatureLines.push('      String? contentType,');
  }
  signatureLines.push('    }');

  const requestArgs = [
    `      backendApiPath('${renderInterpolatedPath(operationBinding.path)}'),`,
  ];

  if (httpMethod !== 'get' && httpMethod !== 'delete') {
    requestArgs.push('      body: body,');
  }
  requestArgs.push('      params: params,');
  requestArgs.push('      headers: headers,');
  if (httpMethod !== 'get' && httpMethod !== 'delete') {
    requestArgs.push('      contentType: contentType,');
  }

  const comment = summary ? `  /// ${summary}` : null;

  return [
    ...(comment ? [comment] : []),
    `  Future<dynamic> ${methodName}(`,
    ...signatureLines,
    '  ) {',
    `    return _client.${httpMethod}(`,
    ...requestArgs,
    '    );',
    '  }',
  ].join('\n');
}

function renderGeneratedApiFile(groupName, operationBindings) {
  const className = `${pascalCase(groupName)}Api`;
  const methods = operationBindings
    .filter((entry) => entry.operationGroup === groupName)
    .map(renderGeneratedOperation)
    .join('\n\n');

  return [
    "import 'paths.dart';",
    "import '../http/client.dart';",
    '',
    `class ${className} {`,
    '  final HttpClient _client;',
    '',
    `  ${className}(this._client);`,
    '',
    methods,
    '}',
  ].join('\n');
}

function renderBackendClient(config, groups) {
  const imports = groups
    .map((groupName) => `import 'src/api/${groupName}.dart';`)
    .join('\n');
  const fields = groups
    .map((groupName) => `  late final ${pascalCase(groupName)}Api ${groupName};`)
    .join('\n');
  const assignments = groups
    .map((groupName) => `    ${groupName} = ${pascalCase(groupName)}Api(_httpClient);`)
    .join('\n');

  return `import 'package:sdkwork_common_flutter/sdkwork_common_flutter.dart';
import 'src/http/client.dart';
${imports}

class SdkworkBackendConfig {
  final String baseUrl;
  final String? apiKey;
  final String? authToken;
  final String? accessToken;
  final Map<String, String> headers;
  final int timeout;
  final String apiKeyHeader;
  final bool apiKeyAsBearer;

  const SdkworkBackendConfig({
    required this.baseUrl,
    this.apiKey,
    this.authToken,
    this.accessToken,
    this.headers = const <String, String>{},
    this.timeout = 30000,
    this.apiKeyHeader = 'Authorization',
    this.apiKeyAsBearer = true,
  });

  SdkConfig toSdkConfig() {
    return SdkConfig(
      baseUrl: baseUrl,
      timeout: timeout,
      headers: headers,
      apiKey: apiKey,
      apiKeyHeader: apiKeyHeader,
      apiKeyAsBearer: apiKeyAsBearer,
      authToken: authToken,
      accessToken: accessToken,
    );
  }
}

class SdkworkBackendClient {
  final HttpClient _httpClient;

${fields}

  SdkworkBackendClient({
    required SdkworkBackendConfig config,
  }) : _httpClient = HttpClient(config: config.toSdkConfig()) {
${assignments}
  }

  factory SdkworkBackendClient.withBaseUrl({
    required String baseUrl,
    String? apiKey,
    String? authToken,
    String? accessToken,
    Map<String, String>? headers,
    int timeout = 30000,
    String apiKeyHeader = 'Authorization',
    bool apiKeyAsBearer = true,
  }) {
    return SdkworkBackendClient(
      config: SdkworkBackendConfig(
        baseUrl: baseUrl,
        apiKey: apiKey,
        authToken: authToken,
        accessToken: accessToken,
        headers: headers ?? const <String, String>{},
        timeout: timeout,
        apiKeyHeader: apiKeyHeader,
        apiKeyAsBearer: apiKeyAsBearer,
      ),
    );
  }

  void setApiKey(String apiKey) {
    _httpClient.setApiKey(apiKey);
  }

  void setAuthToken(String token) {
    _httpClient.setAuthToken(token);
  }

  void setAccessToken(String token) {
    _httpClient.setAccessToken(token);
  }

  void setHeader(String key, String value) {
    _httpClient.setHeader(key, value);
  }
}
`;
}

function renderComposedPubspec(config) {
  return [
    `name: ${config.composedPackageName}`,
    `description: ${config.composedPackageDescription}`,
    'version: 0.1.0',
    '',
    'environment:',
    "  sdk: '>=3.0.0 <4.0.0'",
    '',
    'dependencies:',
    `  ${config.generatedPackageName}: ^0.1.0`,
    '',
    'dev_dependencies:',
    '  lints: ^3.0.0',
  ].join('\n');
}

function renderComposedPubspecOverrides(config, generatedRelativePath, commonFlutterRelativePath) {
  return [
    'dependency_overrides:',
    `  ${config.generatedPackageName}:`,
    `    path: ${generatedRelativePath}`,
    '  sdkwork_common_flutter:',
    `    path: ${commonFlutterRelativePath}`,
  ].join('\n');
}

function renderComposedReadme(config, groups) {
  const groupLines = groups.map((groupName) => `- \`${groupName}\``).join('\n');

  return `# ${config.composedPackageName}

Composed Flutter SDK for ${config.familyLabel}.

This package sits above the generated \`${config.generatedPackageName}\` package and provides:

- the consumer-facing \`${config.composedClientClassName}\`
- business-oriented domain fields
- flattened client creation arguments for straightforward onboarding

The generated \`${config.generatedPackageName}\` package remains generator-owned under
\`../generated/server-openapi\`. This \`composed\` package is manual-owned.

## Usage

\`\`\`dart
import 'package:${config.composedPackageName}/${config.composedLibraryFile}';

final sdk = ${config.composedClientClassName}.create(
  baseUrl: '${config.defaultBaseUrl}',
  authToken: 'your-auth-token',
);

${config.quickStartInvocation.replace('client.', 'sdk.')}
\`\`\`

## Domain Surface

${groupLines}

## Client Creation

The preferred consumer entrypoint is \`${config.composedClientClassName}.create(...)\`.
Use flattened creation arguments for the common path and pass \`backendClient\` only when you
already own a configured generated transport instance.

## Package Boundary

- Consume generated transport symbols only through
  \`package:${config.generatedPackageName}/${config.generatedLibraryFile}\`.
- Do not import \`generated/server-openapi/lib/src\` private paths from this package or
  downstream applications.

## Local Dependency Override

This workspace keeps \`pubspec.yaml\` publish-friendly and resolves local dependencies through
\`pubspec_overrides.yaml\`.
`;
}

function renderComposedContext(config) {
  return `import 'package:${config.generatedPackageName}/${config.generatedLibraryFile}';

class ${config.composedClientClassName}Context {
  final SdkworkBackendClient backendClient;

  ${config.composedClientClassName}Context(this.backendClient);

  void setApiKey(String apiKey) {
    backendClient.setApiKey(apiKey);
  }

  void setAuthToken(String token) {
    backendClient.setAuthToken(token);
  }

  void setAccessToken(String token) {
    backendClient.setAccessToken(token);
  }
}
`;
}

function renderComposedLibrary(config, groups) {
  const exports = [
    `export 'package:${config.generatedPackageName}/${config.generatedLibraryFile}';`,
    "export 'src/context.dart';",
  ].join('\n');
  const fields = groups
    .map((groupName) => `  late final ${pascalCase(groupName)}Api ${groupName};`)
    .join('\n');
  const assignments = groups
    .map((groupName) => `    ${groupName} = options.backendClient.${groupName};`)
    .join('\n');

  return `library ${path.basename(config.composedLibraryFile, '.dart')};

${exports}

import 'package:${config.generatedPackageName}/${config.generatedLibraryFile}';

import 'src/context.dart';

class ${config.composedClientClassName}Options {
  final SdkworkBackendClient backendClient;

  const ${config.composedClientClassName}Options({
    required this.backendClient,
  });
}

class ${config.composedClientClassName} {
  final ${config.composedClientClassName}Context _context;

  final SdkworkBackendClient backendClient;

${fields}

  ${config.composedClientClassName}(${config.composedClientClassName}Options options)
      : backendClient = options.backendClient,
        _context = ${config.composedClientClassName}Context(options.backendClient) {
${assignments}
  }

  factory ${config.composedClientClassName}.create({
    SdkworkBackendClient? backendClient,
    SdkworkBackendConfig? backendConfig,
    String? baseUrl,
    String? apiKey,
    String? authToken,
    String? accessToken,
    Map<String, String>? headers,
    int timeout = 30000,
  }) {
    final resolvedConfig = backendConfig ??
        (baseUrl == null
            ? null
            : SdkworkBackendConfig(
                baseUrl: baseUrl,
                apiKey: apiKey,
                authToken: authToken,
                accessToken: accessToken,
                headers: headers ?? const <String, String>{},
                timeout: timeout,
              ));

    if (backendClient == null && resolvedConfig == null) {
      throw ArgumentError(
        'Provide backendClient or baseUrl/backendConfig when creating ${config.composedClientClassName}.',
      );
    }

    final resolvedBackendClient =
        backendClient ?? SdkworkBackendClient(config: resolvedConfig!);

    return ${config.composedClientClassName}(
      ${config.composedClientClassName}Options(
        backendClient: resolvedBackendClient,
      ),
    );
  }

  void setApiKey(String apiKey) {
    _context.setApiKey(apiKey);
  }

  void setAuthToken(String token) {
    _context.setAuthToken(token);
  }

  void setAccessToken(String token) {
    _context.setAccessToken(token);
  }
}

${config.composedClientClassName} create${config.composedClientClassName}({
  SdkworkBackendClient? backendClient,
  SdkworkBackendConfig? backendConfig,
  String? baseUrl,
  String? apiKey,
  String? authToken,
  String? accessToken,
  Map<String, String>? headers,
  int timeout = 30000,
}) {
  return ${config.composedClientClassName}.create(
    backendClient: backendClient,
    backendConfig: backendConfig,
    baseUrl: baseUrl,
    apiKey: apiKey,
    authToken: authToken,
    accessToken: accessToken,
    headers: headers,
    timeout: timeout,
  );
}
`;
}

function renderWorkspaceReadme(config, groups) {
  const groupLines = groups.map((groupName) => `- \`${groupName}\``).join('\n');

  return `# ${config.flutterWorkspaceName}

This workspace owns the Flutter package surface for the ${config.familyLabel} SDK family.

## Layout

- \`generated/server-openapi\`
  Generator-owned Flutter HTTP SDK output materialized from the checked-in OpenAPI contract.
- \`composed\`
  Manual-owned consumer package \`${config.composedPackageName}\` built above the generated HTTP layer.
- \`bin/\`
  Thin forwarding scripts to the root workspace wrappers.
- \`README.md\`
  Manual-owned workspace documentation.

## Generation Boundary

This workspace follows the layered Flutter SDK pattern:

- generated HTTP SDK lives in \`generated/server-openapi\`
- composed Flutter SDK lives in \`composed\`
- future orchestration or realtime adapters must stay outside generated output

Do not hand-edit the generated package. Change the checked-in OpenAPI inputs or the root workspace
materializer and regenerate.

The manual \`composed\` layer consumes the generated package only through
\`package:${config.generatedPackageName}/${config.generatedLibraryFile}\`; it does not import
\`generated/server-openapi/lib/src\` private paths directly.

## Consumer Package

The primary ${config.consumerLabel}-facing Flutter package is \`composed/pubspec.yaml\`:

- package name: \`${config.composedPackageName}\`
- library entrypoint: \`composed/lib/${config.composedLibraryFile}\`
- main client: \`${config.composedClientClassName}\`
- exposed domains:
${groupLines}

The generated backend transport package is:

- package name: \`${config.generatedPackageName}\`
- library entrypoint: \`generated/server-openapi/lib/${config.generatedLibraryFile}\`

## Generate

From this workspace:

\`\`\`powershell
.\\bin\\sdk-gen.ps1
\`\`\`

\`\`\`bash
./bin/sdk-gen.sh
\`\`\`

These scripts forward to the root \`${config.workspaceName}/bin/generate-sdk.*\` wrapper, refresh
the checked-in authority contract when needed, rematerialize the Flutter workspace, rebuild the
assembly snapshot, and then run the root verification flow.

## Verify

From this workspace:

\`\`\`powershell
.\\bin\\sdk-verify.ps1
\`\`\`

\`\`\`bash
./bin/sdk-verify.sh
\`\`\`

These scripts forward to the root \`${config.workspaceName}/bin/verify-sdk.mjs\` wrapper. The
forwarded verification path includes Flutter workspace verification, package metadata checks,
public API boundary checks, and composed-surface checks for \`${config.composedClientClassName}\`.

## Endpoint Targeting

${config.endpointTargeting.map((line) => `- ${line}`).join('\n')}

## Current Workspace Status

The Flutter workspace is materialized end to end:

- generated transport package: \`${config.generatedPackageName}\`
- composed product package: \`${config.composedPackageName}\`
- public client surface: \`${config.composedClientClassName}\`
- source-level verification: enabled

Publication and version assignment are still pending, but this workspace is no longer
template-only.
`;
}

function renderShellGenerateForwarder(defaultBaseUrl) {
  return `#!/usr/bin/env sh
set -eu

REQUESTED_VERSION="\${1-}"
BASE_URL="\${BASE_URL:-${defaultBaseUrl}}"
export BASE_URL
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
WORKSPACE_DIR="$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)"

if [ -n "$REQUESTED_VERSION" ]; then
  exec "$WORKSPACE_DIR/bin/generate-sdk.sh" "$REQUESTED_VERSION"
fi

exec "$WORKSPACE_DIR/bin/generate-sdk.sh"
`;
}

function renderPowerShellGenerateForwarder(defaultBaseUrl) {
  return `param(
  [string]$RequestedVersion,
  [string]$BaseUrl = '${defaultBaseUrl}'
)

$ErrorActionPreference = 'Stop'

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$workspaceDir = (Resolve-Path (Join-Path $scriptDir '..\\..')).Path
$command = Join-Path $workspaceDir 'bin\\generate-sdk.ps1'

if ($PSBoundParameters.ContainsKey('RequestedVersion')) {
  & $command -RequestedVersion $RequestedVersion -BaseUrl $BaseUrl
} else {
  & $command -BaseUrl $BaseUrl
}

exit $LASTEXITCODE
`;
}

function renderShellAssembleForwarder() {
  return `#!/usr/bin/env sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
WORKSPACE_DIR="$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)"

exec node "$WORKSPACE_DIR/bin/assemble-sdk.mjs"
`;
}

function renderPowerShellAssembleForwarder() {
  return `$ErrorActionPreference = 'Stop'

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$workspaceDir = (Resolve-Path (Join-Path $scriptDir '..\\..')).Path

node (Join-Path $workspaceDir 'bin\\assemble-sdk.mjs')
exit $LASTEXITCODE
`;
}

function renderShellVerifyForwarder() {
  return `#!/usr/bin/env sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
WORKSPACE_DIR="$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)"

exec node "$WORKSPACE_DIR/bin/verify-sdk.mjs"
`;
}

function renderPowerShellVerifyForwarder() {
  return `$ErrorActionPreference = 'Stop'

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$workspaceDir = (Resolve-Path (Join-Path $scriptDir '..\\..')).Path

node (Join-Path $workspaceDir 'bin\\verify-sdk.mjs')
exit $LASTEXITCODE
`;
}

export function materializeFlutterWorkspace(config) {
  const surfaceModel = loadSurfaceModel(config.workspaceRoot, config.derivedSpecRelativePath);
  const groups = surfaceModel.groups;
  const operationBindings = surfaceModel.operationBindings;
  const flutterRoot = path.join(config.workspaceRoot, config.flutterWorkspaceName);
  const generatedRoot = path.join(flutterRoot, 'generated', 'server-openapi');
  const composedRoot = path.join(flutterRoot, 'composed');
  const generatedApiRoot = path.join(generatedRoot, 'lib', 'src', 'api');
  const commonFlutterRoot = resolveCommonFlutterRoot(config.workspaceRoot);
  const generatedCommonFlutterRelativePath = toPosixPath(path.relative(generatedRoot, commonFlutterRoot));
  const composedCommonFlutterRelativePath = toPosixPath(path.relative(composedRoot, commonFlutterRoot));
  const composedGeneratedRelativePath = toPosixPath(path.relative(composedRoot, generatedRoot));

  writeFile(path.join(flutterRoot, 'README.md'), renderWorkspaceReadme(config, groups));
  writeFile(path.join(flutterRoot, 'bin', 'sdk-gen.ps1'), renderPowerShellGenerateForwarder(config.defaultBaseUrl));
  writeFile(path.join(flutterRoot, 'bin', 'sdk-gen.sh'), renderShellGenerateForwarder(config.defaultBaseUrl));
  writeFile(path.join(flutterRoot, 'bin', 'sdk-assemble.ps1'), renderPowerShellAssembleForwarder());
  writeFile(path.join(flutterRoot, 'bin', 'sdk-assemble.sh'), renderShellAssembleForwarder());
  writeFile(path.join(flutterRoot, 'bin', 'sdk-verify.ps1'), renderPowerShellVerifyForwarder());
  writeFile(path.join(flutterRoot, 'bin', 'sdk-verify.sh'), renderShellVerifyForwarder());

  writeFile(path.join(generatedRoot, 'pubspec.yaml'), renderGeneratedPubspec(config));
  writeFile(
    path.join(generatedRoot, 'pubspec_overrides.yaml'),
    renderGeneratedPubspecOverrides(generatedCommonFlutterRelativePath),
  );
  writeFile(path.join(generatedRoot, 'README.md'), renderGeneratedReadme(config, groups));
  writeFile(path.join(generatedRoot, 'sdkwork-sdk.json'), renderGeneratedSdkworkConfig(config));
  writeFile(path.join(generatedRoot, 'lib', config.generatedLibraryFile), renderGeneratedLibrary(config));
  writeFile(path.join(generatedRoot, 'lib', 'backend_client.dart'), renderBackendClient(config, groups));
  writeFile(path.join(generatedRoot, 'lib', 'src', 'models.dart'), renderGeneratedModels());
  writeFile(path.join(generatedRoot, 'lib', 'src', 'http', 'client.dart'), renderGeneratedHttpClient());
  removeStaleGeneratedApiFiles(generatedApiRoot, groups);
  writeFile(path.join(generatedApiRoot, 'paths.dart'), renderGeneratedPaths());
  writeFile(path.join(generatedApiRoot, 'api.dart'), renderGeneratedApiIndex(groups));
  for (const groupName of groups) {
    writeFile(
      path.join(generatedApiRoot, `${groupName}.dart`),
      renderGeneratedApiFile(groupName, operationBindings),
    );
  }

  writeFile(path.join(composedRoot, 'pubspec.yaml'), renderComposedPubspec(config));
  writeFile(
    path.join(composedRoot, 'pubspec_overrides.yaml'),
    renderComposedPubspecOverrides(
      config,
      composedGeneratedRelativePath,
      composedCommonFlutterRelativePath,
    ),
  );
  writeFile(path.join(composedRoot, 'README.md'), renderComposedReadme(config, groups));
  writeFile(path.join(composedRoot, 'lib', config.composedLibraryFile), renderComposedLibrary(config, groups));
  writeFile(path.join(composedRoot, 'lib', 'src', 'context.dart'), renderComposedContext(config));

  return {
    flutterRoot,
    groups,
    operationBindings,
  };
}

export function verifyFlutterWorkspaceShape(config) {
  const failures = [];
  const flutterRoot = path.join(config.workspaceRoot, config.flutterWorkspaceName);
  const generatedRoot = path.join(flutterRoot, 'generated', 'server-openapi');
  const composedRoot = path.join(flutterRoot, 'composed');
  const commonFlutterRoot = resolveCommonFlutterRoot(config.workspaceRoot);

  const generatedPubspecPath = path.join(generatedRoot, 'pubspec.yaml');
  const generatedMetadataPath = path.join(generatedRoot, 'sdkwork-sdk.json');
  const composedPubspecPath = path.join(composedRoot, 'pubspec.yaml');
  const composedOverridesPath = path.join(composedRoot, 'pubspec_overrides.yaml');
  const generatedOverridesPath = path.join(generatedRoot, 'pubspec_overrides.yaml');
  const generatedLibraryPath = path.join(generatedRoot, 'lib', config.generatedLibraryFile);
  const generatedBackendClientPath = path.join(generatedRoot, 'lib', 'backend_client.dart');
  const generatedReadmePath = path.join(generatedRoot, 'README.md');
  const composedLibraryPath = path.join(composedRoot, 'lib', config.composedLibraryFile);
  const composedContextPath = path.join(composedRoot, 'lib', 'src', 'context.dart');
  const composedReadmePath = path.join(composedRoot, 'README.md');

  const requiredPaths = [
    generatedPubspecPath,
    generatedMetadataPath,
    composedPubspecPath,
    composedOverridesPath,
    generatedOverridesPath,
    generatedReadmePath,
    generatedLibraryPath,
    generatedBackendClientPath,
    composedReadmePath,
    composedLibraryPath,
    composedContextPath,
  ];

  for (const requiredPath of requiredPaths) {
    if (!existsSync(requiredPath)) {
      failures.push(`Missing Flutter workspace file: ${toPosixPath(path.relative(config.workspaceRoot, requiredPath))}`);
    }
  }

  if (failures.length > 0) {
    return failures;
  }

  const generatedPubspecName = readYamlScalar(generatedPubspecPath, 'name');
  const generatedMetadata = readJson(generatedMetadataPath);
  const composedPubspecSource = readText(composedPubspecPath);
  const composedOverridesSource = readText(composedOverridesPath);
  const generatedOverridesSource = readText(generatedOverridesPath);
  const generatedLibrarySource = readText(generatedLibraryPath);
  const generatedBackendClientSource = readText(generatedBackendClientPath);
  const generatedReadmeSource = readText(generatedReadmePath);
  const composedLibrarySource = readText(composedLibraryPath);
  const composedContextSource = readText(composedContextPath);
  const composedReadmeSource = readText(composedReadmePath);
  const generatedCommonFlutterOverride = readOverridePath(generatedOverridesSource, 'sdkwork_common_flutter');
  const composedCommonFlutterOverride = readOverridePath(composedOverridesSource, 'sdkwork_common_flutter');
  const generatedPackageOverride = readOverridePath(composedOverridesSource, config.generatedPackageName);

  if (generatedPubspecName !== config.generatedPackageName) {
    failures.push(
      `Flutter generated pubspec.yaml must declare the package name "${config.generatedPackageName}".`,
    );
  }

  if (generatedMetadata.packageName !== config.generatedPackageName) {
    failures.push(
      `Flutter sdkwork-sdk.json must declare packageName "${config.generatedPackageName}".`,
    );
  }

  if (!new RegExp(`\\n\\s{2}${escapeRegExp(config.generatedPackageName)}:\\s`).test(composedPubspecSource)) {
    failures.push(`Flutter composed pubspec.yaml must depend on "${config.generatedPackageName}".`);
  }

  if (!generatedPackageOverride) {
    failures.push(`Flutter pubspec_overrides.yaml must override "${config.generatedPackageName}".`);
  } else {
    const resolvedGeneratedOverride = path.resolve(composedRoot, generatedPackageOverride);
    if (resolvedGeneratedOverride !== generatedRoot) {
      failures.push(
        `Flutter composed pubspec_overrides.yaml must point "${config.generatedPackageName}" to ${toPosixPath(path.relative(composedRoot, generatedRoot))}.`,
      );
    }
  }

  if (!generatedCommonFlutterOverride) {
    failures.push('Flutter generated pubspec_overrides.yaml must override sdkwork_common_flutter.');
  } else {
    const resolvedGeneratedOverride = path.resolve(generatedRoot, generatedCommonFlutterOverride);
    if (resolvedGeneratedOverride !== commonFlutterRoot) {
      failures.push(
        `Flutter generated pubspec_overrides.yaml must resolve sdkwork_common_flutter to ${toPosixPath(commonFlutterRoot)}.`,
      );
    }
  }

  if (!composedCommonFlutterOverride) {
    failures.push('Flutter composed pubspec_overrides.yaml must override sdkwork_common_flutter.');
  } else {
    const resolvedComposedOverride = path.resolve(composedRoot, composedCommonFlutterOverride);
    if (resolvedComposedOverride !== commonFlutterRoot) {
      failures.push(
        `Flutter composed pubspec_overrides.yaml must resolve sdkwork_common_flutter to ${toPosixPath(commonFlutterRoot)}.`,
      );
    }
  }

  if (!generatedLibrarySource.includes("export 'backend_client.dart';")) {
    failures.push('Flutter generated library must export backend_client.dart.');
  }
  if (!generatedLibrarySource.includes("export 'src/models.dart';")) {
    failures.push('Flutter generated library must export src/models.dart.');
  }
  if (!generatedLibrarySource.includes("export 'src/api/api.dart';")) {
    failures.push('Flutter generated library must export src/api/api.dart.');
  }
  for (const requiredSection of [
    '## Package Role',
    '## Quick Start',
    '## Authentication Modes',
    '## Endpoint Targeting',
    '## Surface Groups',
    '## Package Boundary',
  ]) {
    if (!generatedReadmeSource.includes(requiredSection)) {
      failures.push(`Flutter generated README must include "${requiredSection}".`);
    }
  }
  if (!generatedReadmeSource.includes(config.generatedPackageName)) {
    failures.push('Flutter generated README must mention the generated package name.');
  }
  if (!generatedReadmeSource.includes(config.composedPackageName)) {
    failures.push('Flutter generated README must direct business consumers to the composed package.');
  }
  if (!generatedReadmeSource.includes(`package:${config.generatedPackageName}/${config.generatedLibraryFile}`)) {
    failures.push('Flutter generated README must document the generated package root entrypoint.');
  }
  if (!generatedReadmeSource.includes('lib/src/')) {
    failures.push('Flutter generated README must forbid importing generated lib/src private paths.');
  }

  for (const requiredToken of ['setApiKey', 'setAuthToken', 'setAccessToken']) {
    if (!generatedBackendClientSource.includes(requiredToken)) {
      failures.push(`Flutter backend_client.dart must expose ${requiredToken}().`);
    }
  }

  if (!composedLibrarySource.includes(`class ${config.composedClientClassName}`)) {
    failures.push(`Flutter composed library must export ${config.composedClientClassName}.`);
  }
  if (!composedLibrarySource.includes(`package:${config.generatedPackageName}/${config.generatedLibraryFile}`)) {
    failures.push('Flutter composed library must consume the generated package through its root entrypoint.');
  }
  if (composedLibrarySource.includes(`package:${config.generatedPackageName}/src/`)) {
    failures.push('Flutter composed library must not import generated private src paths.');
  }
  if (composedContextSource.includes(`package:${config.generatedPackageName}/src/`)) {
    failures.push('Flutter composed context must not import generated private src paths.');
  }
  for (const requiredSection of [
    '## Usage',
    '## Domain Surface',
    '## Client Creation',
    '## Package Boundary',
    '## Local Dependency Override',
  ]) {
    if (!composedReadmeSource.includes(requiredSection)) {
      failures.push(`Flutter composed README must include "${requiredSection}".`);
    }
  }
  if (!composedReadmeSource.includes(config.composedClientClassName)) {
    failures.push('Flutter composed README must mention the primary composed client.');
  }
  if (!composedReadmeSource.includes(config.generatedPackageName)) {
    failures.push('Flutter composed README must mention the generated package dependency.');
  }
  if (!composedReadmeSource.includes(`package:${config.generatedPackageName}/${config.generatedLibraryFile}`)) {
    failures.push('Flutter composed README must document the generated package root entrypoint.');
  }
  if (!composedReadmeSource.includes('generated/server-openapi/lib/src')) {
    failures.push('Flutter composed README must forbid importing generated private lib/src paths.');
  }

  return failures;
}
