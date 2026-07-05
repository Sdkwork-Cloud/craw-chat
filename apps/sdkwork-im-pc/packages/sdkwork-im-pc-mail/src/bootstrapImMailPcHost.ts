import { getMailAppSdkClient } from '@sdkwork/mail-pc-core';
import {
  createMailAppServices,
  type MailAppServices,
} from '@sdkwork/mail-pc-mail';
import { resolveAppSdkBaseUrl } from '@sdkwork/im-pc-core/sdk/appSdkClient';

export function createImHostedMailAppServices(): MailAppServices {
  return createMailAppServices(getMailAppSdkClient(resolveAppSdkBaseUrl()));
}
