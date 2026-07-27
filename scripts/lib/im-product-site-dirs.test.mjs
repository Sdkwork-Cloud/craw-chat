import assert from 'node:assert/strict';
import { mkdtemp, mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { readFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { resolveImProductSiteDirEnv } from './im-product-site-dirs.mjs';

const repoRoot = path.resolve(import.meta.dirname, '..', '..');

async function writeSiteIndex(siteDir, title = 'SDKWork IM PC') {
  await mkdir(siteDir, { recursive: true });
  await writeFile(path.join(siteDir, 'index.html'), `<!doctype html><title>${title}</title>`);
}

test('product site resolution uses the canonical PC build for every route surface', async () => {
  const tempRoot = await mkdtemp(path.join(os.tmpdir(), 'sdkwork-im-pc-site-dist-'));
  try {
    const pcDistDir = path.join(tempRoot, 'apps', 'sdkwork-im-pc', 'dist');
    await writeSiteIndex(pcDistDir);

    const resolved = await resolveImProductSiteDirEnv({ env: {}, repoRoot: tempRoot });
    assert.equal(resolved.SDKWORK_IM_ADMIN_SITE_DIR, pcDistDir);
    assert.equal(resolved.SDKWORK_IM_PORTAL_SITE_DIR, pcDistDir);
  } finally {
    await rm(tempRoot, { force: true, recursive: true });
  }
});

test('product site resolution creates one shared fallback when the PC build is absent', async () => {
  const tempRoot = await mkdtemp(path.join(os.tmpdir(), 'sdkwork-im-pc-site-fallback-'));
  try {
    const resolved = await resolveImProductSiteDirEnv({ env: {}, repoRoot: tempRoot });
    const expectedFallback = path.join(tempRoot, '.runtime', 'dev-sites', 'sdkwork-im-pc');

    assert.equal(resolved.SDKWORK_IM_ADMIN_SITE_DIR, expectedFallback);
    assert.equal(resolved.SDKWORK_IM_PORTAL_SITE_DIR, expectedFallback);
    assert.match(
      await readFile(path.join(expectedFallback, 'index.html'), 'utf8'),
      /Sdkwork IM PC Dev Renderer/u,
    );
  } finally {
    await rm(tempRoot, { force: true, recursive: true });
  }
});

test('one configured site key selects the shared PC renderer for both compatibility keys', async () => {
  const tempRoot = await mkdtemp(path.join(os.tmpdir(), 'sdkwork-im-pc-site-config-'));
  try {
    const configuredSiteDir = path.join(tempRoot, 'renderer');
    await writeSiteIndex(configuredSiteDir);

    const resolved = await resolveImProductSiteDirEnv({
      env: { SDKWORK_PORTAL_SITE_DIR: configuredSiteDir },
      repoRoot: tempRoot,
    });
    assert.equal(resolved.SDKWORK_IM_ADMIN_SITE_DIR, configuredSiteDir);
    assert.equal(resolved.SDKWORK_IM_PORTAL_SITE_DIR, configuredSiteDir);
  } finally {
    await rm(tempRoot, { force: true, recursive: true });
  }
});

test('product site resolution reports an actionable error when repoRoot is missing', async () => {
  await assert.rejects(
    resolveImProductSiteDirEnv({ env: {} }),
    /repoRoot is required when resolving the SDKWork IM PC renderer directory/u,
  );
});

test('divergent admin and portal site directories fail the shared PC renderer boundary', async () => {
  const tempRoot = await mkdtemp(path.join(os.tmpdir(), 'sdkwork-im-pc-site-conflict-'));
  try {
    const adminSiteDir = path.join(tempRoot, 'admin');
    const portalSiteDir = path.join(tempRoot, 'portal');
    await Promise.all([writeSiteIndex(adminSiteDir, 'admin'), writeSiteIndex(portalSiteDir, 'portal')]);

    await assert.rejects(
      resolveImProductSiteDirEnv({
        env: {
          SDKWORK_IM_ADMIN_SITE_DIR: adminSiteDir,
          SDKWORK_IM_PORTAL_SITE_DIR: portalSiteDir,
        },
        repoRoot: tempRoot,
      }),
      /must reference the same shared apps\/sdkwork-im-pc renderer build/u,
    );
  } finally {
    await rm(tempRoot, { force: true, recursive: true });
  }
});

test('runtime and desktop asset entrypoints do not reference retired frontend app roots', () => {
  const retiredAppRootPattern = /sdkwork-im-(?:admin|portal)|apps[\\/]control-plane/u;
  const executableSources = [
    'scripts/lib/im-product-site-dirs.mjs',
    'scripts/build-sdkwork-im-desktop-assets.mjs',
    'crates/sdkwork-api-product-runtime/src/lib.rs',
  ];

  for (const relativePath of executableSources) {
    const source = readFileSync(path.join(repoRoot, relativePath), 'utf8');
    assert.doesNotMatch(source, retiredAppRootPattern, `${relativePath} must use apps/sdkwork-im-pc`);
  }

  for (const relativePath of ['scripts/gateway-standalone-run.mjs', 'scripts/lib/im-pc-dev.mjs']) {
    const source = readFileSync(path.join(repoRoot, relativePath), 'utf8');
    assert.doesNotMatch(source, /source not found at/u, `${relativePath} must not report retired app roots`);
  }
});
