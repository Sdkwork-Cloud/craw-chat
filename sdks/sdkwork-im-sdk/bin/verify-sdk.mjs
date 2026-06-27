#!/usr/bin/env node
import { runVerifySdkFamily } from '../../workspace-im-v3-sdk-family.mjs';
import { sdkFamilyConfig } from './sdk-family-config.mjs';

await runVerifySdkFamily(sdkFamilyConfig, process.argv.slice(2));
