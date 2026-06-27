export {
  SdkworkAppClient,
  createClient,
  createClient as createCommunityAppSdkClient,
} from 'sdkwork-community-app-sdk-generated-typescript';
export type { SdkworkAppConfig } from 'sdkwork-community-app-sdk-generated-typescript';
export * from 'sdkwork-community-app-sdk-generated-typescript';

export type CommunityAppSdkClient = import('sdkwork-community-app-sdk-generated-typescript').SdkworkAppClient;
