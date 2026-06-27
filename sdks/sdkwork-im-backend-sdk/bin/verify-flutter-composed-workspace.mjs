#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import {
  collectExpectationFailures,
  finishFileExpectationVerification,
  readWorkspaceSource,
  workspacePathExists,
} from '../../workspace-file-expectation-shared.mjs';

const PREFIX = 'sdkwork-im-backend-sdk';

function requiredFiles() {
  return [
    'sdkwork-im-backend-sdk-flutter/composed/README.md',
    'sdkwork-im-backend-sdk-flutter/composed/pubspec.yaml',
    'sdkwork-im-backend-sdk-flutter/composed/pubspec_overrides.yaml',
    'sdkwork-im-backend-sdk-flutter/composed/lib/im_backend_sdk.dart',
    'sdkwork-im-backend-sdk-flutter/composed/lib/src/context.dart',
    'sdkwork-im-backend-sdk-flutter/composed/lib/src/types.dart',
    'sdkwork-im-backend-sdk-flutter/composed/lib/src/ops_module.dart',
    'sdkwork-im-backend-sdk-flutter/composed/lib/src/audit_module.dart',
    'sdkwork-im-backend-sdk-flutter/composed/lib/src/automation_module.dart',
    'sdkwork-im-backend-sdk-flutter/composed/lib/src/control_module.dart',
    'sdkwork-im-backend-sdk-flutter/composed/lib/src/admin_module.dart',
  ];
}

function readIfExists(workspaceRoot, relativePath) {
  if (!workspacePathExists({ workspaceRoot, relativePath })) {
    return '';
  }
  return readWorkspaceSource({ workspaceRoot, relativePath });
}

export function verifyFlutterComposedWorkspace(workspaceRoot) {
  const root = workspaceRoot ?? path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
  const required = requiredFiles();
  const missing = required
    .filter((relativePath) => !workspacePathExists({ workspaceRoot: root, relativePath }))
    .map((relativePath) => `required file ${relativePath}`);

  const pubspecSource = readIfExists(
    root,
    'sdkwork-im-backend-sdk-flutter/composed/pubspec.yaml',
  );
  const pubspecOverridesSource = readIfExists(
    root,
    'sdkwork-im-backend-sdk-flutter/composed/pubspec_overrides.yaml',
  );
  const sdkSource = readIfExists(
    root,
    'sdkwork-im-backend-sdk-flutter/composed/lib/im_backend_sdk.dart',
  );

  const expectations = [
    {
      description: 'composed pubspec name is im_backend_sdk',
      source: pubspecSource,
      pattern: /^name:\s*im_backend_sdk\s*$/m,
    },
    {
      description: 'composed pubspec depends on generated im_backend_api_generated package',
      source: pubspecSource,
      pattern: /^\s*im_backend_api_generated:\s*.+$/m,
    },
    {
      description: 'composed override pins generated package to ../generated/server-openapi',
      source: pubspecOverridesSource,
      pattern: /^\s*path:\s*\.\.\/generated\/server-openapi\s*$/m,
    },
    {
      description: 'composed sdk re-exports generated backend SDK',
      source: sdkSource,
      pattern: /export 'package:im_backend_api_generated\/im_backend_api_generated\.dart';/,
    },
    {
      description: 'composed sdk defines ImBackendSdkClient',
      source: sdkSource,
      pattern: /class ImBackendSdkClient\s*{/,
    },
    {
      description: 'composed sdk exposes module ops',
      source: sdkSource,
      pattern: /late final ImBackendOpsModule ops;/,
    },
    {
      description: 'composed sdk exposes module audit',
      source: sdkSource,
      pattern: /late final ImBackendAuditModule audit;/,
    },
    {
      description: 'composed sdk exposes module automation',
      source: sdkSource,
      pattern: /late final ImBackendAutomationModule automation;/,
    },
    {
      description: 'composed sdk exposes module control',
      source: sdkSource,
      pattern: /late final ImBackendControlModule control;/,
    },
    {
      description: 'composed sdk exposes module admin',
      source: sdkSource,
      pattern: /late final ImBackendAdminModule admin;/,
    },
  ];

  const failures = [...missing, ...collectExpectationFailures(expectations)];
  finishFileExpectationVerification({
    prefix: PREFIX,
    failures,
    successMessage: `[${PREFIX}] Flutter composed workspace verification passed.`,
  });
}

const invokedPath = process.argv[1] ? pathToFileURL(path.resolve(process.argv[1])).href : null;
const isCliEntry = invokedPath === import.meta.url;

if (isCliEntry) {
  verifyFlutterComposedWorkspace();
}
