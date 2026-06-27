#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import {
  collectExpectationFailures,
  finishFileExpectationVerification,
  readWorkspaceSource,
  workspacePathExists,
} from '../../workspace-file-expectation-shared.mjs';

const PREFIX = 'sdkwork-im-app-sdk';

function requiredFiles() {
  return [
    'sdkwork-im-app-sdk-flutter/composed/README.md',
    'sdkwork-im-app-sdk-flutter/composed/pubspec.yaml',
    'sdkwork-im-app-sdk-flutter/composed/pubspec_overrides.yaml',
    'sdkwork-im-app-sdk-flutter/composed/lib/im_app_sdk.dart',
    'sdkwork-im-app-sdk-flutter/composed/lib/src/context.dart',
    'sdkwork-im-app-sdk-flutter/composed/lib/src/types.dart',
    'sdkwork-im-app-sdk-flutter/composed/lib/src/portal_module.dart',
    'sdkwork-im-app-sdk-flutter/composed/lib/src/notifications_module.dart',
    'sdkwork-im-app-sdk-flutter/composed/lib/src/automation_module.dart',
    'sdkwork-im-app-sdk-flutter/composed/lib/src/provider_module.dart',
    'sdkwork-im-app-sdk-flutter/composed/lib/src/rtc_module.dart',
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
    'sdkwork-im-app-sdk-flutter/composed/pubspec.yaml',
  );
  const pubspecOverridesSource = readIfExists(
    root,
    'sdkwork-im-app-sdk-flutter/composed/pubspec_overrides.yaml',
  );
  const sdkSource = readIfExists(
    root,
    'sdkwork-im-app-sdk-flutter/composed/lib/im_app_sdk.dart',
  );

  const expectations = [
    {
      description: 'composed pubspec name is im_app_sdk',
      source: pubspecSource,
      pattern: /^name:\s*im_app_sdk\s*$/m,
    },
    {
      description: 'composed pubspec depends on generated im_app_api_generated package',
      source: pubspecSource,
      pattern: /^\s*im_app_api_generated:\s*.+$/m,
    },
    {
      description: 'composed pubspec depends on rtc_sdk as dependency SDK',
      source: pubspecSource,
      pattern: /^\s*rtc_sdk:\s*.+$/m,
    },
    {
      description: 'composed override pins generated package to ../generated/server-openapi',
      source: pubspecOverridesSource,
      pattern: /^\s*path:\s*\.\.\/generated\/server-openapi\s*$/m,
    },
    {
      description: 'composed override pins rtc_sdk dependency workspace',
      source: pubspecOverridesSource,
      pattern: /^\s*path:\s*(?:\.\.\/\.\.\/\.\.\/\.\.\/\.sdkwork\/dependencies\/sdkwork-rtc|\.\.\/\.\.\/\.\.\/\.\.\/\.\.\/sdkwork-rtc)\/sdks\/sdkwork-rtc-sdk\/sdkwork-rtc-sdk-flutter\s*$/m,
    },
    {
      description: 'composed override pins sdkwork_common_flutter dependency workspace',
      source: pubspecOverridesSource,
      pattern: /^\s*path:\s*(?:\.\.\/\.\.\/\.\.\/\.\.\/\.sdkwork\/dependencies\/sdkwork-sdk-commons|\.\.\/\.\.\/\.\.\/\.\.\/\.\.\/sdkwork-sdk-commons)\/sdkwork-sdk-common-flutter\s*$/m,
    },
    {
      description: 'composed sdk re-exports generated app SDK',
      source: sdkSource,
      pattern: /export 'package:im_app_api_generated\/im_app_api_generated\.dart';/,
    },
    {
      description: 'composed sdk defines ImAppSdkClient',
      source: sdkSource,
      pattern: /class ImAppSdkClient\s*{/,
    },
    {
      description: 'composed sdk exposes module portal',
      source: sdkSource,
      pattern: /late final ImAppPortalModule portal;/,
    },
    {
      description: 'composed sdk exposes module notifications',
      source: sdkSource,
      pattern: /late final ImAppNotificationsModule notifications;/,
    },
    {
      description: 'composed sdk exposes module automation',
      source: sdkSource,
      pattern: /late final ImAppAutomationModule automation;/,
    },
    {
      description: 'composed sdk exposes module provider',
      source: sdkSource,
      pattern: /late final ImAppProviderModule provider;/,
    },
    {
      description: 'composed sdk exposes module rtc',
      source: sdkSource,
      pattern: /late final ImAppRtcModule rtc;/,
    },
    {
      description: 'composed sdk exposes rtc data source from dependency SDK',
      source: sdkSource,
      pattern: /RtcDataSource get rtcDataSource => _context\.rtcDataSource;/,
    },
  ];

  const failures = [...missing, ...collectExpectationFailures(expectations)];
  if (/ImAppDeviceModule|DeviceApi|get deviceApi|src\/device_module/u.test(sdkSource)) {
    failures.push('composed sdk must not expose IM app device module after device ownership moved to sdkwork-aiot');
  }
  if (/ImAppIotModule|IotApi|get iotApi|src\/iot_module/u.test(sdkSource)) {
    failures.push('composed sdk must not expose IM app iot module after AIoT ownership moved to sdkwork-aiot');
  }
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
