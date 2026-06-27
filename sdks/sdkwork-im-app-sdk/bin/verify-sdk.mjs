#!/usr/bin/env node
import { runVerifySdkFamily } from '../../workspace-im-v3-sdk-family.mjs';
import { sdkFamilyConfig } from './sdk-family-config.mjs';
import { verifyFlutterComposedWorkspace } from './verify-flutter-composed-workspace.mjs';
import { verifyFlutterComposedMethodCoverage } from './verify-flutter-composed-method-coverage.mjs';
import { verifyFlutterTypeScriptParity } from './verify-flutter-typescript-parity.mjs';

await runVerifySdkFamily(sdkFamilyConfig, process.argv.slice(2));
await verifyFlutterTypeScriptParity();
await verifyFlutterComposedWorkspace();
await verifyFlutterComposedMethodCoverage();
