#!/usr/bin/env node
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const workspaceRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const packageRoot = path.join(workspaceRoot, 'sdkwork-im-sdk-typescript');
const generatedManifestPath = path.join(packageRoot, 'generated', 'server-openapi', 'package.json');
const generatedManifest = existsSync(generatedManifestPath)
  ? JSON.parse(readFileSync(generatedManifestPath, 'utf8'))
  : { version: '0.1.0' };

const manifest = {
  name: '@sdkwork/im-sdk',
  version: generatedManifest.version || '0.1.0',
  private: true,
  type: 'module',
  description: 'Layered TypeScript SDK for Sdkwork IM IM runtime APIs and realtime adapters.',
  main: './dist/index.cjs',
  module: './dist/index.js',
  types: './dist/index.d.ts',
  exports: {
    '.': {
      types: './dist/index.d.ts',
      import: './dist/index.js',
      require: './dist/index.cjs',
    },
  },
  files: [
    'dist',
    'src',
    'generated/server-openapi/dist',
    'README.md',
  ],
  scripts: {
    build: 'tsc -p tsconfig.json',
    typecheck: 'tsc -p tsconfig.json --noEmit',
  },
  dependencies: {
    '@sdkwork/im-sdk-generated': 'link:./generated/server-openapi',
    '@sdkwork/sdk-common': '^1.0.2',
    ws: '^8.18.1',
  },
};

const tsconfig = {
  compilerOptions: {
    target: 'ES2022',
    module: 'ESNext',
    moduleResolution: 'bundler',
    declaration: true,
    emitDeclarationOnly: false,
    outDir: 'dist',
    rootDir: 'src',
    strict: true,
    skipLibCheck: true,
    isolatedModules: true,
    esModuleInterop: true,
  },
  include: ['src/**/*.ts'],
};

function writeJsonIfChanged(filePath, value) {
  const next = `${JSON.stringify(value, null, 2)}\n`;
  if (!existsSync(filePath) || readFileSync(filePath, 'utf8') !== next) {
    writeFileSync(filePath, next, 'utf8');
  }
}

writeJsonIfChanged(path.join(packageRoot, 'package.json'), manifest);
writeJsonIfChanged(path.join(packageRoot, 'tsconfig.json'), tsconfig);
