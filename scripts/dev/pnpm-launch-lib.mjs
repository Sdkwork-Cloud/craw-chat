import path from 'node:path';
import process from 'node:process';

function quotePowerShellLiteral(value) {
  return `'${String(value).replaceAll("'", "''")}'`;
}

function pnpmExecutable(platform = process.platform, execPath = process.execPath) {
  return platform === 'win32' ? execPath : 'pnpm';
}

function pnpmArgumentPrefix({
  platform = process.platform,
  execPath = process.execPath,
} = {}) {
  if (platform !== 'win32') {
    return [];
  }

  const normalizedExecPath = path.normalize(execPath);
  return [path.join(path.dirname(normalizedExecPath), 'node_modules', 'pnpm', 'bin', 'pnpm.cjs')];
}

function windowsPnpmLauncherArgs(stepArgs = [], options = {}) {
  const command = pnpmExecutable('win32', options.execPath);
  const commandArgs = [
    ...pnpmArgumentPrefix({
      platform: 'win32',
      execPath: options.execPath,
    }),
    ...stepArgs,
  ];
  const commandLine = ['&', quotePowerShellLiteral(command), ...commandArgs.map(quotePowerShellLiteral)].join(' ');

  return [
    '-NoProfile',
    '-ExecutionPolicy',
    'Bypass',
    '-Command',
    commandLine,
  ];
}

export function pnpmProcessSpec(stepArgs = [], {
  platform = process.platform,
  execPath = process.execPath,
} = {}) {
  if (platform === 'win32') {
    return {
      command: 'powershell.exe',
      args: windowsPnpmLauncherArgs(stepArgs, { execPath }),
    };
  }

  return {
    command: pnpmExecutable(platform, execPath),
    args: stepArgs,
  };
}
