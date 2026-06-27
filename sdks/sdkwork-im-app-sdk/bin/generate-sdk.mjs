#!/usr/bin/env node
import { runGenerateSdkFamily } from '../../workspace-im-v3-sdk-family.mjs';
import { sdkFamilyConfig } from './sdk-family-config.mjs';

await runGenerateSdkFamily(sdkFamilyConfig, process.argv.slice(2));
