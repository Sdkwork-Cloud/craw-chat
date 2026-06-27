import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import path from 'node:path';

export function fail(prefix, message) {
  console.error(`[${prefix}] ${message}`);
  process.exit(1);
}

export function parseVerifyArgs(argv, { prefix }) {
  const parsed = {
    languages: [],
    withDart: false,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--language') {
      const value = (argv[index + 1] || '').trim().toLowerCase();
      if (!value) {
        fail(prefix, 'Missing value for --language');
      }
      parsed.languages.push(value);
      index += 1;
      continue;
    }
    if (current === '--with-dart') {
      parsed.withDart = true;
      continue;
    }
    fail(prefix, `Unknown argument: ${current}`);
  }

  return parsed;
}

export function normalizeLanguages({
  parsedArgs,
  defaultLanguages,
  supportedLanguages,
  prefix,
}) {
  const uniqueLanguages = [...new Set(
    parsedArgs.languages.length > 0 ? parsedArgs.languages : defaultLanguages,
  )];

  for (const language of uniqueLanguages) {
    if (!supportedLanguages.includes(language)) {
      fail(prefix, `Unsupported language: ${language}`);
    }
  }

  return uniqueLanguages;
}

export function runVerifyCommand({
  prefix,
  command,
  args,
  cwd,
  step,
  timeoutMs,
}) {
  const result = spawnSync(command, args, {
    cwd,
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

export function runWorkspaceVerificationPrelude({
  prefix,
  workspaceRoot,
  scriptDir,
  additionalSteps = [],
}) {
  const steps = [
    {
      step: 'workspace:automation',
      args: [path.join(scriptDir, 'verify-sdk-automation.mjs')],
    },
    {
      step: 'workspace:automation-meta-test',
      args: [path.join(workspaceRoot, 'tests', 'verify-sdk-automation.test.mjs')],
    },
    {
      step: 'workspace:assembly-meta-test',
      args: [path.join(workspaceRoot, 'tests', 'assemble-sdk.test.mjs')],
    },
    {
      step: 'workspace:powershell-wrapper-args',
      args: [path.join(scriptDir, 'verify-powershell-wrapper-args.mjs')],
    },
    ...additionalSteps,
  ];

  for (const step of steps) {
    runVerifyCommand({
      prefix,
      command: 'node',
      args: step.args,
      cwd: workspaceRoot,
      step: step.step,
    });
  }
}

export function runWorkspaceAssemblyStep({
  prefix,
  workspaceRoot,
  scriptDir,
  languages,
  step = 'workspace:assemble',
}) {
  runVerifyCommand({
    prefix,
    command: 'node',
    args: [
      path.join(scriptDir, 'assemble-sdk.mjs'),
      ...languages.flatMap((language) => ['--language', language]),
    ],
    cwd: workspaceRoot,
    step,
  });
}

export function ensureLanguageRequirements({
  workspaceRoot,
  languages,
  requirements,
  prefix,
}) {
  for (const language of languages) {
    const requiredPaths = requirements[language];
    if (!requiredPaths) {
      fail(prefix, `Unsupported language: ${language}`);
    }
    const missing = requiredPaths.filter(
      (relativePath) => !existsSync(path.join(workspaceRoot, relativePath)),
    );
    if (missing.length > 0) {
      fail(
        prefix,
        `${language} workspace is not ready. Missing required paths: ${missing.join(', ')}`,
      );
    }
  }
}
