#!/usr/bin/env node
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadGeneratorYaml } from '../../workspace-sdk-generator-root-shared.mjs';
import { officialLanguages } from '../../workspace-im-v3-sdk-family.mjs';

const workspaceRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const assemblyPath = path.join(workspaceRoot, '.sdkwork-assembly.json');
const authorityPath = path.join(workspaceRoot, 'openapi', 'sdkwork-im-im.openapi.yaml');
const yaml = await loadGeneratorYaml(workspaceRoot);
const authority = yaml.load(readFileSync(authorityPath, 'utf8'));

const languageManifests = {
  typescript: 'package.json',
  flutter: 'pubspec.yaml',
  rust: 'Cargo.toml',
  java: 'pom.xml',
  csharp: 'Sdkwork.Im.Sdk.Generated.csproj',
  swift: 'Package.swift',
  kotlin: 'build.gradle.kts',
  go: 'go.mod',
  python: 'pyproject.toml',
};

const packageNames = {
  typescript: '@sdkwork/im-sdk-generated',
  flutter: 'im_sdk_generated',
  rust: 'im-sdk-generated',
  java: 'com.sdkwork:im-sdk-generated',
  csharp: 'Sdkwork.Im.Sdk.Generated',
  swift: 'ImSdkGenerated',
  kotlin: 'com.sdkwork:im-sdk-generated',
  go: 'github.com/sdkwork/im-sdk-generated',
  python: 'sdkwork-im-sdk-generated',
};

const languageDescriptions = {
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

const languages = officialLanguages.map((language) => {
  const workspace = `sdkwork-im-sdk-${language}`;
  const generatedPath = `${workspace}/generated/server-openapi`;
  const entry = {
    language,
    workspace,
    generationState: existsSync(path.join(workspaceRoot, generatedPath)) ? 'materialized' : 'pending',
    releaseState: 'not_published',
    generatedPath,
    manifestPath: `${generatedPath}/${languageManifests[language]}`,
    name: packageNames[language],
    version: authority.info?.version || '0.1.0',
    description: `Generator-owned ${languageDescriptions[language]} transport SDK for the Sdkwork IM IM standardized development API.`,
  };
  if (language === 'typescript') {
    entry.consumerSurface = {
      primaryClient: 'SdkworkImClient',
      apiPrefix: '/im/v3/api',
      publicPackage: '@sdkwork/im-sdk',
      composedPath: 'sdkwork-im-sdk-typescript/src',
    };
  }
  return entry;
});

const assembly = {
  workspace: 'sdkwork-im-sdk',
  title: 'SDKWork IM SDK',
  apiVersion: authority.info?.version || '0.1.0',
  openapiVersion: authority.openapi || '3.1.0',
  authoritySpec: '../../apis/open-api/im/sdkwork-im-im.openapi.yaml',
  generationInputSpec: 'openapi/sdkwork-im-im.sdkgen.yaml',
  derivedSpecs: {
    default: 'openapi/sdkwork-im-im.sdkgen.yaml',
    flutter: 'openapi/sdkwork-im-im.flutter.sdkgen.yaml',
  },
  discoverySurface: {
    sdkTarget: 'im',
    apiPrefix: '/im/v3/api',
    schemaUrl: '/im/v3/openapi.json',
    generatedProtocols: ['http'],
    manualTransports: ['websocket'],
  },
  sdkDependencies: [],
  languages,
  sdkOwner: 'sdkwork-im',
  apiAuthority: 'sdkwork-im.im',
};

const next = `${JSON.stringify(assembly, null, 2)}\n`;
if (!existsSync(assemblyPath) || readFileSync(assemblyPath, 'utf8') !== next) {
  writeFileSync(assemblyPath, next, 'utf8');
}
