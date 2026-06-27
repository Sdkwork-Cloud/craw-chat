#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const openApiPath = 'apis/open-api/im/sdkwork-im-im.openapi.yaml';

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function normalizePathTemplate(pathTemplate) {
  return pathTemplate.replace(/\{[^}]+\}/gu, '{}').toLowerCase();
}

function extractOpenApiRoutes(yaml, prefix) {
  const routes = [];
  const lines = yaml.split(/\r?\n/u);
  let currentPath = null;

  for (const line of lines) {
    const pathMatch = line.match(/^  (\/im\/v3\/api\/[^\s:]+):\s*$/u);
    if (pathMatch) {
      currentPath = pathMatch[1];
      continue;
    }

    if (!currentPath || !currentPath.startsWith(prefix)) {
      continue;
    }

    const methodMatch = line.match(/^    (get|post|put|patch|delete):\s*$/u);
    if (methodMatch) {
      routes.push({
        method: methodMatch[1].toUpperCase(),
        path: currentPath,
        normalized: `${methodMatch[1].toUpperCase()} ${normalizePathTemplate(currentPath)}`,
      });
    }
  }

  return routes;
}

function extractMountedRoutesFromSource(source) {
  const routes = new Set();
  const lines = source.split(/\r?\n/u);

  for (let index = 0; index < lines.length; index += 1) {
    const inlineRouteMatch = lines[index].match(
      /\.route\s*\(\s*"(\/im\/v3\/api\/[^"]+)"\s*,([\s\S]*)\)\s*[,;]?/u,
    );
    if (inlineRouteMatch) {
      for (const handlerMatch of inlineRouteMatch[2].matchAll(/\b(get|post|put|patch|delete)\(/gu)) {
        routes.add(`${handlerMatch[1].toUpperCase()} ${normalizePathTemplate(inlineRouteMatch[1])}`);
      }
      continue;
    }

    const pathMatch = lines[index].match(/^\s*"(\/im\/v3\/api\/[^"]+)"\s*,?\s*$/u);
    if (!pathMatch) {
      continue;
    }

    const routePath = pathMatch[1];
    const window = lines.slice(index, index + 8).join('\n');
    for (const handlerMatch of window.matchAll(/\b(get|post|put|patch|delete)\(/gu)) {
      routes.add(`${handlerMatch[1].toUpperCase()} ${normalizePathTemplate(routePath)}`);
    }
  }

  return routes;
}

function collectMountedRoutes(relativePaths) {
  const mounted = new Set();
  for (const relativePath of relativePaths) {
    const source = read(relativePath);
    for (const route of extractMountedRoutesFromSource(source)) {
      mounted.add(route);
    }
  }
  return mounted;
}

const openApiYaml = read(openApiPath);

const socialRouteSources = [
  'services/social-service/src/openapi.rs',
  'services/social-service/src/openapi_contacts.rs',
  'crates/sdkwork-routes-im-social-open-api/src/routes.rs',
];

const socialMounted = collectMountedRoutes(socialRouteSources);
const socialOpenApi = extractOpenApiRoutes(openApiYaml, '/im/v3/api/social');
const socialMissing = socialOpenApi.filter((route) => !socialMounted.has(route.normalized));

assert.equal(
  socialMissing.length,
  0,
  `social open-api routes missing HTTP mounts:\n${socialMissing
    .map((route) => `- ${route.method} ${route.path}`)
    .join('\n')}`,
);

const prefixCoverage = [
  {
    prefix: '/im/v3/api/social',
    routeSources: socialRouteSources,
    required: true,
  },
  {
    prefix: '/im/v3/api/chat',
    routeSources: [
      'crates/sdkwork-routes-im-chat-open-api/src/routes.rs',
      'crates/sdkwork-routes-im-projection-open-api/src/routes.rs',
      'services/projection-service/src/http.rs',
      'services/sdkwork-comms-conversation-service/src/runtime/http.rs',
    ],
    required: false,
  },
  {
    prefix: '/im/v3/api/realtime',
    routeSources: [
      'services/session-gateway/src/lib.rs',
      'crates/sdkwork-routes-im-realtime-open-api/src/lib.rs',
    ],
    required: false,
  },
  {
    prefix: '/im/v3/api/spaces',
    routeSources: ['crates/sdkwork-routes-im-space-open-api/src/routes.rs', 'services/space-service/src/http.rs'],
    required: false,
  },
];

const coverageReport = [];
for (const entry of prefixCoverage) {
  const openApiRoutes = extractOpenApiRoutes(openApiYaml, entry.prefix);
  const mounted = collectMountedRoutes(entry.routeSources.filter((relativePath) => {
    const absolutePath = path.join(repoRoot, relativePath);
    return fs.existsSync(absolutePath);
  }));
  const missing = openApiRoutes.filter((route) => !mounted.has(route.normalized));
  coverageReport.push({
    prefix: entry.prefix,
    openApiCount: openApiRoutes.length,
    mountedCount: mounted.size,
    missingCount: missing.length,
    missing: missing.slice(0, 12),
  });

  if (entry.required) {
    assert.equal(missing.length, 0, `${entry.prefix} must have full open-api route coverage`);
  }
}

process.stdout.write('sdkwork-im open-api route coverage contract passed\n');
for (const entry of coverageReport) {
  process.stdout.write(
    `- ${entry.prefix}: openapi=${entry.openApiCount}, mounted=${entry.mountedCount}, missing=${entry.missingCount}\n`,
  );
  if (entry.missingCount > 0) {
    for (const route of entry.missing) {
      process.stdout.write(`    gap: ${route.method} ${route.path}\n`);
    }
  }
}
