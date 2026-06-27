import { configureNotaryPcRuntime, resetNotaryService } from '@sdkwork/notary-pc-notary';
import type { NotaryPcHostAdapter } from '@sdkwork/notary-pc-commons';

import {
  getNotaryAppSdkClient,
} from './notaryAppSdkClient';
import { getAppbaseAppSdkClient } from './appbaseAppSdkClient';
import { getDriveAppSdkClient } from './driveAppSdkClient';

let notaryPcRuntimeBootstrapped = false;
let imNotaryPcHost: NotaryPcHostAdapter | null = null;

export function bootstrapNotaryPcForIm(host: NotaryPcHostAdapter): void {
  imNotaryPcHost = host;
  configureNotaryPcRuntime({
    host,
    sdkPorts: {
      getNotaryClient: getNotaryAppSdkClient,
      getDriveClient: getDriveAppSdkClient,
      getAppbaseClient: getAppbaseAppSdkClient,
    },
  });
  notaryPcRuntimeBootstrapped = true;
}

export function rebootstrapNotaryPcRuntimeForIm(): void {
  if (!imNotaryPcHost) {
    return;
  }

  bootstrapNotaryPcForIm(imNotaryPcHost);
}

export function isNotaryPcRuntimeBootstrapped(): boolean {
  return notaryPcRuntimeBootstrapped;
}

export function resetNotaryPcRuntime(): void {
  resetNotaryService();
  notaryPcRuntimeBootstrapped = false;
}
