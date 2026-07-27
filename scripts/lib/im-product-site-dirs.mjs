import fs from 'node:fs';
import path from 'node:path';

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function resolveConfiguredSiteDir(value, { envName, repoRoot }) {
  const normalized = normalizeText(value);
  if (!normalized) {
    return undefined;
  }

  const siteDir = path.isAbsolute(normalized)
    ? path.normalize(normalized)
    : path.resolve(repoRoot, normalized);
  if (!fs.existsSync(path.join(siteDir, 'index.html'))) {
    throw new Error(`${envName} must reference a PC renderer directory containing index.html: ${siteDir}`);
  }
  return siteDir;
}

function resolveConfiguredSiteDirFromEnv(envNames, { env, repoRoot }) {
  for (const envName of envNames) {
    const siteDir = resolveConfiguredSiteDir(env[envName], { envName, repoRoot });
    if (siteDir) {
      return siteDir;
    }
  }
  return undefined;
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

function sameSiteDir(left, right) {
  const normalizeForComparison = (siteDir) => {
    const resolved = path.resolve(siteDir);
    return process.platform === 'win32' ? resolved.toLowerCase() : resolved;
  };
  return normalizeForComparison(left) === normalizeForComparison(right);
}

function resolveConfiguredPcSiteDir({ env, repoRoot }) {
  const configuredSiteDirs = [
    resolveConfiguredSiteDirFromEnv(
      ['SDKWORK_IM_ADMIN_SITE_DIR', 'SDKWORK_ADMIN_SITE_DIR'],
      { env, repoRoot },
    ),
    resolveConfiguredSiteDirFromEnv(
      ['SDKWORK_IM_PORTAL_SITE_DIR', 'SDKWORK_PORTAL_SITE_DIR'],
      { env, repoRoot },
    ),
  ].filter(Boolean);

  const [configuredSiteDir, ...otherSiteDirs] = configuredSiteDirs;
  if (configuredSiteDir && otherSiteDirs.some((siteDir) => !sameSiteDir(configuredSiteDir, siteDir))) {
    throw new Error(
      'SDKWORK_IM_ADMIN_SITE_DIR and SDKWORK_IM_PORTAL_SITE_DIR must reference '
      + 'the same shared apps/sdkwork-im-pc renderer build.',
    );
  }
  return configuredSiteDir;
}

export async function resolveImProductSiteDirEnv({
  env = process.env,
  repoRoot,
  runtimeSiteRoot,
}) {
  if (!repoRoot) {
    throw new Error('repoRoot is required when resolving the SDKWork IM PC renderer directory.');
  }

  const resolvedRuntimeSiteRoot = runtimeSiteRoot
    ?? path.join(repoRoot, '.runtime', 'dev-sites');
  const configuredSiteDir = resolveConfiguredPcSiteDir({ env, repoRoot });
  const pcDistDir = path.join(repoRoot, 'apps', 'sdkwork-im-pc', 'dist');
  const pcDevFallbackDir = path.join(resolvedRuntimeSiteRoot, 'sdkwork-im-pc');
  let pcSiteDir = configuredSiteDir;

  if (!pcSiteDir && fs.existsSync(path.join(pcDistDir, 'index.html'))) {
    pcSiteDir = pcDistDir;
  }
  if (!pcSiteDir) {
    writeDevSiteFallback(pcDevFallbackDir, 'Sdkwork IM PC Dev Renderer');
    pcSiteDir = pcDevFallbackDir;
  }

  return {
    SDKWORK_IM_ADMIN_SITE_DIR: pcSiteDir,
    SDKWORK_IM_PORTAL_SITE_DIR: pcSiteDir,
  };
}
