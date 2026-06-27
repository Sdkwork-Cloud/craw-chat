export {
  SdkworkAppClient,
  createClient,
  createClient as createMailAppSdkClient,
} from 'sdkwork-mail-app-sdk-generated-typescript';
export type { SdkworkAppConfig } from 'sdkwork-mail-app-sdk-generated-typescript';
export * from 'sdkwork-mail-app-sdk-generated-typescript';

export type MailAppSdkClient = import('sdkwork-mail-app-sdk-generated-typescript').SdkworkAppClient;
