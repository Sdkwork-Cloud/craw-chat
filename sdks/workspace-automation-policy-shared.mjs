import {
  requireIncludes,
  requireMatch,
} from './workspace-automation-shared.mjs';

export function appendVerificationFlowDocumentationFailures({
  source,
  failures,
  label,
  requireAutomationMetaTest = false,
  requireAssemblyRegression = false,
  requireUsageSurface = false,
  requireDriveMediaSurface = false,
  requirePackageMetadata = false,
}) {
  if (requireAutomationMetaTest) {
    requireMatch({
      source,
      pattern: /automation meta-test/i,
      message: `${label} must document the automation meta-test in the verification flow.`,
      failures,
    });
  }

  if (requireAssemblyRegression) {
    requireMatch({
      source,
      pattern: /assembly regression/i,
      message: `${label} must document the assembly regression test in the verification flow.`,
      failures,
    });
  }

  if (requireUsageSurface) {
    requireMatch({
      source,
      pattern: /usage-surface/i,
      message: `${label} must document usage-surface verification terminology.`,
      failures,
    });
  }

  if (requireDriveMediaSurface) {
    requireMatch({
      source,
      pattern: /Drive media surface/i,
      message: `${label} must document Drive media surface verification terminology.`,
      failures,
    });
  }

  if (requirePackageMetadata) {
    requireMatch({
      source,
      pattern: /package metadata/i,
      message: `${label} must document package metadata verification.`,
      failures,
    });
  }
}

export function appendAssemblyMetadataDocumentationFailures({
  source,
  failures,
  label,
  explainAssemblyMetadata = false,
  requireGeneratedComposed = false,
}) {
  const verb = explainAssemblyMetadata ? 'explain' : 'document';
  const assemblyMetadataPhrase = explainAssemblyMetadata
    ? 'in the assembly metadata'
    : 'assembly metadata';

  requireMatch({
    source,
    pattern: /\.sdkwork-assembly\.json/,
    message: `${label} must ${verb} .sdkwork-assembly.json.`,
    failures,
  });
  requireMatch({
    source,
    pattern: /manifestPath/,
    message: `${label} must ${verb} manifestPath ${assemblyMetadataPhrase}.`,
    failures,
  });
  requireMatch({
    source,
    pattern: /generatedAt/,
    message: `${label} must ${verb} generatedAt ${assemblyMetadataPhrase}.`,
    failures,
  });

  if (requireGeneratedComposed) {
    requireMatch({
      source,
      pattern: /generated[\s\S]*composed/i,
      message: `${label} must ${verb} generated versus composed package layers ${assemblyMetadataPhrase}.`,
      failures,
    });
  }
}

export function appendVerifySdkAutomationEntrypointFailures({
  source,
  failures,
  verb = 'run',
}) {
  appendScriptInvocationFailures({
    source,
    failures,
    label: 'verify-sdk.mjs',
    verb,
    invocations: [
      {
        pattern: /verify-sdk-automation\.mjs/,
        description: 'verify-sdk-automation.mjs',
      },
    ],
  });
}

export function appendGitignorePatternFailures({
  source,
  failures,
  label,
  patterns,
}) {
  for (const pattern of patterns) {
    requireIncludes({
      source,
      value: pattern,
      message: `${label} must ignore ${pattern}.`,
      failures,
    });
  }
}

export function appendScriptInvocationFailures({
  source,
  failures,
  label,
  invocations,
  verb = 'invoke',
}) {
  for (const invocation of invocations) {
    requireMatch({
      source,
      pattern: invocation.pattern,
      message: invocation.message ?? `${label} must ${verb} ${invocation.description}.`,
      failures,
    });
  }
}
