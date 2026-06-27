import { readFileSync } from 'node:fs';
import path from 'node:path';

export function readPowerShellWrapperVerificationInputs({ workspaceRoot, scriptDir }) {
  return {
    assembleSource: readFileSync(path.join(scriptDir, 'assemble-sdk.ps1'), 'utf8'),
    generateSource: readFileSync(path.join(scriptDir, 'generate-sdk.ps1'), 'utf8'),
    verifySource: readFileSync(path.join(scriptDir, 'verify-sdk.ps1'), 'utf8'),
    readmeSource: readFileSync(path.join(workspaceRoot, 'README.md'), 'utf8'),
  };
}

export function finishPowerShellWrapperVerification({ prefix, failures, successMessage }) {
  if (failures.length > 0) {
    console.error(`[${prefix}] PowerShell wrapper argument verification failed:`);
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log(successMessage || `[${prefix}] PowerShell wrapper argument verification passed.`);
}

export function readWorkspacePowerShellWrapperSource({ workspaceRoot, relativePath }) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
}

export function appendLanguagePowerShellForwarderFailures({
  source,
  failures,
  label,
  rootScript,
  language,
}) {
  const normalizedRootScript = rootScript || 'verify-sdk.ps1';
  const rootScriptPattern = new RegExp(`bin\\\\${normalizedRootScript.replace('.', '\\.')}`);
  if (!rootScriptPattern.test(source)) {
    failures.push(`${label} must delegate to the root bin\\${normalizedRootScript} wrapper.`);
  }

  if (!new RegExp(`Languages\\s*=\\s*@\\("${language}"\\)`).test(source)) {
    failures.push(`${label} must pin Languages = @("${language}").`);
  }
}
