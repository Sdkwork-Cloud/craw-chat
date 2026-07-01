#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const packagesRoot = path.join(repoRoot, 'apps/sdkwork-im-pc/packages');

const catalogThirdParty = new Set([
  'react',
  'react-dom',
  'lucide-react',
  'motion',
  'i18next',
  'react-i18next',
  'clsx',
  'tailwind-merge',
  'dompurify',
  'framer-motion',
  'react-qr-code',
  'react-router-dom',
  'react-router',
  'react-markdown',
  'react-hook-form',
  'signature_pad',
  'qrcode',
  'emoji-picker-react',
  'tiptap-markdown',
]);

function collectImportSpecifiers(source) {
  const specifiers = new Set();
  for (const match of source.matchAll(/(?:import|export)\s+(?:type\s+)?(?:[^'";]*?\sfrom\s+)?['"]([^'"]+)['"]/gu)) {
    specifiers.add(match[1]);
  }
  return specifiers;
}

function resolveDeclaredDependencyName(specifier) {
  if (specifier.startsWith('@sdkwork/')) {
    const segments = specifier.split('/');
    return segments.length >= 2 ? `${segments[0]}/${segments[1]}` : specifier;
  }
  if (specifier.startsWith('sdkwork-')) {
    return specifier.split('/')[0];
  }
  if (specifier.startsWith('.') || specifier.startsWith('@/')) {
    return null;
  }
  if (specifier.startsWith('motion/')) {
    return 'motion';
  }
  if (specifier.startsWith('@tiptap/') || specifier.startsWith('@tanstack/') || specifier.startsWith('@zxing/') || specifier.startsWith('@volcengine/')) {
    return specifier.split('/').slice(0, 2).join('/');
  }
  return specifier.split('/')[0];
}

function listSourceFiles(packageDir) {
  const srcRoot = path.join(packageDir, 'src');
  if (!fs.existsSync(srcRoot)) {
    return [];
  }
  const files = [];
  const walk = (dir) => {
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const fullPath = path.join(dir, entry.name);
      if (entry.isDirectory()) {
        walk(fullPath);
        continue;
      }
      if (/\.(ts|tsx)$/.test(entry.name)) {
        files.push(fullPath);
      }
    }
  };
  walk(srcRoot);
  return files;
}

function shouldDeclareDependency(dependencyName) {
  if (!dependencyName || dependencyName.startsWith('node:')) {
    return false;
  }
  return (
    dependencyName.startsWith('@sdkwork/')
    || dependencyName.startsWith('sdkwork-')
    || catalogThirdParty.has(dependencyName)
    || dependencyName.startsWith('@tiptap/')
    || dependencyName.startsWith('@tanstack/')
    || dependencyName.startsWith('@zxing/')
    || dependencyName.startsWith('@volcengine/')
  );
}

const fixes = [];
for (const entry of fs.readdirSync(packagesRoot, { withFileTypes: true })) {
  if (!entry.isDirectory()) {
    continue;
  }
  const packageDir = path.join(packagesRoot, entry.name);
  const packageJsonPath = path.join(packageDir, 'package.json');
  if (!fs.existsSync(packageJsonPath)) {
    continue;
  }
  const pkg = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
  if (pkg.name === '@sdkwork/im-pc-desktop') {
    continue;
  }
  const declared = new Set(Object.keys(pkg.dependencies ?? {}));
  const missing = new Map();
  for (const sourceFile of listSourceFiles(packageDir)) {
    const source = fs.readFileSync(sourceFile, 'utf8');
    for (const specifier of collectImportSpecifiers(source)) {
      const dependencyName = resolveDeclaredDependencyName(specifier);
      if (!shouldDeclareDependency(dependencyName) || declared.has(dependencyName)) {
        continue;
      }
      if (dependencyName.startsWith('@sdkwork/im-pc-') && dependencyName !== pkg.name) {
        missing.set(dependencyName, 'workspace:*');
        continue;
      }
      const version = dependencyName.startsWith('@sdkwork/') || dependencyName.startsWith('sdkwork-')
        ? 'workspace:*'
        : 'catalog:';
      missing.set(dependencyName, version);
    }
  }
  if (missing.size === 0) {
    continue;
  }
  pkg.dependencies = pkg.dependencies ?? {};
  for (const [name, version] of missing.entries()) {
    pkg.dependencies[name] = version;
  }
  fs.writeFileSync(packageJsonPath, `${JSON.stringify(pkg, null, 2)}\n`);
  fixes.push(`${pkg.name}: ${[...missing.keys()].join(', ')}`);
}

if (fixes.length === 0) {
  console.log('no fixes');
} else {
  console.log(fixes.join('\n'));
}
