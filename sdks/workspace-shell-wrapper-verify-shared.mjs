import { readFileSync } from 'node:fs';
import path from 'node:path';

export function readWorkspaceSource({ workspaceRoot, relativePath }) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
}

export function appendShellWrapperForwardingFailures({
  source,
  failures,
  label,
  language,
  rootScript,
}) {
  const expectedRootScript = rootScript || 'verify-sdk.sh';
  const rootScriptPattern = new RegExp(`bin/${expectedRootScript.replace('.', '\\.')}`);

  if (!rootScriptPattern.test(source)) {
    failures.push(`${label} must delegate to the root bin/${expectedRootScript} wrapper.`);
  }

  if (!new RegExp(`--language ${language}`).test(source)) {
    failures.push(`${label} must pin --language ${language}.`);
  }

  if (!/"\$@"/.test(source)) {
    failures.push(`${label} must preserve all trailing shell arguments via "$@".`);
  }
}

export function finishShellWrapperVerification({ prefix, failures, successMessage }) {
  if (failures.length > 0) {
    console.error(`[${prefix}] Shell wrapper argument verification failed:`);
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log(successMessage || `[${prefix}] Shell wrapper argument verification passed.`);
}
