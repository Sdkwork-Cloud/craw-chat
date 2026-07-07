import { configureCommunityPcHost } from '@sdkwork/community-pc-community';
import '@sdkwork/community-pc-community/i18n';
import {
  createImCommunityPcHostAdapter,
  type CreateImCommunityPcHostAdapterOptions,
} from './createImCommunityPcHostAdapter';

let communityPcHostBootstrapped = false;

export function bootstrapImCommunityPcHost(
  options: CreateImCommunityPcHostAdapterOptions,
): void {
  configureCommunityPcHost(createImCommunityPcHostAdapter(options));
  communityPcHostBootstrapped = true;
}

export function isImCommunityPcHostBootstrapped(): boolean {
  return communityPcHostBootstrapped;
}

export function resetImCommunityPcHostBootstrap(): void {
  communityPcHostBootstrapped = false;
}
