import {
  requireIncludes,
  requireMatch,
} from './workspace-automation-shared.mjs';

export function appendVerificationFlowDocumentationFailures({
  source,
  failures,
  label,
  requireAutomationMetaTest = false,
  requireSdkManifestRegression = false,
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

  if (requireSdkManifestRegression) {
    requireMatch({
      source,
      pattern: /SDK manifest regression/i,
      message: `${label} must document the SDK manifest regression test in the verification flow.`,
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

export function appendSdkManifestMetadataDocumentationFailures({
  source,
  failures,
  label,
  explainSdkManifestMetadata = false,
  requireGeneratedComposed = false,
}) {
  const verb = explainSdkManifestMetadata ? 'explain' : 'document';
  const sdkManifestMetadataPhrase = explainSdkManifestMetadata
    ? 'in the SDK manifest metadata'
    : 'SDK manifest metadata';

  requireMatch({
    source,
    pattern: /sdk-manifest\.json/,
    message: `${label} must ${verb} sdk-manifest.json.`,
    failures,
  });
  requireMatch({
    source,
    pattern: /manifestPath/,
    message: `${label} must ${verb} manifestPath ${sdkManifestMetadataPhrase}.`,
    failures,
  });
  requireMatch({
    source,
    pattern: /transportPackageName/,
    message: `${label} must ${verb} transportPackageName ${sdkManifestMetadataPhrase}.`,
    failures,
  });

  if (requireGeneratedComposed) {
    requireMatch({
      source,
      pattern: /generated[\s\S]*composed/i,
      message: `${label} must ${verb} generated versus composed package layers ${sdkManifestMetadataPhrase}.`,
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
