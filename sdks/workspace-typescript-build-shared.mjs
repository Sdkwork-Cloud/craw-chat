import { spawnSync } from 'node:child_process';
import path from 'node:path';

export function resolveTypescriptGeneratedBuildPaths({
  workspaceRoot,
  relativeGeneratedRoot,
}) {
  const generatedRoot = path.join(workspaceRoot, relativeGeneratedRoot);
  return {
    generatedRoot,
    distRoot: path.join(generatedRoot, 'dist'),
  };
}

export function failTypescriptBuild({ prefix, message }) {
  console.error(`[${prefix}] ${message}`);
  process.exit(1);
}

export function runTypescriptBuildCommand({
  prefix,
  command,
  args,
  cwd,
  env,
  step,
}) {
  const result = spawnSync(command, args, {
    cwd,
    env,
    stdio: 'inherit',
    shell: false,
  });

  if (result.error) {
    failTypescriptBuild({
      prefix,
      message: `${step || command} failed to start: ${result.error.message}`,
    });
  }
  if (typeof result.status === 'number' && result.status !== 0) {
    failTypescriptBuild({
      prefix,
      message: `${step || command} failed with exit code ${result.status}`,
    });
  }
  if (result.signal) {
    failTypescriptBuild({
      prefix,
      message: `${step || command} terminated with signal ${result.signal}`,
    });
  }
}

export function runTypescriptBuildNpm({
  prefix,
  args,
  cwd,
  env,
  step,
}) {
  if (process.platform === 'win32') {
    runTypescriptBuildCommand({
      prefix,
      command: 'cmd.exe',
      args: ['/d', '/s', '/c', 'npm', ...args],
      cwd,
      env,
      step,
    });
    return;
  }

  runTypescriptBuildCommand({
    prefix,
    command: 'npm',
    args,
    cwd,
    env,
    step,
  });
}
