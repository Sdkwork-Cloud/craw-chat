import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import path from 'node:path';

function fail(prefix, message) {
  console.error(`[${prefix}] ${message}`);
  process.exit(1);
}

export function assertNoArgs(argv, { prefix }) {
  if (argv.length > 0) {
    fail(prefix, `Unknown argument: ${argv[0]}`);
  }
}

export function parseWithDartArgs(argv, { prefix }) {
  const parsed = {
    withDart: false,
  };

  for (const current of argv) {
    if (current === '--with-dart') {
      parsed.withDart = true;
      continue;
    }
    fail(prefix, `Unknown argument: ${current}`);
  }

  return parsed;
}

function normalizePathEntry(value) {
  return String(value || '')
    .trim()
    .replace(/^"(.*)"$/, '$1')
    .replace(/[\\/]+$/, '');
}

export function resolveDartCommand(env = process.env) {
  if (process.platform !== 'win32') {
    return 'dart';
  }

  const candidates = new Set();
  const pushCandidate = (candidate) => {
    const normalizedCandidate = normalizePathEntry(candidate);
    if (!normalizedCandidate) {
      return;
    }
    candidates.add(normalizedCandidate);
  };

  pushCandidate(env.SDKWORK_DART);
  pushCandidate(env.DART_EXECUTABLE);
  if (env.FLUTTER_ROOT) {
    pushCandidate(path.join(env.FLUTTER_ROOT, 'bin', 'cache', 'dart-sdk', 'bin', 'dart.exe'));
  }

  for (const entry of (env.PATH || '').split(path.delimiter)) {
    const normalizedEntry = normalizePathEntry(entry);
    if (!normalizedEntry) {
      continue;
    }
    pushCandidate(path.join(normalizedEntry, 'dart.exe'));
    pushCandidate(path.join(normalizedEntry, 'cache', 'dart-sdk', 'bin', 'dart.exe'));
  }

  for (const candidate of candidates) {
    if (existsSync(candidate)) {
      return candidate;
    }
  }

  return 'dart';
}

export function buildDartEnv(workspaceRoot) {
  return {
    ...process.env,
    PUB_CACHE: path.join(workspaceRoot, '.sdkwork', 'dart', 'pub-cache'),
    DART_SUPPRESS_ANALYTICS: 'true',
    FLUTTER_SUPPRESS_ANALYTICS: 'true',
    CI: 'true',
  };
}

export function runWorkspaceCommand({
  prefix,
  command,
  args,
  cwd,
  env,
  step,
  timeoutMs,
}) {
  const resolvedCommand =
    command === 'dart' ? resolveDartCommand(env || process.env) : command;
  const result = spawnSync(resolvedCommand, args, {
    cwd,
    env,
    stdio: 'inherit',
    shell: false,
    timeout: timeoutMs,
  });

  if (result.error) {
    fail(prefix, `${step || command} failed to start: ${result.error.message}`);
  }
  if (typeof result.status === 'number' && result.status !== 0) {
    fail(prefix, `${step || command} failed with exit code ${result.status}`);
  }
  if (result.signal) {
    fail(prefix, `${step || command} terminated with signal ${result.signal}`);
  }
}
