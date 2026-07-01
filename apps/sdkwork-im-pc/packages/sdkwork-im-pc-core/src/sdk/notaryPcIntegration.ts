import type { NotaryPcHostAdapter } from '@sdkwork/notary-pc-commons';

import {
  getNotaryAppSdkClient,
} from './notaryAppSdkClient';
import { getAppbaseAppSdkClient } from './appbaseAppSdkClient';
import { getDriveAppSdkClient } from './driveAppSdkClient';

let notaryPcRuntimeBootstrapped = false;
let imNotaryPcHost: NotaryPcHostAdapter | null = null;

function createImNotaryPcSdkPorts() {
  return {
    getNotaryClient: getNotaryAppSdkClient,
    getDriveClient: getDriveAppSdkClient,
    getAppbaseClient: getAppbaseAppSdkClient,
  };
}

export type NotaryPcRuntimeConfigurator = (options: {
  host: NotaryPcHostAdapter;
  sdkPorts: ReturnType<typeof createImNotaryPcSdkPorts>;
}) => void;

function resolveImNotaryPcHost(): NotaryPcHostAdapter {
  if (!imNotaryPcHost) {
    throw new Error('Notary PC host adapter is not configured. Bootstrap notary integration first.');
  }
  return imNotaryPcHost;
}

export function ensureNotaryPcRuntimeOnModule(
  configureRuntime: NotaryPcRuntimeConfigurator,
): void {
  configureRuntime({
    host: resolveImNotaryPcHost(),
    sdkPorts: createImNotaryPcSdkPorts(),
  });
  notaryPcRuntimeBootstrapped = true;
}

export async function bootstrapNotaryPcForIm(host: NotaryPcHostAdapter): Promise<void> {
  imNotaryPcHost = host;
  const { configureNotaryPcRuntime } = await import('@sdkwork/notary-pc-notary');
  ensureNotaryPcRuntimeOnModule(configureNotaryPcRuntime as NotaryPcRuntimeConfigurator);
}

export async function rebootstrapNotaryPcRuntimeForIm(): Promise<void> {
  if (!imNotaryPcHost) {
    return;
  }

  await bootstrapNotaryPcForIm(imNotaryPcHost);
}

export function isNotaryPcRuntimeBootstrapped(): boolean {
  return notaryPcRuntimeBootstrapped;
}

export async function resetNotaryPcRuntime(): Promise<void> {
  const { resetNotaryService } = await import('@sdkwork/notary-pc-notary');
  resetNotaryService();
  notaryPcRuntimeBootstrapped = false;
  imNotaryPcHost = null;
}
