/**
 * WeChat payment OAuth channel backed by the IAM app SDK.
 *
 * Bootstrap code only: binds the IAM `wechatPaymentOAuth.start` endpoint to
 * the `WechatPaymentOAuthChannel` port consumed by the mobile cashier. UI
 * packages never construct HTTP or SDK clients themselves.
 */

import type { SdkworkAppClient as SdkworkIamAppClient } from '@sdkwork/iam-app-sdk';
import type { WechatPaymentOAuthChannel } from '@sdkwork/order-mobile-react-orders';

export function createWechatPaymentOAuthChannel(
  iamAppSdkClient: SdkworkIamAppClient,
): WechatPaymentOAuthChannel {
  return {
    async fetchAuthorizeUrl(redirect: string): Promise<string> {
      const response = await iamAppSdkClient.oauth.wechatPaymentOauth.start({
        redirect,
      });
      const record = (response ?? {}) as { authorizeUrl?: unknown; authUrl?: unknown };
      const authorizeUrl = record.authorizeUrl ?? record.authUrl;
      if (typeof authorizeUrl !== 'string' || authorizeUrl.trim().length === 0) {
        throw new Error('WeChat payment OAuth start did not return an authorizeUrl.');
      }
      return authorizeUrl;
    },
  };
}
