import { spawn } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function existingSiteDir(value) {
  const normalized = normalizeText(value);
  if (!normalized || !fs.existsSync(normalized)) {
    return undefined;
  }
  return normalized;
}

export function writeDevSiteFallback(siteDir, title) {
  fs.mkdirSync(siteDir, { recursive: true });
  fs.writeFileSync(
    path.join(siteDir, 'index.html'),
    [
      '<!doctype html>',
      '<html lang="en">',
      '<head>',
      '  <meta charset="utf-8">',
      `  <title>${title}</title>`,
      '</head>',
      '<body>',
      `  <main>${title}</main>`,
      '</body>',
      '</html>',
      '',
    ].join('\n'),
  );
}

function runCommand(command, args, { cwd, env }) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd,
      env,
      stdio: 'inherit',
      shell: process.platform === 'win32',
    });

    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`${command} ${args.join(' ')} exited with code ${code ?? 'unknown'}`));
    });
  });
}

export async function ensureDevSiteDist({
  build,
  distDir,
  fallbackDir,
  label,
  onFallback,
  sourceDir,
  title,
}) {
  if (fs.existsSync(sourceDir)) {
    await build();
    return distDir;
  }

  writeDevSiteFallback(fallbackDir, title);
  onFallback?.({ fallbackDir, label, sourceDir });
  return fallbackDir;
}

export async function resolveImProductSiteDirEnv({
  buildEnv = process.env,
  env = process.env,
  onFallback,
  repoRoot,
  runtimeSiteRoot = path.join(repoRoot, '.runtime', 'dev-sites'),
}) {
  const adminSiteDir = existingSiteDir(env.SDKWORK_IM_ADMIN_SITE_DIR)
    ?? await ensureDevSiteDist({
      build: () => runCommand('pnpm', ['--dir', 'apps/sdkwork-im-admin', 'build'], {
        cwd: repoRoot,
        env: buildEnv,
      }),
      distDir: path.join(repoRoot, 'apps', 'sdkwork-im-admin', 'dist'),
      fallbackDir: path.join(runtimeSiteRoot, 'admin'),
      label: 'admin',
      onFallback,
      sourceDir: path.join(repoRoot, 'apps', 'sdkwork-im-admin'),
      title: 'Sdkwork IM Admin Dev Placeholder',
    });
  const portalSiteDir = existingSiteDir(env.SDKWORK_IM_PORTAL_SITE_DIR)
    ?? await ensureDevSiteDist({
      build: () => runCommand(process.execPath, ['apps/sdkwork-im-portal/scripts/build.mjs'], {
        cwd: repoRoot,
        env: buildEnv,
      }),
      distDir: path.join(repoRoot, 'apps', 'sdkwork-im-portal', 'dist'),
      fallbackDir: path.join(runtimeSiteRoot, 'portal'),
      label: 'portal',
      onFallback,
      sourceDir: path.join(repoRoot, 'apps', 'sdkwork-im-portal'),
      title: 'Sdkwork IM Portal Dev Placeholder',
    });

  return {
    SDKWORK_IM_ADMIN_SITE_DIR: adminSiteDir,
    SDKWORK_IM_PORTAL_SITE_DIR: portalSiteDir,
  };
}
