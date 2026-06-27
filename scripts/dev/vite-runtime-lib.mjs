import fs from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '..', '..');

function resolveWorkspaceLayout(baseRepoRoot) {
  const normalizedRepoRoot = path.resolve(baseRepoRoot);
  const isWorktreeCheckout = path.basename(path.dirname(normalizedRepoRoot)) === '.worktrees';

  return {
    repoRoot: isWorktreeCheckout
      ? path.resolve(normalizedRepoRoot, '..', '..')
      : normalizedRepoRoot,
    worktreesRoot: isWorktreeCheckout
      ? path.resolve(normalizedRepoRoot, '..')
      : path.join(normalizedRepoRoot, '.worktrees'),
  };
}

function normalizeRelativeEntry(relativeEntry) {
  if (Array.isArray(relativeEntry)) {
    return relativeEntry;
  }

  return [relativeEntry];
}

function defaultFileExists(filePath) {
  return fs.existsSync(filePath);
}

function defaultPathEntryExists(filePath) {
  try {
    fs.lstatSync(filePath);
    return true;
  } catch {
    return false;
  }
}

function defaultSymlinkDirectory(targetPath, linkPath, platform = process.platform) {
  fs.symlinkSync(targetPath, linkPath, platform === 'win32' ? 'junction' : 'dir');
}

function defaultRenamePath(sourcePath, destinationPath) {
  fs.renameSync(sourcePath, destinationPath);
}

function listWorkspaceAppRoots(appsRoot) {
  if (!defaultFileExists(appsRoot)) {
    return [];
  }

  let entries;
  try {
    entries = fs.readdirSync(appsRoot, { withFileTypes: true });
  } catch {
    return [];
  }

  return entries
    .filter((entry) => entry.isDirectory() && !entry.name.startsWith('.'))
    .map((entry) => path.join(appsRoot, entry.name))
    .filter((candidateRoot) => (
      defaultFileExists(path.join(candidateRoot, 'package.json'))
      && defaultFileExists(path.join(candidateRoot, 'node_modules'))
    ));
}

function listWorkspaceWorktreeAppRoots(worktreesRoot) {
  if (!defaultFileExists(worktreesRoot)) {
    return [];
  }

  let entries;
  try {
    entries = fs.readdirSync(worktreesRoot, { withFileTypes: true });
  } catch {
    return [];
  }

  return entries
    .filter((entry) => entry.isDirectory() && !entry.name.startsWith('.'))
    .flatMap((entry) => listWorkspaceAppRoots(path.join(worktreesRoot, entry.name, 'apps')));
}

function listRelativeWorkspacePeerRoots(targetRoot, workspaceLayout) {
  const currentRepoRoot = path.resolve(repoRoot);
  const normalizedTargetRoot = path.resolve(targetRoot);
  const relativeTargetPath = path.relative(currentRepoRoot, normalizedTargetRoot);

  if (
    !relativeTargetPath
    || relativeTargetPath.startsWith('..')
    || path.isAbsolute(relativeTargetPath)
  ) {
    return [];
  }

  const canonicalRootCandidate = path.join(workspaceLayout.repoRoot, relativeTargetPath);
  const worktreeRootCandidates = defaultFileExists(workspaceLayout.worktreesRoot)
    ? fs.readdirSync(workspaceLayout.worktreesRoot, { withFileTypes: true })
      .filter((entry) => entry.isDirectory() && !entry.name.startsWith('.'))
      .map((entry) => path.join(workspaceLayout.worktreesRoot, entry.name, relativeTargetPath))
    : [];

  return [canonicalRootCandidate, ...worktreeRootCandidates]
    .map((candidateRoot) => path.resolve(candidateRoot))
    .filter((candidateRoot) => (
      candidateRoot !== normalizedTargetRoot
      && defaultFileExists(path.join(candidateRoot, 'package.json'))
      && defaultFileExists(path.join(candidateRoot, 'node_modules'))
    ));
}

function defaultOpenFile(filePath) {
  return fs.openSync(filePath, 'r');
}

function defaultCloseFile(fileDescriptor) {
  fs.closeSync(fileDescriptor);
}

function defaultResolveFromRoot(root, specifier) {
  return createRequire(path.join(root, 'package.json')).resolve(specifier);
}

function nodeModulesHasPackage(nodeModulesPath, packageName, fileExists = defaultFileExists) {
  return fileExists(path.join(nodeModulesPath, ...packageName.split('/'), 'package.json'));
}

export function probeReadableFile(
  filePath,
  {
    fileExists = defaultFileExists,
    openFile = defaultOpenFile,
    closeFile = defaultCloseFile,
  } = {},
) {
  if (!fileExists(filePath)) {
    return false;
  }

  try {
    const fileDescriptor = openFile(filePath);
    closeFile(fileDescriptor);
    return true;
  } catch {
    return false;
  }
}

function defaultIsReadable(filePath) {
  return probeReadableFile(filePath);
}

export function resolveWorkspaceDonorRoots(appRoot) {
  const normalizedAppRoot = path.resolve(appRoot);
  const workspaceLayout = resolveWorkspaceLayout(repoRoot);
  const knownWorkspaceApps = [
    ...listRelativeWorkspacePeerRoots(normalizedAppRoot, workspaceLayout),
    ...listWorkspaceAppRoots(path.join(workspaceLayout.repoRoot, 'apps')),
    ...listWorkspaceAppRoots(path.resolve(workspaceLayout.repoRoot, '..')),
    ...listWorkspaceWorktreeAppRoots(workspaceLayout.worktreesRoot),
  ];

  return knownWorkspaceApps
    .map((candidateRoot) => path.resolve(candidateRoot))
    .filter((candidateRoot, index, roots) => (
      candidateRoot !== normalizedAppRoot
      && roots.indexOf(candidateRoot) === index
    ));
}

export function ensureLocalNodeModules({
  appRoot,
  donorRoots = resolveWorkspaceDonorRoots(appRoot),
  requiredPackages = [],
  fileExists = defaultFileExists,
  pathEntryExists = defaultPathEntryExists,
  renamePath = defaultRenamePath,
  symlinkDirectory = defaultSymlinkDirectory,
  platform = process.platform,
} = {}) {
  if (!appRoot) {
    throw new Error('appRoot is required');
  }

  const normalizedAppRoot = path.resolve(appRoot);
  const localNodeModulesPath = path.join(normalizedAppRoot, 'node_modules');
  const hasRequiredPackages = requiredPackages.every((packageName) => (
    nodeModulesHasPackage(localNodeModulesPath, packageName, fileExists)
  ));

  if (fileExists(localNodeModulesPath) && hasRequiredPackages) {
    return localNodeModulesPath;
  }

  const donorNodeModulesPaths = donorRoots
    .map((candidateRoot) => path.join(path.resolve(candidateRoot), 'node_modules'))
    .filter((candidatePath, index, candidates) => (
      candidates.indexOf(candidatePath) === index
      && fileExists(candidatePath)
    ));
  const donorNodeModulesPath = donorNodeModulesPaths.find((candidatePath) => (
    requiredPackages.every((packageName) => (
      nodeModulesHasPackage(candidatePath, packageName, fileExists)
    ))
  )) ?? donorNodeModulesPaths[0];

  if (!donorNodeModulesPath) {
    throw new Error(
      `unable to materialize local node_modules for ${normalizedAppRoot}; no readable donor node_modules were found`,
    );
  }

  if (pathEntryExists(localNodeModulesPath)) {
    let backupIndex = 0;
    let backupPath = `${localNodeModulesPath}.__stale__donor`;
    while (pathEntryExists(backupPath)) {
      backupIndex += 1;
      backupPath = `${localNodeModulesPath}.__stale__donor_${backupIndex}`;
    }

    renamePath(localNodeModulesPath, backupPath);
  }

  try {
    symlinkDirectory(donorNodeModulesPath, localNodeModulesPath, platform);
  } catch (error) {
    if (!(error && error.code === 'EEXIST' && pathEntryExists(localNodeModulesPath))) {
      throw error;
    }
  }

  return localNodeModulesPath;
}

export function resolveReadablePackageEntry({
  appRoot,
  donorRoots = [],
  packageName,
  relativeEntry,
  fileExists = defaultFileExists,
  isReadable = defaultIsReadable,
}) {
  const entrySegments = normalizeRelativeEntry(relativeEntry);
  const candidateRoots = [appRoot, ...donorRoots]
    .map((candidateRoot) => path.resolve(candidateRoot))
    .filter((candidateRoot, index, roots) => roots.indexOf(candidateRoot) === index);

  for (const candidateRoot of candidateRoots) {
    const candidateEntry = path.join(
      candidateRoot,
      'node_modules',
      packageName,
      ...entrySegments,
    );
    if (fileExists(candidateEntry) && isReadable(candidateEntry)) {
      return candidateEntry;
    }
  }

  throw new Error(
    `unable to resolve a readable ${packageName} entry (${entrySegments.join('/')}) from ${candidateRoots.join(', ')}`,
  );
}

export function resolveReadablePackageImportUrl(options) {
  return pathToFileURL(resolveReadablePackageEntry(options)).href;
}

export function findReadableModuleResolution({
  appRoot,
  donorRoots = [],
  specifier,
  resolveFromRoot = defaultResolveFromRoot,
  isReadable = defaultIsReadable,
}) {
  const candidateRoots = [appRoot, ...donorRoots]
    .map((candidateRoot) => path.resolve(candidateRoot))
    .filter((candidateRoot, index, roots) => roots.indexOf(candidateRoot) === index);

  for (const candidateRoot of candidateRoots) {
    let resolvedPath;
    try {
      resolvedPath = resolveFromRoot(candidateRoot, specifier);
    } catch {
      continue;
    }

    if (isReadable(resolvedPath)) {
      return {
        candidateRoot,
        resolvedPath,
      };
    }
  }

  throw new Error(
    `unable to resolve readable module specifier "${specifier}" from ${candidateRoots.join(', ')}`,
  );
}

export function resolveReadableModuleSpecifier(options) {
  return findReadableModuleResolution(options).resolvedPath;
}

export function resolveReadablePackageRoot({
  relativeEntry,
  ...options
}) {
  return path.dirname(resolveReadablePackageEntry({
    ...options,
    relativeEntry: 'package.json',
  }));
}

export async function importReadablePackageDefault(options) {
  const moduleUrl = resolveReadablePackageImportUrl(options);
  const loadedModule = await import(moduleUrl);
  return loadedModule.default ?? loadedModule;
}

export async function applyWindowsVitePreload({
  platform = process.platform,
} = {}) {
  if (platform !== 'win32') {
    return;
  }

  await import(pathToFileURL(path.join(scriptDir, 'vite-windows-realpath-preload.mjs')).href);
}
