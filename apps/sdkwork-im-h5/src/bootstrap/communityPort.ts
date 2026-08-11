/**
 * IM H5 community (圈子) runtime port wiring.
 *
 * Binds the shared `@sdkwork/community-mobile-react-community` package to the
 * IM host:
 *
 * - importing `@sdkwork/im-h5-community` runs its side effect that binds the
 *   IM auth session port (current-user lookup for the payment sheet);
 * - `configureCommunityRuntimePort` switches the package to the generated
 *   Community App SDK port constructed from the IM gateway base URL and the
 *   shared H5 token manager;
 * - `configureCommunityOrderRuntime` routes circle membership order creation
 *   through the IM-composed order App SDK (`memberships.orders.create`) so
 *   the whole purchase flow settles on sdkwork-order. The official cashier
 *   (`configureOrderMobileRuntime`) is already composed by the host.
 */

import '@sdkwork/im-h5-community';
import {
  configureCommunityOrderRuntime,
  configureCommunityRuntimePort,
  type CreateCircleMembershipOrderOptions,
  type CircleMembershipOrder,
} from '@sdkwork/community-mobile-react-community';
import { getSdkClients } from './sdkClients';

let bootstrapped = false;

export function bootstrapImCommunityH5Port(): void {
  if (bootstrapped) {
    return;
  }
  bootstrapped = true;

  configureCommunityRuntimePort(getSdkClients().communityAppSdkPort);

  configureCommunityOrderRuntime({
    async createMembershipOrder(
      options: CreateCircleMembershipOrderOptions,
    ): Promise<CircleMembershipOrder> {
      const result = await getSdkClients().orderAppSdkClient.memberships.orders.create(
        {
          action: 'purchase',
          packageId: options.packageId,
          paymentMethod: options.paymentMethod,
          paymentProduct: 'mobile_cashier_h5',
          source: options.source ?? 'community-circle',
        },
        { idempotencyKey: crypto.randomUUID() },
      );
      return {
        orderId: result.orderId,
        orderNo: result.orderNo,
        amount: result.amount,
        cashierUrl: result.cashierUrl,
      };
    },
  });
}

export function isImCommunityH5PortBootstrapped(): boolean {
  return bootstrapped;
}
