#!/usr/bin/env node

import path from 'node:path';
import process from 'node:process';
import { fileURLToPath, pathToFileURL } from 'node:url';

import {
  DEFAULT_SDKWORK_IM_PC_DEV_HOST,
  SDKWORK_IM_PC_DEV_HOST_ENV,
  SDKWORK_IM_PC_DEV_PORT_ENV,
  resolveAvailableSdkworkChatPcDevPort,
  resolveSdkworkChatPcDevServer,
} from '../lib/im-pc-dev.mjs';
import { resolveSdkworkImPcViteDevEnv } from '../lib/im-pc-vite-dev-env.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const viteRunnerPath = path.join(scriptDir, 'run-vite-cli.mjs');

const configuredServer = resolveSdkworkChatPcDevServer({
  env: process.env,
  host: process.env[SDKWORK_IM_PC_DEV_HOST_ENV] ?? DEFAULT_SDKWORK_IM_PC_DEV_HOST,
});
const selectedPort = process.env[SDKWORK_IM_PC_DEV_PORT_ENV]
  ? configuredServer.port
  : await resolveAvailableSdkworkChatPcDevPort({
    env: process.env,
    host: configuredServer.host,
  });

process.env[SDKWORK_IM_PC_DEV_HOST_ENV] = process.env[SDKWORK_IM_PC_DEV_HOST_ENV]
  ?? configuredServer.host;
process.env[SDKWORK_IM_PC_DEV_PORT_ENV] = String(selectedPort);

const resolvedViteEnv = await resolveSdkworkImPcViteDevEnv(process.env);
Object.assign(process.env, resolvedViteEnv);

process.argv = [
  process.argv[0],
  viteRunnerPath,
  '--host',
  process.env[SDKWORK_IM_PC_DEV_HOST_ENV],
  '--port',
  String(selectedPort),
  ...process.argv.slice(2),
];

await import(pathToFileURL(viteRunnerPath).href);
